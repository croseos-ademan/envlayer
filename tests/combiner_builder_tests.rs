use std::collections::HashMap;
use envlayer::combiner_builder::CombinerBuilder;
use envlayer::env_combiner::CombineStrategy;

fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
fn test_builder_default_strategy_is_override() {
    let result = CombinerBuilder::new()
        .layer(map(&[("X", "a")]))
        .layer(map(&[("X", "b")]))
        .build()
        .unwrap();
    assert_eq!(result["X"], "b");
}

#[test]
fn test_builder_fill_missing_strategy() {
    let result = CombinerBuilder::new()
        .strategy(CombineStrategy::FillMissing)
        .layer(map(&[("X", "first")]))
        .layer(map(&[("X", "second")]))
        .build()
        .unwrap();
    assert_eq!(result["X"], "first");
}

#[test]
fn test_builder_concat_strategy() {
    let result = CombinerBuilder::new()
        .strategy(CombineStrategy::Concat(" ".to_string()))
        .layer(map(&[("MSG", "hello")]))
        .layer(map(&[("MSG", "world")]))
        .build()
        .unwrap();
    assert_eq!(result["MSG"], "hello world");
}

#[test]
fn test_builder_empty_layers() {
    let result = CombinerBuilder::new().build().unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_builder_layer_count() {
    let builder = CombinerBuilder::new()
        .layer(map(&[("A", "1")]))
        .layer(map(&[("B", "2")]))
        .layer(map(&[("C", "3")]));
    assert_eq!(builder.layer_count(), 3);
}

#[test]
fn test_build_combiner_returns_combiner() {
    let combiner = CombinerBuilder::new()
        .strategy(CombineStrategy::FillMissing)
        .build_combiner();
    assert_eq!(combiner.strategy(), &CombineStrategy::FillMissing);
}

#[test]
fn test_builder_multiple_keys_override() {
    let result = CombinerBuilder::new()
        .layer(map(&[("A", "1"), ("B", "2")]))
        .layer(map(&[("B", "99"), ("C", "3")]))
        .build()
        .unwrap();
    assert_eq!(result["A"], "1");
    assert_eq!(result["B"], "99");
    assert_eq!(result["C"], "3");
}
