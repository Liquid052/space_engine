extern crate bevy;
extern crate bevy_inspector_egui;
extern crate space_engine;

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
                cam_background_enabled: true,
                cam_target: Some("Kerbin".into()),
            })
        )
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
    commands.space_cam_follow("Kerbin");

    commands.add(Star::new("Sol")
        .mass(1.7565459e28)
        .radius(261_600_000.0)
        .color(Color::WHITE)
        .belt(4_900_000_000.0, 80_000_000.0, Color::srgb(1.0, 0.0, 0.0))
    );

    commands.add(
        Planet::new("Kerbin")
            .mass(5.2915158e22)
            .radius(600_000.0)
            .color(Color::WHITE)
            .semi_major_axis(23_599_840_256.0)
    );

    commands.add(
        Moon::new("Mun")
            .mass(9.7599050e20)
            .radius(300_000.0)
            .color(Color::WHITE)
            .semi_major_axis(18_000_000.0)
            .mean_anomaly_at_epoch(-1.0)
            .orbiting("Kerbin"),
    );

    commands.add(
        Moon::new("Mun 2")
            .mass(9.7599068e20)
            .radius(300_000.0)
            .color(Color::WHITE)
            .eccentricity(0.03565)
            .semi_major_axis(23608596822.4)
            .argument_of_periapsis(1.845)
            .mean_anomaly_at_epoch(-1.773),
    );

    commands.add(
        SpaceShip::new("Vessel")
            .semi_major_axis(2_000_000.0)
            .mean_anomaly_at_epoch(-0.7)
            .color(Color::srgb(1.0,1.0,0.0).into())
            .orbiting("Kerbin")
    );
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
