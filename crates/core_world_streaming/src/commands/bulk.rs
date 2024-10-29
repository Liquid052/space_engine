use crate::prelude::*;
use bevy::ecs::world::Command;
use bevy::prelude::*;

pub struct SaveAllActiveSceneNodes;

impl Command for SaveAllActiveSceneNodes {
    fn apply(self, world: &mut World) {
        if !world.resource::<SceneLayerReg>().contains_path() {
            return;
        }

        let mut query = world
            .query_filtered::<(Entity, &mut SceneNode, &FullPosKey), With<SceneNodeActive>>();
        let reg = world.resource::<SceneLayerReg>().clone();

        let vec: Vec<Entity> = query.iter_mut(world)
            .sort::<&SceneNode>()
            .map(|(ent, mut scene_node, scene_key)| {
                // generate scene_path
                if let None = scene_node.scene_path {
                    scene_node.scene_path = Some(format!("{}/{}.scn.ron",
                                                         reg.write_dir.clone().expect("Initialize scene tree"),
                                                         scene_key
                    ).into()
                    );
                }

                ent
            })
            .collect();


        let mut query = world.query::<&SceneNode>();
        world.resource::<SceneLayerReg>().write_dir.clone().expect("Initialize scene tree");
        {
            let reader = reg.read();

            let mut scene_nodes = vec![];
            query.iter_many(world, vec.as_slice())
                .for_each(|scene_node| {
                    // let layer_name = &scene_node.layer_name;
                    scene_nodes.push(scene_node.clone());


                    // layer.save(&scene_node);
                });

            scene_nodes.iter()
                .for_each(|node| {
                    let layer_name = &node.layer_name;
                    // scene_nodes.push(scene_node.clone());

                    let layer = reader.get(layer_name)
                        .expect("Layer not found");


                    layer.save(&node, world);
                });
        }
    }
}


pub struct LoadAllActiveSceneNodes(pub SceneLayerReg);

impl Command for LoadAllActiveSceneNodes {
    fn apply(self, world: &mut World) {
        let mut query = world
            .query_filtered::<(Entity, &mut SceneNode, &FullPosKey), With<SceneNodeActive>>();
        let reg = self.0;

        let vec: Vec<Entity> = query.iter_mut(world)
            .sort::<&SceneNode>()
            .filter(|(_, scene_node, _)| {
                scene_node.scene_path.is_some()
            })
            .map(|(ent, _, _)| {
                ent
            })
            .collect();


        let mut query = world.query::<&SceneNode>();
        reg.write_dir.clone().expect("Write dir not found");
        {
            let reader = reg.read();

            let mut scene_nodes = vec![];
            query.iter_many(world, vec.as_slice())
                .for_each(|scene_node|
                    scene_nodes.push(scene_node.clone())
                );

            scene_nodes.iter()
                .for_each(|node| {
                    let layer_name = &node.layer_name;
                    // scene_nodes.push(scene_node.clone());

                    let layer = reader.get(layer_name)
                        .expect("Layer not found");

                    layer.load(&node, world);
                });
        }
    }
}