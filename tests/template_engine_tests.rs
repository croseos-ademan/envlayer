use envlayer::template_engine::{TemplateEngine, TemplateEngineConfig};
use std::collections::HashMap;

fn vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
fn test_expand_simple() {
    let engine = TemplateEngine::default();
    let v = vars(&[("BASE", "/app"), ("PATH", "${BASE}/bin")]);
    let result = engine.expand(&v).unwrap();
    assert_eq!(result["PATH"], "/app/bin");
}

#[test]
fn test_expand_transitive() {
    let engine = TemplateEngine::default();
    let v = vars(&[("A", "hello"), ("B", "${A}_world"), ("C", "${B}!")]);
    let result = engine.expand(&v).unwrap();
    assert_eq!(result["C"], "hello_world!");
}

#[test]
fn test_expand_missing_key_errors() {
    let engine = TemplateEngine::new(TemplateEngineConfig {
        allow_partial: false,
        max_passes: 3,
    });
    let v = vars(&[("KEY", "${UNDEFINED}")]);
    assert!(engine.expand(&v).is_err());
}

#[test]
fn test_expand_partial_allows_missing() {
    let engine = TemplateEngine::new(TemplateEngineConfig {
        allow_partial: true,
        max_passes: 3,
    });
    let v = vars(&[("KEY", "${UNDEFINED}")]);
    let result = engine.expand(&v).unwrap();
    assert_eq!(result["KEY"], "${UNDEFINED}");
}

#[test]
fn test_expand_value_direct() {
    let engine = TemplateEngine::default();
    let v = vars(&[("HOST", "localhost"), ("PORT", "5432")]);
    let out = engine.expand_value("${HOST}:${PORT}", &v).unwrap();
    assert_eq!(out, "localhost:5432");
}

#[test]
fn test_expand_no_placeholders_unchanged() {
    let engine = TemplateEngine::default();
    let v = vars(&[("X", "plain")]);
    let result = engine.expand(&v).unwrap();
    assert_eq!(result["X"], "plain");
}

#[test]
fn test_expand_respects_max_passes() {
    // Circular reference should not loop forever
    let engine = TemplateEngine::new(TemplateEngineConfig {
        allow_partial: true,
        max_passes: 2,
    });
    let v = vars(&[("A", "${B}"), ("B", "${A}")]);
    // Should not panic, result is partial
    let result = engine.expand(&v);
    assert!(result.is_ok());
}
