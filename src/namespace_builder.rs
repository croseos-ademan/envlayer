use crate::env_namespace::EnvNamespace;
use crate::namespace_registry::NamespaceRegistry;

/// Builder for constructing a NamespaceRegistry with fluent API.
#[derive(Default)]
pub struct NamespaceBuilder {
    entries: Vec<(String, EnvNamespace)>,
}

impl NamespaceBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(mut self, name: impl Into<String>, prefix: impl Into<String>) -> Self {
        self.entries
            .push((name.into(), EnvNamespace::new(prefix)));
        self
    }

    pub fn add_with_separator(
        mut self,
        name: impl Into<String>,
        prefix: impl Into<String>,
        sep: impl Into<String>,
    ) -> Self {
        self.entries.push((
            name.into(),
            EnvNamespace::new(prefix).with_separator(sep),
        ));
        self
    }

    pub fn build(self) -> NamespaceRegistry {
        let mut registry = NamespaceRegistry::new();
        for (name, ns) in self.entries {
            registry.register(name, ns);
        }
        registry
    }
}
