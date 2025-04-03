use bevy::prelude::*;
use engine_core::prelude::*;
use std::fmt::Debug;

#[doc(hidden)]
#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct SpaceLayer;


#[doc(hidden)]
#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct VesselMarker;

#[doc(hidden)]
#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct StarMarker;

// Visual
#[doc(hidden)]
#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct OrbitOutline;

#[doc(hidden)]
#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct SOI;

#[doc(hidden)]
#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct Dynamic;


#[derive(Reflect, Component, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct SpaceDepth(pub u8);

impl SpaceDepth {
    pub fn redirect(&mut self, up: bool) {
        match up {
            true => self.up(),
            false => self.down()
        };
    }
    pub fn up(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }
    pub fn down(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    pub fn return_up(&self) -> Self {
        Self(self.0.saturating_sub(1))
    }

    pub fn return_down(&self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

#[doc(hidden)]
#[derive(Reflect, Component, Clone, Copy, Debug, PartialEq)]
#[reflect(Component)]
pub struct TwoBody;

// For bodies which shouldn't have certain values recalculated - such as soi
#[doc(hidden)]
#[derive(Reflect, Component, Clone, Copy, Debug, PartialEq)]
#[reflect(Component)]
pub struct Restricted;
