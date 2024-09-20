use bevy::{prelude::*, render::view::RenderLayers, sprite::Mesh2dHandle};
use bevy_prototype_lyon::prelude::*;

use crate::{
    constants::{SPACE_LAYER, SPACE_SCALE},
    prelude::*,
};

#[derive(Bundle)]
pub struct BeltBundle {
    pub path:          Path,
    pub mesh:          Mesh2dHandle,
    pub material:      Handle<ColorMaterial>,
    pub spatial:       SpatialBundle,
    pub stroke:        Stroke,
    pub name:          Name,
    pub render_layers: RenderLayers,

    // markers
    pub restrict_draw: RestrictDraw,
}

impl Default for BeltBundle {
    #[inline]
    fn default() -> BeltBundle {
        BeltBundle {
            path:          default(),
            mesh:          default(),
            material:      default(),
            spatial:       default(),
            restrict_draw: default(),
            render_layers: SPACE_LAYER,
            name:          Name::new("Belt"),
            stroke:        Stroke::new(Color::WHITE, 5.0),
        }
    }
}

impl BeltBundle {
    pub fn new(path: Path, handle: Handle<ColorMaterial>, width: f64, col: Color) -> Self {
        let line_width = (width / SPACE_SCALE) as f32;

        Self {
            path,
            material: handle,
            stroke: Stroke::new(col, line_width),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, -0.4),
                ..default()
            },
            ..default()
        }
    }
}
