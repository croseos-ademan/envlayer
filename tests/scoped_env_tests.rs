use envlayer::scoped_env::ScopedEnv;
use std::collections::HashMap;

fn make_scope() -> ScopedEnv {
    let mut vars = HashMap::new();
    vars.insert("HOST".to_string(), "localhost".to_string());
    vars.insert("PORT".to_string(), "5432".to_string());
    ScopedEnv::new("DB", vars)
}

#[test]
fn test_get_existing_key() {
    let scope = make_scope();
    assert_eq!(scope.get("HOST"), Some(&"localhost".to_string()));
}

#[test]
fn test_get_missing_key() {
    let scope = make_scope();
    assert!(scope.get("USER").is_none());
}

#[test]
fn test_set_and_get() {
    let mut scope = make_scope();
    scope.set("USER", "admin");
    assert_eq!(scope.get("USER"), Some(&"admin".to_string()));
}

#[test]
fn test_remove() {
    let mut scope = make_scope();
    let removed = scope.remove("HOST");
    assert_eq!(removed, Some("localhost".to_string()));
    assert!(scope.get("HOST").is_none());
}

#[test]
fn test_prefixed_vars() {
    let scope = make_scope();
    let prefixed = scope.prefixed_vars();
    assert_eq!(prefixed.get("DB_HOST"), Some(&"localhost".to_string()));
    assert_eq!(prefixed.get("DB_PORT"), Some(&"5432".to_string()));
}

#[test]
fn test_merge_same_prefix() {
    let mut base = make_scope();
    let mut extra_vars = HashMap::new();
    extra_vars.insert("NAME".to_string(), "mydb".to_string());
    extra_vars.insert("PORT".to_string(), "5433".to_string());
    let extra = ScopedEnv::new("DB", extra_vars);
    base.merge(&extra).unwrap();
    assert_eq!(base.get("NAME"), Some(&"mydb".to_string()));
    assert_eq!(base.get("PORT"), Some(&"5433".to_string()));
}

#[test]
fn test_merge_different_prefix_errors() {
    let mut base = make_scope();
    let other = ScopedEnv::new("APP", HashMap::new());
    assert!(base.merge(&other).is_err());
}

#[test]
fn test_len_and_is_empty() {
    let scope = make_scope();
    assert_eq!(scope.len(), 2);
    assert!(!scope.is_empty());
    let empty = ScopedEnv::new("EMPTY", HashMap::new());
    assert!(empty.is_empty());
}
