use bevy::prelude::*;

#[doc(hidden)]
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub enum FocusMode {
    #[default]
    Sun,
    Body(String),
    Vessel(String),
}

#[doc(hidden)]
#[derive(Component, Reflect, Clone, Copy, Deref, DerefMut)]
#[reflect(Component)]
pub struct CamEnabled(pub bool);
