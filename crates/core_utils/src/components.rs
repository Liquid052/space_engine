use bevy::prelude::*;




#[derive(Reflect, Component, Default, Copy, Clone, Debug)]
#[reflect(Component)]
pub struct Active;

#[derive(Reflect, Component, Default, Copy, Clone, Debug)]
#[reflect(Component)]
pub struct Enabled;

#[derive(Reflect, Component, Default, Copy, Clone, Debug)]
#[reflect(Component)]
pub struct DefaultMarker;