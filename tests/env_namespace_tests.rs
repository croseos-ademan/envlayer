use std::collections::HashMap;
use envlayer::env_namespace::EnvNamespace;

#[test]
fn test_qualify_key() {
    let ns = EnvNamespace::new("APP");
    assert_eq!(ns.qualify("DATABASE_URL"), "APP_DATABASE_URL");
}

#[test]
fn test_qualify_with_custom_separator() {
    let ns = EnvNamespace::new("APP").with_separator(".");
    assert_eq!(ns.qualify("HOST"), "APP.HOST");
}

#[test]
fn test_strip_prefix_valid() {
    let ns = EnvNamespace::new("APP");
    assert_eq!(ns.strip("APP_HOST"), Some("HOST".to_string()));
}

#[test]
fn test_strip_prefix_invalid() {
    let ns = EnvNamespace::new("APP");
    assert_eq!(ns.strip("OTHER_HOST"), None);
}

#[test]
fn test_apply_to_map() {
    let ns = EnvNamespace::new("CI");
    let mut map = HashMap::new();
    map.insert("BUILD".to_string(), "1".to_string());
    map.insert("BRANCH".to_string(), "main".to_string());
    let result = ns.apply_to_map(&map);
    assert!(result.contains_key("CI_BUILD"));
    assert!(result.contains_key("CI_BRANCH"));
    assert_eq!(result.get("CI_BUILD").unwrap(), "1");
}

#[test]
fn test_extract_from_map() {
    let ns = EnvNamespace::new("DB");
    let mut map = HashMap::new();
    map.insert("DB_HOST".to_string(), "localhost".to_string());
    map.insert("DB_PORT".to_string(), "5432".to_string());
    map.insert("APP_NAME".to_string(), "myapp".to_string());
    let result = ns.extract_from_map(&map).unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result.get("HOST").unwrap(), "localhost");
    assert_eq!(result.get("PORT").unwrap(), "5432");
    assert!(!result.contains_key("APP_NAME"));
}

#[test]
fn test_prefix_accessor() {
    let ns = EnvNamespace::new("TEST");
    assert_eq!(ns.prefix(), "TEST");
    assert_eq!(ns.separator(), "_");
}
