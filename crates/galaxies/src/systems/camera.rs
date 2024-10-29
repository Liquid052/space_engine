use crate::prelude::Galaxy;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;


// constants
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 1.0;
const ZOOM_SPEED: f32 = 5.0;
const MOUSE_SPEED: f32 = 100.0;

pub fn cam_controls(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform)>,
    galaxy: Query<&Galaxy>,
    time: Res<Time>,
    mut scroll_evr: EventReader<MouseWheel>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,  // Detect mouse input
    mut evr_motion: EventReader<MouseMotion>,  // Track mouse position
) {
    let Ok(galaxy) = galaxy.get_single() else {
        return;
    };
    let (mut projection, mut transform) = camera_query.single_mut();

    projection.scaling_mode = ScalingMode::AutoMax { max_width: galaxy.map_size.x as f32, max_height: galaxy.map_size.y as f32 };

    // Handle zoom
    for ev in scroll_evr.read() {
        let zoom_delta = -ev.y * ZOOM_SPEED * time.delta_seconds();
        projection.scale += zoom_delta;
    }


    // Handle camera movement
    if input.pressed(KeyCode::ControlLeft) || mouse_input.pressed(MouseButton::Right) {
        let computed_delta = projection.scale * time.delta_seconds() * MOUSE_SPEED;

        evr_motion.read()
            .for_each(|ev| {
                transform.translation.x -= ev.delta.x * computed_delta;
                transform.translation.y += ev.delta.y * computed_delta;
            });
    }
}

pub fn clamp_camera_values(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform)>,
    galaxy: Query<&Galaxy>,
) {
    let Ok(galaxy) = galaxy.get_single() else {
        return;
    };
    let (mut ortho, mut transform) = camera_query.single_mut();

    // clamp camera's zoom
    ortho.scale = ortho.scale.clamp(MIN_ZOOM, MAX_ZOOM);

    // Clamp camera position
    let projection_width_half = ortho.area.width() / 2.0;
    let projection_height_half = ortho.area.height() / 2.0;

    if projection_height_half.is_nan() || projection_width_half.is_nan() {
        return;
    }

    let map = galaxy.map_size.as_vec2() / 2.0;

    let max_x = map.x - projection_width_half;
    let max_y = map.y - projection_height_half;

    transform.translation.x = transform.translation.x.clamp(-max_x, max_x);
    transform.translation.y = transform.translation.y.clamp(-max_y, max_y);

    // center camera's position if the camera is larger than the map
    if ortho.area.width() >= map.x * 2.0 {
        transform.translation.x = 0.0;
    }
    if ortho.area.height() >= map.y * 2.0 {
        transform.translation.y = 0.0;
    }

    transform.translation.x = transform.translation.x.round();
    transform.translation.y = transform.translation.y.round();
}


