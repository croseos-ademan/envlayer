use std::collections::HashMap;
use envlayer::env_partitioner::EnvPartitioner;
use envlayer::partition_builder::PartitionBuilder;

fn sample_vars() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("APP_HOST".into(), "localhost".into());
    m.insert("APP_PORT".into(), "8080".into());
    m.insert("DB_HOST".into(), "db.local".into());
    m.insert("DB_PORT".into(), "5432".into());
    m.insert("LOG_LEVEL".into(), "info".into());
    m
}

#[test]
fn test_partition_by_prefix_splits_correctly() {
    let vars = sample_vars();
    let mut partitioner = EnvPartitioner::new();
    partitioner.partition_by_prefix(&vars, &["APP_", "DB_"]).unwrap();

    let app = partitioner.get_partition("APP_").unwrap();
    assert_eq!(app.len(), 2);
    let keys: Vec<&str> = app.iter().map(|(k, _)| k.as_str()).collect();
    assert!(keys.contains(&"HOST"));
    assert!(keys.contains(&"PORT"));
}

#[test]
fn test_unmatched_contains_non_prefixed() {
    let vars = sample_vars();
    let mut partitioner = EnvPartitioner::new();
    partitioner.partition_by_prefix(&vars, &["APP_", "DB_"]).unwrap();

    let unmatched = partitioner.get_unmatched();
    assert_eq!(unmatched.len(), 1);
    assert_eq!(unmatched[0].0, "LOG_LEVEL");
}

#[test]
fn test_flatten_restores_original_keys() {
    let vars = sample_vars();
    let mut partitioner = EnvPartitioner::new();
    partitioner.partition_by_prefix(&vars, &["APP_", "DB_"]).unwrap();

    let flat = partitioner.flatten();
    assert_eq!(flat.get("APP_HOST").unwrap(), "localhost");
    assert_eq!(flat.get("DB_PORT").unwrap(), "5432");
    assert_eq!(flat.get("LOG_LEVEL").unwrap(), "info");
    assert_eq!(flat.len(), 5);
}

#[test]
fn test_partition_names_returned() {
    let vars = sample_vars();
    let mut partitioner = EnvPartitioner::new();
    partitioner.partition_by_prefix(&vars, &["APP_", "DB_"]).unwrap();

    let mut names = partitioner.partition_names();
    names.sort();
    assert_eq!(names, vec!["APP_", "DB_"]);
}

#[test]
fn test_builder_produces_correct_partitioner() {
    let partitioner = PartitionBuilder::new()
        .with_prefix("APP_")
        .with_prefix("DB_")
        .with_var("APP_HOST", "localhost")
        .with_var("DB_HOST", "db.local")
        .with_var("UNMATCHED", "value")
        .build()
        .unwrap();

    assert!(partitioner.get_partition("APP_").is_some());
    assert!(partitioner.get_partition("DB_").is_some());
    assert_eq!(partitioner.get_unmatched().len(), 1);
}

#[test]
fn test_empty_vars_yields_empty_partitioner() {
    let partitioner = PartitionBuilder::new()
        .with_prefix("APP_")
        .with_vars(HashMap::new())
        .build()
        .unwrap();

    assert!(partitioner.get_partition("APP_").is_none());
    assert!(partitioner.get_unmatched().is_empty());
}
