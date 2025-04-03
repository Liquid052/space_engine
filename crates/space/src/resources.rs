use bevy::prelude::*;


/// A resource that holds the current star system name and the sun entity.
#[derive(Debug, Default, Resource)]
pub struct StarSystem {
    pub(crate) system_name: String,
    pub(crate) sun:  Option<Entity>,
}

impl StarSystem {
    pub fn system_name(&self) -> &str { &self.system_name }

    pub fn loaded(&self) -> bool { !self.system_name.is_empty() }

    pub fn set_sun(&mut self, sun: Entity) {
        self.sun = Some(sun);
    }

    pub fn get_sun(&self) -> Option<Entity> { self.sun }
}

/// A resource that holds the current time scale for the simulation.
#[derive(Resource, Reflect, Component, Debug, Deref, DerefMut)]
#[reflect(Resource)]
pub struct SpaceTimeScale(pub f64);

impl Default for SpaceTimeScale {
    fn default() -> Self { Self(1.0) }
}

#[doc(hidden)]
#[derive(Resource, Default)]
pub struct OrbitBuffer {
    pub(crate) current: f32,
    pub(crate) vec:     Vec<Entity>,
}
