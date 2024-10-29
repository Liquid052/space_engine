extern crate bevy;
extern crate engine;


use bevy::asset::StrongHandle;
use bevy::color::palettes::basic::*;
use bevy::color::palettes::css::DARK_SLATE_GRAY;
use bevy::color::palettes::tailwind::CYAN_300;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::ecs::system::SystemId;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use engine::prelude::*;
use std::sync::Arc;

use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;

#[derive(Component, Default, Debug)]
pub struct WorldLayer;
#[derive(Component, Default, Debug)]
pub struct GalaxyLayer;
#[derive(Component, Default, Debug)]
pub struct StarSystemLayer;

impl SceneLayerHandler for WorldLayer {
    fn save(&self, node: &SceneNode, _: &Config, world: &mut World) {
        SceneSaver::new(world, node)
            .extract_entities::<(With<Sprite>, With<Name>)>()
            .filter_entities(|ent_ref| {
                if !ent_ref.contains::<Name>() {
                    return false;
                }

                ent_ref.get::<Name>().unwrap()
                    .as_str()
                    .eq("Wall 1")
            })
            .extract_resource::<TestRes>()
            .pack()
            .save();
    }

    fn load(&self, node: &SceneNode, _: &Config, world: &mut World) {
        SceneLoader::new(world, node)
            .load();
    }
}
impl SceneLayerHandler for GalaxyLayer {
    fn save(&self, node: &SceneNode, _: &Config, world: &mut World) {
        SceneSaver::new(world, node)
            .extract_entities::<(With<Sprite>, With<Name>)>()
            .filter_entities(|ent_ref| {
                if !ent_ref.contains::<Name>() {
                    return false;
                }

                ent_ref.get::<Name>().unwrap()
                    .as_str()
                    .eq("Wall 2")
            })
            .pack()
            .save();
    }

    fn load(&self, node: &SceneNode, _: &Config, world: &mut World) {
        SceneLoader::new(world, node)
            .load();
    }
}
impl SceneLayerHandler for StarSystemLayer {
    fn save(&self, node: &SceneNode, _: &Config, world: &mut World) {
        SceneSaver::new(world, node)
            .extract_entities::<(With<Sprite>, With<Name>)>()
            .filter_entities(|ent_ref| {
                if !ent_ref.contains::<Name>() {
                    return false;
                }

                ent_ref.get::<Name>().unwrap()
                    .as_str()
                    .eq("Wall 3")
            })
            .pack()
            .save();
    }

    fn load(&self, node: &SceneNode, _: &Config, world: &mut World) {
        SceneLoader::new(world, node)
            .load();
    }
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct TestRes;

fn main() {
    App::new()
        .add_plugins(EnginePlugin::new("Streaming example"))
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(TestRes {})
        .register_type::<TestRes>()
        .camera_manager()
        .clear_color(DARK_SLATE_GRAY)
        .app()
        .register_type::<Handle<Image>>()
        .register_type::<Arc<StrongHandle>>()
        // define layers for level streaming
        .scene_layer("world", SceneLayerConfig::new(WorldLayer))
        .scene_layer("galaxy", SceneLayerConfig::new(GalaxyLayer))
        .scene_layer("star_system", SceneLayerConfig::new(StarSystemLayer))
        // systems
        .add_systems(Startup, setup)
        .add_systems(Update, (
            menu,
            print_tree.run_if(input_just_pressed(KeyCode::KeyP))
        ))
        .run();
}

fn print_tree(map: Res<SceneMap>, names: Query<&Name>) {
    let Some(root) = map.root() else {
        return;
    };

    info!("Root: {}", names.get(root).unwrap());

    map.map().iter()
        .map(|(_, ent)| *ent)
        .filter(|ent| names.contains(*ent))
        .map(|ent| names.get(ent).unwrap())
        .for_each(|name| info!("{:?}", name));
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::new_with_far(10.0),
        BloomSettings::default(),
        RenderLayers::layer(0),
        DefaultCameraLayer,
        MainCamera
    ));
}

fn menu(mut systems: Local<Vec<SystemId>>, map: Res<SceneMap>, mut egui: EguiContexts, mut commands: Commands) {
    let Some(egui) = egui.try_ctx_mut() else {
        return;
    };

    if systems.is_empty() {
        systems.push(commands.register_one_shot_system(spawn_walls));
        systems.push(commands.register_one_shot_system(disable_nodes));
        systems.push(commands.register_one_shot_system(enable_lowest_node));
    }

    egui::Window::new("Scene edit").show(egui, |ui| {
        ui.horizontal(|ui| {
            ui.label("scene");
            ui.button("init").clicked()
                .then(|| {
                    if map.root().is_some() {
                        return;
                    };

                    commands.scene_init("test_world");

                    let root = commands.scene_root("world")
                        .id();

                    let galaxy = commands.scene_node_under(root, PosKey::Pos(IVec2::ONE * 3), "galaxy")
                        .id();

                    let star_key = PosKey::Pos(IVec2::new(14, 7));
                    commands.scene_node_under(galaxy, star_key, "star_system")
                        .enable_scene();

                    commands.entity(root);

                    commands.run_system(systems[0]);
                });
            ui.button("save").clicked()
                .then(|| {
                    commands.add(SaveAllActiveSceneNodes);
                    commands.add(SaveCurrentSceneTree);
                });
            ui.button("load").clicked()
                .then(|| {
                    commands.add(LoadSceneTree("test_world".into()));
                });
        });
        ui.separator();

        ui.button("Disable world node").clicked()
            .then(|| commands.run_system(systems[1]));
        ui.button("Enable lowest node").clicked()
            .then(|| commands.run_system(systems[2]));
    });
}
fn spawn_walls(mut commands: Commands) {
    let id0 = commands.register_one_shot_system(spawn_wall_1);
    let id1 = commands.register_one_shot_system(spawn_wall_2);
    let id2 = commands.register_one_shot_system(spawn_wall_3);

    commands.run_system(id0);
    commands.run_system(id1);
    commands.run_system(id2);
}

fn spawn_wall_1(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("mods/vanilla/textures/temple_wall.png");

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: default(),
            ..default()
        },
        transform: Transform::default().with_translation(Vec3::new(-200.0, 0.0, 0.0)),
        texture: texture.clone(),
        ..default()
    }, Name::new("Wall 1"), Pack::default().with_support("sprite_path"), Unpacked
    ));
}
fn spawn_wall_2(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("mods/vanilla/textures/temple_wall.png");

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: CYAN_300.into(),
            ..default()
        },
        transform: Default::default(),
        texture: texture.clone(),
        ..default()
    }, Name::new("Wall 2"), Pack::default().with_support("sprite_path"), Unpacked));
}
fn spawn_wall_3(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("mods/vanilla/textures/temple_wall.png");

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: RED.into(),
            ..default()
        },
        transform: Transform::default().with_translation(Vec3::new(200.0, 0.0, 0.0)),
        texture: texture.clone(),
        ..default()
    }, Name::new("Wall 3"), Pack::default().with_support("sprite_path"), Unpacked));
}


fn disable_nodes(mut commands: Commands, scene_map: Res<SceneMap>) {
    let root = scene_map.root();

    let Some(root) = root else {
        return;
    };

    commands.entity(root)
        .disable_scene();
}
fn enable_lowest_node(mut commands: Commands, query: Query<(Entity, &FullPosKey)>) {
    let mut last_ent = Entity::PLACEHOLDER;
    let mut last_key = FullPosKey::default();


    query.iter().for_each(|(ent, key)| {
        if last_key.len() < key.len() {
            last_key = key.clone();
            last_ent = ent;
        }
    });

    if last_ent != Entity::PLACEHOLDER {
        commands.entity(last_ent)
            .enable_scene();
    }
}