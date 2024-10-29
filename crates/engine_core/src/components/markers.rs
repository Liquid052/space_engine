use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
pub struct Active;

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
pub struct Focus;