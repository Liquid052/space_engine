use crate::prelude::NameReg;
use bevy::core::Name;
use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

#[derive(Reflect, Default, Copy, Clone)]
#[reflect(Component)]
pub struct UniquelyNamed;

impl Component for UniquelyNamed {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|world, target_ent, _component_id| {
            let mut world: DeferredWorld = world;

            let name = world.get::<Name>(target_ent).cloned();

            if name.is_none() {
                panic!("UniquelyNamed on_add(...) hook - requires Name component");
            }

            let name = name.unwrap().to_string();

            world.resource_mut::<NameReg>().insert(target_ent, &name);
        });

        hooks.on_remove(|mut world, target_ent, _component_id| {
            world.resource_mut::<NameReg>().remove(target_ent);
        });
    }
}