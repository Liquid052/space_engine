use bevy::prelude::*;



#[derive(Debug, Default, Resource)]
pub struct SpaceMap {
    pub(crate) name: String,
    pub(crate) sun:  Option<Entity>,
}

impl SpaceMap {
    pub fn name(&self) -> &str { &self.name }

    pub fn loaded(&self) -> bool { !self.name.is_empty() }

    pub fn set_sun(&mut self, sun: Entity) {
        self.sun = Some(sun);
    }

    pub fn get_sun(&self) -> Option<Entity> { self.sun }
}

#[derive(Resource, Reflect, Component, Debug, Deref, DerefMut)]
#[reflect(Resource)]
pub struct SpaceTimeScale(pub f64);

impl Default for SpaceTimeScale {
    fn default() -> Self { Self(1.0) }
}

#[derive(Resource, Default)]
pub struct OrbitBuffer {
    pub(crate) current: f32,
    pub(crate) vec:     Vec<Entity>,
}
