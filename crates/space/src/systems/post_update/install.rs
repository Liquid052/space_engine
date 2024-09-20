use bevy::prelude::*;
use bevy_prototype_lyon::{draw::Stroke, entity::ShapeBundle};

use crate::{constants::SPACE_LAYER, prelude::*};

//aliases
type OrbitDrawNames<'a> = (&'a Name, &'a Orbit, &'a mut DrawSpace);
type OrbitDraws<'a> = (Entity, &'a mut DrawSpace);

pub fn install_draw(
    query: Query<Entity, (Added<Orbit>, Added<Keplerian>)>,
    mut commands: Commands,
) {
    query.iter().for_each(|ent| {
        commands.entity(ent).insert(DrawSpace::default());
    });
    
    
}

pub fn install_orbit_outline(
    mut bodies: Query<OrbitDrawNames, (Added<Orbit>, Without<StarMarker>)>,
    mut commands: Commands,
) {
    bodies.iter_mut().for_each(|(name, orbit, mut draw_space)| {
        let orbiting = orbit.parent();

        const B: f32 = 0.5;
        let mut id = Entity::from_raw(0);

        commands.entity(orbiting).with_children(|ch_builder| {
            id = ch_builder
                .spawn((
                    ShapeBundle {
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(0.0, 0.0, -0.5),
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
    });
}

pub fn install_soi_outline(
    mut query: Query<OrbitDraws, (Or<(With<Body>, With<TwoBody>)>, Added<Orbit>)>,
    mut commands: Commands,
) {
    const SOI_NAME: &str = "SOI";

    query.iter_mut().for_each(|(ent, mut draw_space)| {
        const B: f32 = 0.7;

        let mut id = Entity::from_raw(0);

        commands.entity(ent).with_children(|ch_builder| {
            id = ch_builder
                .spawn((
                    ShapeBundle {
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(0.0, 0.0, -0.4),
                            ..default()
                        },
                        ..default()
                    },
                    Stroke::new(Color::srgba(B, B, 0.0, 0.2), 20.0),
                    Name::new(SOI_NAME),
                    SPACE_LAYER,
                    SOI,
                ))
                .id();

            draw_space.soi = Some(id);
        });
    });
}
