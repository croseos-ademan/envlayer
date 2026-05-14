use envlayer::scope_builder::ScopeBuilder;
use envlayer::scope_registry::ScopeRegistry;

#[test]
fn test_register_and_get() {
    let mut registry = ScopeRegistry::new();
    let scope = ScopeBuilder::new("APP")
        .var("ENV", "production")
        .build();
    registry.register(scope).unwrap();
    let retrieved = registry.get("APP").unwrap();
    assert_eq!(retrieved.get("ENV"), Some(&"production".to_string()));
}

#[test]
fn test_register_duplicate_prefix_errors() {
    let mut registry = ScopeRegistry::new();
    let s1 = ScopeBuilder::new("APP").var("KEY", "val1").build();
    let s2 = ScopeBuilder::new("APP").var("KEY", "val2").build();
    registry.register(s1).unwrap();
    assert!(registry.register(s2).is_err());
}

#[test]
fn test_remove_scope() {
    let mut registry = ScopeRegistry::new();
    let scope = ScopeBuilder::new("DB").var("HOST", "localhost").build();
    registry.register(scope).unwrap();
    let removed = registry.remove("DB");
    assert!(removed.is_some());
    assert!(registry.get("DB").is_none());
}

#[test]
fn test_flatten_all_scopes() {
    let mut registry = ScopeRegistry::new();
    registry
        .register(ScopeBuilder::new("APP").var("ENV", "staging").build())
        .unwrap();
    registry
        .register(ScopeBuilder::new("DB").var("PORT", "5432").build())
        .unwrap();
    let flat = registry.flatten();
    assert_eq!(flat.get("APP_ENV"), Some(&"staging".to_string()));
    assert_eq!(flat.get("DB_PORT"), Some(&"5432".to_string()));
}

#[test]
fn test_len_and_is_empty() {
    let mut registry = ScopeRegistry::new();
    assert!(registry.is_empty());
    registry
        .register(ScopeBuilder::new("X").build())
        .unwrap();
    assert_eq!(registry.len(), 1);
}

#[test]
fn test_get_mut_and_update() {
    let mut registry = ScopeRegistry::new();
    registry
        .register(ScopeBuilder::new("SVC").var("TIMEOUT", "30").build())
        .unwrap();
    if let Some(scope) = registry.get_mut("SVC") {
        scope.set("TIMEOUT", "60");
    }
    assert_eq!(
        registry.get("SVC").unwrap().get("TIMEOUT"),
        Some(&"60".to_string())
    );
}
