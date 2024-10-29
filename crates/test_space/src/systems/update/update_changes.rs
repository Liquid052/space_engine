use bevy::prelude::*;

use crate::prelude::*;

pub fn update_period(
    mut changed: Query<(&Keplerian, &mut Orbit), Changed<Keplerian>>,
    bodies: Query<&Body>,
) {
    changed.par_iter_mut().for_each(|(keplerian, mut orbit)| {
        let parent_body = bodies.get(orbit.parent()).unwrap();

        if parent_body.is_two_body() {
            orbit.period = keplerian.period(parent_body.reduced_mass);
            return;
        }

        orbit.period = keplerian.period(parent_body.mass);
    });
}