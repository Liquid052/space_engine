use bevy::prelude::*;
use engine_core::prelude::*;
use std::fmt::Debug;

#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct SpaceLayer;

// impl SceneLayerHandler for SpaceLayer {
//     fn save(&self, _path: &SceneNode, _cfg: &Config, _world: &mut World) {}

    // fn load(&self, _path: &SceneNode, _cfg: &Config, _world: &mut World) {}
// }

#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct VesselMarker;

#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct StarMarker;

// Visual
#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct OrbitOutline;

#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct SOI;

#[derive(Reflect, Component, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct Dynamic;

#[doc(hidden)]
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

// For bodies which shouldn't have certain values recalculated - such as soi
#[derive(Reflect, Component, Clone, Copy, Debug, PartialEq)]
#[reflect(Component)]
pub struct Restricted;
