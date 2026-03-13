#[cfg(test)]
mod tests {
    use actix_web::body::to_bytes;
    use actix_web::ResponseError;
    use ratemyhackathons_backend::errors::ApiError;

    #[test]
    fn not_found_returns_404() {
        let err = ApiError::NotFound("Event xyz not found".to_string());
        let response = err.error_response();
        assert_eq!(response.status(), 404);
    }

    #[test]
    fn bad_request_returns_400() {
        let err = ApiError::BadRequest("Invalid rating".to_string());
        let response = err.error_response();
        assert_eq!(response.status(), 400);
    }

    #[test]
    fn internal_error_returns_500() {
        let err = ApiError::InternalError("Something broke".to_string());
        let response = err.error_response();
        assert_eq!(response.status(), 500);
    }

    #[actix_rt::test]
    async fn not_found_body_contains_error_type() {
        let err = ApiError::NotFound("Event abc not found".to_string());
        let response = err.error_response();
        let body = to_bytes(response.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["error"], "not_found");
        assert_eq!(json["message"], "Event abc not found");
    }

    #[actix_rt::test]
    async fn bad_request_body_contains_error_type() {
        let err = ApiError::BadRequest("Rating must be 1-5".to_string());
        let response = err.error_response();
        let body = to_bytes(response.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["error"], "bad_request");
        assert_eq!(json["message"], "Rating must be 1-5");
    }

    #[actix_rt::test]
    async fn database_error_hides_details() {
        // Database errors should NOT expose internal details to the client
        let sqlx_err = sqlx::Error::RowNotFound;
        let err = ApiError::from(sqlx_err);
        let response = err.error_response();
        assert_eq!(response.status(), 500);

        let body = to_bytes(response.into_body()).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["error"], "database_error");
        assert_eq!(json["message"], "An unexpected database error occurred");
    }

    #[test]
    fn display_impl_works() {
        let err = ApiError::NotFound("test".to_string());
        assert_eq!(format!("{}", err), "Not Found: test");

        let err = ApiError::BadRequest("bad".to_string());
        assert_eq!(format!("{}", err), "Bad Request: bad");
    }
}
