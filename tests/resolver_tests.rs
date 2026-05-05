use envlayer::{
    error::EnvLayerError,
    layer::Layer,
    registry::Registry,
    resolver::Resolver,
};

fn make_registry() -> Registry {
    let mut registry = Registry::new();

    let mut base = Layer::new("base", 0);
    base.set("APP_ENV", "development");
    base.set("LOG_LEVEL", "info");
    base.set("TIMEOUT", "30");

    let mut ci = Layer::new("ci", 10);
    ci.set("APP_ENV", "ci");
    ci.set("LOG_LEVEL", "debug");

    registry.add_layer(base).unwrap();
    registry.add_layer(ci).unwrap();
    registry
}

#[test]
fn test_resolve_uses_highest_priority() {
    let registry = make_registry();
    let resolver = Resolver::new(&registry);
    assert_eq!(resolver.resolve("APP_ENV"), Some("ci".to_string()));
    assert_eq!(resolver.resolve("LOG_LEVEL"), Some("debug".to_string()));
}

#[test]
fn test_resolve_falls_back_to_lower_priority() {
    let registry = make_registry();
    let resolver = Resolver::new(&registry);
    assert_eq!(resolver.resolve("TIMEOUT"), Some("30".to_string()));
}

#[test]
fn test_resolve_missing_key_returns_none() {
    let registry = make_registry();
    let resolver = Resolver::new(&registry);
    assert_eq!(resolver.resolve("NONEXISTENT"), None);
}

#[test]
fn test_require_returns_value() {
    let registry = make_registry();
    let resolver = Resolver::new(&registry);
    assert_eq!(resolver.require("TIMEOUT").unwrap(), "30");
}

#[test]
fn test_require_missing_returns_error() {
    let registry = make_registry();
    let resolver = Resolver::new(&registry);
    let err = resolver.require("MISSING").unwrap_err();
    assert_eq!(err, EnvLayerError::MissingKey("MISSING".to_string()));
}

#[test]
fn test_resolve_all_merges_layers() {
    let registry = make_registry();
    let resolver = Resolver::new(&registry);
    let all = resolver.resolve_all();
    assert_eq!(all.get("APP_ENV").map(String::as_str), Some("ci"));
    assert_eq!(all.get("TIMEOUT").map(String::as_str), Some("30"));
    assert_eq!(all.len(), 3);
}

#[test]
fn test_require_all_success() {
    let registry = make_registry();
    let resolver = Resolver::new(&registry);
    let result = resolver.require_all(&["APP_ENV", "TIMEOUT"]).unwrap();
    assert_eq!(result["APP_ENV"], "ci");
    assert_eq!(result["TIMEOUT"], "30");
}

#[test]
fn test_require_all_partial_missing_returns_error() {
    let registry = make_registry();
    let resolver = Resolver::new(&registry);
    let err = resolver.require_all(&["APP_ENV", "MISSING_A", "MISSING_B"]).unwrap_err();
    match err {
        EnvLayerError::MissingKeys(keys) => {
            assert!(keys.contains(&"MISSING_A".to_string()));
            assert!(keys.contains(&"MISSING_B".to_string()));
        }
        other => panic!("unexpected error: {:?}", other),
    }
}
