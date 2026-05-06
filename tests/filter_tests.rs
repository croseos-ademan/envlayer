use std::collections::HashMap;
use envlayer::filter::Filter;

fn sample_vars() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("APP_HOST".into(), "localhost".into());
    map.insert("APP_PORT".into(), "8080".into());
    map.insert("DB_URL".into(), "postgres://localhost/db".into());
    map.insert("SECRET_KEY".into(), "abc123".into());
    map.insert("LOG_LEVEL".into(), "info".into());
    map
}

#[test]
fn test_no_rules_includes_all() {
    let filter = Filter::new();
    let result = filter.apply(&sample_vars()).unwrap();
    assert_eq!(result.len(), 5);
}

#[test]
fn test_include_prefix() {
    let filter = Filter::new().with_include_prefix("APP_");
    let result = filter.apply(&sample_vars()).unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.contains_key("APP_HOST"));
    assert!(result.contains_key("APP_PORT"));
}

#[test]
fn test_exclude_prefix() {
    let filter = Filter::new().with_exclude_prefix("SECRET_");
    let result = filter.apply(&sample_vars()).unwrap();
    assert!(!result.contains_key("SECRET_KEY"));
    assert_eq!(result.len(), 4);
}

#[test]
fn test_include_exact_key() {
    let filter = Filter::new().with_include_key("LOG_LEVEL");
    let result = filter.apply(&sample_vars()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result["LOG_LEVEL"], "info");
}

#[test]
fn test_exclude_exact_key() {
    let filter = Filter::new().with_exclude_key("DB_URL");
    let result = filter.apply(&sample_vars()).unwrap();
    assert!(!result.contains_key("DB_URL"));
}

#[test]
fn test_include_prefix_with_exclude_key() {
    let filter = Filter::new()
        .with_include_prefix("APP_")
        .with_exclude_key("APP_PORT");
    let result = filter.apply(&sample_vars()).unwrap();
    assert_eq!(result.len(), 1);
    assert!(result.contains_key("APP_HOST"));
}

#[test]
fn test_empty_vars() {
    let filter = Filter::new().with_include_prefix("APP_");
    let result = filter.apply(&HashMap::new()).unwrap();
    assert!(result.is_empty());
}
