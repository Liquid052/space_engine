use bevy::prelude::*;

use crate::prelude::*;

pub fn update_period(
    mut changed: Query<(&Keplerian, &mut Orbit), Changed<Keplerian>>,
    bodies: Query<&Body>,
) {
    changed.par_iter_mut().for_each(|(keplerian, mut orbit)| {
        let parent_body = bodies.get(orbit.parent()).unwrap();


        orbit.period = keplerian.period(parent_body.mass);
    });
}