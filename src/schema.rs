//! Schema definition and validation for environment variable declarations.

use std::collections::HashMap;
use crate::error::EnvLayerError;

/// The expected type of an environment variable value.
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaType {
    String,
    Integer,
    Boolean,
    Float,
}

/// A single field definition in the schema.
#[derive(Debug, Clone)]
pub struct SchemaField {
    pub name: String,
    pub schema_type: SchemaType,
    pub required: bool,
    pub default: Option<String>,
    pub description: Option<String>,
}

impl SchemaField {
    pub fn new(name: impl Into<String>, schema_type: SchemaType, required: bool) -> Self {
        Self {
            name: name.into(),
            schema_type,
            required,
            default: None,
            description: None,
        }
    }

    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn validate_value(&self, value: &str) -> Result<(), EnvLayerError> {
        match self.schema_type {
            SchemaType::String => Ok(()),
            SchemaType::Integer => value.parse::<i64>().map(|_| ()).map_err(|_| {
                EnvLayerError::ValidationFailed(format!(
                    "Field '{}' expected integer, got '{}'" , self.name, value
                ))
            }),
            SchemaType::Boolean => match value.to_lowercase().as_str() {
                "true" | "false" | "1" | "0" | "yes" | "no" => Ok(()),
                _ => Err(EnvLayerError::ValidationFailed(format!(
                    "Field '{}' expected boolean, got '{}'" , self.name, value
                ))),
            },
            SchemaType::Float => value.parse::<f64>().map(|_| ()).map_err(|_| {
                EnvLayerError::ValidationFailed(format!(
                    "Field '{}' expected float, got '{}'" , self.name, value
                ))
            }),
        }
    }
}

/// A schema describing the expected environment variables.
#[derive(Debug, Default, Clone)]
pub struct Schema {
    pub fields: HashMap<String, SchemaField>,
}

impl Schema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_field(&mut self, field: SchemaField) {
        self.fields.insert(field.name.clone(), field);
    }

    pub fn get_field(&self, name: &str) -> Option<&SchemaField> {
        self.fields.get(name)
    }
}
