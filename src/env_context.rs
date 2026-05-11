use std::collections::HashMap;
use crate::error::EnvLayerError;

/// Represents the detected execution context (local, CI, docker, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextKind {
    Local,
    CI,
    Docker,
    Custom(String),
}

impl ContextKind {
    pub fn as_str(&self) -> &str {
        match self {
            ContextKind::Local => "local",
            ContextKind::CI => "ci",
            ContextKind::Docker => "docker",
            ContextKind::Custom(s) => s.as_str(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "local" => ContextKind::Local,
            "ci" => ContextKind::CI,
            "docker" => ContextKind::Docker,
            other => ContextKind::Custom(other.to_string()),
        }
    }
}

/// Detects and holds the current environment context
#[derive(Debug, Clone)]
pub struct EnvContext {
    pub kind: ContextKind,
    pub metadata: HashMap<String, String>,
}

impl EnvContext {
    pub fn new(kind: ContextKind) -> Self {
        Self {
            kind,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn detect() -> Result<Self, EnvLayerError> {
        let kind = if std::env::var("CI").is_ok()
            || std::env::var("GITHUB_ACTIONS").is_ok()
            || std::env::var("CIRCLECI").is_ok()
            || std::env::var("TRAVIS").is_ok()
        {
            ContextKind::CI
        } else if std::env::var("container").is_ok()
            || std::path::Path::new("/.dockerenv").exists()
        {
            ContextKind::Docker
        } else {
            ContextKind::Local
        };

        let mut ctx = EnvContext::new(kind);
        if let Ok(runner) = std::env::var("GITHUB_ACTIONS") {
            ctx.metadata.insert("github_actions".to_string(), runner);
        }
        if let Ok(runner) = std::env::var("CI_RUNNER_DESCRIPTION") {
            ctx.metadata.insert("runner".to_string(), runner);
        }
        Ok(ctx)
    }

    pub fn is_ci(&self) -> bool {
        self.kind == ContextKind::CI
    }

    pub fn is_local(&self) -> bool {
        self.kind == ContextKind::Local
    }
}
