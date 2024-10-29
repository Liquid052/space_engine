use crate::prelude::NameReg;
use bevy::ecs::world::Command;
use bevy::prelude::{Commands, Entity, Name, World};

pub trait RenameEntity {
    fn rename(&mut self, ent: Entity, name: &str);
}

//noinspection RsExternalLinter
impl<'w, 's> RenameEntity for Commands<'w, 's> {
    fn rename(&mut self, ent: Entity, name: &str) {
        self.add(Rename(ent, name.to_string()));
    }
}


pub(crate) struct Rename(pub Entity,pub String);

impl Command for Rename {
    fn apply(self, world: &mut World) {
        let contains = world.entity_mut(self.0).contains::<Name>();
        let name = Name::new(self.1.clone());

        if contains {
            *world.get_mut::<Name>(self.0).unwrap() = name.clone();
            world.resource_mut::<NameReg>().update(self.0, name.as_str());

            return;
        }

        world.entity_mut(self.0).insert(name.clone());
        world.resource_mut::<NameReg>().insert(self.0, name.as_str());
    }
}