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

pub fn update_soi(
    changed: Query<(Entity, &Keplerian, &Orbit), (Without<Restricted>, Changed<Keplerian>)>,
    mut bodies: Query<&mut Body>,
) {
    changed.iter().for_each(|(ent, keplerian, orbit)| {
        let Ok(mut masses) = bodies.get_many_mut([ent, orbit.parent()]) else {
            return;
        };

        let soi = keplerian.calculate_soi(masses[0].mass, masses[1].mass);
        masses[0].soi = soi;
    });
}

pub fn _recalculate_soi_mass(
    _query: Query<Entity, Changed<Body>>,
    _bodies: Query<&mut Body>,
    _frames: Query<&mut RefFrame>,
) {
}
