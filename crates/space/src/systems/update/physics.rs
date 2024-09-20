use std::f32::consts::PI;
use bevy::prelude::*;

use crate::prelude::*;

pub fn update_epochs(
    mut query: Query<(&mut Orbit, &Keplerian)>,
    time: Res<Time>,
    scale: Res<SpaceTimeScale>,
) {
    let time = time.delta_seconds_f64() * scale.0;

    query.iter_mut().for_each(|(mut orbit, keplerian)| {
        orbit.epoch += time;

        if (orbit.epoch > orbit.period) && keplerian.is_elliptical() {
            orbit.epoch %= orbit.period;
        }
    });
}

pub fn vessel_rotation(mut vessels: Query<(&mut Transform, &Orbit, &Keplerian, &SpacePos), (With<Space>, With<VesselMarker>)>, pos: Query<&SpacePos>) {
    vessels.iter_mut()
        .for_each(|(mut transform, orbit, keplerian, space_pos)| {
            let parent_pos = pos.get(orbit.parent).unwrap();
            let diff          = (**parent_pos - **space_pos).as_vec2();

            let angle = diff.y.atan2(diff.x);
            let multiplier = if keplerian.is_reversed() { 2.0 } else { 1.0 };
            
            transform.rotation = Quat::from_rotation_z(angle + PI * multiplier);
        });
}

pub fn update_orbits(
    mut bodies: Query<(&mut StateVec, &mut Keplerian, &Orbit), Without<Restricted>>,
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

pub fn update_restricted_orbits(
    mut bodies: Query<(Entity, &mut StateVec, &Keplerian, &Orbit), With<Restricted>>,
    masses: Query<&Body>,
    orbits: Query<(&Orbit, &Keplerian)>,
) {
    bodies
        .par_iter_mut()
        .for_each(|(ent, mut state_vec, keplerian, orbiting)| {
            let parent = orbiting.parent();

            let parent = masses.get(parent).unwrap();

            let second = if parent.child1.unwrap() == ent {
                parent.child2.unwrap()
            } else {
                parent.child1.unwrap()
            };

            if masses.get(ent).unwrap().mass < masses.get(second).unwrap().mass {
                *state_vec = keplerian.state_vectors_at_epoch(
                    parent.reduced_mass,
                    orbiting.epoch,
                    2.220_446_049_250_313e-10,
                );
            } else {
                let orbit2 = orbits.get(second).unwrap().0;
                let percentage = orbit2.epoch / orbit2.period;
                let time = percentage * orbiting.period;

                *state_vec = keplerian.state_vectors_at_epoch(
                    parent.reduced_mass,
                    time,
                    2.220_446_049_250_313e-10,
                );
            }

            // if orbit bugs - try to check restrictions related to 3D going to 2D
            state_vec.position.z = 0.0;
            state_vec.velocity.z = 0.0;
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
