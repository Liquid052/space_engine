use crate::components::*;
use crate::prelude::Pack;
use bevy::ecs::query::QueryFilter;
use bevy::ecs::world::Command;
use bevy::prelude::*;
use std::marker::PhantomData;

pub struct PackEntities;

pub struct UnpackEntities;

impl Command for PackEntities {
    fn apply(self, world: &mut World) {
        let mut entities_to_pack = Vec::new();
        {
            let mut query = world.query_filtered::<Entity, (With<Pack>, With<Unpacked>)>();
            entities_to_pack.extend(query.iter(world));
        }

        for entity in entities_to_pack {
            world.entity_mut(entity).remove::<Unpacked>();
        }
    }
}

pub struct PackEntitiesVec(pub Vec<Entity>);

impl Command for PackEntitiesVec {
    fn apply(self, world: &mut World) {
        for entity in self.0 {
            world.entity_mut(entity).remove::<Unpacked>();
        }
    }
}

pub struct PackEntitiesFilter<T> {
    phantom_data: PhantomData<T>,
}

impl<T> PackEntitiesFilter<T> {
    //noinspection RsExternalLinter
    pub fn new() -> Self {
        Self {
            phantom_data: PhantomData
        }
    }
}

impl<T: QueryFilter + Send + Sync + 'static> Command for PackEntitiesFilter<T> {
    fn apply(self, world: &mut World) {
        let mut entities_to_pack = Vec::new();
        {
            let mut query = world.query_filtered::<Entity, T>();
            entities_to_pack.extend(query.iter(world));
        }

        for entity in entities_to_pack {
            world.entity_mut(entity).remove::<Unpacked>();
        }
    }
}


impl Command for UnpackEntities {
    fn apply(self, world: &mut World) {
        let mut entities_to_unpack = Vec::new();
        {
            let mut query = world.query_filtered::<Entity, (With<Pack>, Without<Unpacked>)>();
            entities_to_unpack.extend(query.iter(world));
        }

        for entity in entities_to_unpack {
            world.entity_mut(entity).insert(Unpacked);
        }
    }
}