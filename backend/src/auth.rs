use std::sync::Arc;

use actix_web::{dev::ServiceRequest, web, Error};
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Claims extracted from Clerk JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct ClerkClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    #[serde(default)]
    pub iss: String,
}

/// Authenticated user info attached to request extensions
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub clerk_id: String,
}

/// Cached JWKS with expiry
pub struct JwksCache {
    jwks: Option<JwkSet>,
    fetched_at: Option<std::time::Instant>,
}

impl JwksCache {
    pub fn new() -> Self {
        Self {
            jwks: None,
            fetched_at: None,
        }
    }

    fn is_stale(&self) -> bool {
        match self.fetched_at {
            Some(t) => t.elapsed() > std::time::Duration::from_secs(3600), // 1 hour cache
            None => true,
        }
    }
}

/// Shared auth state
#[derive(Clone)]
pub struct AuthState {
    pub jwks_url: String,
    pub issuer: Option<String>,
    pub cache: Arc<RwLock<JwksCache>>,
}

impl AuthState {
    pub fn new(jwks_url: String, issuer: Option<String>) -> Self {
        Self {
            jwks_url,
            issuer,
            cache: Arc::new(RwLock::new(JwksCache::new())),
        }
    }

    /// Fetch or return cached JWKS
    async fn get_jwks(&self) -> Result<JwkSet, String> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if !cache.is_stale() {
                if let Some(ref jwks) = cache.jwks {
                    return Ok(jwks.clone());
                }
            }
        }

        // Fetch fresh JWKS
        let resp = reqwest::get(&self.jwks_url)
            .await
            .map_err(|e| format!("Failed to fetch JWKS: {}", e))?;

        let jwks: JwkSet = resp.json().await
            .map_err(|e| format!("Failed to parse JWKS: {}", e))?;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.jwks = Some(jwks.clone());
            cache.fetched_at = Some(std::time::Instant::now());
        }

        Ok(jwks)
    }

    /// Verify a JWT token and return the claims
    pub async fn verify_token(&self, token: &str) -> Result<ClerkClaims, String> {
        let header = decode_header(token)
            .map_err(|e| format!("Invalid JWT header: {}", e))?;

        let jwks = self.get_jwks().await?;

        // Find the matching key by kid
        let kid = header.kid.as_deref()
            .ok_or_else(|| "JWT missing kid header".to_string())?;

        let jwk = jwks.find(kid)
            .ok_or_else(|| format!("No matching JWK for kid: {}", kid))?;

        let decoding_key = DecodingKey::from_jwk(jwk)
            .map_err(|e| format!("Invalid JWK: {}", e))?;

        let mut validation = Validation::new(Algorithm::RS256);
        if let Some(ref issuer) = self.issuer {
            validation.set_issuer(&[issuer]);
        }

        let token_data = decode::<ClerkClaims>(token, &decoding_key, &validation)
            .map_err(|e| format!("JWT verification failed: {}", e))?;

        Ok(token_data.claims)
    }
}

/// Extract Bearer token from Authorization header
pub fn extract_bearer_token(req: &ServiceRequest) -> Option<String> {
    req.headers()
        .get("Authorization")?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(|s| s.to_string())
}

/// Look up or create user by clerk_id, returning the internal user_id
pub async fn sync_clerk_user(pool: &PgPool, clerk_id: &str) -> Result<Uuid, sqlx::Error> {
    // Try to find existing user
    let existing: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM users WHERE clerk_id = $1"
    )
    .bind(clerk_id)
    .fetch_optional(pool)
    .await?;

    if let Some((user_id,)) = existing {
        return Ok(user_id);
    }

    // Create new user
    let user_id = Uuid::now_v7();
    sqlx::query(
        r#"
        INSERT INTO users (id, username, email, clerk_id)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (clerk_id) DO UPDATE SET clerk_id = EXCLUDED.clerk_id
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(format!("user_{}", &clerk_id[..8.min(clerk_id.len())]))
    .bind(format!("{}@clerk.placeholder", &clerk_id[..8.min(clerk_id.len())]))
    .bind(clerk_id)
    .execute(pool)
    .await?;

    // In case of race condition, fetch again
    let result: (Uuid,) = sqlx::query_as(
        "SELECT id FROM users WHERE clerk_id = $1"
    )
    .bind(clerk_id)
    .fetch_one(pool)
    .await?;

    Ok(result.0)
}

/// Gate a handler: if Clerk is configured, require a valid JWT; otherwise allow (dev mode).
/// Use this for endpoints that don't need a user_id but must be auth-gated in production.
pub async fn require_auth_if_configured(
    req: &actix_web::HttpRequest,
    auth_state: &Option<web::Data<AuthState>>,
    pool: &PgPool,
) -> Result<Option<AuthenticatedUser>, crate::errors::ApiError> {
    if let Some(state) = auth_state {
        let user = require_auth(req, state, pool).await
            .map_err(|e| crate::errors::ApiError::Unauthorized(e.to_string()))?;
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

/// Middleware-like function to authenticate a request.
/// Call this at the start of protected handlers.
pub async fn require_auth(
    req: &actix_web::HttpRequest,
    auth_state: &AuthState,
    pool: &PgPool,
) -> Result<AuthenticatedUser, Error> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            actix_web::error::ErrorUnauthorized("Missing Authorization header")
        })?;

    let claims = auth_state.verify_token(token).await.map_err(|e| {
        log::warn!("Auth failed: {}", e);
        actix_web::error::ErrorUnauthorized("Invalid or expired token")
    })?;

    let user_id = sync_clerk_user(pool, &claims.sub).await.map_err(|e| {
        log::error!("User sync failed: {}", e);
        actix_web::error::ErrorInternalServerError("Authentication error")
    })?;

    Ok(AuthenticatedUser {
        user_id,
        clerk_id: claims.sub,
    })
}
