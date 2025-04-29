extern crate bevy;
extern crate bevy_inspector_egui;
extern crate space_engine;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use space_engine::prelude::*;

fn main() {
    App::new()
        .add_plugins(EnginePlugin::new("SpaceApp")
            .enable_space()
            .set(SpacePlugin {
                auto_soi_update: true,
                draw_enabled: true,
                camera_enabled: true,
                cam_background_enabled: false,
                cam_target: Some("5".into()),
                test: true
            })
        )
        .add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ResourceInspectorPlugin::<SpaceTimeScale>::new())
        .add_systems(Startup, setup_space)
        .add_systems(OnEnter(AppState::Menu), move_to_running)
        .add_systems(Update, mouse_wheel_zoom)
        .run();
}

fn move_to_running(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::InGame { paused: false })
}

fn setup_space(mut commands: Commands) {
    commands.create_space("Test space");
    commands.space_cam_follow("5");

    commands.add(Star::new("Sol")
        .mass(1.7565459e28)
        .radius(261_600_000.0)
        .color(Color::WHITE)
    );

    const SOI_DIFF: f64 = 40160887.7;
    const COUNT: usize = 100;

    for i in 0..COUNT {
        commands.add(
            Planet::new(i.to_string())
                .mass(5.2915158e22)
                .radius(600_000.0)
                .color(Color::WHITE)
                .semi_major_axis(1_599_840_256.0)
        );
    }
}

fn mouse_wheel_zoom(
    mut evs: EventReader<MouseWheel>,
    mut cam: Query<&mut OrthographicProjection, With<SpaceLayer>>,
) {
    let Ok(mut orto) = cam.get_single_mut() else {
        return;
    };


    evs.read().for_each(|ev| {
        match orto.scale {
            0.1..=1.0 => orto.scale -= ev.y / 10.0,
            1.0..=10. => orto.scale -= ev.y,
            0.5..=200. => orto.scale -= ev.y * 10.0,
            200.0..=1000.0 => orto.scale -= ev.y * 300.0,
            1000.0..=1000000.0 => orto.scale -= ev.y * 1000.0,
            _ => {},
        }

        orto.scale = orto.scale.clamp(0.1, 1000000.0);
    });
}
