extern crate bevy;
extern crate engine;

use bevy::color::palettes::basic::*;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use engine::galaxy::constants::GALAXY_LAYER;
use engine::prelude::*;

//noinspection RsExternalLinter
fn main() {
    App::new()
        .add_plugins(EnginePlugin::new("naming test"))
        .add_plugins(WorldInspectorPlugin::new())
        // camera
        .camera_manager()
        .clear_color(BLACK)
        .config_layer(CamLayerConfig::new(SpaceLayer)
            .clamp_zoom(0.1, 1000.0)
            .enable_hdr()
            .tone_mapping(Tonemapping::AcesFitted)
            .render_layer(GALAXY_LAYER)
            .depth(100.0)
        )
        .config_layer(CamLayerConfig::new(GalaxyLayer)
            .render_layer(SPACE_LAYER)
            .clamp_zoom(0.1, 1000.0)
            .depth(100.0)
        )
        .app()
        .add_systems(Startup, setup)
        .run();
}


//noinspection RsExternalLinter
fn setup(mut com: Commands) {
    // Spawn camera
    com.spawn((
        Name::new("Camera"),
        Camera2dBundle::new_with_far(100.0),
        BloomSettings::default(),
        RenderLayers::layer(0),
        DefaultCameraLayer,
        MainCamera
    ));

    com.set_camera_layer(SpaceLayer);
}
