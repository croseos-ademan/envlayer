use std::collections::HashMap;
use envlayer::schema_loader::SchemaLoader;
use envlayer::schema::SchemaType;

fn descriptor() -> HashMap<String, String> {
    let mut d = HashMap::new();
    d.insert("APP_PORT".into(), "integer".into());
    d.insert("DEBUG".into(), "bool?=false".into());
    d.insert("APP_NAME".into(), "string".into());
    d.insert("RATIO".into(), "float?=1.0".into());
    d
}

#[test]
fn test_loader_parses_required_integer() {
    let schema = SchemaLoader::from_descriptor(&descriptor()).unwrap();
    let field = schema.get_field("APP_PORT").unwrap();
    assert_eq!(field.schema_type, SchemaType::Integer);
    assert!(field.required);
    assert!(field.default.is_none());
}

#[test]
fn test_loader_parses_optional_with_default() {
    let schema = SchemaLoader::from_descriptor(&descriptor()).unwrap();
    let field = schema.get_field("DEBUG").unwrap();
    assert_eq!(field.schema_type, SchemaType::Boolean);
    assert!(!field.required);
    assert_eq!(field.default.as_deref(), Some("false"));
}

#[test]
fn test_loader_parses_float_with_default() {
    let schema = SchemaLoader::from_descriptor(&descriptor()).unwrap();
    let field = schema.get_field("RATIO").unwrap();
    assert_eq!(field.schema_type, SchemaType::Float);
    assert!(!field.required);
    assert_eq!(field.default.as_deref(), Some("1.0"));
}

#[test]
fn test_loader_rejects_unknown_type() {
    let mut d = HashMap::new();
    d.insert("BAD".into(), "uuid".into());
    let result = SchemaLoader::from_descriptor(&d);
    assert!(result.is_err());
}

#[test]
fn test_loader_parses_string_field() {
    let schema = SchemaLoader::from_descriptor(&descriptor()).unwrap();
    let field = schema.get_field("APP_NAME").unwrap();
    assert_eq!(field.schema_type, SchemaType::String);
    assert!(field.required);
}
