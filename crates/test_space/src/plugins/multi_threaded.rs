use crate::plugins::physics::PhysicsPlugin;
use bevy::prelude::*;

pub struct MultithreadedSpacePlugin;

impl Plugin for MultithreadedSpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugin);
    }
}