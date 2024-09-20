use bevy::prelude::*;
use bevy_prototype_lyon::{geometry::GeometryBuilder, shapes};
use crate::{constants::SPACE_SCALE, prelude::*};

pub fn update_belts(
    mut query: Query<(Entity, &mut Belt), Changed<Belt>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    // install belts as children of the body
    query.iter_mut().for_each(|(ent, mut belt)| {
        if belt.is_added() {
           return; 
        }
        
        // iter each belt
        let (belts, entities) = belt.get_pair();

        belts.iter().for_each(|(radius, width, col)| {
            // spawn children and assign it
            commands.entity(ent).with_children(|builder| {
                let path = GeometryBuilder::build_as(&shapes::Circle {
                    radius: (radius / SPACE_SCALE) as f32,
                    ..default()
                });

                let handle = mats.add(ColorMaterial {
                    color: *col,
                    ..default()
                });

                let child_ent = builder
                    .spawn(BeltBundle::new(path, handle, *width, *col))
                    .id();

                
                entities.push(child_ent);
            });
        });
    });
}
