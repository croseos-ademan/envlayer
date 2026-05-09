use std::collections::HashMap;
use envlayer::schema::{Schema, SchemaField, SchemaType};
use envlayer::schema_validator::SchemaValidator;

fn make_schema() -> Schema {
    let mut schema = Schema::new();
    schema.add_field(SchemaField::new("APP_PORT", SchemaType::Integer, true));
    schema.add_field(
        SchemaField::new("DEBUG", SchemaType::Boolean, false)
            .with_default("false")
    );
    schema.add_field(SchemaField::new("APP_NAME", SchemaType::String, true));
    schema
}

#[test]
fn test_valid_env_passes_schema() {
    let schema = make_schema();
    let mut env = HashMap::new();
    env.insert("APP_PORT".into(), "8080".into());
    env.insert("APP_NAME".into(), "myapp".into());

    let validator = SchemaValidator::new(&schema);
    let report = validator.validate(&env).unwrap();
    assert!(report.is_valid(), "Expected valid: {:?}", report.errors());
}

#[test]
fn test_missing_required_field_reported() {
    let schema = make_schema();
    let env: HashMap<String, String> = HashMap::new();

    let validator = SchemaValidator::new(&schema);
    let report = validator.validate(&env).unwrap();
    assert!(!report.is_valid());
    assert!(report.missing_required.iter().any(|e| e.contains("APP_PORT")));
    assert!(report.missing_required.iter().any(|e| e.contains("APP_NAME")));
}

#[test]
fn test_type_error_reported_for_invalid_integer() {
    let schema = make_schema();
    let mut env = HashMap::new();
    env.insert("APP_PORT".into(), "not_a_number".into());
    env.insert("APP_NAME".into(), "myapp".into());

    let validator = SchemaValidator::new(&schema);
    let report = validator.validate(&env).unwrap();
    assert!(!report.is_valid());
    assert!(report.type_errors.iter().any(|e| e.contains("APP_PORT")));
}

#[test]
fn test_apply_defaults_fills_missing_optional() {
    let schema = make_schema();
    let mut env = HashMap::new();
    env.insert("APP_PORT".into(), "3000".into());
    env.insert("APP_NAME".into(), "svc".into());

    let validator = SchemaValidator::new(&schema);
    validator.apply_defaults(&mut env);
    assert_eq!(env.get("DEBUG").map(|s| s.as_str()), Some("false"));
}

#[test]
fn test_boolean_validation_accepts_variants() {
    let field = SchemaField::new("FLAG", SchemaType::Boolean, true);
    for val in &["true", "false", "1", "0", "yes", "no", "True", "FALSE"] {
        assert!(field.validate_value(val).is_ok(), "Should accept: {}", val);
    }
    assert!(field.validate_value("maybe").is_err());
}

#[test]
fn test_schema_get_field() {
    let schema = make_schema();
    assert!(schema.get_field("APP_PORT").is_some());
    assert!(schema.get_field("NONEXISTENT").is_none());
}
