use std::ops::Mul;

use bevy::{
    core::Name,
    hierarchy::BuildChildren,
    math::Quat,
    prelude::{default, Color, Commands, Entity, Mut, Query, SpatialBundle, Transform},
};
use bevy_prototype_lyon::{draw::Stroke, entity::ShapeBundle};

pub use bodies::*;
pub use vessels::*;

use crate::{
    components::{Body, DrawSpace, Keplerian, Orbit, OrbitOutline, RefFrame, SpacePos, StateVec},
    constants::SPACE_LAYER,
};

mod bodies;
mod vessels;

// aliases
type ObjectParams<'a> = (
    Entity,
    &'a mut Keplerian,
    &'a mut StateVec,
    &'a Name,
    Option<&'a mut DrawSpace>,
);
type BodyParams<'a> = (&'a mut RefFrame, &'a mut Body);

type Objects<'a> = (
    Entity,
    Mut<'a, Keplerian>,
    Mut<'a, StateVec>,
    &'a Name,
    Option<Mut<'a, DrawSpace>>,
);
type Bodies<'a> = (Mut<'a, RefFrame>, Mut<'a, Body>);

// helper functions
fn iter_children(
    parents: &Query<(Entity, &Body, &RefFrame, &SpacePos)>,
    parent: Entity,
    func: impl FnMut((Entity, &Body, &SpacePos)),
) {
    let parent_ref_frame = parents.get(parent).unwrap().2;

    parent_ref_frame
        .children_bodies()
        .iter()
        .map(|&child_ent| {
            let (child_ent, body, _, space_pos) = parents.get(child_ent).unwrap();
            (child_ent, body, space_pos)
        })
        .for_each(func);
}

fn orbit(objects: &mut [Objects], parent_mass: f64, orbit: &mut Orbit) {
    let new_parent = objects[2].0;
    let period = objects[0].1.period(parent_mass);

    orbit.period = period;
    orbit.epoch = 0.0;
    orbit.parent = new_parent;
}

fn recalculate_keplerian(bodies: &mut [Objects], parent_mass: f64, up: bool) {
    let st_vec1 = *bodies[0].2;
    let parent = if up { 1 } else { 2 };
    let st_vec2 = *bodies[parent].2;
    let mul = if up { 1.0 } else { -1.0 };

    let new_st_vec = StateVec {
        position: st_vec1.position + st_vec2.position.mul(mul),
        velocity: st_vec1.velocity + st_vec2.velocity.mul(mul),
    };

    // update state vectors and keplerian
    *bodies[0].2 = new_st_vec;
    *bodies[0].1 = new_st_vec.to_elements(parent_mass);

    // correction due to inclination occasionally flipping
    let st = bodies[0]
        .1
        .state_vectors_at_epoch(parent_mass, 0.0, 2.220_446_049_250_313e-10);

    if st.position.distance_squared(new_st_vec.position) > 1000.0 {
        bodies[0].1.inclination -= std::f64::consts::PI;
    }
}

fn update_orbit_if_exists(
    commands: &mut Commands,
    old_parent: Entity,
    new_parent: Entity,
    name: Name,
    objects: &mut [Objects],
) {
    if let Some(draw) = objects[0].4.as_mut() {
        if let Some(orbit_ent) = draw.orbit {
            commands.entity(old_parent).remove_children(&[orbit_ent]);
            commands.entity(orbit_ent).despawn();

            update_orbit(
                commands,
                draw.as_mut(),
                name.as_str(),
                &objects[0].1,
                new_parent,
            );
        }
    }
}

fn update_orbit(
    com: &mut Commands,
    draw_space: &mut DrawSpace,
    name: &str,
    keplerian: &Keplerian,
    parent_parent: Entity,
) {
    const B: f32 = 0.5;

    let mut id = Entity::from_raw(0);

    com.entity(parent_parent).with_children(|ch_builder| {
        id = ch_builder
            .spawn((
                ShapeBundle {
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, -0.5).with_rotation(
                            Quat::from_rotation_z(keplerian.argument_of_periapsis as f32),
                        ),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(Color::srgba(B, B, B, 0.2), 20.0),
                SPACE_LAYER,
                Name::new(format!("Orbit {}", name)),
                OrbitOutline,
            ))
            .id();
    });

    draw_space.orbit = Some(id);
}
