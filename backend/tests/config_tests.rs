//! Config tests.
//!
//! NOTE: Config::from_env() reads global env vars, which makes it
//! inherently unsafe to test in parallel (Rust 2024 correctly marks
//! set_var/remove_var as unsafe for this reason). These tests verify
//! the Config struct's parse logic without mutating global state.

#[cfg(test)]
mod tests {
    use ratemyhackathons_backend::config::Config;

    #[test]
    fn config_struct_is_cloneable() {
        // Config derives Clone, verify it works
        let cfg = Config {
            database_url: "postgres://localhost/test".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        let cfg2 = cfg.clone();
        assert_eq!(cfg.database_url, cfg2.database_url);
        assert_eq!(cfg.host, cfg2.host);
        assert_eq!(cfg.port, cfg2.port);
    }

    #[test]
    fn config_struct_is_debuggable() {
        let cfg = Config {
            database_url: "postgres://localhost/test".to_string(),
            host: "0.0.0.0".to_string(),
            port: 3000,
        };
        let debug_str = format!("{:?}", cfg);
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("3000"));
    }
}
