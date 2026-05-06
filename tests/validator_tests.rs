use envlayer::validator::{ValidationRule, Validator};
use std::collections::HashMap;

fn make_vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[test]
test_non_empty_passes() {
    let mut v = Validator::new();
    v.add_rule("APP_NAME", ValidationRule::NonEmpty);
    let vars = make_vars(&[("APP_NAME", "myapp")]);
    assert!(v.validate(&vars).is_ok());
}

#[test]
fn test_non_empty_fails_on_empty_string() {
    let mut v = Validator::new();
    v.add_rule("APP_NAME", ValidationRule::NonEmpty);
    let vars = make_vars(&[("APP_NAME", "")]);
    assert!(v.validate(&vars).is_err());
}

#[test]
fn test_non_empty_fails_when_key_missing() {
    let mut v = Validator::new();
    v.add_rule("MISSING_KEY", ValidationRule::NonEmpty);
    let vars = make_vars(&[]);
    assert!(v.validate(&vars).is_err());
}

#[test]
fn test_one_of_passes() {
    let mut v = Validator::new();
    v.add_rule(
        "ENV",
        ValidationRule::OneOf(vec!["dev".into(), "staging".into(), "prod".into()]),
    );
    let vars = make_vars(&[("ENV", "staging")]);
    assert!(v.validate(&vars).is_ok());
}

#[test]
fn test_one_of_fails() {
    let mut v = Validator::new();
    v.add_rule(
        "ENV",
        ValidationRule::OneOf(vec!["dev".into(), "prod".into()]),
    );
    let vars = make_vars(&[("ENV", "unknown")]);
    assert!(v.validate(&vars).is_err());
}

#[test]
fn test_integer_passes() {
    let mut v = Validator::new();
    v.add_rule("PORT", ValidationRule::Integer);
    let vars = make_vars(&[("PORT", "8080")]);
    assert!(v.validate(&vars).is_ok());
}

#[test]
fn test_integer_fails_on_non_numeric() {
    let mut v = Validator::new();
    v.add_rule("PORT", ValidationRule::Integer);
    let vars = make_vars(&[("PORT", "eighty")]);
    assert!(v.validate(&vars).is_err());
}

#[test]
fn test_length_range_passes() {
    let mut v = Validator::new();
    v.add_rule("TOKEN", ValidationRule::LengthRange(8, 64));
    let vars = make_vars(&[("TOKEN", "abcd1234")]);
    assert!(v.validate(&vars).is_ok());
}

#[test]
fn test_length_range_fails_too_short() {
    let mut v = Validator::new();
    v.add_rule("TOKEN", ValidationRule::LengthRange(8, 64));
    let vars = make_vars(&[("TOKEN", "abc")]);
    assert!(v.validate(&vars).is_err());
}

#[test]
fn test_multiple_rules_combined_error_message() {
    let mut v = Validator::new();
    v.add_rule("PORT", ValidationRule::NonEmpty);
    v.add_rule("PORT", ValidationRule::Integer);
    let vars = make_vars(&[("PORT", "not-a-number")]);
    let err = v.validate(&vars).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Validation failed"));
    assert!(msg.contains("PORT"));
}
