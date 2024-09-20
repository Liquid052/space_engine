use crate::components::*;
use bevy::{prelude::*, render::view::RenderLayers};

#[derive(Bundle)]
pub struct TwoBodyBundle {
    pub name:          Name,
    pub body_params:   TwoBody,
    pub abs_pos:       SpacePos,
    pub ref_frame:     RefFrame,
    pub keplerian:     Keplerian,
    pub state_vectors: StateVec,
    // markers
    pub space_layer:   RenderLayers,
    pub depth:         SpaceDepth,
    pub star:          StarMarker,
    pub space:         Space,
}
