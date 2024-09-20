//! Primary use: smart asset handling and cross-platform saving

mod systems;
pub mod states;
pub mod resources;
pub mod plugins;
pub mod assets;
pub mod bevy_api;
pub mod traits;
mod helpers;

pub mod prelude {
    pub use super::assets::*;
    pub use super::bevy_api::*;
    pub use super::plugins::*;
    pub use super::resources::*;
    pub use super::states::*;
}