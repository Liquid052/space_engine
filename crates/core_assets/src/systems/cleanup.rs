use bevy::prelude::*;

use crate::prelude::*;

pub fn cleanup(world: &mut World) {
    world.remove_resource::<ModPaths>();
    world.remove_resource::<ModsCache>();
    world.remove_resource::<ContentCache>();
}