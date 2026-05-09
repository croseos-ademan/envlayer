//! Validates an environment map against a Schema.

use std::collections::HashMap;
use crate::schema::Schema;
use crate::error::EnvLayerError;

/// Result of schema validation containing any errors found.
#[derive(Debug, Default)]
pub struct SchemaValidationReport {
    pub missing_required: Vec<String>,
    pub type_errors: Vec<String>,
}

impl SchemaValidationReport {
    pub fn is_valid(&self) -> bool {
        self.missing_required.is_empty() && self.type_errors.is_empty()
    }

    pub fn errors(&self) -> Vec<String> {
        let mut all = self.missing_required.clone();
        all.extend(self.type_errors.clone());
        all
    }
}

/// Validates an environment variable map against a given schema.
pub struct SchemaValidator<'a> {
    schema: &'a Schema,
}

impl<'a> SchemaValidator<'a> {
    pub fn new(schema: &'a Schema) -> Self {
        Self { schema }
    }

    pub fn validate(
        &self,
        env: &HashMap<String, String>,
    ) -> Result<SchemaValidationReport, EnvLayerError> {
        let mut report = SchemaValidationReport::default();

        for (name, field) in &self.schema.fields {
            match env.get(name) {
                Some(value) => {
                    if let Err(e) = field.validate_value(value) {
                        report.type_errors.push(e.to_string());
                    }
                }
                None => {
                    if field.required && field.default.is_none() {
                        report.missing_required.push(format!(
                            "Required field '{}' is missing", name
                        ));
                    }
                }
            }
        }

        Ok(report)
    }

    pub fn apply_defaults(&self, env: &mut HashMap<String, String>) {
        for (name, field) in &self.schema.fields {
            if !env.contains_key(name) {
                if let Some(default) = &field.default {
                    env.insert(name.clone(), default.clone());
                }
            }
        }
    }
}
