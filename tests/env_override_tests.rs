use std::collections::HashMap;
use envlayer::env_override::{EnvOverride, OverrideSet};

fn base_env() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("APP_ENV".to_string(), "development".to_string());
    m.insert("LOG_LEVEL".to_string(), "info".to_string());
    m.insert("PORT".to_string(), "8080".to_string());
    m
}

#[test]
fn test_env_override_new() {
    let o = EnvOverride::new("KEY", "VALUE");
    assert_eq!(o.key, "KEY");
    assert_eq!(o.value, "VALUE");
    assert!(o.reason.is_none());
}

#[test]
fn test_env_override_with_reason() {
    let o = EnvOverride::new("KEY", "VALUE").with_reason("CI override");
    assert_eq!(o.reason.as_deref(), Some("CI override"));
}

#[test]
fn test_override_set_apply_adds_new_key() {
    let base = base_env();
    let mut set = OverrideSet::new();
    set.add(EnvOverride::new("NEW_KEY", "new_value"));
    let result = set.apply(&base).unwrap();
    assert_eq!(result.get("NEW_KEY").map(String::as_str), Some("new_value"));
    assert_eq!(result.get("APP_ENV").map(String::as_str), Some("development"));
}

#[test]
fn test_override_set_apply_overrides_existing_key() {
    let base = base_env();
    let mut set = OverrideSet::new();
    set.add(EnvOverride::new("APP_ENV", "production"));
    let result = set.apply(&base).unwrap();
    assert_eq!(result.get("APP_ENV").map(String::as_str), Some("production"));
}

#[test]
fn test_override_set_apply_empty_key_returns_error() {
    let base = base_env();
    let mut set = OverrideSet::new();
    set.add(EnvOverride::new("", "value"));
    assert!(set.apply(&base).is_err());
}

#[test]
fn test_override_set_from_map() {
    let mut map = HashMap::new();
    map.insert("A".to_string(), "1".to_string());
    map.insert("B".to_string(), "2".to_string());
    let set = OverrideSet::from_map(map);
    assert_eq!(set.len(), 2);
}

#[test]
fn test_effective_filters_unchanged_keys() {
    let base = base_env();
    let mut set = OverrideSet::new();
    set.add(EnvOverride::new("APP_ENV", "development")); // same value
    set.add(EnvOverride::new("LOG_LEVEL", "debug"));     // changed
    set.add(EnvOverride::new("NEW_KEY", "val"));          // new
    let effective = set.effective(&base);
    assert_eq!(effective.len(), 2);
    assert!(effective.iter().any(|e| e.key == "LOG_LEVEL"));
    assert!(effective.iter().any(|e| e.key == "NEW_KEY"));
}

#[test]
fn test_override_set_is_empty() {
    let set = OverrideSet::new();
    assert!(set.is_empty());
}
