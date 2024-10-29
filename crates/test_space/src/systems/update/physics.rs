use bevy::prelude::*;

use crate::prelude::*;

pub fn update_epochs(
    mut query: Query<(&mut Orbit, &Keplerian)>,
    time: Res<Time>,
    scale: Res<SpaceTimeScale>,
) {
    let time = time.delta_seconds_f64() * scale.0;

    query.par_iter_mut().for_each(|(mut orbit, keplerian)| {
        orbit.epoch += time;

        if (orbit.epoch > orbit.period) && keplerian.is_elliptical() {
            orbit.epoch %= orbit.period;
        }
    });
}

pub fn update_orbits(
    mut bodies: Query<(&mut StateVec, &mut Keplerian, &Orbit)>,
    masses: Query<&Body>,
) {
    bodies
        .par_iter_mut()
        .for_each(|(mut state_vec, keplerian, orbiting)| {
            let parent = orbiting.parent();

            let parent_mass = masses.get(parent).unwrap().mass;

            let mut new_state = keplerian.state_vectors_at_epoch(
                parent_mass,
                orbiting.epoch,
                2.220_446_049_250_313e-10,
            );
            // if orbit bugs - try to check restrictions related to 3D going to 2D
            new_state.position.z = 0.0;
            new_state.velocity.z = 0.0;

            *state_vec = new_state;
        });
}

pub fn map_orbit_pos(
    bodies: Query<(Entity, &Orbit, &StateVec, &SpaceDepth)>,
    mut positions: Query<&mut SpacePos>,
) {
    bodies.iter()
        .sort_unstable::<&SpaceDepth>()
        .for_each(|(ent, orbit, state_vec, _)| {
            let mut pos = positions.get_many_mut([ent, orbit.parent])
                .unwrap();

            **pos[0] = state_vec.position.xy() + **pos[1];
        });
}
