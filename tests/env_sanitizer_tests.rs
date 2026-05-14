use envlayer::env_sanitizer::{EnvSanitizer, SanitizerConfig};
use std::collections::HashMap;

fn make_env(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
test_sanitize_key_normalizes_special_chars() {
    let san = EnvSanitizer::new(SanitizerConfig::default());
    let result = san.sanitize_key("my-key.name").unwrap();
    assert_eq!(result, "MY_KEY_NAME");
}

#[test]
fn test_sanitize_key_uppercase() {
    let san = EnvSanitizer::new(SanitizerConfig::default());
    let result = san.sanitize_key("database_url").unwrap();
    assert_eq!(result, "DATABASE_URL");
}

#[test]
fn test_sanitize_key_empty_returns_error() {
    let san = EnvSanitizer::new(SanitizerConfig::default());
    assert!(san.sanitize_key("").is_err());
}

#[test]
fn test_sanitize_value_trims_whitespace() {
    let san = EnvSanitizer::new(SanitizerConfig::default());
    assert_eq!(san.sanitize_value("  hello  "), "hello");
}

#[test]
fn test_sanitize_value_no_trim_when_disabled() {
    let config = SanitizerConfig {
        trim_values: false,
        ..Default::default()
    };
    let san = EnvSanitizer::new(config);
    assert_eq!(san.sanitize_value("  hello  "), "  hello  ");
}

#[test]
fn test_sanitize_drops_empty_values_when_configured() {
    let config = SanitizerConfig {
        drop_empty_values: true,
        ..Default::default()
    };
    let san = EnvSanitizer::new(config);
    let env = make_env(&[("KEY_A", "value"), ("KEY_B", "   ")]);
    let result = san.sanitize(&env).unwrap();
    assert!(result.contains_key("KEY_A"));
    assert!(!result.contains_key("KEY_B"));
}

#[test]
fn test_sanitize_keeps_empty_values_by_default() {
    let san = EnvSanitizer::new(SanitizerConfig::default());
    let env = make_env(&[("KEY_A", "")]);
    let result = san.sanitize(&env).unwrap();
    assert!(result.contains_key("KEY_A"));
}

#[test]
fn test_sanitize_full_map() {
    let san = EnvSanitizer::new(SanitizerConfig::default());
    let env = make_env(&[("app-name", "  myapp  "), ("PORT", "8080")]);
    let result = san.sanitize(&env).unwrap();
    assert_eq!(result.get("APP_NAME").map(String::as_str), Some("myapp"));
    assert_eq!(result.get("PORT").map(String::as_str), Some("8080"));
}

#[test]
fn test_sanitize_no_normalize_keys() {
    let config = SanitizerConfig {
        normalize_keys: false,
        uppercase_keys: false,
        ..Default::default()
    };
    let san = EnvSanitizer::new(config);
    let result = san.sanitize_key("my-key").unwrap();
    assert_eq!(result, "my-key");
}
