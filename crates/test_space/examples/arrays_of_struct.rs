use std::cell::Cell;
use bevy::prelude::{default, Entity};
use test_space::prelude::*;
use test_space::celestial_body::*;
use test_space::N_BODIES;

fn main() {
    let (star, mut bodies) = setup(N_BODIES);

    update(&star, &mut bodies);
}

fn update(star: &CelestialBody, bodies: &mut Vec<CelestialBody>) {
    update_period_aos(star, bodies);
    update_epochs_aos(0.2, &SpaceTimeScale(1.0), bodies);
    update_orbits_aos(star, bodies);
}

fn setup(n: u32) -> (CelestialBody, Vec<CelestialBody>) {
    let star = gen_star();
    let mut planets = Vec::with_capacity(n as usize);

    for x in 0..n {
        let planet = gen_planet();
        planets.push(planet);
    }

    (star, planets)
}

fn gen_star() -> CelestialBody {
    CelestialBody {
        body: Body {
            radius: 261_600_000.0,
            mass: 1.7565459e28,
            ..default()
        },
        space_pos: SpacePos::default(),
        ref_frame: RefFrame::new(),
        keplerian: Keplerian::default(),
        state_vectors: StateVec::default(),
        space_depth: SpaceDepth(0),
        orbit: Orbit::default()
    }
}
fn gen_planet() -> CelestialBody {
    let keplerian = Keplerian {
        semi_major_axis: 23_599_840_256.0,
        ..default()
    };

    let aos_body = CelestialBody {
        body: Body::new(600_000.0, 5.2915158e22),
        space_pos: SpacePos::default(),
        ref_frame: RefFrame::new(),
        keplerian,
        state_vectors: StateVec::default(),
        orbit: Orbit::default(),
        space_depth: SpaceDepth(1),
    };

    aos_body
}
