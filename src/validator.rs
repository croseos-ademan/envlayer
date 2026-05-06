use crate::error::{EnvLayerError, Result};
use std::collections::HashMap;

/// A rule that can be applied to validate an environment variable value.
#[derive(Debug, Clone)]
pub enum ValidationRule {
    /// Value must not be empty.
    NonEmpty,
    /// Value must match the given regex pattern.
    Pattern(String),
    /// Value must be one of the given allowed values.
    OneOf(Vec<String>),
    /// Value must be parseable as an integer.
    Integer,
    /// Value length must be within the given range (min, max).
    LengthRange(usize, usize),
}

/// Validates environment variable values against a set of rules.
#[derive(Debug, Default)]
pub struct Validator {
    rules: HashMap<String, Vec<ValidationRule>>,
}

impl Validator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register one or more rules for a given key.
    pub fn add_rule(&mut self, key: impl Into<String>, rule: ValidationRule) {
        self.rules.entry(key.into()).or_default().push(rule);
    }

    /// Validate all provided key-value pairs against registered rules.
    /// Returns a list of validation errors, or Ok(()) if all pass.
    pub fn validate(&self, vars: &HashMap<String, String>) -> Result<()> {
        let mut errors: Vec<String> = Vec::new();

        for (key, rules) in &self.rules {
            let value = vars.get(key).map(|s| s.as_str()).unwrap_or("");
            for rule in rules {
                if let Err(msg) = apply_rule(key, value, rule) {
                    errors.push(msg);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(EnvLayerError::ValidationFailed(errors.join("; ")))
        }
    }
}

fn apply_rule(key: &str, value: &str, rule: &ValidationRule) -> std::result::Result<(), String> {
    match rule {
        ValidationRule::NonEmpty => {
            if value.is_empty() {
                Err(format!("'{}' must not be empty", key))
            } else {
                Ok(())
            }
        }
        ValidationRule::Pattern(pattern) => {
            if value.contains(pattern.as_str()) {
                Ok(())
            } else {
                Err(format!("'{}' does not match pattern '{}'", key, pattern))
            }
        }
        ValidationRule::OneOf(allowed) => {
            if allowed.iter().any(|a| a == value) {
                Ok(())
            } else {
                Err(format!("'{}' value '{}' is not one of {:?}", key, value, allowed))
            }
        }
        ValidationRule::Integer => value
            .parse::<i64>()
            .map(|_| ())
            .map_err(|_| format!("'{}' value '{}' is not a valid integer", key, value)),
        ValidationRule::LengthRange(min, max) => {
            let len = value.len();
            if len >= *min && len <= *max {
                Ok(())
            } else {
                Err(format!(
                    "'{}' length {} is not in range [{}, {}]",
                    key, len, min, max
                ))
            }
        }
    }
}
