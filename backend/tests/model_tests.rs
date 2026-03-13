#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use ratemyhackathons_backend::models::event::*;
    use ratemyhackathons_backend::models::company::*;
    use ratemyhackathons_backend::models::user::*;
    use ratemyhackathons_backend::models::review::*;

    // ── Event model tests ──

    #[test]
    fn create_event_serializes_correctly() {
        let json = r#"{
            "name": "HackMIT 2025",
            "description": "Annual hackathon",
            "location": "Cambridge, MA",
            "start_date": "2025-10-01",
            "end_date": "2025-10-02"
        }"#;

        let event: CreateEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.name, "HackMIT 2025");
        assert_eq!(event.description, Some("Annual hackathon".to_string()));
        assert_eq!(event.location, Some("Cambridge, MA".to_string()));
        assert!(event.start_date.is_some());
        assert!(event.end_date.is_some());
        assert!(event.company_ids.is_none());
        assert!(event.url.is_none());
        assert!(event.image_url.is_none());
    }

    #[test]
    fn create_event_minimal_fields() {
        let json = r#"{"name": "Quick Hack"}"#;
        let event: CreateEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.name, "Quick Hack");
        assert!(event.description.is_none());
        assert!(event.company_ids.is_none());
    }

    #[test]
    fn create_event_with_company_ids() {
        let id1 = Uuid::now_v7();
        let id2 = Uuid::now_v7();
        let json = format!(
            r#"{{"name": "Hack", "company_ids": ["{}", "{}"]}}"#,
            id1, id2
        );
        let event: CreateEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event.company_ids.unwrap().len(), 2);
    }

    #[test]
    fn create_event_missing_name_fails() {
        let json = r#"{"description": "No name"}"#;
        let result = serde_json::from_str::<CreateEvent>(json);
        assert!(result.is_err());
    }

    #[test]
    fn event_summary_serializes_to_json() {
        let summary = EventSummary {
            id: Uuid::now_v7(),
            name: "Test Event".to_string(),
            description: None,
            location: Some("NYC".to_string()),
            url: None,
            start_date: None,
            end_date: None,
            image_url: None,
            companies: vec![],
            avg_rating: Some(4.5),
            review_count: 10,
            created_at: Utc::now(),
        };

        let json = serde_json::to_value(&summary).unwrap();
        assert_eq!(json["name"], "Test Event");
        assert_eq!(json["avg_rating"], 4.5);
        assert_eq!(json["review_count"], 10);
        assert!(json["description"].is_null());
    }

    // ── Company model tests ──

    #[test]
    fn create_company_deserializes() {
        let json = r#"{
            "name": "Google",
            "website": "https://google.com",
            "description": "Tech giant"
        }"#;

        let company: CreateCompany = serde_json::from_str(json).unwrap();
        assert_eq!(company.name, "Google");
        assert_eq!(company.website, Some("https://google.com".to_string()));
        assert!(company.logo_url.is_none());
    }

    #[test]
    fn create_company_minimal() {
        let json = r#"{"name": "Startup Inc"}"#;
        let company: CreateCompany = serde_json::from_str(json).unwrap();
        assert_eq!(company.name, "Startup Inc");
    }

    #[test]
    fn create_company_missing_name_fails() {
        let json = r#"{"website": "https://example.com"}"#;
        let result = serde_json::from_str::<CreateCompany>(json);
        assert!(result.is_err());
    }

    // ── User model tests ──

    #[test]
    fn create_user_deserializes() {
        let json = r#"{
            "username": "alice",
            "email": "alice@example.com",
            "avatar_url": "https://avatar.com/alice.png"
        }"#;

        let user: CreateUser = serde_json::from_str(json).unwrap();
        assert_eq!(user.username, "alice");
        assert_eq!(user.email, "alice@example.com");
        assert_eq!(user.avatar_url, Some("https://avatar.com/alice.png".to_string()));
    }

    #[test]
    fn create_user_missing_email_fails() {
        let json = r#"{"username": "bob"}"#;
        let result = serde_json::from_str::<CreateUser>(json);
        assert!(result.is_err());
    }

    #[test]
    fn create_user_missing_username_fails() {
        let json = r#"{"email": "bob@example.com"}"#;
        let result = serde_json::from_str::<CreateUser>(json);
        assert!(result.is_err());
    }

    // ── Review model tests ──

    #[test]
    fn create_review_deserializes() {
        let event_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let json = format!(
            r#"{{
                "event_id": "{}",
                "user_id": "{}",
                "rating": 5,
                "title": "Amazing!",
                "body": "Best hackathon ever"
            }}"#,
            event_id, user_id
        );

        let review: CreateReview = serde_json::from_str(&json).unwrap();
        assert_eq!(review.event_id, event_id);
        assert_eq!(review.user_id, user_id);
        assert_eq!(review.rating, 5);
        assert_eq!(review.title, Some("Amazing!".to_string()));
    }

    #[test]
    fn create_review_minimal() {
        let event_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let json = format!(
            r#"{{"event_id": "{}", "user_id": "{}", "rating": 3}}"#,
            event_id, user_id
        );

        let review: CreateReview = serde_json::from_str(&json).unwrap();
        assert_eq!(review.rating, 3);
        assert!(review.title.is_none());
        assert!(review.body.is_none());
    }

    #[test]
    fn create_review_missing_rating_fails() {
        let json = format!(
            r#"{{"event_id": "{}", "user_id": "{}"}}"#,
            Uuid::now_v7(),
            Uuid::now_v7()
        );
        let result = serde_json::from_str::<CreateReview>(&json);
        assert!(result.is_err());
    }

    // ── UUID v7 ordering test ──

    #[test]
    fn uuid_v7_is_time_ordered() {
        let id1 = Uuid::now_v7();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let id2 = Uuid::now_v7();

        // v7 UUIDs sort chronologically
        assert!(id1 < id2, "UUIDv7 should be time-ordered: {} < {}", id1, id2);
    }
}
