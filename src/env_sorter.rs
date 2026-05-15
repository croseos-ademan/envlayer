use std::collections::HashMap;

/// Sort order for environment variable keys.
#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
    Custom(Vec<String>),
}

/// Sorts environment variable maps by key according to a specified order.
#[derive(Debug, Clone)]
pub struct EnvSorter {
    order: SortOrder,
}

impl EnvSorter {
    /// Create a new `EnvSorter` with the given sort order.
    pub fn new(order: SortOrder) -> Self {
        Self { order }
    }

    /// Sort the given map and return a `Vec` of `(key, value)` pairs.
    pub fn sort(&self, env: &HashMap<String, String>) -> Vec<(String, String)> {
        let mut pairs: Vec<(String, String)> = env
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        match &self.order {
            SortOrder::Ascending => {
                pairs.sort_by(|a, b| a.0.cmp(&b.0));
            }
            SortOrder::Descending => {
                pairs.sort_by(|a, b| b.0.cmp(&a.0));
            }
            SortOrder::Custom(keys) => {
                pairs.sort_by(|a, b| {
                    let pos_a = keys.iter().position(|k| k == &a.0);
                    let pos_b = keys.iter().position(|k| k == &b.0);
                    match (pos_a, pos_b) {
                        (Some(i), Some(j)) => i.cmp(&j),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => a.0.cmp(&b.0),
                    }
                });
            }
        }

        pairs
    }

    /// Return only the sorted keys.
    pub fn sorted_keys(&self, env: &HashMap<String, String>) -> Vec<String> {
        self.sort(env).into_iter().map(|(k, _)| k).collect()
    }
}

impl Default for EnvSorter {
    fn default() -> Self {
        Self::new(SortOrder::Ascending)
    }
}
