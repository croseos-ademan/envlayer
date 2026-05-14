//! Template-based environment variable expansion with placeholder substitution.

use crate::error::EnvLayerError;
use std::collections::HashMap;

/// Represents a parsed template with variable placeholders.
#[derive(Debug, Clone)]
pub struct EnvTemplate {
    raw: String,
}

impl EnvTemplate {
    /// Create a new template from a raw string.
    pub fn new(raw: impl Into<String>) -> Self {
        Self { raw: raw.into() }
    }

    /// Returns the raw template string.
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Render the template using the provided variable map.
    /// Placeholders use `${VAR_NAME}` syntax.
    pub fn render(&self, vars: &HashMap<String, String>) -> Result<String, EnvLayerError> {
        let mut result = self.raw.clone();
        let mut i = 0;
        let bytes = self.raw.as_bytes();
        let mut output = String::with_capacity(self.raw.len());

        while i < bytes.len() {
            if bytes[i] == b'$' && i + 1 < bytes.len() && bytes[i + 1] == b'{' {
                let start = i + 2;
                if let Some(end) = self.raw[start..].find('}') {
                    let key = &self.raw[start..start + end];
                    match vars.get(key) {
                        Some(val) => output.push_str(val),
                        None => {
                            return Err(EnvLayerError::MissingKey(key.to_string()));
                        }
                    }
                    i = start + end + 1;
                    continue;
                }
            }
            output.push(bytes[i] as char);
            i += 1;
        }

        result = output;
        Ok(result)
    }

    /// Render the template, leaving unresolved placeholders as-is.
    pub fn render_partial(&self, vars: &HashMap<String, String>) -> String {
        let mut output = String::with_capacity(self.raw.len());
        let bytes = self.raw.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'$' && i + 1 < bytes.len() && bytes[i + 1] == b'{' {
                let start = i + 2;
                if let Some(end) = self.raw[start..].find('}') {
                    let key = &self.raw[start..start + end];
                    if let Some(val) = vars.get(key) {
                        output.push_str(val);
                    } else {
                        output.push_str(&format!("${{{}}}", key));
                    }
                    i = start + end + 1;
                    continue;
                }
            }
            output.push(bytes[i] as char);
            i += 1;
        }

        output
    }
}
