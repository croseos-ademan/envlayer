//! Variable interpolation support for environment values.
//!
//! Supports `${VAR}` and `$VAR` syntax within values, resolving
//! references against a provided context map.

use std::collections::HashMap;
use crate::error::EnvLayerError;

/// Interpolates environment variable references within a string value.
///
/// Supports `${VAR_NAME}` and `$VAR_NAME` syntax. Unknown variables
/// are left as-is unless `strict` mode is enabled.
pub struct Interpolator {
    strict: bool,
}

impl Interpolator {
    /// Create a new interpolator. If `strict` is true, missing variables
    /// will return an error instead of being left unchanged.
    pub fn new(strict: bool) -> Self {
        Self { strict }
    }

    /// Interpolate all variable references in `value` using `context`.
    pub fn interpolate(
        &self,
        value: &str,
        context: &HashMap<String, String>,
    ) -> Result<String, EnvLayerError> {
        let mut result = String::with_capacity(value.len());
        let mut chars = value.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch != '$' {
                result.push(ch);
                continue;
            }

            match chars.peek() {
                Some('{') => {
                    chars.next(); // consume '{'
                    let var_name: String = chars.by_ref().take_while(|&c| c != '}').collect();
                    self.resolve_var(&var_name, context, &mut result)?;
                }
                Some(&c) if c.is_alphanumeric() || c == '_' => {
                    let var_name: String = chars
                        .by_ref()
                        .take_while(|&c| c.is_alphanumeric() || c == '_')
                        .collect();
                    self.resolve_var(&var_name, context, &mut result)?;
                }
                _ => result.push('$'),
            }
        }

        Ok(result)
    }

    fn resolve_var(
        &self,
        name: &str,
        context: &HashMap<String, String>,
        output: &mut String,
    ) -> Result<(), EnvLayerError> {
        match context.get(name) {
            Some(val) => output.push_str(val),
            None if self.strict => {
                return Err(EnvLayerError::MissingVariable(name.to_string()));
            }
            None => {
                output.push('$');
                output.push('{');
                output.push_str(name);
                output.push('}');
            }
        }
        Ok(())
    }
}
