use envlayer::mask::{MaskStrategy, Masker};

#[test]
test_full_mask_replaces_entire_value() {
    let masker = Masker::new(MaskStrategy::Full);
    let masked = masker.mask_value("supersecret");
    assert!(!masked.contains('s'));
    assert!(masked.chars().all(|c| c == '*'));
    assert_eq!(masked.len(), "supersecret".len());
}

#[test]
fn test_full_mask_minimum_length() {
    let masker = Masker::new(MaskStrategy::Full);
    // Short values should still produce at least 8 asterisks.
    let masked = masker.mask_value("hi");
    assert_eq!(masked, "********");
}

#[test]
fn test_partial_mask_shows_last_n_chars() {
    let masker = Masker::new(MaskStrategy::Partial(4));
    let masked = masker.mask_value("abcdefgh");
    assert!(masked.ends_with("efgh"));
    assert!(masked.starts_with("****"));
    assert_eq!(masked.len(), 8);
}

#[test]
fn test_partial_mask_short_value() {
    let masker = Masker::new(MaskStrategy::Partial(4));
    // Value shorter than show window → full mask placeholder.
    let masked = masker.mask_value("abc");
    assert_eq!(masked, "********");
}

#[test]
fn test_placeholder_mask() {
    let masker = Masker::new(MaskStrategy::Placeholder("[REDACTED]".to_string()));
    let masked = masker.mask_value("my-token-123");
    assert_eq!(masked, "[REDACTED]");
}

#[test]
fn test_add_sensitive_key() {
    let mut masker = Masker::new(MaskStrategy::Full);
    masker.add_sensitive_key("API_KEY");
    assert!(masker.is_sensitive("API_KEY"));
    assert!(!masker.is_sensitive("PATH"));
}

#[test]
fn test_maybe_mask_sensitive_key() {
    let mut masker = Masker::new(MaskStrategy::Placeholder("***".to_string()));
    masker.add_sensitive_key("SECRET_TOKEN");
    let result = masker.maybe_mask("SECRET_TOKEN", "abc123");
    assert_eq!(result, "***");
}

#[test]
fn test_maybe_mask_non_sensitive_key() {
    let mut masker = Masker::new(MaskStrategy::Full);
    masker.add_sensitive_key("SECRET_TOKEN");
    let result = masker.maybe_mask("APP_ENV", "production");
    assert_eq!(result, "production");
}

#[test]
fn test_default_strategy_is_full() {
    let masker = Masker::default();
    assert_eq!(masker.mask_value("value").chars().all(|c| c == '*'), true);
}
