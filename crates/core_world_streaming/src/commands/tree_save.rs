use std::any::TypeId;

use std::fs::File;
use std::io::Write;

use bevy::ecs::system::RunSystemOnce;
use bevy::ecs::world::Command;
use bevy::prelude::*;

use crate::prelude::{ProcessedScene, SceneProcessor};
use bevy::tasks::IoTaskPool;

use crate::helpers::scene_tree_load;
use crate::prelude::{LoadAllActiveSceneNodes, SceneLayerReg, SceneNode, SceneTree};

pub struct SaveCurrentSceneTree;

impl Command for SaveCurrentSceneTree {
    fn apply(self, world: &mut World) {
        if !world.resource::<SceneLayerReg>().contains_path() {
            return;
        }

        let name = world.get_resource::<SceneLayerReg>().unwrap().write_dir.clone().unwrap();
        let name = format!("{}/scene_tree.scn.ron", name);
        let mut query = world.query_filtered::<Entity, With<SceneNode>>();

        let entities: Vec<Entity> = query.iter(world).collect();

        let entities_2 = entities.clone();
        let mut scene_world = World::new();

        // insert tree
        scene_world.insert_resource(world.resource::<SceneTree>().clone());

        let mut hash_set = std::collections::HashSet::new();
        hash_set.insert(TypeId::of::<SceneNode>());

        let mut scene = DynamicSceneBuilder::from_world(&world)
            .deny::<Handle<Image>>()
            .extract_entities(entities.into_iter())

            .build();


        let scene_tree = world.remove_resource::<SceneTree>().unwrap();
        scene.resources.push(Box::new(scene_tree));

        // cleanup
        world.remove_resource::<SceneTree>();
        entities_2.iter()
            .for_each(|ent| {
                world.despawn(*ent);
            });
        scene_world.insert_resource(world.resource::<AppTypeRegistry>().clone());

        // Scenes can be serialized like this:
        let type_registry = world.resource::<AppTypeRegistry>();
        let type_registry = type_registry.read();
        let serialized_scene = scene.serialize(&type_registry).unwrap();

        // write to disk
        #[cfg(not(target_arch = "wasm32"))]
        IoTaskPool::get()
            .spawn(async move {
                // Write the scene RON data to file
                File::create(&name)
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing scene to file");
            })
            .detach();
    }
}

pub struct LoadSceneTree(pub String);

impl Command for LoadSceneTree {
    fn apply(self, world: &mut World) {
        if world.contains_resource::<SceneTree>() {
            return;
        }

        world.run_system_once_with(self.0, load_scene_tree);
    }
}

fn load_scene_tree(
    In(path): In<String>,
    mut scene_processor: ResMut<SceneProcessor>,
    mut scene_layer_reg: ResMut<SceneLayerReg>,
    asset_server: Res<AssetServer>,
) {
    let scene_path = scene_tree_load(&path);
    let dyn_scene_handle = asset_server.load(scene_path);

    scene_processor.add(ProcessedScene::new(dyn_scene_handle.clone())
        .hook(Box::new(load))
    );

    // set scene layer reg
    scene_layer_reg.init_save(path);
}


fn load(world: &mut World, mut scene: Scene) -> Result<Scene, String> {
    let scene_world = &mut scene.world;

    scene_world.commands()
        .add(LoadAllActiveSceneNodes(world.resource::<SceneLayerReg>().clone()));

    Ok(scene)
}

//     In(path): In<String>
// let path = crate::helpers::scene_tree_load(path.as_str());

// scenes: Res<Assets<Scene>>,
// mut post_processor: ScenePostProcessor,
// let processed_scene = post_processor.process(handle, vec![
//     Arc::new(move |world: &mut World| {
//         info!("LOADED");
//         //world.commands()
//         //     .add(LoadAllActiveSceneNodes);
//
//         // world.flush_commands();
//
//         Ok(())
//     })
// ]);

