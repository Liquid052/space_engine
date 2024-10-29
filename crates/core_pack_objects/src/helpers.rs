use crate::components::Pack;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;


// helper for smoother interaction
#[derive(Deref, DerefMut)]
pub struct EntityTransformer<'w> {
    pub(crate) entity: Entity,

    #[deref]
    pub(crate) world: DeferredWorld<'w>
}

impl<'w> EntityTransformer<'w> {
    pub(crate) fn new(entity: Entity, world: DeferredWorld<'w>) -> Self {
        Self {
            entity,
            world,
        }
    }

    pub fn main_entity(&self) -> Entity {
        self.entity
    }
    pub fn main_entity_ref(&self) -> EntityRef {
        self.world.entity(self.entity)
    }
    pub fn main_entity_mut(&mut self) -> EntityMut {
        self.world.entity_mut(self.entity)
    }
    pub fn pack(&self) -> &Pack {
        self.world.entity(self.entity).get::<Pack>().expect("Entity doesnt have Tags compo")
    }


    pub fn world(&mut self) -> &mut DeferredWorld<'w> {
        &mut self.world
    }
}