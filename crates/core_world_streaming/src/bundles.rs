use crate::prelude::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct SceneNodeBundle {
    pub name: Name,
    pub full_pos_key: FullPosKey,
    pub scene_node: SceneNode,
}

