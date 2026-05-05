//! envlayer — Composable environment variable management with layered overrides.
//!
//! Layers are applied in order; later layers override earlier ones.

pub mod layer;
pub mod registry;
pub mod error;

pub use layer::{EnvLayer, LayerSource};
pub use registry::LayerRegistry;
pub use error::EnvLayerError;
