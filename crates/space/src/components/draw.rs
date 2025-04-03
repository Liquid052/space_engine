use bevy::prelude::*;

#[doc(hidden)]
#[derive(Component, Default, Debug)]
pub struct DrawSpace {
    pub orbit: Option<Entity>,
    pub soi:   Option<Entity>,
}

#[doc(hidden)]
#[derive(Component, Default, Debug)]
pub struct RestrictDraw;
