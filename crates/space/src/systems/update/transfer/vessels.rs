use bevy::prelude::*;

use crate::{
    components::{Orbit, RefFrame, SpacePos, StateVec},
    events::*,
    prelude::*,
    systems::transfer::{
        iter_children, orbit, recalculate_keplerian, update_orbit_if_exists, Bodies, BodyParams,
        ObjectParams, Objects,
    },
};

// aliases
type Vessels<'a> = (Entity, &'a StateVec, &'a SpacePos, &'a Orbit);

pub fn check_vessel_transfer(
    orbits: Query<Vessels, (Changed<StateVec>, With<VesselMarker>)>,
    parents: Query<(Entity, &Body, &RefFrame, &SpacePos)>,
    orbits_p: Query<&Orbit>,
    mut writer: EventWriter<VesselTransferEvent>,
) {
    orbits
        .iter()
        .for_each(|(ent, state_vec, space_pos, orbit)| {
            let parent = orbit.parent();
            let planet_soi_squared = parents.get(parent).unwrap().1.soi.powi(2);

            // check if exits soi of parent
            if planet_soi_squared < state_vec.position.length_squared() {
                let new_parent = orbits_p.get(parent).unwrap().parent();

                writer.send(VesselTransferEvent::new(ent, parent, new_parent, true));
                return;
            }

            // check if body enters child soi
            iter_children(&parents, parent, |tuple| {
                let (child_ent, child_body, child_space_pos) = tuple;

                let relative_dist = space_pos.0 - child_space_pos.0;
                let soi_squared = child_body.soi.powi(2);

                if soi_squared > relative_dist.length_squared() {
                    writer.send(VesselTransferEvent::new(ent, parent, child_ent, false));
                }
            });
        });
}

pub fn handle_vessel_transfer(
    mut reader: EventReader<VesselTransferEvent>,
    mut commands: Commands,
    mut objects_q: Query<ObjectParams>,
    mut bodies_q: Query<BodyParams>,
    mut orbits: Query<&mut Orbit>,
    mut depths: Query<&mut SpaceDepth>,
) {
    reader.read().for_each(|ev| {
        let ent = ev.entity;
        let old_parent = ev.old_parent;
        let new_parent = ev.new_parent;
        let name = objects_q.get(ent).unwrap().3.clone();
        let up = ev.up;

        let mut objects = objects_q
            .get_many_mut([ent, old_parent, new_parent])
            .unwrap();
        let mut bodies = bodies_q.get_many_mut([old_parent, new_parent]).unwrap();

        recalculate_keplerian(&mut objects, bodies[1].1.mass, up);

        ref_params(
            &mut objects,
            &mut bodies,
            orbits.get_mut(ent).unwrap().as_mut(),
        );

        orbit(
            &mut objects,
            bodies[1].1.mass,
            orbits.get_mut(ent).unwrap().as_mut(),
        );

        update_orbit_if_exists(&mut commands, old_parent, new_parent, name, &mut objects);

        match up {
            true => depths.get_mut(ent).unwrap().up(),
            false => depths.get_mut(ent).unwrap().down(),
        }
    });
}

fn ref_params(objects: &mut [Objects], bodies: &mut [Bodies], orbit: &mut Orbit) {
    let ent = objects[0].0;
    let new_parent = objects[2].0;

    orbit.parent = new_parent;
    bodies[0].0.remove_vessel(ent);
    bodies[1].0.push_vessel(ent);
}
