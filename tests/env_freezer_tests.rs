use std::collections::HashMap;
use envlayer::env_freezer::EnvFreezer;

fn sample_vars() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("APP_ENV".to_string(), "production".to_string());
    m.insert("LOG_LEVEL".to_string(), "info".to_string());
    m
}

#[test]
fn test_get_existing_key() {
    let freezer = EnvFreezer::new(sample_vars());
    assert_eq!(freezer.get("APP_ENV"), Some("production"));
}

#[test]
fn test_get_missing_key_returns_none() {
    let freezer = EnvFreezer::new(sample_vars());
    assert!(freezer.get("MISSING_KEY").is_none());
}

#[test]
fn test_set_before_freeze_succeeds() {
    let mut freezer = EnvFreezer::new(sample_vars());
    assert!(freezer.set("NEW_KEY", "new_value").is_ok());
    assert_eq!(freezer.get("NEW_KEY"), Some("new_value"));
}

#[test]
fn test_set_after_freeze_fails() {
    let mut freezer = EnvFreezer::new(sample_vars());
    freezer.freeze();
    let result = freezer.set("NEW_KEY", "new_value");
    assert!(result.is_err());
}

#[test]
fn test_remove_before_freeze_succeeds() {
    let mut freezer = EnvFreezer::new(sample_vars());
    let removed = freezer.remove("LOG_LEVEL").unwrap();
    assert_eq!(removed, Some("info".to_string()));
    assert!(freezer.get("LOG_LEVEL").is_none());
}

#[test]
fn test_remove_after_freeze_fails() {
    let mut freezer = EnvFreezer::new(sample_vars());
    freezer.freeze();
    let result = freezer.remove("LOG_LEVEL");
    assert!(result.is_err());
}

#[test]
fn test_is_frozen_reflects_state() {
    let mut freezer = EnvFreezer::new(sample_vars());
    assert!(!freezer.is_frozen());
    freezer.freeze();
    assert!(freezer.is_frozen());
}

#[test]
fn test_thaw_allows_mutation_again() {
    let mut freezer = EnvFreezer::new(sample_vars());
    freezer.freeze();
    freezer.thaw();
    assert!(!freezer.is_frozen());
    assert!(freezer.set("AFTER_THAW", "yes").is_ok());
    assert_eq!(freezer.get("AFTER_THAW"), Some("yes"));
}

#[test]
fn test_snapshot_returns_full_copy() {
    let mut freezer = EnvFreezer::new(sample_vars());
    freezer.freeze();
    let snap = freezer.snapshot();
    assert_eq!(snap.get("APP_ENV").map(String::as_str), Some("production"));
    assert_eq!(snap.len(), 2);
}
