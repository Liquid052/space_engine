use crate::prelude::SceneLayerReg;
use bevy::ecs::world::Command;
use bevy::prelude::*;

pub struct NameScene(pub String);

impl Command for NameScene {
    fn apply(self, world: &mut World) {
        world.resource_mut::<SceneLayerReg>()
            .init_save(self.0);
    }
}

pub struct ClearSceneName;

impl Command for ClearSceneName {
    fn apply(self, world: &mut World) {
        world.resource_mut::<SceneLayerReg>()
            .write_dir = None;
    }
}