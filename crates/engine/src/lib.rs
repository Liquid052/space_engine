//! The main entry point for the engine which bundles all plugins and functionality together into
//! EnginePlugin.

pub mod plugin;

pub use engine_core as core;

// export
pub use plugin::EnginePlugin;
pub use space;

#[doc(hidden)]
pub use utils::*;

#[doc(hidden)]
pub mod prelude;

