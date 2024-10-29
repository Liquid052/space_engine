use bevy::prelude::*;

#[derive(Reflect, Default, Component, Copy, Clone)]
#[reflect(Component, Default)]
pub struct DefaultCameraLayer;

#[derive(Reflect, Default, Component, Copy, Clone)]
#[reflect(Component, Default)]
pub struct MainCamera;