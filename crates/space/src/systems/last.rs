use bevy::prelude::*;

use crate::{
    components::{Body, Exited},
    prelude::*,
};

pub fn cleanup_restrictions(
    mut commands: Commands,
    bodies: Query<(&Body, &SpacePos)>,
    exited: Query<(Entity, &Exited)>,
) {
    exited
        .iter()
        .filter(|(ent, exited)| {
            let exited = ***exited;

            let bodies = bodies.get_many([*ent, exited]).unwrap();
            let dist_squared = bodies[0].1.distance_squared(**bodies[1].1);
            let soi_squared = (bodies[0].0.soi + bodies[1].0.soi).powi(2);

            dist_squared > soi_squared
        })
        .for_each(|(ent, _)| {
            commands.entity(ent).remove::<Exited>();
        });
}
