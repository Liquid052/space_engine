extern crate bevy;
extern crate bevy_inspector_egui;
extern crate engine;

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use engine::prelude::*;

fn main() {
    App::new()
        .add_plugins(EnginePlugin::new("SpaceApp")
            .enable_space()
            .set(SpacePlugin {
                draw_enabled: true,
                camera_enabled: true,
                cam_background_enabled: true,
                cam_target: Some("Kerbin + Mun".into()),
            })
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ResourceInspectorPlugin::<SpaceTimeScale>::new())
        .add_systems(Startup, setup_space)
        .add_systems(Update, mouse_wheel_zoom)
        .run();
}

fn setup_space(mut commands: Commands) {
    commands.create_space("Test space");
    commands.space_cam_follow("Kerbin + Mun");

    commands.add(
        Star::new("Sol")
            .mass(1.7565459e28)
            .radius(261_600_000.0)
            .color(Color::WHITE)
            .belt(
                70_000_000_000.0,
                4_000_000_000.0,
                Color::srgb(0.0, 0.5, 0.0)
            )
    );

    commands.add(
        TwoBodyBuilder::new(
            Planet::new("Kerbin")
                .mass(5.2915158e22)
                .radius(700_000.0)
                .color(Srgba::gray(0.7).into())
                .semi_major_axis(23_599_840_256.0)
                .belt(1_000_000.0, 1_00_000.0, Color::srgb(0.0, 0.5, 0.0)),
            Moon::new("Mun")
                .mass(4.2915158e22)
                .radius(500_000.0)
                .color(Srgba::gray(0.7).into())
                .semi_major_axis(28_000_000.0),
        )
            .orbiting("Sol")
            .eccentricity_1(0.0)
    );

    commands.add(
        Moon::new("Mun 2")
            .mass(6.2099068e21)
            .radius(450_000.0)
            .color(Color::LinearRgba(LinearRgba::new(1.0,1.0,0.0,1.0)))
            .eccentricity(0.034)
            .semi_major_axis(23608596822.4)
            .argument_of_periapsis(1.845)
            .mean_anomaly_at_epoch(-1.773)
    );

}

fn mouse_wheel_zoom(
    mut evs: EventReader<MouseWheel>,
    mut cam: Query<&mut OrthographicProjection, With<SpaceLayer>>,
) {
    let mut orto = cam.single_mut();

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
