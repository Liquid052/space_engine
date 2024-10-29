use crate::prelude::*;
use bevy::asset::LoadState;
use bevy::ecs::entity::{MapEntities, SceneEntityMapper};
use bevy::prelude::*;
use bevy::scene::SceneInstance;
use bevy::utils::EntityHashMap;
use core_pack_objects::prelude::{Pack, TaggedObjDB, Unpacked};


pub fn process_scene_hooks(world: &mut World, _cache: Local<Vec<Entity>>) {
    let asset_server = world.resource::<AssetServer>().clone();
    let app_type_reg = world.resource::<AppTypeRegistry>().clone();
    let mut scene_processor = world.get_resource::<SceneProcessor>().unwrap().clone();
    {
        let mut reader = scene_processor.lock();

        reader.retain_mut(|processed_scene| {
            match asset_server.get_load_state(&processed_scene.dyn_scene).unwrap() {
                LoadState::NotLoaded => { return false; }
                LoadState::Loading => { return true; }
                LoadState::Loaded => {}
                LoadState::Failed(err) => {
                    error!("Failed to load scene: {:?}", err);
                    return false;
                }
            };

            let handle = processed_scene.dyn_scene.clone();
            let mut dyn_scenes = world.get_resource_mut::<Assets<DynamicScene>>().unwrap();
            let dyn_scene = dyn_scenes.get_mut(&handle).unwrap();


            // let mut tree = None;
            // let mut o_id = None;
            // dyn_scene.resources.iter().enumerate().for_each(|(id,key)| {
            //     if key.is::<SceneTree>() {
            //         tree = Some(key.downcast_ref::<SceneTree>().unwrap().clone());
            //         o_id = Some(id);
            //     };

            // info!("Resource: {:?}", key);
            // });

            // if let Some(id) = o/**/_id {
            //     dyn_scene.resources.remove(id);
            // }


            // if let Some(tree) = tree {
            //     world.insert_resource(tree);
            // }

            let dyn_scenes = world.get_resource::<Assets<DynamicScene>>().unwrap();
            let dyn_scene = dyn_scenes.get(&handle).unwrap();

            let mut scene = Scene::from_dynamic_scene(&dyn_scene, &app_type_reg)
                .unwrap();


            processed_scene.hooks.push(Box::new(unpack_entities));

            let scene_world = &mut scene.world;
            scene_world.insert_resource(world.resource::<AssetServer>().clone());
            scene_world.insert_resource(world.resource::<TaggedObjDB>().clone());
            scene_world.insert_resource(world.resource::<AppTypeRegistry>().clone());
            scene_world.insert_resource(world.resource::<SceneLayerReg>().clone());
            scene_world.insert_resource(world.resource::<SceneProcessor>().clone());

            for hook in &processed_scene.hooks {
                let next_scene = hook(world, scene);

                if let Err(err) = next_scene {
                    error!("Failed to process scene hook: {:?}", err);
                    return false;
                }

                scene = next_scene.unwrap();
            }

            scene.world.flush_commands();

            let scene_world = &mut scene.world;
            scene_world.remove_resource::<AssetServer>();
            scene_world.remove_resource::<TaggedObjDB>();
            scene_world.remove_resource::<AppTypeRegistry>();
            scene_world.remove_resource::<SceneLayerReg>();
            scene_world.remove_resource::<SceneProcessor>();


            // turn to handle and spawn
            let scene = world.get_resource_mut::<Assets<Scene>>().unwrap().add(scene);

            world.spawn(SceneBundle {
                scene,
                ..default()
            });

            false
        });
    }

    scene_processor.move_subscenes();
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SceneHookSet;


fn unpack_entities(_: &mut World, mut scene: Scene) -> Result<Scene, String> {
    let scene_world = &mut scene.world;

    let mut query = scene_world.query_filtered::<Entity, (With<Pack>, Without<Unpacked>)>();
    let vec: Vec<Entity> = query.iter(scene_world).collect();

    for ent in vec {
        scene_world.entity_mut(ent).insert(Unpacked);
    }

    Ok(scene)
}

pub fn remove_parents_from_scene_hooks(query: Query<(Entity, &Children), Added<SceneInstance>>, mut commands: Commands) {
    query.iter().for_each(|(ent, children)| {
        commands.entity(ent)
            .remove::<SceneInstance>()
            // .remove::<Handle<Scene>>()
            .remove_children(*&children)
            .despawn();
    });
}

