extern crate bevy;
extern crate engine;

use bevy::color::palettes::basic::*;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use engine::prelude::*;

#[derive(Component, Default)]
struct PlanetLayer;

//noinspection RsExternalLinter
fn main() {
    App::new()
        .add_plugins(EnginePlugin::new("Camera test"))
        .add_plugins(WorldInspectorPlugin::new())
        // camera
        .camera_manager()
        .clear_color(PURPLE)
        .config_layer(CamLayerConfig::new(SpaceLayer) // Configure SpaceLayer
                          .clamp_zoom(0.1, 1000.0) // Set zoom limits for the camera
                          .enable_hdr() // Enable HDR rendering
                          .tone_mapping(Tonemapping::AcesFitted) // Set tonemapping method
                          .render_layer(RenderLayers::layer(0)) // Set render layer
                          .depth(100.0) // Set layer depth
        )
        .config_layer(CamLayerConfig::new(PlanetLayer) // Configure PlanetLayer
                          .render_layer(SPACE_LAYER) // Set render layer
                          .clamp_zoom(0.1, 1000.0) // Set zoom limits for the camera
                          .depth(100.0) // Set layer depth
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
