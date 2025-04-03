use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    math::DVec2,
    prelude::*,
};

// export
pub use bodies::*;
pub use keplerian::*;
pub use state_vectors::*;

mod bodies;
pub mod keplerian;
pub mod state_vectors;
// double precision for accurate calculations
#[derive(Reflect, Component, Clone, Copy, Debug, Default, PartialEq, DerefMut, Deref)]
#[reflect(Component)]
pub struct SpacePos(pub DVec2);

#[derive(Component, Reflect, Default, Clone, Debug)]
#[reflect(Component)]
pub struct RefFrame {
    pub(crate) children_bodies: Vec<Entity>,
    pub(crate) vessels: Vec<Entity>,
}

impl RefFrame {
    pub fn new() -> Self {
        Self {
            children_bodies: Vec::new(),
            vessels: Vec::new(),
            // soi: None,
            // orbital_line: None,
        }
    }

    pub fn children_bodies(&self) -> &[Entity] { self.children_bodies.as_slice() }

    // children - bodies
    pub fn push_body(&mut self, entity: Entity) {
        if self.children_bodies.contains(&entity) {
            warn!("Entity already in reference frame {:?}", entity);
            return;
        }

        self.children_bodies.push(entity);
    }

    pub fn remove_body(&mut self, entity: Entity) { self.children_bodies.retain(|&x| x != entity); }

    pub fn remove_vessel(&mut self, entity: Entity) { self.vessels.retain(|&x| x != entity); }

    pub fn push_vessel(&mut self, entity: Entity) {
        if self.vessels.contains(&entity) {
            warn!("Entity already in reference frame");
            return;
        }

        self.vessels.push(entity);
    }
}

#[derive(Reflect, Component, Clone, Copy, Debug, PartialEq)]
#[reflect(Component)]
pub struct Orbit {
    pub parent: Entity,
    pub epoch: f64,
    pub period: f64,
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            parent: Entity::PLACEHOLDER,
            epoch: Default::default(),
            period: Default::default(),
        }
    }
}

impl Orbit {
    pub fn new(parent: Entity) -> Self {
        Self {
            parent,
            epoch: 0.0,
            period: 0.0,
        }
    }

    pub fn parent(&self) -> Entity { self.parent }
}
