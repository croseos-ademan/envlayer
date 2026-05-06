use envlayer::layer::Layer;
use envlayer::merger::{merge_layers, MergeStrategy};

fn make_layer(entries: &[(&str, &str)]) -> Layer {
    let mut layer = Layer::new();
    for (k, v) in entries {
        layer.set(k, v);
    }
    layer
}

#[test]
test_last_wins_strategy() {
    let layer_a = make_layer(&[("APP_ENV", "development"), ("LOG_LEVEL", "debug")]);
    let layer_b = make_layer(&[("APP_ENV", "production")]);

    let merged = merge_layers(&[layer_a, layer_b], &MergeStrategy::LastWins)
        .expect("merge should succeed");

    assert_eq!(merged.get("APP_ENV").map(String::as_str), Some("production"));
    assert_eq!(merged.get("LOG_LEVEL").map(String::as_str), Some("debug"));
}

#[test]
fn test_first_wins_strategy() {
    let layer_a = make_layer(&[("APP_ENV", "development")]);
    let layer_b = make_layer(&[("APP_ENV", "production"), ("LOG_LEVEL", "warn")]);

    let merged = merge_layers(&[layer_a, layer_b], &MergeStrategy::FirstWins)
        .expect("merge should succeed");

    assert_eq!(merged.get("APP_ENV").map(String::as_str), Some("development"));
    assert_eq!(merged.get("LOG_LEVEL").map(String::as_str), Some("warn"));
}

#[test]
fn test_error_on_conflict_no_overlap() {
    let layer_a = make_layer(&[("APP_ENV", "development")]);
    let layer_b = make_layer(&[("LOG_LEVEL", "info")]);

    let merged = merge_layers(&[layer_a, layer_b], &MergeStrategy::ErrorOnConflict)
        .expect("merge should succeed with no conflicts");

    assert_eq!(merged.len(), 2);
}

#[test]
fn test_error_on_conflict_with_overlap() {
    let layer_a = make_layer(&[("APP_ENV", "development")]);
    let layer_b = make_layer(&[("APP_ENV", "production")]);

    let result = merge_layers(&[layer_a, layer_b], &MergeStrategy::ErrorOnConflict);
    assert!(result.is_err(), "expected conflict error");
}

#[test]
fn test_empty_layers() {
    let merged = merge_layers(&[], &MergeStrategy::LastWins)
        .expect("merging empty slice should succeed");
    assert!(merged.is_empty());
}

#[test]
fn test_single_layer() {
    let layer = make_layer(&[("KEY", "value")]);
    let merged = merge_layers(&[layer], &MergeStrategy::LastWins)
        .expect("single layer merge should succeed");
    assert_eq!(merged.get("KEY").map(String::as_str), Some("value"));
}
