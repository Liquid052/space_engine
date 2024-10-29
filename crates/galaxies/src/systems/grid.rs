use crate::prelude::*;
use bevy::prelude::*;
use engine_core::prelude::PosKey;

pub fn update_grid(
    mut galaxies: Query<&mut Galaxy>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(mut galaxy) = galaxies.get_single_mut() else {
        return;
    };
    galaxy.current_grid_pos = None;
    galaxy.mouse_pos = None;
    galaxy.active_entity = None;

    let (Ok(window), Ok(cam)) = (windows.get_single(), camera.get_single()) else {
        return;
    };
    let (camera, camera_transform) = cam;

    let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    else {
        return;
    };

    let min = IVec2::ZERO;
    let max = galaxy.map_size / galaxy.cell_size;

    let grid_start = galaxy.map_size.as_vec2() / -2.0;
    let cell_size = galaxy.cell_size as f32;

    let offset = Vec2::splat(cell_size / 2.0);
    let pos = world_position - grid_start - offset;
    let snapped_pos = (pos / cell_size).round() * cell_size;
    let grid_coords = snapped_pos.as_ivec2() / cell_size as i32;

    if grid_coords.x <= min.x || grid_coords.y <= min.y ||
        grid_coords.x >= max.x || grid_coords.y >= max.y
    {
        return;
    }

    if let Some(child) = galaxy.children.get_mut(&PosKey::Pos(grid_coords)).copied() {
        galaxy.selected_ent = Some(child);
    }
    galaxy.current_grid_pos = Some(grid_coords);
    galaxy.mouse_pos = Some(pos);
}