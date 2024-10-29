use crate::prelude::FullPosKey;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct SceneMap {
    root: Option<Entity>,
    map: HashMap<FullPosKey, Entity>,
    rev: HashMap<Entity, FullPosKey>,
}

impl SceneMap {
    pub fn root(&self) -> Option<Entity> {
        self.root
    }
    pub fn set_root(&mut self, root: Entity) {
        self.root = Some(root);
    }
    pub fn clear_root(&mut self) {
        self.root = None;
    }
    pub fn map(&self) -> &HashMap<FullPosKey, Entity> {
        &self.map
    }
    pub fn rev(&self) -> &HashMap<Entity, FullPosKey> {
        &self.rev
    }

    pub fn insert(&mut self, key: FullPosKey, ent: Entity) {
        self.map.insert(key.clone(), ent);
        self.rev.insert(ent, key);
    }

    pub fn remove(&mut self, ent: Entity) {
        if let Some(key) = self.rev.remove(&ent) {
            self.map.remove(&key);
        }
    }

    pub fn get(&self, key: &FullPosKey) -> Option<Entity> {
        self.map.get(key).copied()
    }

    pub fn get_key(&self, ent: Entity) -> Option<&FullPosKey> {
        self.rev.get(&ent)
    }
    pub fn contains(&self, key: &FullPosKey) -> bool {
        self.map.contains_key(key)
    }
    pub fn contains_key(&self, ent: Entity) -> bool {
        self.rev.contains_key(&ent)
    }
}

