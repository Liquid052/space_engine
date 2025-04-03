use crate::celestial_body::CelestialBody;
use crate::prelude::SpaceTimeScale;

pub fn update_period_aos(
    star: &CelestialBody,
    bodies: &mut Vec<CelestialBody>,
) {
    for body in bodies.iter_mut() {
        let parent_mass = star.body.mass;
        let period = body.keplerian.period(parent_mass);
        body.orbit.period = period;
    }
}

pub fn update_epochs_aos(
    delta: f64,
    scale: &SpaceTimeScale,
    bodies: &mut Vec<CelestialBody>,
) {
    let time = delta * scale.0;

    for body in bodies.iter_mut() {
        body.orbit.epoch += time;

        if (body.orbit.epoch > body.orbit.period) && body.keplerian.is_elliptical() {
            body.orbit.epoch %= body.orbit.period;
        }
    }
}

pub fn update_orbits_aos(
    star: &CelestialBody,
    bodies: &mut Vec<CelestialBody>,
) {
    let parent_mass = star.body.mass;

    for body in bodies.iter_mut() {
        let mut new_state = body.keplerian.state_vectors_at_epoch(
            parent_mass,
            body.orbit.epoch,
            1e-6,
        );

        new_state.position.z = 0.0;
        new_state.velocity.z = 0.0;

        body.state_vectors = new_state;
    }
}