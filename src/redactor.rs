use std::collections::HashSet;

/// Redacts sensitive environment variable values for safe display or logging.
#[derive(Debug, Clone)]
pub struct Redactor {
    sensitive_keys: HashSet<String>,
    redact_placeholder: String,
}

impl Redactor {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            sensitive_keys: HashSet::new(),
            redact_placeholder: placeholder.into(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut r = Self::new("[REDACTED]");
        for key in &[
            "PASSWORD", "SECRET", "TOKEN", "API_KEY", "PRIVATE_KEY",
            "AUTH", "CREDENTIAL", "PASS", "PWD",
        ] {
            r.sensitive_keys.insert(key.to_string());
        }
        r
    }

    pub fn add_key(&mut self, key: impl Into<String>) {
        self.sensitive_keys.insert(key.into().to_uppercase());
    }

    /// Removes a previously added sensitive key. Returns `true` if the key was present.
    pub fn remove_key(&mut self, key: &str) -> bool {
        self.sensitive_keys.remove(&key.to_uppercase())
    }

    pub fn is_sensitive(&self, key: &str) -> bool {
        let upper = key.to_uppercase();
        self.sensitive_keys.iter().any(|s| upper.contains(s.as_str()))
    }

    pub fn redact(&self, key: &str, value: &str) -> String {
        if self.is_sensitive(key) {
            self.redact_placeholder.clone()
        } else {
            value.to_string()
        }
    }

    pub fn redact_map(
        &self,
        vars: &std::collections::HashMap<String, String>,
    ) -> std::collections::HashMap<String, String> {
        vars.iter()
            .map(|(k, v)| (k.clone(), self.redact(k, v)))
            .collect()
    }
}

impl Default for Redactor {
    fn default() -> Self {
        Self::with_defaults()
    }
}
