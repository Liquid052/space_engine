use bevy::render::view::{Layer, RenderLayers};

pub const GALAXY_LAYER: RenderLayers = RenderLayers::layer(GALAXY_LAYER_DEPTH);

pub const GALAXY_LAYER_DEPTH: Layer = 13;
