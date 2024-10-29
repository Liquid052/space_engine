use crate::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct TestLayer;

impl SceneLayerHandler for TestLayer {
    fn save(&self, _node: &SceneNode, _cfg: &Config, _world: &mut World) {}
    fn load(&self, _node: &SceneNode, _cfg: &Config, _world: &mut World) {}
}

#[test]
fn test_scene_tree() {
    App::new()
        .add_plugins(WorldStreamingPlugin)
        // camera
        .scene_layer("Space", SceneLayerConfig::new(TestLayer))
        .add_systems(Startup, (setup, check)
            .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let root = commands.scene_root("world")
        .scene_sub_node(PosKey::Pos(IVec2::ZERO), "galaxy")
        .scene_sub_node(PosKey::Pos(IVec2::ONE), "galaxy")
        .id();

    let galaxy = commands.scene_node_under(root, PosKey::Pos(IVec2::ONE * 3), "galaxy")
        .id();

    commands.scene_node_under(galaxy, PosKey::Pos(IVec2::new(14, 7)), "star_system");
}

fn check(scene_q: Query<(&SceneNode, &FullPosKey)>, child_q: Query<Option<&Children>>, scene_tree: Res<SceneMap>) {
    let root = scene_tree.root().unwrap();


    check_validity(root, &scene_q, &child_q, scene_tree.as_ref(), 0);
}

fn check_validity(
    ent: Entity,
    scene_q: &Query<(&SceneNode, &FullPosKey)>,
    child_q: &Query<Option<&Children>>,
    scene_tree: &SceneMap,
    depth: u8,
) {
    let Some(full_pos_key) = scene_tree.get_key(ent) else {
        panic!();
    };
    let Ok((scene_node, pos_key)) = scene_q.get(ent) else {
        panic!();
    };

    // check that the full pos key is correct
    assert_eq!(full_pos_key, pos_key);
    assert_eq!(scene_node.depth, depth);

    // check that the parent is correct
    let Some(parent) = scene_tree.get(full_pos_key) else {
        panic!();
    };
    assert_eq!(parent, ent);

    // check that the children are correct
    if let Ok(Some(children)) = child_q.get(ent) {
        for child in children.iter() {
            check_validity(*child, scene_q, child_q, scene_tree, depth + 1);
        }
    }
}