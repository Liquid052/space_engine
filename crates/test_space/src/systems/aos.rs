use crate::components::{SpaceDepth, StarMarker};
use crate::prelude::{AOSBody, SpaceTimeScale};
use bevy::prelude::*;

pub fn update_period_aos(
    mut bodies: Query<(Entity, &mut AOSBody)>,
    stars: Query<(), With<StarMarker>>,
) {
    // First collect all the necessary data
    let pairs: Vec<_> = bodies
        .iter()
        .filter(|(ent, _)| !stars.contains(*ent))
        .map(|(entity, body)| (entity, body.orbit.parent))
        .collect();

    // Now update periods
    pairs.iter()
        .for_each(|(ent, parent_ent)| {
            let parent_mass = bodies.get(*parent_ent).unwrap().1.body.mass;
            let mut body = bodies.get_mut(*ent).unwrap();
            let period = body.1.keplerian.period(parent_mass);

            body.1.orbit.period = period;
        });
}

pub fn update_epochs_aos(
    mut bodies: Query<&mut AOSBody, Without<StarMarker>>,
    time: Res<Time>,
    scale: Res<SpaceTimeScale>,
) {
    let time = time.delta_seconds_f64() * scale.0;

    bodies.par_iter_mut().for_each(|mut aos_body| {
        aos_body.orbit.epoch += time;

        if (aos_body.orbit.epoch > aos_body.orbit.period) && aos_body.keplerian.is_elliptical() {
            aos_body.orbit.epoch %= aos_body.orbit.period;
        }
    });
}

pub fn update_orbits_aos(
    mut bodies: Query<(Entity, &mut AOSBody)>,
    stars: Query<(), With<StarMarker>>,
) {
    // Collect parent masses first
    let pairs: Vec<_> = bodies
        .iter()
        .filter(|(ent, _)| !stars.contains(*ent))
        .map(|(entity, body)| (entity, body.orbit.parent))
        .collect();

    pairs.iter().for_each(|(ent, parent)| {
        let parent_mass = bodies.get(*parent).unwrap().1.body.mass;
        let mut aos_body = bodies.get_mut(*ent).unwrap().1;

        let mut new_state = aos_body.keplerian.state_vectors_at_epoch(
            parent_mass,
            aos_body.orbit.epoch,
            2.220_446_049_250_313e-10,
        );

        new_state.position.z = 0.0;
        new_state.velocity.z = 0.0;

        aos_body.state_vectors = new_state;
    });
}

pub fn map_orbit_pos_aos(
    mut bodies: Query<(Entity, &mut AOSBody)>,
    stars: Query<(), With<StarMarker>>,
) {
    let mut pairs: Vec<(Entity, Entity, SpaceDepth)> = bodies
        .iter()
        .filter(|(ent, _)| !stars.contains(*ent))
        .map(|(entity, body)| (entity, body.orbit.parent, body.space_depth))
        .collect();

    pairs.sort_by(|a, b| a.2.cmp(&b.2));

    pairs.iter()
        .map(|(ent, parent, _)| (*ent, *parent))
        .for_each(|(ent, parent)| {
            let parent_pos = bodies.get(parent).unwrap().1.space_pos.0;
            let mut aos_body = bodies.get_mut(ent).unwrap().1;
            let new_pos = aos_body.state_vectors.position.xy() + parent_pos;
            aos_body.space_pos.0 = new_pos;
        });
}