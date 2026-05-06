use envlayer::audit::{AuditEvent, AuditLog};

#[test]
fn test_record_and_retrieve() {
    let mut log = AuditLog::new(10);
    log.record(AuditEvent::Set {
        key: "FOO".into(),
        value: "bar".into(),
        layer: "local".into(),
    });
    assert_eq!(log.len(), 1);
}

#[test]
fn test_capacity_eviction() {
    let mut log = AuditLog::new(3);
    for i in 0..5 {
        log.record(AuditEvent::Set {
            key: format!("KEY_{i}"),
            value: "v".into(),
            layer: "base".into(),
        });
    }
    assert_eq!(log.len(), 3);
}

#[test]
fn test_entries_for_key_filters_correctly() {
    let mut log = AuditLog::new(20);
    log.record(AuditEvent::Set { key: "FOO".into(), value: "1".into(), layer: "l1".into() });
    log.record(AuditEvent::Set { key: "BAR".into(), value: "2".into(), layer: "l1".into() });
    log.record(AuditEvent::Override {
        key: "FOO".into(),
        old_value: "1".into(),
        new_value: "2".into(),
        layer: "l2".into(),
    });
    let foo_entries: Vec<_> = log.entries_for_key("FOO").collect();
    assert_eq!(foo_entries.len(), 2);
    let bar_entries: Vec<_> = log.entries_for_key("BAR").collect();
    assert_eq!(bar_entries.len(), 1);
}

#[test]
fn test_clear_empties_log() {
    let mut log = AuditLog::new(10);
    log.record(AuditEvent::Remove { key: "X".into(), layer: "ci".into() });
    log.clear();
    assert!(log.is_empty());
}

#[test]
fn test_merge_event_not_matched_by_key_filter() {
    let mut log = AuditLog::new(10);
    log.record(AuditEvent::Merge {
        source_layer: "base".into(),
        target_layer: "ci".into(),
        key_count: 5,
    });
    let results: Vec<_> = log.entries_for_key("base").collect();
    assert!(results.is_empty());
}

#[test]
fn test_default_capacity_is_1024() {
    let log = AuditLog::default();
    assert!(log.is_empty());
}
