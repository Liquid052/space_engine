use bevy::render::view::*;

/// gravitational constant
pub const G: f64 = 6.67430e-11;

/// astronomical unit in km
pub const AU: f64 = 1.496e+8;

/// used for scaling **real space** to **world/camera space**
pub const SPACE_SCALE: f64 = 6_000.0;

/// *Z* depth - 2D
pub const BODY_DEPTH: f32 = 1.0;
/// *Z* depth - 2D
pub const ORBIT_DEPTH: f32 = 0.5;

/// Layer that is reserved for space rendering
pub const SPACE_LAYER: RenderLayers = RenderLayers::layer(SPACE_LAYER_DEPTH);
/// used for [SPACE_LAYER]
pub const SPACE_LAYER_DEPTH: Layer = 13;

// export
pub const TWO_PI: f64 = 2.0 * PI;
pub use std::f64::consts::PI;
