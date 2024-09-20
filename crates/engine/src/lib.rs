

pub mod plugin;

// export
pub use plugin::EnginePlugin;
pub use save_pipeline;
pub use engine_core;
pub use space;

pub mod prelude {
    pub use super::plugin::*;
    pub use engine_core::prelude::*;
    pub use space::prelude::*;

}