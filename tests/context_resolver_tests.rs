use envlayer::context_resolver::ContextResolver;
use envlayer::env_context::{ContextKind, EnvContext};

fn make_ctx(kind: ContextKind) -> EnvContext {
    EnvContext::new(kind)
}

#[test]
fn test_resolve_returns_none_when_empty() {
    let resolver = ContextResolver::new();
    let ctx = make_ctx(ContextKind::Local);
    let result = resolver.resolve("DATABASE_URL", &ctx).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_resolve_default_value() {
    let mut resolver = ContextResolver::new();
    resolver.set_default("LOG_LEVEL", "info");
    let ctx = make_ctx(ContextKind::Local);
    let result = resolver.resolve("LOG_LEVEL", &ctx).unwrap();
    assert_eq!(result, Some("info".to_string()));
}

#[test]
fn test_context_override_takes_priority_over_default() {
    let mut resolver = ContextResolver::new();
    resolver.set_default("LOG_LEVEL", "info");
    resolver.set_for_context(&ContextKind::CI, "LOG_LEVEL", "debug");

    let local_ctx = make_ctx(ContextKind::Local);
    let ci_ctx = make_ctx(ContextKind::CI);

    assert_eq!(
        resolver.resolve("LOG_LEVEL", &local_ctx).unwrap(),
        Some("info".to_string())
    );
    assert_eq!(
        resolver.resolve("LOG_LEVEL", &ci_ctx).unwrap(),
        Some("debug".to_string())
    );
}

#[test]
fn test_resolve_all_merges_defaults_and_overrides() {
    let mut resolver = ContextResolver::new();
    resolver.set_default("APP_ENV", "development");
    resolver.set_default("LOG_LEVEL", "info");
    resolver.set_for_context(&ContextKind::CI, "APP_ENV", "ci");
    resolver.set_for_context(&ContextKind::CI, "CACHE", "true");

    let ci_ctx = make_ctx(ContextKind::CI);
    let all = resolver.resolve_all(&ci_ctx).unwrap();

    assert_eq!(all.get("APP_ENV"), Some(&"ci".to_string()));
    assert_eq!(all.get("LOG_LEVEL"), Some(&"info".to_string()));
    assert_eq!(all.get("CACHE"), Some(&"true".to_string()));
}

#[test]
fn test_resolve_all_local_does_not_include_ci_overrides() {
    let mut resolver = ContextResolver::new();
    resolver.set_default("APP_ENV", "development");
    resolver.set_for_context(&ContextKind::CI, "APP_ENV", "ci");

    let local_ctx = make_ctx(ContextKind::Local);
    let all = resolver.resolve_all(&local_ctx).unwrap();

    assert_eq!(all.get("APP_ENV"), Some(&"development".to_string()));
}

#[test]
fn test_custom_context_resolution() {
    let mut resolver = ContextResolver::new();
    let staging = ContextKind::Custom("staging".to_string());
    resolver.set_for_context(&staging, "API_URL", "https://staging.example.com");

    let ctx = make_ctx(ContextKind::Custom("staging".to_string()));
    let result = resolver.resolve("API_URL", &ctx).unwrap();
    assert_eq!(result, Some("https://staging.example.com".to_string()));
}
