use crate::components::SceneNode;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use core_pack_objects::prelude::Unpacked;
use std::fs::File;
use std::io::Write;

// extracting - removes original entities
// cloning - keeps original entities and adds them to a scene
pub struct SceneSaver<'w> {
    world: &'w mut World,
    scene_node: &'w SceneNode,
    entities: Vec<Entity>,
    entity_cache: Vec<Entity>,
    pack: bool,

    dyn_scene_builders: Vec<Box<dyn Fn(DynamicSceneBuilder) -> DynamicSceneBuilder>>,
    resource_moves: Vec<Box<dyn Fn(&mut World, DynamicScene) -> DynamicScene>>,
}

impl<'w> SceneSaver<'w> {
    pub fn new(world: &'w mut World, scene_node: &'w SceneNode) -> Self {
        Self {
            world,
            scene_node,
            dyn_scene_builders: default(),
            resource_moves: default(),
            entity_cache: default(),
            entities: default(),
            pack: false,
        }
    }
    pub fn save(mut self) {
        let mut pack_entities: Vec<usize> = vec![];
        // gather entities to unpack
        if self.pack {
            let mut query = self.world.query::<(Entity, Option<&Unpacked>)>();

            let mut id = 0;
            query.iter_many(self.world, &mut self.entities)
                .for_each(|(_, pack)| {
                    if pack.is_some() {
                        pack_entities.push(id);
                    }

                    id += 1;
                });

            pack_entities.iter()
                .map(|id| { self.entities[*id] })
                .for_each(|ent| {
                    self.world.entity_mut(ent)
                        .remove::<Unpacked>();
                });
            self.world.flush_commands();
        }

        let mut dyn_scene_builder = DynamicSceneBuilder::from_world(&self.world)
            .extract_entities(self.entities.iter()
                .map(|ent| *ent)
            )
            .deny::<Handle<Image>>();

        // pass through filters
        for builder in self.dyn_scene_builders {
            dyn_scene_builder = builder(dyn_scene_builder);
        }
        let mut dyn_scene = dyn_scene_builder.build();

        // pack_entities.iter()
        //     .for_each(|id| {
        //         dyn_scene.entities[*id].components.push(Box::new(Unpacked));
        //     });
        self.entities.iter().for_each(|ent| {
            self.world.entity_mut(*ent).despawn();
        });

        // apply resource moves
        for move_fn in self.resource_moves {
            dyn_scene = move_fn(&mut self.world, dyn_scene);
        }

        let type_registry = self.world.get_resource::<AppTypeRegistry>().unwrap().clone();
        let type_registry = type_registry.read();

        let serialized_scene = dyn_scene.serialize(&type_registry).unwrap();

        let path = self.scene_node.scene_path.as_ref().unwrap().to_string();

        // write to disk
        #[cfg(not(target_arch = "wasm32"))]
        IoTaskPool::get()
            .spawn(async move {
                File::create(path)
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing scene to file");
            })
            .detach();
    }
    // add method that would filter out entities that satisfy the predicate
    pub fn filter_entities(mut self, predicate: impl Fn(&EntityRef) -> bool + 'static) -> Self {
        let mut query = self.world.query::<EntityRef>();

        let cache = &mut self.entity_cache;
        query.iter_many(self.world, &mut self.entities)
            .filter(|ent_ref| { predicate(ent_ref) })
            .for_each(|ent_ref| cache.push(ent_ref.id()));

        self.entities.clear();
        self.entities.append(cache);

        self
    }
    pub fn pack(mut self) -> Self {
        self.pack = true;
        self
    }
    pub fn extract_entities<Q: QueryFilter>(mut self) -> Self {
        let mut query = self.world.query_filtered::<Entity, Q>();

        query.iter(self.world)
            .for_each(|ent| self.entities.push(ent));

        self
    }


    // component based
    pub fn deny<T: Component>(self) -> Self {
        // self.builder_update.push(Box::new(deny_component::<T>));
        self
    }
    pub fn extract_resource<T: Resource + Reflect>(mut self) -> Self {
        self.resource_moves.push(Box::new(extract_resource::<T>));
        self
    }
    pub fn allow<T: Component>(self) -> Self {
        // self.builder_update.push(Box::new(deny_component::<T>));
        // let mut k: DynamicSceneBuilder = todo!();

        self
    }
}

// helpers
fn extract_resource<T: Resource + Reflect>(world: &mut World, mut dynamic_scene: DynamicScene) -> DynamicScene {
    let res = world.remove_resource::<T>().unwrap();

    dynamic_scene.resources.push(Box::new(res));

    dynamic_scene
}


fn _allow<T: Component>(dynamic_scene_builder: DynamicSceneBuilder) -> DynamicSceneBuilder {
    dynamic_scene_builder.allow::<T>()
}
fn _deny<T: Component>(dynamic_scene_builder: DynamicSceneBuilder) -> DynamicSceneBuilder {
    dynamic_scene_builder.deny::<T>()
}

fn _allow_resource<T: Resource>(dynamic_scene_builder: DynamicSceneBuilder) -> DynamicSceneBuilder {
    dynamic_scene_builder.allow_resource::<T>()
}
fn _deny_resource<T: Resource>(dynamic_scene_builder: DynamicSceneBuilder) -> DynamicSceneBuilder {
    dynamic_scene_builder.deny_resource::<T>()
}

