#[cfg(test)]
mod tests {
    use envlayer::defaults_builder::DefaultsBuilder;
    use envlayer::env_defaults::EnvDefaults;
    use std::collections::HashMap;

    #[test]
    fn test_builder_with_single_pair() {
        let defaults = DefaultsBuilder::new()
            .with("APP_ENV", "development")
            .build();
        assert_eq!(defaults.get("APP_ENV"), Some(&"development".to_string()));
    }

    #[test]
    fn test_builder_with_multiple_pairs() {
        let defaults = DefaultsBuilder::new()
            .with("HOST", "localhost")
            .with("PORT", "3000")
            .build();
        assert!(defaults.contains("HOST"));
        assert!(defaults.contains("PORT"));
    }

    #[test]
    fn test_builder_with_pairs_iterator() {
        let pairs = vec![("KEY_A", "val_a"), ("KEY_B", "val_b")];
        let defaults = DefaultsBuilder::new().with_pairs(pairs).build();
        assert_eq!(defaults.get("KEY_A"), Some(&"val_a".to_string()));
        assert_eq!(defaults.get("KEY_B"), Some(&"val_b".to_string()));
    }

    #[test]
    fn test_builder_merge_non_overwriting() {
        let mut existing = EnvDefaults::new();
        existing.set("APP_ENV", "staging");
        existing.set("EXTRA", "extra_val");

        let defaults = DefaultsBuilder::new()
            .with("APP_ENV", "development") // set first
            .merge(&existing)              // merge should not overwrite APP_ENV
            .build();

        assert_eq!(defaults.get("APP_ENV"), Some(&"development".to_string()));
        assert_eq!(defaults.get("EXTRA"), Some(&"extra_val".to_string()));
    }

    #[test]
    fn test_builder_apply_to_env() {
        let defaults = DefaultsBuilder::new()
            .with("LOG_LEVEL", "warn")
            .with("TIMEOUT", "30")
            .build();

        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("LOG_LEVEL".into(), "debug".into());
        defaults.apply(&mut env);

        assert_eq!(env["LOG_LEVEL"], "debug");  // not overwritten
        assert_eq!(env["TIMEOUT"], "30");        // filled in
    }

    #[test]
    fn test_builder_empty_produces_empty_defaults() {
        let defaults = DefaultsBuilder::new().build();
        assert!(defaults.all().is_empty());
    }
}
