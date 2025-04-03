use soa_rs::{Soa, Soars};
use crate::components::{Body, Keplerian, Orbit, RefFrame, SpaceDepth, SpacePos, StateVec};
use crate::prelude::SpaceTimeScale;

pub mod soa;
pub mod aos;

#[derive(Soars, Debug)]
#[soa_derive(Debug)]
pub struct CelestialBody {
    pub body:          Body,
    pub orbit:         Orbit,
    pub keplerian:     Keplerian,
    pub state_vectors: StateVec,
    pub space_depth:   SpaceDepth,
    pub space_pos:     SpacePos,
    pub ref_frame:     RefFrame,
}

pub use soa::*;
pub use aos::*;