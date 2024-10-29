use crate::{
    components::{
        Body, Keplerian, Orbit, RefFrame, SpaceDepth, SpaceLayer, SpacePos, StarMarker, StateVec,
        VesselMarker,
    },
    constants::{BODY_DEPTH, SPACE_LAYER, SPACE_SCALE},
    helpers::calculate_galactic_soi,
};
use bevy::{
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use engine_core::components::naming::UniquelyNamed;
#[derive(Bundle)]
pub struct StarBundle {
    pub name: Name,
    pub material_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub body: RenderLayers,
    pub body_params: Body,
    pub abs_pos: SpacePos,
    pub ref_frame: RefFrame,
    pub keplerian: Keplerian,
    pub state_vectors: StateVec,
    // markers
    pub depth: SpaceDepth,
    pub star: StarMarker,
    pub space: SpaceLayer,
    pub uniquely: UniquelyNamed
}

impl Default for StarBundle {
    fn default() -> Self {
        Self {
            material_bundle: default(),
            state_vectors: default(),
            ref_frame: default(),
            name: Name::from("Star"),
            keplerian: default(),
            body: SPACE_LAYER,
            body_params: Body::new(50000.0, 10000.0),
            depth: default(),
            star: StarMarker,
            space: SpaceLayer,
            abs_pos: default(),
            uniquely: default(),
        }
    }
}

impl StarBundle {
    pub fn new(
        name: &str,
        radius: f64,
        mass: f64,
        col: &Color,
        materials: &mut Assets<ColorMaterial>,
        meshes: &mut Assets<Mesh>,
    ) -> Self {
        let mut body_params = Body::new(radius, mass);

        body_params.soi = calculate_galactic_soi(mass);

        StarBundle {
            material_bundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle {
                    radius: (radius / SPACE_SCALE) as f32,
                })),
                transform: Transform::from_xyz(0.0, 0.0, BODY_DEPTH),
                material: materials.add(*col),
                ..default()
            },
            name: Name::from(name),
            body_params,
            ..default()
        }
    }
}

#[derive(Bundle)]
pub struct CelestialBodyBundle {
    pub material_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub render_layer: RenderLayers,
    pub ref_frame: RefFrame,
    pub name: Name,
    pub state_vectors: StateVec,
    pub depth: SpaceDepth,
    pub keplerian: Keplerian,
    pub body: Body,
    pub orbit: Orbit,
    pub abs_pos: SpacePos,

    // markers
    pub space: SpaceLayer,
    pub uniquely: UniquelyNamed
}

impl Default for CelestialBodyBundle {
    fn default() -> Self {
        Self {
            material_bundle: default(),
            render_layer: SPACE_LAYER,
            ref_frame: default(),
            name: default(),
            state_vectors: default(),
            keplerian: default(),
            depth: default(),
            body: default(),
            orbit: default(),
            abs_pos: default(),
            space: default(),
            uniquely: default(),
        }
    }
}

#[derive(Bundle)]
pub struct VesselBundle {
    pub material_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub render_layer: RenderLayers,
    pub name: Name,
    pub state_vectors: StateVec,
    pub keplerian: Keplerian,
    pub orbit: Orbit,
    pub abs_pos: SpacePos,
    pub depth: SpaceDepth,

    // markers
    pub space: SpaceLayer,
    pub uniquely_named: UniquelyNamed,
    pub vessel: VesselMarker,
}

impl Default for VesselBundle {
    fn default() -> Self {
        Self {
            material_bundle: default(),
            render_layer: SPACE_LAYER,
            name: default(),
            state_vectors: default(),
            depth: default(),
            keplerian: default(),
            vessel: default(),
            orbit: default(),
            uniquely_named: default(),
            abs_pos: default(),
            space: default(),
        }
    }
}