use bevy::ecs::entity::MapEntities;
use bevy::prelude::*;
use bevy::utils::HashMap;
use engine_core::prelude::PosKey;

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct SpaceWorld {
    pub(crate) galaxies: HashMap<PosKey, Entity>,
    pub(crate) root: Entity,
}

impl SpaceWorld {
    // constructors
    pub fn new(root: Entity) -> Self {
        Self {
            galaxies: HashMap::default(),
            root,
        }
    }

    // manager
    pub fn add_galaxy(&mut self, key: PosKey, ent: Entity) {
        self.galaxies.insert(key, ent);
    }
}

impl MapEntities for SpaceWorld {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.galaxies.iter_mut()
            .for_each(|(_key, ent)| *ent = entity_mapper.map_entity(*ent));

        self.root = entity_mapper.map_entity(self.root);
    }
}