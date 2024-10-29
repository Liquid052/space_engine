use crate::bundles::SceneNodeBundle;
use crate::prelude::*;
use bevy::core::Name;
use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::Command;
use bevy::hierarchy::{BuildChildren, BuildWorldChildren};
use bevy::prelude::{default, Commands, Entity, World};

pub trait SceneTreeCommandsBuilderExt<'w, 's> {
    fn scene_init(&mut self, name: impl Into<String>) -> &mut Self;
    fn scene_clear(&mut self) -> &mut Self;

    fn scene_root(&mut self, type_name: impl Into<String>) -> EntityCommands;
    fn scene_node_under(&mut self, ent: Entity, child_key: PosKey, type_name: impl Into<String>) -> EntityCommands;
}

pub trait SceneTreeBuilderEntCommandsExt<'a> {
    fn scene_sub_node(&mut self, child_key: PosKey, type_name: impl Into<String>) -> &mut Self;
    fn enable_scene(&mut self) -> &mut Self;
    fn disable_scene(&mut self) -> &mut Self;
}

impl<'a> SceneTreeBuilderEntCommandsExt<'a> for EntityCommands<'a> {
    fn scene_sub_node(&mut self, child_key: PosKey, type_name: impl Into<String>) -> &mut Self {
        let mut ent = Entity::from_raw(0);
        let parent = self.id();

        self.with_children(|child_builder| {
            ent = child_builder.spawn(())
                .id();
        });

        self.commands().add(SceneNodeInsert {
            entity: ent,
            pos_key: child_key,
            node_type: type_name.into(),
            parent,
        });

        self
    }
    fn enable_scene(&mut self) -> &mut Self {
        let id = self.id();

        self.commands()
            .add(EnableSceneNode { ent: id });

        self
    }
    fn disable_scene(&mut self) -> &mut Self {
        let id = self.id();

        self.commands().add(DisableSceneNode { ent: id });

        self
    }
}

impl<'w, 's> SceneTreeCommandsBuilderExt<'w, 's> for Commands<'w, 's> {
    fn scene_init(&mut self, name: impl Into<String>) -> &mut Self {
        self.add(NameScene(name.into()));

        self
    }
    fn scene_clear(&mut self) -> &mut Self {
        self.add(ClearSceneName);


        self
    }

    fn scene_root(&mut self, type_name: impl Into<String>) -> EntityCommands {
        let mut ent_com = self.spawn(());
        let entity = ent_com.id();

        ent_com.commands().add(SceneRoot {
            entity,
            node_type: type_name.into(),
        });

        ent_com
    }

    fn scene_node_under(&mut self, parent: Entity, child_key: PosKey, type_name: impl Into<String>) -> EntityCommands {
        let mut ent_com = self.spawn(());
        let id = ent_com.id();

        ent_com.commands().add(SceneNodeInsert {
            entity: id,
            pos_key: child_key,
            node_type: type_name.into(),
            parent,
        });

        ent_com
    }
}

struct SceneRoot {
    entity: Entity,
    node_type: String,
}

struct SceneNodeInsert {
    entity: Entity,
    pos_key: PosKey,
    node_type: String,
    parent: Entity,
}

impl Command for SceneRoot {
    fn apply(self, world: &mut World) {
        world.insert_resource(SceneTree::new(self.entity));


        let name = Name::new(format!("Scene[{:?}]: {}", PosKey::Root, self.node_type));

        world.entity_mut(self.entity)
            .insert(SceneNodeBundle {
                name,
                full_pos_key: FullPosKey(vec![PosKey::Root]),
                scene_node: SceneNode {
                    depth: 0,
                    layer_name: self.node_type.into(),
                    scene_path: None,
                    children: default(),
                },
            });
    }
}

impl Command for SceneNodeInsert {
    fn apply(self, world: &mut World) {
        let scene_map = world.resource_mut::<SceneMap>();


        let mut full_pos_key = scene_map.get_key(self.parent).unwrap().clone();
        full_pos_key.0.push(self.pos_key);

        let name = Name::new(format!("Scene[{:?}]: {}", full_pos_key, self.node_type));

        let mut q = world.query::<&SceneNode>();
        let depth = q.get(world, self.parent).unwrap().depth;

        world.entity_mut(self.entity)
            .insert(SceneNodeBundle {
                name: name.clone(),
                full_pos_key: full_pos_key.clone(),
                scene_node: SceneNode {
                    depth: depth + 1,
                    layer_name: self.node_type.into(),
                    scene_path: None,
                    children: default(),
                },
            });

        // register in scene tree
        // world.resource_mut::<SceneTree>().register(self.pos_key, self.entity, Some(self.parent));

        // add to hierarchy
        world.entity_mut(self.parent)
            .add_child(self.entity);


        // add to scene_bundle
        let mut ent = world.entity_mut(self.parent);
        let mut node = ent.get_mut::<SceneNode>().unwrap();
        node.children.insert(format!("{}", self.pos_key), self.entity);
    }
}