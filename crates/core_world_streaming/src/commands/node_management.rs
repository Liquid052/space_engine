use crate::prelude::{FullPosKey, SceneLayerReg, SceneNode, SceneNodeActive};
use bevy::ecs::world::Command;
use bevy::prelude::*;


pub struct EnableSceneNode {
    pub ent: Entity,
}

impl Command for EnableSceneNode {
    fn apply(self, world: &mut World) {
        let reg = world.resource::<SceneLayerReg>().layers.clone();

        // Check for SceneNodeActive without holding the entity reference
        let should_load = {
            let ent = world.entity(self.ent);
            !ent.contains::<SceneNodeActive>()
        };


        // Load the scene if needed
        if should_load {
            let scene_path = {
                let ent = world.entity(self.ent);
                ent.get::<SceneNode>()
                    .map(|scene_node| (scene_node.scene_path.clone(), scene_node.layer_name.clone()))
            };

            // Process scene loading if we have a path
            if let Some((Some(_), layer_name)) = scene_path {
                let reader = reg.read().unwrap();
                if let Some(layer) = reader.get(layer_name.as_ref()) {
                    // Get scene node again for loading
                    let scene_node = world.entity(self.ent).get::<SceneNode>().unwrap().clone();
                    layer.load(&scene_node, world);
                }
            }
        } else {
            info!("skipping loading");
        }

        // Insert active component
        let parent_entity = {
            let mut ent = world.entity_mut(self.ent);
            ent.insert(SceneNodeActive);
            ent.get::<Parent>().map(|parent| parent.get())
        };

        // Handle parent recursion
        if let Some(parent_ent) = parent_entity {
            world.commands().add(EnableSceneNode { ent: parent_ent });
        }
    }
}
pub struct DisableSceneNode {
    pub ent: Entity,
}

impl Command for DisableSceneNode {
    fn apply(self, world: &mut World) {
        let is_active = world.entity(self.ent).contains::<SceneNodeActive>();
        if !is_active {
            return;
        }

        let mut vec = vec![];

        let mut scene_q = world
            .query_filtered::<(Entity, &SceneNode), With<SceneNodeActive>>();
        let mut active = world
            .query_filtered::<(), With<SceneNodeActive>>();

        collect_active_children(&mut vec, self.ent, &mut scene_q, &mut active, world);

        if vec.is_empty() {
            return;
        }

        let scene_layer_reg = world.resource::<SceneLayerReg>().clone();
        let reg = scene_layer_reg.layers.clone();
        let reader = reg.read()
            .unwrap();

        vec.into_iter()
            .for_each(|ent| {
                let mut ent = world.entity_mut(ent);
                ent.remove::<SceneNodeActive>();

                let scene_key = ent.get::<FullPosKey>().unwrap().clone();
                let mut scene_node = ent.get_mut::<SceneNode>().unwrap().clone();

                if scene_node.scene_path.is_none() {
                    let name = scene_layer_reg.write_dir.as_ref().unwrap();
                    let path = format!("{}/{}.scn.ron", name, scene_key);
                    scene_node.scene_path = Some(path.into());
                }

                ent.get_mut::<SceneNode>().unwrap().scene_path = scene_node.scene_path.clone();

                reader.get(scene_node.layer_name.as_ref())
                    .expect("SCENE LAYER NOT FOUND")
                    .save(&scene_node, world);
            });
    }
}

// todo! - refactor into one_shot system
fn collect_active_children(
    vec: &mut Vec<Entity>,
    ent: Entity,
    query_s: &mut QueryState<(Entity, &SceneNode), With<SceneNodeActive>>,
    active: &mut QueryState<(), With<SceneNodeActive>>,
    w: &World,
) {
    vec.push(ent);

    let (_, scene_node) = query_s.get(w, ent).unwrap();

    let mut active_children = vec![];
    scene_node.children.iter()
        .filter(|(_, ent)| active.get(w, **ent).is_ok())
        .for_each(|(_, ent)| {
            active_children.push(*ent);
        });

    active_children.into_iter()
        .for_each(|ent| collect_active_children(
            vec,
            ent,
            query_s,
            active,
            w,
        ));
}