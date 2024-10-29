use crate::prelude::Galaxy;
use bevy::color::palettes::css::*;
use bevy::ecs::system::RunSystemOnce;
use bevy::ecs::world::Command;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct PopulateGalaxyGrid;

impl Command for PopulateGalaxyGrid {
    fn apply(self, world: &mut World) {
        world.run_system_once(populate_grid);
    }
}

fn populate_grid(
    mut commands: Commands,
    galaxies: Query<&Galaxy>,
) {
    let Ok(galaxy) = galaxies.get_single() else {
        return;
    };

    let cell_size = galaxy.cell_size;
    let (map_width, map_height) = (galaxy.map_size.x as f32, galaxy.map_size.y as f32);
    let (width_cnt, height_cnt) = (map_width as i32, map_height as i32);
    let (width_edge, height_edge) = (width_cnt / cell_size, height_cnt / cell_size);
    let offset = Vec2::new(map_width / 2.0, map_height / 2.0);

    let mut path_builder = PathBuilder::new();
    for x in 1..=width_edge {
        let pos = Vec2::new(
            (x * cell_size) as f32 - offset.x,
            0.0 * cell_size as f32 - offset.y,
        );

        path_builder.move_to(pos);
        path_builder.line_to(pos + Vec2::Y * map_height);
        path_builder.close();
    }
    for y in 1..=height_edge {
        let pos = Vec2::new(
            -offset.x,
            (y * cell_size) as f32 - offset.y,
        );

        path_builder.move_to(pos);
        path_builder.line_to(pos + Vec2::X * map_width);
        path_builder.close();
    }


    commands.spawn((
        ShapeBundle {
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
            path: path_builder.build(),
            ..default()
        },
        Stroke::new(BLACK, 2.0),
    ));
}