use crate::prelude::*;
use bevy::ecs::world::Command;
use bevy::{
    asset::Assets,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_prototype_lyon::entity::ShapeBundle;
use engine_core::components::naming::UniquelyNamed;
use engine_core::prelude::*;


pub struct TwoBodyBuilder<T> {
    body1: T,
    body2: T,

    override1:     bool,
    override2:     bool,
    eccentricity1: f64,
    eccentricity2: f64,
    orbiting:      Option<String>,
}

impl TwoBodyBuilder<BodyBuilder> {
    pub fn orbiting(mut self, p0: impl Into<String>) -> Self {
        self.orbiting = Some(p0.into());

        self
    }
}

impl<T> TwoBodyBuilder<T> {
    pub fn new(body_1: T, body_2: T) -> Self {
        Self {
            body1:         body_1,
            body2:         body_2,
            eccentricity1: 0.0,
            eccentricity2: 0.0,
            override1:     false,
            override2:     false,
            orbiting: None,
        }
    }

    pub fn set_1(mut self, body_1: T) -> Self {
        self.body1 = body_1;
        self.override1 = true;

        self
    }

    pub fn set_2(mut self, body_2: T) -> Self {
        self.body2 = body_2;
        self.override2 = true;

        self
    }

    pub fn override_orbits(mut self, override_orbits: bool) -> Self {
        self.override1 = override_orbits;
        self.override2 = override_orbits;
        self
    }

    pub fn eccentricity_1(mut self, eccentricity: f64) -> Self {
        self.eccentricity1 = eccentricity;
        self.override1 = true;

        self
    }

    pub fn eccentricity_2(mut self, eccentricity: f64) -> Self {
        self.eccentricity2 = eccentricity;
        self.override2 = true;

        self
    }

    pub fn eccentricity(mut self, eccentricity: f64) -> Self {
        self.eccentricity1 = eccentricity;
        self.eccentricity2 = eccentricity;
        self.override1 = true;
        self.override2 = true;

        self
    }
}


impl Command for TwoBodyBuilder<BodyBuilder> {
    fn apply(self, world: &mut World) {
        let parent: Entity = match self.orbiting.is_some() {
            true => {
                let name = &self.orbiting.unwrap();
                
                let name_reg = world.resource::<NameReg>();
                
                let Some(ent) = name_reg.get(name) else {
                    panic!("Body named {} doesn't exist", name);    
                };
                
                ent
            }
            false => {
                world.resource::<StarSystem>().get_sun().expect("Attempted to add two body system where sun doesn't exist")
            }
        };

        let mass = world.entity_mut(parent).get::<Body>().unwrap().mass;

        let t = 0.0;
        let k_1 = self.body1.keplerian();
        let k_2 = self.body2.keplerian();

        let st_1 = k_1.state_vectors_at_epoch(mass, t, 0.001);
        let mut st_2 = k_2.state_vectors_at_epoch(self.body1.mass, t, 0.001);
        st_2 = StateVec {
            position: st_2.position + st_1.position,
            velocity: st_1.velocity + st_2.velocity,
        };
        let st_b = barycenter(self.body1.mass, self.body2.mass, st_1, st_2);

        let rel_st_1 = StateVec {
            position: st_1.position - st_b.position,
            velocity: st_1.velocity - st_b.velocity,
        };
        let rel_st_2 = StateVec {
            position: st_2.position - st_b.position,
            velocity: st_2.velocity - st_b.velocity,
        };

        let mu = reduced_mass(self.body1.mass, self.body2.mass);
        let k_b = st_b.to_elements(mass);
        let mut k_1 = rel_st_1.to_elements(mu);
        let mut k_2 = rel_st_2.to_elements(mu);

        if self.override1 {
            k_1.eccentricity = self.eccentricity1;
        }
        if self.override2 {
            k_2.eccentricity = self.eccentricity2;
        }

        let soi = k_b.calculate_soi(self.body1.mass + self.body2.mass, mass);

        let name = format!("{} + {}", self.body1.name, self.body2.name);

        let depth = world.get::<SpaceDepth>(parent).unwrap().return_down();

        // Create meshes and materials before spawning entities
        let circle_mesh = world.resource_mut::<Assets<Mesh>>().add(Circle { radius: 10000.0 });
        let color_material = world.resource_mut::<Assets<ColorMaterial>>().add(ColorMaterial {
            color: Color::srgba(1.0, 0.0, 0.0, 1.0),
            ..default()
        });
        let body1_mesh = world.resource_mut::<Assets<Mesh>>().add(Circle {
            radius: (self.body1.radius / SPACE_SCALE) as f32,
        });
        let body2_mesh = world.resource_mut::<Assets<Mesh>>().add(Circle {
            radius: (self.body2.radius / SPACE_SCALE) as f32,
        });
        let body1_material = world.resource_mut::<Assets<ColorMaterial>>().add(self.body1.color);
        let body2_material = world.resource_mut::<Assets<ColorMaterial>>().add(self.body2.color);

        let ent = world.spawn((
            ShapeBundle {
                mesh: Mesh2dHandle(circle_mesh),
                material: color_material,
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                },
                ..default()
            },
            TwoBody,
            RefFrame::new(),
            UniquelyNamed,
            st_b,
            k_b,
            SpacePos(st_b.position.truncate()),
            Orbit {
                parent,
                epoch:  0.0,
                period: k_b.period(mass),
            },
            Name::new(name.clone()),
            SPACE_LAYER,
        )).id();

        let ent1 = world.spawn((
            CelestialBodyBundle {
                name: Name::from(self.body1.name.clone()),
                material_bundle: MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(body1_mesh),
                    transform: Transform::from_xyz(0.0, 0.0, BODY_DEPTH),
                    material: body1_material,
                    ..default()
                },
                orbit: Orbit::new(ent),
                ref_frame: RefFrame::new(),
                render_layer: SPACE_LAYER,
                keplerian: k_1,
                state_vectors: rel_st_1,
                body: Body::new(self.body1.radius, self.body1.mass),
                ..default()
            },
            Restricted,
        )).id();

        let ent2 = world.spawn((
            CelestialBodyBundle {
                name: Name::from(self.body2.name.clone()),
                material_bundle: MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(body2_mesh),
                    transform: Transform::from_xyz(0.0, 0.0, BODY_DEPTH),
                    material: body2_material,
                    ..default()
                },
                orbit: Orbit::new(ent),
                ref_frame: RefFrame::new(),
                keplerian: k_2,
                state_vectors: rel_st_2,
                render_layer: SPACE_LAYER,
                body: Body::new(self.body2.radius, self.body2.mass),
                ..default()
            },
            Restricted,
        )).id();

        world.entity_mut(ent).insert(Body {
            soi,
            radius: 0.0,
            mass: self.body1.mass + self.body2.mass,
            reduced_mass: mu,
            child1: Some(ent1),
            child2: Some(ent2),
        });

        world.entity_mut(ent).insert(depth);
        let down = depth.return_down();

        world.entity_mut(ent1).insert(down);
        world.entity_mut(ent2).insert(down);

        if !self.body1.belts.is_empty() {
            let mut belt_b = Belt::default();
            self.body1.belts.iter().for_each(|belt| {
                belt_b.add((belt.0, belt.1, belt.2));
            });
            world.entity_mut(ent1).insert(belt_b);
        }
        if !self.body2.belts.is_empty() {
            let mut belt_b = Belt::default();
            self.body2.belts.iter().for_each(|belt| {
                belt_b.add((belt.0, belt.1, belt.2));
            });
            world.entity_mut(ent2).insert(belt_b);
        }


        world.resource_mut::<StarSystem>();
        // space.planet_info.insert(ent, (name.clone(), mu, depth));
        // space.planet_info.insert(ent1, (self.body1.name.clone(), self.body1.mass, down));
        // space.planet_info.insert(ent2, (self.body2.name.clone(), self.body2.mass, down));
        // 
        // 
        // space.planet_map.insert(name.clone(), ent);
        // space.planet_map.insert(self.body1.name.clone(), ent1);
        // space.planet_map.insert(self.body2.name.clone(), ent2);
        
        world.entity_mut(parent).get_mut::<RefFrame>().unwrap().push_body(ent);
    }
}


// helpers
fn reduced_mass(mass1: f64, mass2: f64) -> f64 { mass1 * mass2 / (mass1 + mass2) }
fn barycenter(mass1: f64, mass2: f64, st1: StateVec, st2: StateVec) -> StateVec {
    let total_mass = mass1 + mass2;
    let pos = (st1.position * mass1 + st2.position * mass2) / total_mass;
    let vel = (st1.velocity * mass1 + st2.velocity * mass2) / total_mass;

    StateVec {
        position: pos,
        velocity: vel,
    }
}
