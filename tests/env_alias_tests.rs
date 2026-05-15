use std::collections::HashMap;
use envlayer::env_alias::EnvAlias;
use envlayer::alias_registry::AliasRegistry;

#[test]
fn test_register_and_resolve() {
    let mut alias = EnvAlias::new();
    alias.register("DB_URL", "DATABASE_URL");
    assert_eq!(alias.resolve("DB_URL"), "DATABASE_URL");
}

#[test]
fn test_resolve_unknown_returns_input() {
    let alias = EnvAlias::new();
    assert_eq!(alias.resolve("UNKNOWN_KEY"), "UNKNOWN_KEY");
}

#[test]
fn test_resolve_strict_ok() {
    let mut alias = EnvAlias::new();
    alias.register("PORT", "APP_PORT");
    assert_eq!(alias.resolve_strict("PORT").unwrap(), "APP_PORT");
}

#[test]
fn test_resolve_strict_error() {
    let alias = EnvAlias::new();
    assert!(alias.resolve_strict("MISSING").is_err());
}

#[test]
fn test_is_alias() {
    let mut alias = EnvAlias::new();
    alias.register("HOST", "APP_HOST");
    assert!(alias.is_alias("HOST"));
    assert!(!alias.is_alias("APP_HOST"));
}

#[test]
fn test_remove_alias() {
    let mut alias = EnvAlias::new();
    alias.register("X", "Y");
    assert!(alias.remove("X").is_some());
    assert!(!alias.is_alias("X"));
}

#[test]
fn test_apply_to_map() {
    let mut alias = EnvAlias::new();
    alias.register("DB", "DATABASE_URL");
    let mut map = HashMap::new();
    map.insert("DB".to_string(), "postgres://localhost".to_string());
    map.insert("LOG_LEVEL".to_string(), "info".to_string());
    let resolved = alias.apply_to_map(&map);
    assert_eq!(resolved.get("DATABASE_URL").unwrap(), "postgres://localhost");
    assert_eq!(resolved.get("LOG_LEVEL").unwrap(), "info");
}

#[test]
fn test_registry_register_and_resolve() {
    let mut registry = AliasRegistry::new();
    let mut alias = EnvAlias::new();
    alias.register("PORT", "APP_PORT");
    registry.register("default", alias);
    assert_eq!(registry.resolve("default", "PORT").unwrap(), "APP_PORT");
}

#[test]
fn test_registry_resolve_passthrough() {
    let mut registry = AliasRegistry::new();
    registry.register("default", EnvAlias::new());
    assert_eq!(registry.resolve("default", "MY_KEY").unwrap(), "MY_KEY");
}

#[test]
fn test_registry_missing_namespace_error() {
    let registry = AliasRegistry::new();
    assert!(registry.resolve("nonexistent", "KEY").is_err());
}

#[test]
fn test_registry_namespaces() {
    let mut registry = AliasRegistry::new();
    registry.register("ns1", EnvAlias::new());
    registry.register("ns2", EnvAlias::new());
    let mut ns = registry.namespaces();
    ns.sort();
    assert_eq!(ns, vec!["ns1", "ns2"]);
}
