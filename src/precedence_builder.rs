//! Fluent builder for constructing a [`PrecedenceResolver`].

use crate::precedence::{PrecedenceResolver, PrecedenceStrategy};

/// Builder for [`PrecedenceResolver`].
#[derive(Debug, Default)]
pub struct PrecedenceBuilder {
    strategy: Option<PrecedenceStrategy>,
}

impl PrecedenceBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Use the last-layer-wins strategy (default).
    pub fn last_wins(mut self) -> Self {
        self.strategy = Some(PrecedenceStrategy::LastWins);
        self
    }

    /// Use the first-layer-wins strategy.
    pub fn first_wins(mut self) -> Self {
        self.strategy = Some(PrecedenceStrategy::FirstWins);
        self
    }

    /// Use an explicit ranked order. Layers listed first have the highest
    /// priority. Layers not listed fall back to lowest priority.
    pub fn ranked<I, S>(mut self, order: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.strategy = Some(PrecedenceStrategy::Ranked(
            order.into_iter().map(Into::into).collect(),
        ));
        self
    }

    /// Consume the builder and produce a [`PrecedenceResolver`].
    pub fn build(self) -> PrecedenceResolver {
        PrecedenceResolver::new(self.strategy.unwrap_or_default())
    }
}
