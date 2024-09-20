use bevy::utils::HashMap;
use bevy::prelude::*;

#[derive(Resource, Debug, Reflect, Default)]
#[reflect(Resource)]
pub struct NameReg {
    names:   HashMap<String, Entity>,
    reverse: HashMap<Entity, String>,
}

impl NameReg {
    pub fn get_name(&self, ent: Entity) -> Option<&str> {
        self.reverse.get(&ent).and_then(|string| { Some(string.as_str()) })
    }

    pub fn get(&self, name: &String) -> Option<Entity> {
        self.names.get(name).copied()
    }

    pub(crate) fn remove(&mut self, ent: Entity) -> Option<String> {
        if let Some(name) = self.reverse.remove(&ent) {
            self.names.remove(&name);

            trace!("NameReg: Removed {:?} with name {:?}", ent, name);

            Some(name)
        } else {

            trace!("NameReg: Failed to remove {:?} with name {:?}", ent, self.reverse.get(&ent));

            None
        }
    }

    pub(crate) fn insert(&mut self, ent: Entity, name: impl Into<String>) {
        let name = name.into();

        trace!("NameReg: Inserted {:?} with name {:?}", ent, name);
        if let Some(old_entity) = self.names.insert(name.clone(), ent) {
            self.reverse.remove(&old_entity);
        }

        if let Some(old_name) = self.reverse.insert(ent, name) {
            self.names.remove(&old_name);
        }
    }

    pub(crate) fn update(&mut self, ent: Entity, new_name: &str) {
        if let Some(old_name) = self.reverse.get_mut(&ent) {
            let new_name = String::from(new_name);

            self.names.remove(old_name);
            self.names.insert(new_name.clone(), ent);

            trace!("NameReg: Updated {:?} with name {:?}", ent, new_name);

            *old_name = new_name;


            return;
        }

        self.insert(ent, new_name);
    }
}
