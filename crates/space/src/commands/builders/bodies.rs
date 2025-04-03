use std::f64::consts::PI;

use crate::bundles::BeltBundle;
use crate::helpers::calculate_galactic_soi;
use crate::prelude::SpaceDepth;
use crate::{
    bundles::{CelestialBodyBundle, StarBundle},
    components::*,
    constants::{BODY_DEPTH, SPACE_SCALE},
    prelude::{Belt, Body},
    resources::StarSystem,
};
use bevy::ecs::world::Command;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::shapes;
use engine_core::prelude::*;

/// Builder command for creating a star
#[derive(Default, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct Star {
    pub name: String,
    pub mass: f64,
    pub radius: f64,
    pub color: Color,

    pub belts: Vec<(f64, f64, Color)>,
}

impl Star {
    /// Creates a new `Star` instance with a specified name and default values for other properties.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mass: 1.0,
            radius: 1.0,
            color: Color::srgba(1.0, 1.0, 0.0, 1.0),
            ..default()
        }
    }

    /// Sets the mass of the star and returns the builder instance.
    pub fn mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }

    /// Sets the radius of the star and returns the builder instance.
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the color of the star and returns the builder instance.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Adds a belt to the star with specified radius, width, and color, returning the builder instance.
    pub fn belt(mut self, radius: f64, width: f64, col: Color) -> Self {
        self.belts.push((radius, width, col));
        self
    }

    
}

/// Builder command for creating a celestial body (planet or moon, used with aliases)
#[derive(Default)]
pub struct BodyBuilder {
    pub(crate) radius:   f64,
    pub(crate) mass:     f64,
    pub(crate) name:     String,
    pub(crate) color:    Color,
    pub(crate) orbiting: Option<String>,

    pub(crate) eccentricity:          f64,
    pub(crate) semi_major_axis:       f64,
    pub(crate) argument_of_periapsis: f64,
    pub(crate) reversed_orbit:        bool,
    pub(crate) mean_anomaly_at_epoch: f64,

    pub(crate) belts: Vec<(f64, f64, Color)>,
}

impl BodyBuilder {
    /// Creates a new `BodyBuilder` instance with a specified name and default values for other properties.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..default()
        }
    }

    /// Sets the mass of the celestial body and returns the builder instance.
    pub fn mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }

    /// Sets the radius of the celestial body and returns the builder instance.
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the color of the celestial body and returns the builder instance.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the parent celestial body that this body will orbit and returns the builder instance.
    pub fn orbiting(mut self, orbiting: impl Into<String>) -> Self {
        self.orbiting = Some(orbiting.into());
        self
    }

    /// Sets the eccentricity of the body's orbit and returns the builder instance.
    pub fn eccentricity(mut self, eccentricity: f64) -> Self {
        self.eccentricity = eccentricity;
        self
    }

    /// Sets the semi-major axis of the body's orbit and returns the builder instance.
    pub fn semi_major_axis(mut self, semi_major_axis: f64) -> Self {
        self.semi_major_axis = semi_major_axis;
        self
    }

    /// Sets the argument of periapsis for the body's orbit and returns the builder instance.
    pub fn argument_of_periapsis(mut self, argument_of_periapsis: f64) -> Self {
        self.argument_of_periapsis = argument_of_periapsis;
        self
    }

    /// Sets the mean anomaly at epoch for the body's orbit and returns the builder instance.
    pub fn mean_anomaly_at_epoch(mut self, mean_anomaly_at_epoch: f64) -> Self {
        self.mean_anomaly_at_epoch = mean_anomaly_at_epoch;
        self
    }

    /// Sets whether the body orbits in a reversed direction and returns the builder instance.
    pub fn reversed_orbit(mut self, reversed_orbit: bool) -> Self {
        self.reversed_orbit = reversed_orbit;
        self
    }

    #[doc(hidden)]
    //Adds a belt to the celestial body with specified radius, width, and color, returning the builder instance.
    pub fn belt(mut self, radius: f64, width: f64, col: Color) -> Self {
        self.belts.push((radius, width, col));
        self
    }

    pub(crate) fn keplerian(&self) -> Keplerian {
        Keplerian {
            eccentricity: self.eccentricity,
            semi_major_axis: self.semi_major_axis,
            inclination: if self.reversed_orbit { PI } else { 0.00001 },
            argument_of_periapsis: self.argument_of_periapsis,
            mean_anomaly_at_epoch: self.mean_anomaly_at_epoch,
            ..default()
        }
    }
}

impl Command for BodyBuilder {
    fn apply(self, world: &mut World) {
        let keplerian = self.keplerian();

        let parent = match self.orbiting.is_some() {
            true  => world.resource_ref::<NameReg>().get(&self.orbiting.unwrap()).unwrap(),
            false => world.resource_ref::<StarSystem>().get_sun()
                .expect("Can't place planet without Sun")
        };

        let name = Name::new(self.name);
        let depth = world.get_mut::<SpaceDepth>(parent)
            .unwrap()
            .return_down();
        let orbit = Orbit::new(parent);
        let body = Body::new(self.radius, self.mass);

        let mesh = world.resource_mut::<Assets<Mesh>>()
            .add(Circle {
                radius: (self.radius / SPACE_SCALE) as f32,
            });
        let material = world.resource_mut::<Assets<ColorMaterial>>()
            .add(self.color);

        let ent = world.spawn(CelestialBodyBundle {
            material_bundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh),
                transform: Transform::from_xyz(0.0, 0.0, BODY_DEPTH),
                material,
                ..default()
            },
            name,
            depth,
            keplerian,
            body,
            orbit,
            ..default()
        }).id();

        // update parent
        world.get_mut::<RefFrame>(parent).unwrap().push_body(ent);

        // if no belts are required, return, otherwise install
        if self.belts.is_empty() {
            return;
        }

        let mut belt = Belt::new();

        self.belts.iter()
            .for_each(|(radius, width, col)| {
                let path = GeometryBuilder::build_as(&shapes::Circle {
                    radius: (radius / SPACE_SCALE) as f32,
                    ..default()
                });
                let handle = world.resource_mut::<Assets<ColorMaterial>>()
                    .add(self.color);

                let belt_id = world.spawn(BeltBundle::new(path,handle, *width, *col))
                    .id();

                world.entity_mut(ent).add_child(belt_id);
                belt.add((*radius, *width, *col));
                belt.entities.push(belt_id);
            });

        world.entity_mut(ent).insert(belt);
    }
}
impl Command for Star {
    fn apply(self, world: &mut World) {

        let name = Name::new(self.name);
        let radius = self.radius;
        let material = world.resource_mut::<Assets<ColorMaterial>>()
            .add(self.color);

        let mut body_params = Body::new(radius, self.mass);

        body_params.soi = calculate_galactic_soi(self.mass);


        let mesh = world.resource_mut::<Assets<Mesh>>()
            .add(Circle {
                radius: (radius / SPACE_SCALE) as f32,
            });


        let ent = world.spawn(StarBundle {
            name: name.clone(),
            material_bundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh),
                transform: Transform::from_xyz(0.0, 0.0, BODY_DEPTH),
                material,
                ..default()
            },
            body_params,
            abs_pos: Default::default(),
            ref_frame: Default::default(),
            keplerian: Default::default(),
            depth: Default::default(),
            ..default()
        }).id();


        world.resource_mut::<StarSystem>().set_sun(ent);

        // if no belts are required, return, otherwise install
        if self.belts.is_empty() {
            return;
        }

        let mut belt = Belt::new();

        self.belts.iter()
            .for_each(|(radius, width, col)| {
                let path = GeometryBuilder::build_as(&shapes::Circle {
                    radius: (radius / SPACE_SCALE) as f32,
                    ..default()
                });
                let handle = world.resource_mut::<Assets<ColorMaterial>>()
                    .add(self.color);

                let belt_id = world.spawn(BeltBundle::new(path,handle, *width, *col))
                    .id();

                world.entity_mut(ent).add_child(belt_id);
                belt.add((*radius, *width, *col));
                belt.entities.push(belt_id);
            });


        world.entity_mut(ent).insert(belt);
    }
}


pub type Planet = BodyBuilder;
pub type Moon = BodyBuilder;