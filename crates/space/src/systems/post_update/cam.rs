use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use engine_core::prelude::*;

use crate::{constants::*, prelude::*};

// aliases
type CamsChildren<'a> = (&'a OrthographicProjection, &'a Children);
type EnabledCams<'a> = (&'a CamEnabled, &'a mut Camera);

pub fn spawn_cam_with_background(mut commands: Commands, server: Res<AssetServer>) {
    let mut cam = Camera2dBundle::new_with_far(100.0);
    let mut bloom = BloomSettings::OLD_SCHOOL;

    cam.tonemapping = Tonemapping::AcesFitted;
    cam.camera.hdr = true;

    bloom.intensity = 0.6;
    bloom.low_frequency_boost = 0.5;
    cam.camera.order = SPACE_LAYER_DEPTH as isize;

    commands
        .spawn((
            FocusMode::Body("Kerbin + Mun".into()),
            cam,
            InheritedVisibility::default(),
            bloom,
            SPACE_LAYER,
            Space,
            SpacePos::default(),
        ))
        .with_children(|ch_b| {
            ch_b.spawn(SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, -100.0),
                texture: server.load("background.png"),
                ..default()
            })
            .insert(SPACE_LAYER);
        });
}

pub fn spawn_cam(mut commands: Commands) {
    let mut cam = Camera2dBundle::new_with_far(100.0);
    let mut bloom = BloomSettings::OLD_SCHOOL;

    cam.tonemapping = Tonemapping::AcesFitted;
    cam.camera.hdr = true;

    bloom.intensity = 0.6;
    bloom.low_frequency_boost = 0.5;
    cam.camera.order = SPACE_LAYER_DEPTH as isize;

    commands
        .spawn((
            FocusMode::Body("Kerbin + Mun".into()),
            cam,
            InheritedVisibility::default(),
            bloom,
            SPACE_LAYER,
            Space,
            SpacePos::default(),
        ));
}

pub fn handle_camera_target(
    mut reader: EventReader<CamTargetEv>,
    mut cam: Query<&mut FocusMode, (With<Camera>, With<Space>)>,
) {
    let Ok(mut focus) = cam.get_single_mut() else {
        return;
    };

    reader
        .read()
        .last()
        .iter()
        .for_each(|event| *focus = FocusMode::Body(event.0.clone()));
}

pub fn cam_follow(
    name_reg: Res<NameReg>,
    mut cameras: Query<(&mut SpacePos, &FocusMode), With<Camera>>,
    transforms: Query<&SpacePos, Without<Camera>>,
    space: Res<SpaceMap>,
) {
    cameras
        .iter_mut()
        .for_each(|(mut transform, focus)| match focus {
            FocusMode::Sun => {
                if space.get_sun().is_none() {
                    return;
                };

                transform.x = 0.0;
                transform.y = 0.0;
            },
            FocusMode::Body(name) => {
                let Some(ent) = name_reg.get(name) else {
                    return;
                };
                let target = transforms.get(ent).unwrap();

                *transform = *target;
            },
            _ => {},
        });
}

pub fn update_cam_enabled(mut query: Query<EnabledCams, (With<Space>, Changed<CamEnabled>)>) {
    query
        .iter_mut()
        .for_each(|(enabled, mut cam)| cam.is_active = **enabled);
}

pub fn scale_camera_background(
    cams: Query<CamsChildren, (With<Space>, Changed<OrthographicProjection>)>,
    mut background: Query<&mut Transform>,
) {
    cams.iter().for_each(|(orto, child)| {
        let first = child.first().copied();

        if first.is_none() {
            return;
        }

        if let Ok(mut transform) = background.get_mut(first.unwrap()) {
            transform.scale.x = orto.scale;
            transform.scale.y = orto.scale;
        }
    });
}

pub fn map_pos_transforms(
    cam: Query<&SpacePos, (With<Space>, With<Camera>)>,
    mut bodies: Query<(&mut Transform, &SpacePos)>,
) {
    let cam_abs = cam.single();

    bodies.iter_mut().for_each(|(mut transform, abs_pos)| {
        let mut rel_pos = **abs_pos - **cam_abs;

        rel_pos /= SPACE_SCALE;
        let rel_pos = rel_pos.as_vec2();

        transform.translation.x = rel_pos.x;
        transform.translation.y = rel_pos.y;
    });
}
