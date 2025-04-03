#![allow(non_snake_case)]
use constants::{PI, TWO_PI};

pub const N_BODIES: u32 = 3_000_000;

pub mod celestial_body;

pub(crate) mod helpers;
pub(crate) mod systems;

pub mod components;
pub mod constants;
pub mod resources;
pub mod plugins;

// export
pub mod prelude;
