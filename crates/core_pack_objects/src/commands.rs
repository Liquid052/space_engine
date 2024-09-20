use bevy::ecs::world::Command;
use bevy::prelude::*;
use crate::compos::*;
use crate::prelude::Pack;

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