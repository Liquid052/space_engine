use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub enum FocusMode {
    #[default]
    Sun,
    Body(String),
    Vessel(String),
}

#[derive(Component, Reflect, Clone, Copy, Deref, DerefMut)]
#[reflect(Component)]
pub struct CamEnabled(pub bool);
