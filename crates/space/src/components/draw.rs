use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct DrawSpace {
    pub orbit: Option<Entity>,
    pub soi:   Option<Entity>,
}

#[derive(Component, Default, Debug)]
pub struct RestrictDraw;
