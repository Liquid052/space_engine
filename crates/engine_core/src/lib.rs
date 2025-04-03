//! The engine core bundles essential functionalities, allowing different parts of the 
//! engine to build on top of it. It serves as the backbone of the engine, providing 
//! necessary resources, systems, and plugins that enhance the overall architecture.


pub mod plugins;
#[doc(hidden)]
pub mod resources;
#[doc(hidden)]
pub mod systems;

#[doc(hidden)]
pub mod components;
pub mod states;
#[doc(hidden)]
pub mod bundles;
#[doc(hidden)]
pub mod attributes;
#[doc(hidden)]
pub mod commands;
#[doc(hidden)]
mod bevy_api;
#[doc(hidden)]
pub mod prelude;


pub use core_camera as camera;
pub use core_assets as assets;
