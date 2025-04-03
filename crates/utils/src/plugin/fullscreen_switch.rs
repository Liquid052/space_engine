use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use crate::systems::fullscreen_switch;

pub struct FullscreenSwitchPlugin;

impl Plugin for FullscreenSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fullscreen_switch.run_if(input_just_pressed(KeyCode::F11)));
    }
}