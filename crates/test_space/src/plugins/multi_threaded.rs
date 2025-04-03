use crate::plugins::physics::PhysicsPlugin;
use bevy::prelude::*;

pub struct SOASpacePlugin;

impl Plugin for SOASpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugin);
    }
}