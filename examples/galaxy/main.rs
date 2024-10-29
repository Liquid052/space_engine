extern crate bevy;
extern crate bevy_inspector_egui;
extern crate decay_engine;

use bevy::color::palettes::basic::*;
use bevy::color::palettes::css::ORANGE;
use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
use decay_engine::prelude::*;

//noinspection RsExternalLinter
struct Blocked(bool);

fn main() {
    App::new()
        .add_plugins(EnginePlugin::new("SpaceApp")
            .enable_space()
            .set(SpacePlugin {
                draw_enabled: false,
                camera_enabled: false,
                cam_background_enabled: false,
                cam_target: None,
            })
        )
        // debug
        .add_plugins((
            WorldInspectorPlugin::new(),
            StateInspectorPlugin::<AppState>::new()
        ))
        .insert_resource(ClearColor(BLACK.into()))
        .add_systems(OnEnter(AppState::Menu), setup)
        .add_systems(Update, spawn_star
            .run_if(galaxy_active),
        )
        .add_systems(Last, draw_region.run_if(galaxy_active))
        .run();
}

fn spawn_star(
    mut galaxies: Query<&mut Galaxy>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut egui_context: EguiContexts,
    mut commands: Commands,
) {
    let mut galaxy = galaxies.single_mut();


    if let Some(context) = egui_context.try_ctx_mut() {
        if context.is_pointer_over_area() || context.is_using_pointer() {
            return;
        }
    }
    if mouse_button.just_pressed(MouseButton::Left) {
        let key = galaxy.grid_pos().unwrap();

        if let Some(ent) = galaxy.get_child(&key)
        {
            if galaxy.active_entity().is_some() {
                return;
            }

            info!("Generating star [{}]", galaxy.grid_pos().unwrap());

            galaxy.set_active_entity(ent);
            commands.generate_star_system(ent);


            return;
        }

        info!("Spawn star [{}]", galaxy.grid_pos().unwrap());


        commands.add_star(galaxy.grid_pos().unwrap(), "[DX-K427-MT879]", Star {
            name: "Star Sector".to_string(),
            mass: 89740.0,
            radius: 1000.0,
            color: YELLOW.into(),
            ..default()
        })
            .cursor_pos(galaxy.mouse_pos().unwrap())
            .build();
    }
}

fn setup(mut commands: Commands, mut next: ResMut<NextState<AppState>>) {
    next.set(AppState::InGame {
        paused: false
    });

    let mut cam_bundle = Camera2dBundle::new_with_far(100.0);
    cam_bundle.tonemapping = Tonemapping::AcesFitted;
    cam_bundle.camera.hdr = true;
    cam_bundle.projection.scale = 1.0;

    commands
        .spawn((cam_bundle,
                BloomSettings {
                    intensity: 0.3,
                    low_frequency_boost: 0.5,
                    low_frequency_boost_curvature: 0.95,
                    high_pass_frequency: 1.0,
                    prefilter_settings: BloomPrefilterSettings {
                        threshold: 0.1,
                        threshold_softness: 0.3,
                    },
                    composite_mode: BloomCompositeMode::Additive,
                },
                Name::new("Cam"),
        )
        );

    commands
        .galaxy_builder("Test space")
        .description("This is a test space")
        .cell_size(256)
        .build();
}

pub fn draw_region(
    mut gizmos: Gizmos,
    galaxies: Query<&Galaxy>,
    mut egui_context: EguiContexts,
) {
    if let Some(context) = egui_context.try_ctx_mut() {
        if context.is_pointer_over_area() {
            return;
        }
    }

    let galaxy = galaxies.single();

    let grid_start = galaxy.map_size().as_vec2() / -2.0;
    let cell_size = galaxy.cell_size() as f32;
    let current_pos = galaxy.grid_pos().unwrap().as_vec2() * cell_size;

    let world_snapped_pos = current_pos + grid_start + Vec2::splat(cell_size / 2.0);

    let col = if galaxy.is_occupied(&galaxy.grid_pos().unwrap()) {
        WHITE
    } else {
        ORANGE
    };

    gizmos.rect_2d(
        world_snapped_pos,
        Rot2::radians(0.0),
        Vec2::ONE * cell_size,
        col,
    );
}