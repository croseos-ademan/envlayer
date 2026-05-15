use envlayer::env_namespace::EnvNamespace;
use envlayer::namespace_builder::NamespaceBuilder;
use envlayer::namespace_registry::NamespaceRegistry;

#[test]
fn test_register_and_get() {
    let mut registry = NamespaceRegistry::new();
    registry.register("app", EnvNamespace::new("APP"));
    assert!(registry.get("app").is_some());
    assert_eq!(registry.get("app").unwrap().prefix(), "APP");
}

#[test]
fn test_get_missing_returns_none() {
    let registry = NamespaceRegistry::new();
    assert!(registry.get("missing").is_none());
}

#[test]
fn test_remove_namespace() {
    let mut registry = NamespaceRegistry::new();
    registry.register("ci", EnvNamespace::new("CI"));
    assert!(registry.remove("ci").is_some());
    assert!(!registry.contains("ci"));
}

#[test]
fn test_list_namespaces() {
    let mut registry = NamespaceRegistry::new();
    registry.register("app", EnvNamespace::new("APP"));
    registry.register("db", EnvNamespace::new("DB"));
    let mut list = registry.list();
    list.sort();
    assert_eq!(list, vec!["app", "db"]);
}

#[test]
fn test_resolve_qualified_key() {
    let mut registry = NamespaceRegistry::new();
    registry.register("app", EnvNamespace::new("APP"));
    let qualified = registry.resolve("app", "PORT").unwrap();
    assert_eq!(qualified, "APP_PORT");
}

#[test]
fn test_resolve_unknown_namespace_errors() {
    let registry = NamespaceRegistry::new();
    assert!(registry.resolve("unknown", "KEY").is_err());
}

#[test]
fn test_builder_creates_registry() {
    let registry = NamespaceBuilder::new()
        .add("app", "APP")
        .add("ci", "CI")
        .add_with_separator("dot", "DOT", ".")
        .build();
    assert!(registry.contains("app"));
    assert!(registry.contains("ci"));
    assert!(registry.contains("dot"));
    assert_eq!(
        registry.get("dot").unwrap().qualify("KEY"),
        "DOT.KEY"
    );
}
