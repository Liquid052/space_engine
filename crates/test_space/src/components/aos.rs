use crate::components::{Orbit, SpaceDepth};
use crate::prelude::{Body, Keplerian, RefFrame, SpacePos, SpaceTimeScale, StateVec};
use bevy::prelude::*;

#[derive(Reflect, Component, Clone)]
#[reflect(Component)]
pub struct AOSBody {
    pub body: Body,
    pub orbit: Orbit,
    pub keplerian: Keplerian,
    pub state_vectors: StateVec,
    pub space_depth: SpaceDepth,
    pub space_pos: SpacePos,
    pub ref_frame: RefFrame,
}
