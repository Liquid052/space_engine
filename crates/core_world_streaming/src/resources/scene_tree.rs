use crate::components::{FullPosKey, PosKey};
use bevy::ecs::entity::MapEntities;
use bevy::prelude::*;
use bevy::prelude::{default, Entity, EntityMapper, Reflect, Resource};
use bevy::utils::HashMap;

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct SceneTree {
    pub(crate) layers: Vec<String>,
}


impl SceneTree {
    // constructor
    pub(crate) fn new(root: Entity) -> Self {
        SceneTree {
            layers: Vec::new(),
        }
    }
    // pub(crate) fn register(&mut self, key: PosKey, ent: Entity, parent: Option<Entity>) {
    //     // get full pos key
    // 
    //     let full_key = match parent {
    //         Some(parent) => {
    //             let parent_key = self.rev.get(&parent).unwrap();
    //             let mut full_key = parent_key.clone();
    //             full_key.0.push(key);
    // 
    //             full_key
    //         },
    //         None => FullPosKey(vec![key])
    //     };
    // 
    //     // insert into map
    //     self.map.insert(format!("{}",full_key), ent);
    //     self.rev.insert(ent, full_key);
    // }
    pub(crate) fn remove(&mut self, ent: Entity) {
        // if let Some(key) = self.rev.remove(&ent) {
        //     self.map.remove(&format!("{}",key));
        // }
    }
    pub(crate) fn register_key(&mut self, key: FullPosKey, ent: Entity) {


        // insert into map
        // self.map.insert(format!("{}",key), ent);
        // self.rev.insert(ent, key);
    }

    // pub fn root(&self) -> Entity {
    // self.root
    // }
    // pub fn get(&self, key: &FullPosKey) -> Option<Entity> {
    // self.map.get(&format!("{}",key)).copied()
    // }
    // pub fn get_key(&self, ent: Entity) -> Option<&FullPosKey> {
    // self.rev.get(&ent)
    // }

    // pub fn active_entities(&self) -> &[Entity] {
    // &self.active_entities
    // }
    // pub fn last_active_entity(&self) -> Option<Entity> {
    // self.active_entities.last().copied()
    // }
}