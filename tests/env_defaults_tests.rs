#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use envlayer::env_defaults::EnvDefaults;

    fn make_defaults() -> EnvDefaults {
        let mut d = EnvDefaults::new();
        d.set("APP_ENV", "development");
        d.set("LOG_LEVEL", "info");
        d
    }

    #[test]
    fn test_get_existing_default() {
        let d = make_defaults();
        assert_eq!(d.get("APP_ENV"), Some(&"development".to_string()));
    }

    #[test]
    fn test_get_missing_default() {
        let d = make_defaults();
        assert!(d.get("UNKNOWN_KEY").is_none());
    }

    #[test]
    fn test_apply_fills_missing_keys() {
        let d = make_defaults();
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("APP_ENV".into(), "production".into());
        d.apply(&mut env);
        // Existing key should not be overwritten
        assert_eq!(env["APP_ENV"], "production");
        // Missing key should be filled with default
        assert_eq!(env["LOG_LEVEL"], "info");
    }

    #[test]
    fn test_apply_empty_env() {
        let d = make_defaults();
        let mut env: HashMap<String, String> = HashMap::new();
        d.apply(&mut env);
        assert_eq!(env["APP_ENV"], "development");
        assert_eq!(env["LOG_LEVEL"], "info");
    }

    #[test]
    fn test_remove_default() {
        let mut d = make_defaults();
        let removed = d.remove("LOG_LEVEL");
        assert_eq!(removed, Some("info".to_string()));
        assert!(!d.contains("LOG_LEVEL"));
    }

    #[test]
    fn test_contains() {
        let d = make_defaults();
        assert!(d.contains("APP_ENV"));
        assert!(!d.contains("MISSING"));
    }

    #[test]
    fn test_merge_from_does_not_overwrite() {
        let mut base = make_defaults();
        let mut other = EnvDefaults::new();
        other.set("APP_ENV", "staging"); // should NOT overwrite
        other.set("PORT", "8080");       // should be added
        base.merge_from(&other);
        assert_eq!(base.get("APP_ENV"), Some(&"development".to_string()));
        assert_eq!(base.get("PORT"), Some(&"8080".to_string()));
    }

    #[test]
    fn test_load_pairs_ok() {
        let mut d = EnvDefaults::new();
        let result = d.load_pairs(&[("DB_HOST", "localhost"), ("DB_PORT", "5432")]);
        assert!(result.is_ok());
        assert_eq!(d.get("DB_HOST"), Some(&"localhost".to_string()));
    }

    #[test]
    fn test_load_pairs_empty_key_error() {
        let mut d = EnvDefaults::new();
        let result = d.load_pairs(&[("", "value")]);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_returns_clone() {
        let d = make_defaults();
        let all = d.all();
        assert_eq!(all.len(), 2);
    }
}
