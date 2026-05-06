use std::collections::HashMap;
use envlayer::filter::Filter;
use envlayer::filter_pipeline::FilterPipeline;

fn sample_vars() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("APP_HOST".into(), "localhost".into());
    map.insert("APP_PORT".into(), "8080".into());
    map.insert("APP_SECRET".into(), "topsecret".into());
    map.insert("DB_URL".into(), "postgres://localhost/db".into());
    map.insert("LOG_LEVEL".into(), "info".into());
    map
}

#[test]
fn test_empty_pipeline_returns_all() {
    let pipeline = FilterPipeline::new();
    let result = pipeline.run(&sample_vars()).unwrap();
    assert_eq!(result.len(), 5);
}

#[test]
fn test_single_stage_pipeline() {
    let pipeline = FilterPipeline::new()
        .add(Filter::new().with_include_prefix("APP_"));
    let result = pipeline.run(&sample_vars()).unwrap();
    assert_eq!(result.len(), 3);
}

#[test]
fn test_two_stage_pipeline_narrows_result() {
    let pipeline = FilterPipeline::new()
        .add(Filter::new().with_include_prefix("APP_"))
        .add(Filter::new().with_exclude_key("APP_SECRET"));
    let result = pipeline.run(&sample_vars()).unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.contains_key("APP_HOST"));
    assert!(result.contains_key("APP_PORT"));
    assert!(!result.contains_key("APP_SECRET"));
}

#[test]
fn test_stage_count() {
    let pipeline = FilterPipeline::new()
        .add(Filter::new())
        .add(Filter::new());
    assert_eq!(pipeline.stage_count(), 2);
}

#[test]
fn test_pipeline_with_all_excluded() {
    let pipeline = FilterPipeline::new()
        .add(Filter::new().with_include_prefix("NONEXISTENT_"));
    let result = pipeline.run(&sample_vars()).unwrap();
    assert!(result.is_empty());
}
