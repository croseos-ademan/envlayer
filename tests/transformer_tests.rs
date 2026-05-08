use std::collections::HashMap;
use envlayer::transformer::{Transformer, TransformerPipeline};
use envlayer::error::EnvLayerError;

fn make_vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
fn test_single_transformer_uppercase() {
    let mut pipeline = TransformerPipeline::new();
    pipeline.add(Transformer::new("uppercase", Box::new(|v| Ok(v.to_uppercase()))));

    let vars = make_vars(&[("KEY", "hello"), ("OTHER", "world")]);
    let result = pipeline.apply_all(&vars).unwrap();

    assert_eq!(result["KEY"], "HELLO");
    assert_eq!(result["OTHER"], "WORLD");
}

#[test]
fn test_chained_transformers() {
    let mut pipeline = TransformerPipeline::new();
    pipeline.add(Transformer::new("trim", Box::new(|v| Ok(v.trim().to_string()))));
    pipeline.add(Transformer::new("uppercase", Box::new(|v| Ok(v.to_uppercase()))));

    let vars = make_vars(&[("KEY", "  hello  ")]);
    let result = pipeline.apply_all(&vars).unwrap();

    assert_eq!(result["KEY"], "HELLO");
}

#[test]
fn test_empty_pipeline_returns_original() {
    let pipeline = TransformerPipeline::new();
    let vars = make_vars(&[("KEY", "value")]);
    let result = pipeline.apply_all(&vars).unwrap();
    assert_eq!(result["KEY"], "value");
}

#[test]
fn test_transformer_error_propagates() {
    let mut pipeline = TransformerPipeline::new();
    pipeline.add(Transformer::new(
        "fail_on_empty",
        Box::new(|v| {
            if v.is_empty() {
                Err(EnvLayerError::InvalidValue("empty value".to_string()))
            } else {
                Ok(v.to_string())
            }
        }),
    ));

    let vars = make_vars(&[("KEY", "")]);
    let result = pipeline.apply_all(&vars);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("KEY"));
}

#[test]
fn test_pipeline_len() {
    let mut pipeline = TransformerPipeline::new();
    assert!(pipeline.is_empty());
    pipeline.add(Transformer::new("t1", Box::new(|v| Ok(v.to_string()))));
    pipeline.add(Transformer::new("t2", Box::new(|v| Ok(v.to_string()))));
    assert_eq!(pipeline.len(), 2);
    assert!(!pipeline.is_empty());
}

#[test]
fn test_transformer_name() {
    let t = Transformer::new("my_transform", Box::new(|v| Ok(v.to_string())));
    assert_eq!(t.name(), "my_transform");
}

#[test]
fn test_apply_does_not_mutate_original() {
    let mut pipeline = TransformerPipeline::new();
    pipeline.add(Transformer::new("uppercase", Box::new(|v| Ok(v.to_uppercase()))));

    let vars = make_vars(&[("KEY", "original")]);
    let _result = pipeline.apply_all(&vars).unwrap();
    assert_eq!(vars["KEY"], "original");
}
