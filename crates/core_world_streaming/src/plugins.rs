use crate::prelude::*;
use crate::resources::SceneTree;
use crate::systems::{process_scene_hooks, remove_parents_from_scene_hooks, SceneHookSet};
use bevy::prelude::*;

pub struct WorldStreamingPlugin;

impl Plugin for WorldStreamingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (process_scene_hooks, remove_parents_from_scene_hooks)
            .chain()
            .in_set(SceneHookSet),
        )
            .configure_sets(PreUpdate, SceneHookSet)
            .init_resource::<SceneProcessor>()
            .init_resource::<SceneLayerReg>()
            .init_resource::<SceneMap>()
            .register_type::<SceneMap>()
            .register_type::<PosKey>()
            .register_type::<FullPosKey>()
            .register_type::<SceneNodeActive>()
            .register_type::<SceneNode>()
            .register_type::<SceneTree>();
    }
}