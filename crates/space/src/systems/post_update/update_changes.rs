use bevy::prelude::{default, Changed, Query, Without};
use bevy_prototype_lyon::{entity::Path, geometry::GeometryBuilder};

use crate::{
    components::{Body, DrawSpace, Keplerian, Restricted, StarMarker},
    constants::SPACE_SCALE,
};

// aliases
type BodiesWithSoi<'a> = (&'a Body, &'a DrawSpace);

pub fn update_soi_outline(
    bodies: Query<BodiesWithSoi, (Changed<Keplerian>, Without<Restricted>, Without<StarMarker>)>,
    mut path: Query<&mut Path>,
) {
    bodies.iter().for_each(|(body_p, ref_frame)| {
        let soi = ref_frame.soi.unwrap();

        let circle = bevy_prototype_lyon::shapes::Circle {
            radius: (body_p.soi / SPACE_SCALE) as f32,
            ..default()
        };

        *path.get_mut(soi).unwrap() = GeometryBuilder::build_as(&circle);
    });
}
