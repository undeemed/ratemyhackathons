use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    InternalError(String),
    DatabaseError(sqlx::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
            ApiError::DatabaseError(e) => write!(f, "Database Error: {}", e),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse {
                error: "not_found".to_string(),
                message: msg.clone(),
            }),
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                error: "bad_request".to_string(),
                message: msg.clone(),
            }),
            ApiError::Unauthorized(msg) => HttpResponse::Unauthorized().json(ErrorResponse {
                error: "unauthorized".to_string(),
                message: msg.clone(),
            }),
            ApiError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "internal_error".to_string(),
                    message: msg.clone(),
                })
            }
            ApiError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: "An unexpected database error occurred".to_string(),
                })
            }
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        log::error!("Database error: {:?}", e);
        ApiError::DatabaseError(e)
    }
}
