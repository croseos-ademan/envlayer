use std::collections::HashMap;
use envlayer::diff::{EnvDiff};
use envlayer::diff_formatter::{DiffFormat, DiffFormatter};

fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
fn test_diff_added() {
    let before = map(&[("FOO", "bar")]);
    let after = map(&[("FOO", "bar"), ("BAZ", "qux")]);
    let diff = EnvDiff::compute(&before, &after);
    assert_eq!(diff.added.get("BAZ").map(String::as_str), Some("qux"));
    assert!(diff.removed.is_empty());
    assert!(diff.changed.is_empty());
}

#[test]
fn test_diff_removed() {
    let before = map(&[("FOO", "bar"), ("OLD", "val")]);
    let after = map(&[("FOO", "bar")]);
    let diff = EnvDiff::compute(&before, &after);
    assert!(diff.added.is_empty());
    assert_eq!(diff.removed.get("OLD").map(String::as_str), Some("val"));
    assert!(diff.changed.is_empty());
}

#[test]
fn test_diff_changed() {
    let before = map(&[("FOO", "old")]);
    let after = map(&[("FOO", "new")]);
    let diff = EnvDiff::compute(&before, &after);
    assert!(diff.added.is_empty());
    assert!(diff.removed.is_empty());
    let change = diff.changed.get("FOO").unwrap();
    assert_eq!(change, &("old".to_string(), "new".to_string()));
}

#[test]
fn test_diff_is_empty_when_identical() {
    let env = map(&[("FOO", "bar"), ("BAZ", "qux")]);
    let diff = EnvDiff::compute(&env, &env);
    assert!(diff.is_empty());
}

#[test]
fn test_plain_summary_format() {
    let before = map(&[("A", "1")]);
    let after = map(&[("B", "2")]);
    let diff = EnvDiff::compute(&before, &after);
    let summary = diff.summary();
    assert!(summary.contains("+ B=2"));
    assert!(summary.contains("- A=1"));
}

#[test]
fn test_json_format_contains_keys() {
    let before = map(&[("X", "old")]);
    let after = map(&[("X", "new"), ("Y", "added")]);
    let diff = EnvDiff::compute(&before, &after);
    let formatter = DiffFormatter::new(DiffFormat::Json);
    let output = formatter.render(&diff);
    assert!(output.contains("\"added\""));
    assert!(output.contains("\"changed\""));
    assert!(output.contains("\"removed\""));
    assert!(output.contains("\"key\":\"Y\""));
    assert!(output.contains("\"old\":\"old\""));
    assert!(output.contains("\"new\":\"new\""));
}

#[test]
fn test_plain_formatter_delegates_to_summary() {
    let before = map(&[("K", "v1")]);
    let after = map(&[("K", "v2")]);
    let diff = EnvDiff::compute(&before, &after);
    let formatter = DiffFormatter::new(DiffFormat::Plain);
    let output = formatter.render(&diff);
    assert_eq!(output, diff.summary());
}
