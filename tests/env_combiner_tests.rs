use std::collections::HashMap;
use envlayer::env_combiner::{CombineStrategy, EnvCombiner};

fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
test_combine_empty_returns_empty() {
    let combiner = EnvCombiner::with_override();
    let result = combiner.combine(&[]).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_override_later_wins() {
    let combiner = EnvCombiner::with_override();
    let a = map(&[("KEY", "base")]);
    let b = map(&[("KEY", "override")]);
    let result = combiner.combine(&[a, b]).unwrap();
    assert_eq!(result["KEY"], "override");
}

#[test]
fn test_override_new_keys_merged() {
    let combiner = EnvCombiner::with_override();
    let a = map(&[("A", "1")]);
    let b = map(&[("B", "2")]);
    let result = combiner.combine(&[a, b]).unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result["A"], "1");
    assert_eq!(result["B"], "2");
}

#[test]
fn test_fill_missing_first_wins() {
    let combiner = EnvCombiner::with_fill_missing();
    let a = map(&[("KEY", "first")]);
    let b = map(&[("KEY", "second")]);
    let result = combiner.combine(&[a, b]).unwrap();
    assert_eq!(result["KEY"], "first");
}

#[test]
fn test_fill_missing_fills_gaps() {
    let combiner = EnvCombiner::with_fill_missing();
    let a = map(&[("A", "1")]);
    let b = map(&[("A", "99"), ("B", "2")]);
    let result = combiner.combine(&[a, b]).unwrap();
    assert_eq!(result["A"], "1");
    assert_eq!(result["B"], "2");
}

#[test]
fn test_concat_combines_values() {
    let combiner = EnvCombiner::with_concat(":");
    let a = map(&[("PATH", "/usr/bin")]);
    let b = map(&[("PATH", "/usr/local/bin")]);
    let result = combiner.combine(&[a, b]).unwrap();
    assert_eq!(result["PATH"], "/usr/bin:/usr/local/bin");
}

#[test]
fn test_concat_single_value_no_separator() {
    let combiner = EnvCombiner::with_concat(",");
    let a = map(&[("LIST", "item1")]);
    let result = combiner.combine(&[a]).unwrap();
    assert_eq!(result["LIST"], "item1");
}

#[test]
fn test_strategy_accessor() {
    let combiner = EnvCombiner::new(CombineStrategy::FillMissing);
    assert_eq!(combiner.strategy(), &CombineStrategy::FillMissing);
}
