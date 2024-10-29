use crate::prelude::{SpaceWorld, WorldManager};
use bevy::ecs::world::Command;
use bevy::prelude::World;

pub struct SpawnWorld;

impl Command for SpawnWorld {
    fn apply(self, world: &mut World) {
        let id = world.spawn(WorldManager).id();

        world.insert_resource(SpaceWorld::new(id));
    }
}