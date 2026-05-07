use std::collections::HashMap;
use envlayer::redactor::Redactor;

#[test]
fn test_default_redactor_masks_password() {
    let r = Redactor::with_defaults();
    assert_eq!(r.redact("DB_PASSWORD", "supersecret"), "[REDACTED]");
}

#[test]
fn test_default_redactor_masks_token() {
    let r = Redactor::with_defaults();
    assert_eq!(r.redact("GITHUB_TOKEN", "ghp_abc123"), "[REDACTED]");
}

#[test]
fn test_default_redactor_passes_safe_key() {
    let r = Redactor::with_defaults();
    assert_eq!(r.redact("APP_ENV", "production"), "production");
}

#[test]
fn test_custom_placeholder() {
    let r = Redactor::new("***");
    let mut r2 = r.clone();
    r2.add_key("MY_SECRET");
    assert_eq!(r2.redact("MY_SECRET", "value"), "***");
}

#[test]
fn test_add_custom_key() {
    let mut r = Redactor::new("[HIDDEN]");
    r.add_key("INTERNAL_KEY");
    assert!(r.is_sensitive("INTERNAL_KEY"));
    assert_eq!(r.redact("INTERNAL_KEY", "abc"), "[HIDDEN]");
}

#[test]
fn test_is_sensitive_case_insensitive() {
    let r = Redactor::with_defaults();
    assert!(r.is_sensitive("db_password"));
    assert!(r.is_sensitive("DB_PASSWORD"));
    assert!(r.is_sensitive("DbPassword"));
}

#[test]
fn test_redact_map() {
    let r = Redactor::with_defaults();
    let mut vars = HashMap::new();
    vars.insert("APP_ENV".to_string(), "staging".to_string());
    vars.insert("API_KEY".to_string(), "key-xyz".to_string());
    vars.insert("PORT".to_string(), "8080".to_string());

    let redacted = r.redact_map(&vars);
    assert_eq!(redacted["APP_ENV"], "staging");
    assert_eq!(redacted["API_KEY"], "[REDACTED]");
    assert_eq!(redacted["PORT"], "8080");
}

#[test]
fn test_non_sensitive_key_unchanged() {
    let r = Redactor::with_defaults();
    assert!(!r.is_sensitive("DATABASE_HOST"));
    assert_eq!(r.redact("DATABASE_HOST", "localhost"), "localhost");
}
