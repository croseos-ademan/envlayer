use std::collections::HashMap;
use envlayer::snapshot::{Snapshot, SnapshotStore};

fn make_vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
fn test_snapshot_creation() {
    let vars = make_vars(&[("APP_ENV", "production"), ("PORT", "8080")]);
    let snap = Snapshot::new("v1", vars);
    assert_eq!(snap.label, "v1");
    assert_eq!(snap.len(), 2);
    assert!(!snap.is_empty());
}

#[test]
fn test_snapshot_get() {
    let vars = make_vars(&[("DB_URL", "postgres://localhost/db")]);
    let snap = Snapshot::new("db-snap", vars);
    assert_eq!(snap.get("DB_URL"), Some(&"postgres://localhost/db".to_string()));
    assert!(snap.get("MISSING").is_none());
}

#[test]
fn test_snapshot_diff_added() {
    let base = Snapshot::new("base", make_vars(&[("A", "1")]));
    let next = Snapshot::new("next", make_vars(&[("A", "1"), ("B", "2")]));
    let diff = base.diff(&next);
    assert!(diff.added.contains(&"B"));
    assert!(diff.removed.is_empty());
    assert!(diff.changed.is_empty());
}

#[test]
fn test_snapshot_diff_removed() {
    let base = Snapshot::new("base", make_vars(&[("A", "1"), ("B", "2")]));
    let next = Snapshot::new("next", make_vars(&[("A", "1")]));
    let diff = base.diff(&next);
    assert!(diff.removed.contains(&"B"));
    assert!(diff.added.is_empty());
    assert!(diff.changed.is_empty());
}

#[test]
fn test_snapshot_diff_changed() {
    let base = Snapshot::new("base", make_vars(&[("PORT", "3000")]));
    let next = Snapshot::new("next", make_vars(&[("PORT", "8080")]));
    let diff = base.diff(&next);
    assert!(diff.changed.contains(&"PORT"));
    assert!(diff.added.is_empty());
    assert!(diff.removed.is_empty());
}

#[test]
fn test_store_save_and_get() {
    let mut store = SnapshotStore::new();
    let snap = Snapshot::new("snap1", make_vars(&[("X", "10")]));
    store.save(snap);
    assert_eq!(store.count(), 1);
    let retrieved = store.get("snap1").expect("should find snap1");
    assert_eq!(retrieved.get("X"), Some(&"10".to_string()));
}

#[test]
fn test_store_labels() {
    let mut store = SnapshotStore::new();
    store.save(Snapshot::new("alpha", make_vars(&[])));
    store.save(Snapshot::new("beta", make_vars(&[])));
    let labels = store.labels();
    assert_eq!(labels, vec!["alpha", "beta"]);
}

#[test]
fn test_store_remove() {
    let mut store = SnapshotStore::new();
    store.save(Snapshot::new("to-remove", make_vars(&[("K", "V")])));
    assert!(store.remove("to-remove").is_ok());
    assert_eq!(store.count(), 0);
}

#[test]
fn test_store_remove_missing_returns_error() {
    let mut store = SnapshotStore::new();
    let result = store.remove("ghost");
    assert!(result.is_err());
}

#[test]
fn test_store_get_returns_latest_by_label() {
    let mut store = SnapshotStore::new();
    store.save(Snapshot::new("env", make_vars(&[("VER", "1")])));
    store.save(Snapshot::new("env", make_vars(&[("VER", "2")])));
    let snap = store.get("env").unwrap();
    assert_eq!(snap.get("VER"), Some(&"2".to_string()));
}
