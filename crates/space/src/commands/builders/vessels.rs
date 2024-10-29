use std::f64::consts::PI;

use crate::prelude::*;
use bevy::ecs::world::Command;
use bevy::prelude::{RegularPolygon, World};
use bevy::{
    asset::Assets,
    core::Name,
    prelude::{default, Circle, Color, ColorMaterial, Mesh, Transform},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use engine_core::prelude::*;

#[derive(Default)]
pub struct Vessel {
    name:     String,
    color:    Color,
    orbiting: Option<String>,
    eccentricity:          f64,
    semi_major_axis:       f64,
    argument_of_periapsis: f64,
    reversed_orbit:        bool,
    mean_anomaly_at_epoch: f64,
}

pub type SpaceShip = Vessel;

impl Vessel {
    // constructors
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..default()
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn orbiting(mut self, orbiting: impl Into<String>) -> Self {
        self.orbiting = Some(orbiting.into());
        self
    }

    pub fn eccentricity(mut self, eccentricity: f64) -> Self {
        self.eccentricity = eccentricity;
        self
    }

    pub fn semi_major_axis(mut self, semi_major_axis: f64) -> Self {
        self.semi_major_axis = semi_major_axis;
        self
    }

    pub fn argument_of_periapsis(mut self, argument_of_periapsis: f64) -> Self {
        self.argument_of_periapsis = argument_of_periapsis;
        self
    }

    pub fn mean_anomaly_at_epoch(mut self, mean_anomaly_at_epoch: f64) -> Self {
        self.mean_anomaly_at_epoch = mean_anomaly_at_epoch;
        self
    }

    pub fn reversed_orbit(mut self, reversed_orbit: bool) -> Self {
        self.reversed_orbit = reversed_orbit;
        self
    }
}

impl Command for Vessel {
    fn apply(self, world: &mut World) {
        let keplerian = Keplerian {
            eccentricity: self.eccentricity,
            semi_major_axis: self.semi_major_axis,
            inclination: if self.reversed_orbit { PI } else { 0.0 },
            argument_of_periapsis: self.argument_of_periapsis,
            mean_anomaly_at_epoch: self.mean_anomaly_at_epoch,
            ..default()
        };

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

        // todo - actual spawn
        let mesh = world.resource_mut::<Assets<Mesh>>()
            .add(RegularPolygon {
                circumcircle: Circle { radius: 10.0 },
                sides: 3,
            });
        let material = world.resource_mut::<Assets<ColorMaterial>>()
            .add(self.color);

        let ent = world.spawn(VesselBundle {
            material_bundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh),
                transform: Transform::from_xyz(0.0, 0.0, BODY_DEPTH),
                material,
                ..default()
            },
            name,
            depth,
            keplerian,
            orbit,
            ..default()
        }).id();


        // update parent
        world.get_mut::<RefFrame>(parent).unwrap().push_vessel(ent);
    }
}

