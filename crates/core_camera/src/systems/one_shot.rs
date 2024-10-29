use crate::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use std::any::{type_name, TypeId};

pub fn add_to_cam<T: Default + Component>(
    query: Query<Entity, With<MainCamera>>,
    mut commands: Commands,
) {
    info!("Adding component: {:?}", type_name::<T>());
    query.get_single().expect("MainCamera not found");

    commands.entity(query.single())
        .insert(T::default());
}

pub fn update_style<T: Default + Component>(
    mut query: Query<(&mut RenderLayers, &mut Camera, &mut BloomSettings, &mut Tonemapping), With<MainCamera>>,
    cam_man: Res<CameraManager>,
) {
    info!("Updating Style: {:?}", type_name::<T>());
    let (mut render_layer, mut camera, mut bloom_settings, mut tone_mapping) = query.get_single_mut().expect("MainCamera not found");

    let style = cam_man.cam_styles.get(&TypeId::of::<T>()).unwrap();

    if let Some(new_render_layer) = style.render_layer.clone() {
        *render_layer = new_render_layer;
    }
    if let Some(bloom) = &style.bloom {
        *bloom_settings = bloom.clone();
    }
    if let Some(tone_mapping1) = style.tone_mapping {
        *tone_mapping = tone_mapping1;
    }

    camera.hdr = style.hdr;
}

pub fn remove_from_cam<T: Default + Component>(
    query: Query<Entity, With<MainCamera>>,
    mut commands: Commands,
) {
    info!("Removing component: {:?}", type_name::<T>());
    query.get_single().expect("MainCamera not found");

    commands.entity(query.single())
        .remove::<T>();
}