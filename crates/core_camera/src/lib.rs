//! This camera management system in Bevy allows defining various application states where the
//! camera with the MainCamera component resides. Users can switch between different camera layers
//! using commands, facilitating display management across different game scenes.
//! The system supports various camera layer configurations, including clamping zoom, HDR,
//! tonemapping, and render layers.

#[doc(hidden)]
pub mod plugins;
#[doc(hidden)]
pub mod prelude;
#[doc(hidden)]
pub mod commands;
#[doc(hidden)]
pub mod resources;
#[doc(hidden)]
pub mod components;


mod systems;

pub mod commands_api;
pub mod app_api;
