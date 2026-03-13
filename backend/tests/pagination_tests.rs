#[cfg(test)]
mod tests {
    use ratemyhackathons_backend::routes::PaginationParams;

    #[test]
    fn default_pagination() {
        let params = PaginationParams {
            page: None,
            per_page: None,
        };
        assert_eq!(params.page(), 1);
        assert_eq!(params.limit(), 20);
        assert_eq!(params.offset(), 0);
    }

    #[test]
    fn custom_pagination() {
        let params = PaginationParams {
            page: Some(3),
            per_page: Some(50),
        };
        assert_eq!(params.page(), 3);
        assert_eq!(params.limit(), 50);
        assert_eq!(params.offset(), 100); // (3-1) * 50
    }

    #[test]
    fn page_clamps_to_minimum_1() {
        let params = PaginationParams {
            page: Some(0),
            per_page: None,
        };
        assert_eq!(params.page(), 1);
        assert_eq!(params.offset(), 0);

        let params_negative = PaginationParams {
            page: Some(-5),
            per_page: None,
        };
        assert_eq!(params_negative.page(), 1);
    }

    #[test]
    fn per_page_clamps_to_max_100() {
        let params = PaginationParams {
            page: Some(1),
            per_page: Some(500),
        };
        assert_eq!(params.limit(), 100);
    }

    #[test]
    fn offset_calculation_page_2() {
        let params = PaginationParams {
            page: Some(2),
            per_page: Some(25),
        };
        assert_eq!(params.offset(), 25); // (2-1) * 25
    }

    #[test]
    fn offset_calculation_large_page() {
        let params = PaginationParams {
            page: Some(10),
            per_page: Some(20),
        };
        assert_eq!(params.offset(), 180); // (10-1) * 20
    }
}
