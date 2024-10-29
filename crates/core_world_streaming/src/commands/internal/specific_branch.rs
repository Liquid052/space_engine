use bevy::ecs::world::Command;
use bevy::prelude::*;

pub struct Save {
    pub ent: Entity,
}

impl Command for Save {
    fn apply(self, _world: &mut World) {
        todo!()
    }
}

pub struct Load {
    pub ent: Entity,
}

impl Command for Load {
    fn apply(self, _world: &mut World) {
        todo!()
    }
}