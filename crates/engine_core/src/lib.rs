//! Primary use: Reflection, engine state and type based hierarchy of entities


pub mod plugins;
pub mod resources;
mod systems;
// TODO: Remove with bevy 0.12
pub mod components;
pub mod states;
pub mod bundles;

pub mod commands;
mod bevy_api;

/// Export
pub mod prelude {
    use crate::systems;

    // export

    pub use systems::*;
    // pub use macros::*;
    pub use crate::bundles::*;
    pub use crate::components::*;
    pub use crate::plugins::*;
    pub use crate::resources::*;
    pub use crate::states::plugins::*;
    pub use crate::commands::*;
    pub use crate::states::*;

    pub use core_assets::prelude::*;
    pub use core_pack_objects::prelude::*;
}
