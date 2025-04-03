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

// #[reflect_trait]
// pub(crate) trait Depth: Send + Sync + Debug {
//     fn up(&self, ent: &mut EntityCommands);
//     fn down(&self, ent: &mut EntityCommands);
//
//     fn clone(&self) -> Box<dyn Depth>;
//     fn inject(&self, ent: &mut EntityCommands);
//
//     fn return_down(&self) -> Box<dyn Depth>;
// }

// macro_rules! impl_depth {
//     (0) => {
//         impl Depth for SpaceDepth<0> {
//             fn up(&self, ent: &mut EntityCommands) { ent.remove::<SpaceDepth<0>>(); }
//
//             fn down(&self, ent: &mut EntityCommands) {
//                 ent.remove::<SpaceDepth<0>>();
//                 ent.insert(SpaceDepth::<1>);
//             }
//
//             fn clone(&self) -> Box<dyn Depth> { Box::new(SpaceDepth::<0>) }
//
//             fn inject(&self, ent: &mut EntityCommands) { ent.insert(SpaceDepth::<0>); }
//
//             fn return_down(&self) -> Box<dyn Depth> { Box::new(SpaceDepth::<1>) }
//         }
//     };
//     (9) => {
//         impl Depth for SpaceDepth<9> {
//             fn up(&self, ent: &mut EntityCommands) {
//                 ent.remove::<SpaceDepth<9>>();
//                 ent.insert(SpaceDepth::<8>);
//             }
//
//             fn down(&self, _: &mut EntityCommands) { unimplemented!() }
//
//             fn return_down(&self) -> Box<dyn Depth> { unimplemented!() }
//
//             fn clone(&self) -> Box<dyn Depth> { Box::new(SpaceDepth::<9>) }
//
//             fn inject(&self, ent: &mut EntityCommands) { ent.insert(SpaceDepth::<9>); }
//         }
//     };
//     ($depth:literal) => {
//         impl Depth for SpaceDepth<$depth> {
//             fn up(&self, ent: &mut EntityCommands) {
//                 ent.remove::<SpaceDepth<$depth>>();
//                 ent.insert(SpaceDepth::<{ $depth - 1 }>);
//             }
//
//             fn down(&self, ent: &mut EntityCommands) {
//                 ent.remove::<SpaceDepth<$depth>>();
//                 ent.insert(SpaceDepth::<{ $depth + 1 }>);
//             }
//
//             fn return_down(&self) -> Box<dyn Depth> { Box::new(SpaceDepth::<{ $depth + 1 }>) }
//
//             fn clone(&self) -> Box<dyn Depth> { Box::new(SpaceDepth::<$depth>) }
//
//             fn inject(&self, ent: &mut EntityCommands) { ent.insert(SpaceDepth::<$depth>); }
//         }
//     };
// }
//
// impl_depth!(0);
// impl_depth!(1);
// impl_depth!(2);
// impl_depth!(3);
// impl_depth!(4);
// impl_depth!(5);
// impl_depth!(6);
// impl_depth!(7);
// impl_depth!(8);
// impl_depth!(9);

#[derive(Reflect, Component, Clone, Copy, Debug, PartialEq)]
#[reflect(Component)]
pub struct TwoBody;

// For bodies which shouldn't have certain values recalculated - such as soi
#[derive(Reflect, Component, Clone, Copy, Debug, PartialEq)]
#[reflect(Component)]
pub struct Restricted;
