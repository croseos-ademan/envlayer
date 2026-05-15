use crate::env_sorter::{EnvSorter, SortOrder};

/// Builder for constructing an [`EnvSorter`] with a fluent API.
#[derive(Debug, Default)]
pub struct SorterBuilder {
    order: Option<SortOrder>,
}

impl SorterBuilder {
    /// Create a new `SorterBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sort keys in ascending (A–Z) order.
    pub fn ascending(mut self) -> Self {
        self.order = Some(SortOrder::Ascending);
        self
    }

    /// Sort keys in descending (Z–A) order.
    pub fn descending(mut self) -> Self {
        self.order = Some(SortOrder::Descending);
        self
    }

    /// Sort keys according to a custom priority list.
    /// Keys not in the list are appended alphabetically after the prioritised ones.
    pub fn custom(mut self, keys: Vec<impl Into<String>>) -> Self {
        self.order = Some(SortOrder::Custom(
            keys.into_iter().map(Into::into).collect(),
        ));
        self
    }

    /// Build the [`EnvSorter`].
    pub fn build(self) -> EnvSorter {
        EnvSorter::new(self.order.unwrap_or(SortOrder::Ascending))
    }
}
