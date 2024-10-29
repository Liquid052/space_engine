use crate::prelude::SceneMap;
use bevy::ecs::component::{ComponentHooks, ComponentId, StorageType};
use bevy::ecs::entity::MapEntities;
use bevy::ecs::reflect::*;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;

#[derive(Reflect, Component, Clone, Eq, PartialEq, Hash, Debug)]
#[reflect(Component, PartialEq, Hash)]
pub struct SceneNodeActive;

#[derive(Reflect, Component, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[reflect(Component, PartialEq, Hash)]
pub enum PosKey {
    Root,
    Pos(IVec2),
    Depth(u32),
}

impl fmt::Display for PosKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PosKey::Root => write!(f, "R"),
            PosKey::Pos(ivec2) => write!(f, "P{}x{}", ivec2.x, ivec2.y),
            PosKey::Depth(depth) => write!(f, "D{}", depth),
        }
    }
}

// new-type for Vec<PosKey>
#[derive(Reflect, Default, Hash, Eq, PartialEq, Clone, Debug, Deref, DerefMut)]
#[reflect(Component, Hash, PartialEq)]
pub struct FullPosKey(pub Vec<PosKey>);

impl Component for FullPosKey {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world: DeferredWorld, ent, _| {
            if !world.contains_resource::<SceneMap>() {
                return;
            };

            let key = world.entity(ent)
                .get::<FullPosKey>()
                .unwrap()
                .clone();

            let mut scene_map = world.get_resource_mut::<SceneMap>().unwrap();
            if key.len() == 1 {
                scene_map.set_root(ent);
            }

            scene_map.insert(key, ent);
        });

        hooks.on_remove(|mut world: DeferredWorld, ent, _| {
            if !world.contains_resource::<SceneMap>() {
                return;
            }

            let key_len = world.entity(ent)
                .get::<FullPosKey>()
                .unwrap()
                .len();

            let mut scene_map = world.resource_mut::<SceneMap>();

            if key_len == 1 {
                scene_map.clear_root();
            }
            scene_map.remove(ent);
        });
    }
}


impl fmt::Display for FullPosKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pos_key) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "_")?;
            }
            write!(f, "{}", pos_key)?;
        }
        Ok(())
    }
}

// conversion impls
impl From<&[PosKey]> for FullPosKey {
    fn from(keys: &[PosKey]) -> Self {
        FullPosKey(keys.to_vec())
    }
}


#[derive(Reflect, Component, Clone, Default)]
#[reflect(Component, MapEntities)]
pub struct SceneNode {
    pub depth: u8,
    pub layer_name: Cow<'static, str>,
    pub scene_path: Option<Cow<'static, str>>,

    pub children: HashMap<String, Entity>,
}

impl SceneNode {
    pub fn load_path(&self) -> Option<&str> {
        match self.scene_path.as_deref() {
            Some(str) => Some(&str[7..]),
            None => None
        }
    }
    pub fn load_path_unchecked(&self) -> &str {
        &self.scene_path.as_ref().unwrap()[7..]
    }
}


impl MapEntities for SceneNode {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.children.iter_mut()
            .for_each(|(_, ent)| *ent = entity_mapper.map_entity(*ent));
    }
}

impl PartialEq for SceneNode {
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth
    }
}

// Implement Eq for SceneNode
impl Eq for SceneNode {}

//noinspection RsExternalLinter
// Implement PartialOrd for SceneNode
impl PartialOrd for SceneNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.depth.partial_cmp(&other.depth)
    }
}

// Implement Ord for SceneNode
impl Ord for SceneNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.depth.cmp(&other.depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poskey_display() {
        assert_eq!(format!("{}", PosKey::Root), "root");
        assert_eq!(format!("{}", PosKey::Pos(IVec2::new(1, 2))), "pos_1_2");
        assert_eq!(format!("{}", PosKey::Depth(3)), "depth_3");
    }

    #[test]
    fn test_fullposkey_display() {
        let full_pos_key = FullPosKey(vec![
            PosKey::Root,
            PosKey::Pos(IVec2::new(1, 2)),
            PosKey::Depth(3)
        ]);
        assert_eq!(format!("{}", full_pos_key), "root_pos_1_2_depth_3");
    }
}