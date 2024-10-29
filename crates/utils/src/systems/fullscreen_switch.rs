use bevy::prelude::*;
use bevy::window::WindowMode;

pub fn fullscreen_switch(mut windows: Query<&mut Window>) {
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    window.mode = match window.mode {
        WindowMode::BorderlessFullscreen => WindowMode::Windowed,
        _ => WindowMode::BorderlessFullscreen
    }
}