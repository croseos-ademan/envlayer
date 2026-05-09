//! Loads a Schema from a TOML-like simple text format or programmatic builder.

use std::collections::HashMap;
use crate::schema::{Schema, SchemaField, SchemaType};
use crate::error::EnvLayerError;

/// Parses a simple schema definition from a map of string descriptors.
/// Format per field: "type[?][=default]" e.g. "string", "integer?", "boolean=true"
pub struct SchemaLoader;

impl SchemaLoader {
    /// Load schema from a descriptor map: field_name -> "type[?][=default]"
    pub fn from_descriptor(
        descriptor: &HashMap<String, String>,
    ) -> Result<Schema, EnvLayerError> {
        let mut schema = Schema::new();

        for (name, spec) in descriptor {
            let field = Self::parse_spec(name, spec)?;
            schema.add_field(field);
        }

        Ok(schema)
    }

    fn parse_spec(name: &str, spec: &str) -> Result<SchemaField, EnvLayerError> {
        let (type_part, default_part) = if let Some(idx) = spec.find('=') {
            (&spec[..idx], Some(&spec[idx + 1..]))
        } else {
            (spec, None)
        };

        let (type_str, required) = if type_part.ends_with('?') {
            (&type_part[..type_part.len() - 1], false)
        } else {
            (type_part, true)
        };

        let schema_type = match type_str.trim().to_lowercase().as_str() {
            "string" => SchemaType::String,
            "integer" | "int" => SchemaType::Integer,
            "boolean" | "bool" => SchemaType::Boolean,
            "float" | "f64" => SchemaType::Float,
            other => {
                return Err(EnvLayerError::ValidationFailed(format!(
                    "Unknown schema type '{}' for field '{}'" , other, name
                )))
            }
        };

        let mut field = SchemaField::new(name, schema_type, required);
        if let Some(default) = default_part {
            field = field.with_default(default);
        }

        Ok(field)
    }
}
