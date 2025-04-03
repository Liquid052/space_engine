use bevy::prelude::*;

/// Properties describing a celestial body such as sphere of influence, its radius and mass
#[derive(Reflect, Component, Default, Clone, Copy, Debug, PartialEq)]
#[reflect(Component)]
pub struct Body {
    pub soi:    f64,
    pub radius: f64,
    pub mass:   f64,

    // two-body
    #[doc(hidden)]
    pub reduced_mass: f64,
    #[doc(hidden)]
    pub child1:       Option<Entity>,
    #[doc(hidden)]
    pub child2:       Option<Entity>,
}

// for special cases where SOI is recalculated. Works as a marker that skips checks for orbital transfers
#[doc(hidden)]
#[derive(Reflect, Component, Clone, Copy, Debug, PartialEq, DerefMut, Deref)]
#[reflect(Component)]
pub struct Exited(pub Entity);

impl Body {
    // constructors
    pub fn new(radius: f64, mass: f64) -> Self {
        Self {
            soi: 0.0,
            radius,
            mass,
            reduced_mass: 0.0,
            child1: None,
            child2: None,
        }
    }

    pub fn mass_percentual_diff(&self, rhs_mass: f64) -> f64 {
        let absolute_difference = (self.mass - rhs_mass + 1.0).abs();

        let smaller_mass = if self.mass > rhs_mass {
            rhs_mass
        } else {
            self.mass
        };

        smaller_mass / absolute_difference
    }

    pub fn is_two_body(&self) -> bool { self.child1.is_some() && self.child2.is_some() }
}


#[derive(Reflect, Component, Default, Clone, Debug, PartialEq)]
#[reflect(Component)]
#[doc(hidden)]
pub struct Belt {
    pub(crate) belts: Vec<(f64, f64, Color)>,

    #[reflect(skip_serializing)]
    pub(crate) entities: Vec<Entity>,

    #[reflect(skip_serializing)]
    pub(crate) to_update: Vec<usize>,
}

impl Belt {
    pub fn new() -> Self {
        Self {
            belts:     Vec::new(),
            entities:  Vec::new(),
            to_update: Vec::new(),
        }
    }

    pub fn add(&mut self, radius: (f64, f64, Color)) { self.belts.push(radius); }

    pub(crate) fn get_pair(&mut self) -> (&mut Vec<(f64, f64, Color)>, &mut Vec<Entity>) {
        (&mut self.belts, &mut self.entities)
    }

    pub fn change(&mut self, index: usize, data: (f64, f64, Color)) {
        self.belts[index] = data;
        self.to_update.push(index);
    }
}
