use soa_rs::Soa;
use crate::celestial_body::CelestialBody;
use crate::prelude::SpaceTimeScale;

pub fn update_period_soa(
    star: &CelestialBody,
    mut bodies: &mut Soa<CelestialBody>,
) {
    let mut slices = bodies.slices_mut();
    let (keplerian, orbit) = (slices.keplerian, slices.orbit);

    keplerian
        .iter_mut()
        .zip(orbit)
        .for_each(|(keplerian, orbit)| {
            let parent_mass = star.body.mass;
            let period = keplerian.period(parent_mass);
            orbit.period = period;
        });
}

pub fn update_epochs_soa(
    delta: f64,
    scale: &SpaceTimeScale,
    mut bodies: &mut Soa<CelestialBody>,
) {
    let time = delta * scale.0;
    let mut slices = bodies.slices_mut();
    let (keplerian, orbit) = (slices.keplerian.as_ref(), slices.orbit);

    keplerian
        .iter()
        .zip(orbit)
        .for_each(|(keplerian, orbit)| {
            orbit.epoch += time;

            if (orbit.epoch > orbit.period) && keplerian.is_elliptical() {
                orbit.epoch %= orbit.period;
            }
        });
}



pub fn update_orbits_soa(
    star: &CelestialBody,
    mut bodies: &mut Soa<CelestialBody>,
) {
    let parent_mass = star.body.mass;
    let mut slices = bodies.slices_mut();
    let (keplerian, state_vec, orbit) = (
        slices.keplerian,
        slices.state_vectors,
        slices.orbit
            .as_ref()
    );

    keplerian
        .iter_mut()
        .zip(state_vec)
        .zip(orbit)
        .map(|((keplerian, state_vec), orbit)|
            (keplerian, state_vec, orbit)
        )
        .for_each(|(keplerian, state_vec, orbit)| {
            let mut new_state = keplerian.state_vectors_at_epoch(
                parent_mass,
                orbit.epoch,
                1e-6,
            );

            new_state.position.z = 0.0;
            new_state.velocity.z = 0.0;

            *state_vec = new_state;
        });
}


