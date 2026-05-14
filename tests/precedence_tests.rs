use envlayer::layer::Layer;
use envlayer::precedence::{PrecedenceResolver, PrecedenceStrategy};
use envlayer::precedence_builder::PrecedenceBuilder;

fn make_layer(name: &str, entries: &[(&str, &str)]) -> Layer {
    let mut layer = Layer::new(name);
    for (k, v) in entries {
        layer.set(k, v);
    }
    layer
}

#[test]
fn last_wins_overrides_earlier_layers() {
    let base = make_layer("base", &[("APP_ENV", "development"), ("LOG_LEVEL", "info")]);
    let ci = make_layer("ci", &[("APP_ENV", "ci")]);

    let resolver = PrecedenceBuilder::new().last_wins().build();
    let result = resolver.resolve(&[base, ci]);

    assert_eq!(result.get("APP_ENV").map(String::as_str), Some("ci"));
    assert_eq!(result.get("LOG_LEVEL").map(String::as_str), Some("info"));
}

#[test]
fn first_wins_preserves_earliest_definition() {
    let base = make_layer("base", &[("APP_ENV", "development"), ("LOG_LEVEL", "info")]);
    let ci = make_layer("ci", &[("APP_ENV", "ci"), ("LOG_LEVEL", "debug")]);

    let resolver = PrecedenceBuilder::new().first_wins().build();
    let result = resolver.resolve(&[base, ci]);

    assert_eq!(result.get("APP_ENV").map(String::as_str), Some("development"));
    assert_eq!(result.get("LOG_LEVEL").map(String::as_str), Some("info"));
}

#[test]
fn ranked_strategy_respects_explicit_order() {
    let base = make_layer("base", &[("APP_ENV", "development"), ("DB_URL", "localhost")]);
    let override_layer = make_layer("override", &[("APP_ENV", "staging")]);
    let ci = make_layer("ci", &[("APP_ENV", "ci"), ("DB_URL", "ci-db")]);

    // ci has highest priority, then override, then base
    let resolver = PrecedenceBuilder::new()
        .ranked(["ci", "override", "base"])
        .build();
    let result = resolver.resolve(&[base, override_layer, ci]);

    assert_eq!(result.get("APP_ENV").map(String::as_str), Some("ci"));
    assert_eq!(result.get("DB_URL").map(String::as_str), Some("ci-db"));
}

#[test]
fn ranked_strategy_unlisted_layer_has_lowest_priority() {
    let base = make_layer("base", &[("SECRET", "base-secret")]);
    let unknown = make_layer("unknown", &[("SECRET", "unknown-secret")]);

    // Only "base" is ranked; "unknown" is unranked and loses.
    let resolver = PrecedenceBuilder::new().ranked(["base"]).build();
    let result = resolver.resolve(&[base, unknown]);

    assert_eq!(result.get("SECRET").map(String::as_str), Some("base-secret"));
}

#[test]
fn default_strategy_is_last_wins() {
    let a = make_layer("a", &[("KEY", "first")]);
    let b = make_layer("b", &[("KEY", "second")]);

    let resolver = PrecedenceResolver::new(PrecedenceStrategy::default());
    let result = resolver.resolve(&[a, b]);

    assert_eq!(result.get("KEY").map(String::as_str), Some("second"));
}

#[test]
fn empty_layers_returns_empty_map() {
    let resolver = PrecedenceBuilder::new().last_wins().build();
    let result = resolver.resolve(&[]);
    assert!(result.is_empty());
}
