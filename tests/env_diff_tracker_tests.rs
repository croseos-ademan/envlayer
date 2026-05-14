use std::collections::HashMap;
use envlayer::env_diff_tracker::EnvDiffTracker;
use envlayer::diff::DiffKind;
use envlayer::diff_tracker_builder::DiffTrackerBuilder;

fn make_map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
fn test_no_changes_returns_empty_diff() {
    let vars = make_map(&[("FOO", "bar"), ("BAZ", "qux")]);
    let tracker = DiffTrackerBuilder::new()
        .baseline(vars.clone())
        .current(vars)
        .build();
    assert!(!tracker.has_changes());
    assert!(tracker.compute_diff().is_empty());
}

#[test]
fn test_added_key_detected() {
    let tracker = DiffTrackerBuilder::new()
        .baseline([("FOO", "bar")])
        .current([("FOO", "bar"), ("NEW_KEY", "value")])
        .build();
    let diffs = tracker.compute_diff();
    assert_eq!(diffs.len(), 1);
    assert_eq!(diffs[0].key, "NEW_KEY");
    assert_eq!(diffs[0].kind, DiffKind::Added);
    assert_eq!(diffs[0].new_value.as_deref(), Some("value"));
}

#[test]
fn test_removed_key_detected() {
    let tracker = DiffTrackerBuilder::new()
        .baseline([("FOO", "bar"), ("GONE", "bye")])
        .current([("FOO", "bar")])
        .build();
    let diffs = tracker.compute_diff();
    assert_eq!(diffs.len(), 1);
    assert_eq!(diffs[0].key, "GONE");
    assert_eq!(diffs[0].kind, DiffKind::Removed);
    assert_eq!(diffs[0].old_value.as_deref(), Some("bye"));
}

#[test]
fn test_modified_key_detected() {
    let tracker = DiffTrackerBuilder::new()
        .baseline([("FOO", "old")])
        .current([("FOO", "new")])
        .build();
    let diffs = tracker.compute_diff();
    assert_eq!(diffs.len(), 1);
    assert_eq!(diffs[0].kind, DiffKind::Modified);
    assert_eq!(diffs[0].old_value.as_deref(), Some("old"));
    assert_eq!(diffs[0].new_value.as_deref(), Some("new"));
}

#[test]
fn test_commit_promotes_current_to_baseline() {
    let mut tracker = DiffTrackerBuilder::new()
        .baseline([("FOO", "v1")])
        .current([("FOO", "v2")])
        .build();
    assert!(tracker.has_changes());
    tracker.commit();
    assert!(!tracker.has_changes());
}

#[test]
fn test_diffs_sorted_by_key() {
    let tracker = DiffTrackerBuilder::new()
        .baseline([])
        .current([("ZEBRA", "1"), ("ALPHA", "2"), ("MIDDLE", "3")])
        .build();
    let diffs = tracker.compute_diff();
    let keys: Vec<&str> = diffs.iter().map(|d| d.key.as_str()).collect();
    assert_eq!(keys, vec!["ALPHA", "MIDDLE", "ZEBRA"]);
}
