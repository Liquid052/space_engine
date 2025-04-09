use crate::{
    components::{Orbit, RefFrame, StateVec},
    prelude::*,
    systems::transfer::{
        iter_children, orbit, recalculate_keplerian, update_orbit_if_exists, Bodies, BodyParams,
        ObjectParams, Objects,
    },
};
use bevy::prelude::*;

type BodyP<'a> = (
    Entity,
    &'a StateVec,
    &'a SpacePos,
    &'a Body,
    &'a Orbit,
    Option<&'a Exited>,
);

pub fn check_body_transfer(
    orbits: Query<BodyP, (Changed<StateVec>, With<Orbit>, Without<Restricted>)>,
    parents: Query<(Entity, &Body, &RefFrame, &SpacePos)>,
    orbits_p: Query<&Orbit>,
    mut writer: EventWriter<BodyTransferEvent>,
) {
    orbits
        .iter()
        .for_each(|(ent, state_vec, space_pos, body, orbit, exit)| {
            let parent = orbit.parent();
            let planet_soi_squared = parents.get(parent).unwrap().1.soi.powi(2);

            // check if exits soi of parent
            if planet_soi_squared < state_vec.position.length_squared() {
                let new_parent = orbits_p.get(parent).unwrap().parent();

                writer.send(BodyTransferEvent::new(ent, parent, new_parent, true));
                return;
            }

            // check if body enters child soi
            iter_children(&parents, parent, |tuple| {
                let (child_ent, child_body, child_space_pos) = tuple;

                if child_ent == ent || child_body.mass < body.mass {
                    return;
                }

                if let Some(ent) = exit {
                    if child_ent == **ent {
                        return;
                    }
                }

                let relative_dist = space_pos.0 - child_space_pos.0;
                let soi_squared = child_body.soi.powi(2);

                if soi_squared > relative_dist.length_squared() {
                    writer.send(BodyTransferEvent::new(ent, parent, child_ent, false));
                }
            });
        });
}

pub fn handle_body_transfer(
    mut reader: EventReader<BodyTransferEvent>,
    mut commands: Commands,
    mut objects: Query<ObjectParams>,
    mut bodies_q: Query<BodyParams>,
    mut orbits: Query<&mut Orbit>,
    mut depth: Query<&mut SpaceDepth>,
) {
    reader.read().for_each(|ev| {
        let ent = ev.entity;
        let old_parent = ev.old_parent;
        let new_parent = ev.new_parent;
        let name = objects.get(ent).unwrap().3.clone();

        let up = ev.up;

        let mut objects = objects.get_many_mut([ent, old_parent, new_parent]).unwrap();
        let mut bodies = bodies_q
            .get_many_mut([ent, old_parent, new_parent])
            .unwrap();

        recalculate_keplerian(&mut objects, bodies[2].1.mass, up);

        ref_params(
            &mut objects,
            &mut bodies,
            orbits.get_mut(ent).unwrap().as_mut(),
        );

        body_params(&mut objects, &mut bodies);

        if up {
            commands.entity(ent).insert(Exited(old_parent));
        }

        orbit(
            &mut objects,
            bodies[2].1.mass,
            orbits.get_mut(ent).unwrap().as_mut(),
        );

        // clear old orbit
        update_orbit_if_exists(&mut commands, old_parent, new_parent, name, &mut objects);

        
        let mut frames_lens = bodies_q.transmute_lens::<&RefFrame>();
        let frames = frames_lens.query();
        
        redirect(ent, &mut commands, &frames, &mut depth, up);
        
        // match up {
        //     true => go_up(ent, &mut commands, &frames, &mut depth),
        //     false => go_down(ent, &mut commands, &frames, &mut depth),
        // }
    });
}

fn ref_params(objects: &mut [Objects], bodies: &mut [Bodies], orbit: &mut Orbit) {
    let ent = objects[0].0;
    let new_parent = objects[2].0;

    orbit.parent = new_parent;
    bodies[1].0.remove_body(ent);
    bodies[2].0.push_body(ent);
}

fn body_params(objects: &mut [Objects], bodies: &mut [Bodies]) {
    bodies[0].1.soi = objects[0]
        .1
        .calculate_soi(bodies[0].1.mass, bodies[2].1.mass)
}


//noinspection RsExternalLinter
fn redirect(ent: Entity,
            com: &mut Commands,
            frames: &Query<&RefFrame>,
            depths: &mut Query<&mut SpaceDepth>,
            up: bool
    ) {
    let ref_frame = frames.get(ent).unwrap();

    depths.get_mut(ent).unwrap().redirect(up);

    ref_frame.vessels.iter()
        .for_each(|child_vessel| {
            depths.get_mut(*child_vessel).unwrap().redirect(up);
        });

    ref_frame
        .children_bodies
        .iter()
        .for_each(|child| redirect(*child, com, frames, depths, up));
}
