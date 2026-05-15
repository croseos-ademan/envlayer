use std::time::Duration;

/// Defines how caching behaves for a given layer or key pattern.
#[derive(Debug, Clone, PartialEq)]
pub enum CachePolicy {
    /// Never cache; always resolve fresh.
    NoCache,
    /// Cache indefinitely until explicitly invalidated.
    Forever,
    /// Cache for a fixed duration.
    Ttl(Duration),
}

impl CachePolicy {
    /// Convert the policy to an optional TTL suitable for `EnvCache::insert`.
    ///
    /// Returns `None` for both `NoCache` (caching disabled) and `Forever`
    /// (no expiry). Returns `Some(duration)` for `Ttl` variants.
    pub fn to_ttl(&self) -> Option<Duration> {
        match self {
            CachePolicy::NoCache => None,
            CachePolicy::Forever => None,
            CachePolicy::Ttl(d) => Some(*d),
        }
    }

    /// Returns true when the policy allows caching at all.
    pub fn should_cache(&self) -> bool {
        !matches!(self, CachePolicy::NoCache)
    }

    /// Returns true if this policy has a finite expiry (i.e. is a `Ttl` variant).
    pub fn is_expiring(&self) -> bool {
        matches!(self, CachePolicy::Ttl(_))
    }

    /// Returns the TTL duration if this policy is `Ttl`, otherwise `None`.
    pub fn duration(&self) -> Option<Duration> {
        if let CachePolicy::Ttl(d) = self {
            Some(*d)
        } else {
            None
        }
    }
}

impl Default for CachePolicy {
    fn default() -> Self {
        CachePolicy::Ttl(Duration::from_secs(60))
    }
}

/// Builder for constructing a `CachePolicy`.
#[derive(Debug, Default)]
pub struct CachePolicyBuilder {
    policy: CachePolicy,
}

impl CachePolicyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn no_cache(mut self) -> Self {
        self.policy = CachePolicy::NoCache;
        self
    }

    pub fn forever(mut self) -> Self {
        self.policy = CachePolicy::Forever;
        self
    }

    pub fn ttl_secs(mut self, secs: u64) -> Self {
        self.policy = CachePolicy::Ttl(Duration::from_secs(secs));
        self
    }

    pub fn build(self) -> CachePolicy {
        self.policy
    }
}
