use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Ellipse};

use crate::{
    components::{Keplerian, Space},
    constants::SPACE_SCALE,
    prelude::{DrawSpace, RestrictDraw},
    resources::OrbitBuffer,
};

pub fn update_strokes(
    space_cams: Query<&OrthographicProjection, (Changed<OrthographicProjection>, With<Space>)>,
    mut buf: ResMut<OrbitBuffer>,
    mut strokes: Query<Entity, (With<Stroke>, Without<RestrictDraw>)>,
) {
    let cam = space_cams.single();

    buf.vec.clear();
    buf.current = cam.scale;

    strokes.iter_mut().for_each(|ent| {
        buf.vec.push(ent);
    });
}

pub fn update_orbit_shape(
    mut bodies: Query<(&Keplerian, &DrawSpace), Changed<Keplerian>>,
    mut orbits: Query<(&mut Transform, &mut Path)>,
) {
    bodies.iter_mut().for_each(|(keplerian, draw)| {
        let orbit_outline = draw.orbit.unwrap();

        let Ok((mut transform, mut path)) = orbits.get_mut(orbit_outline) else {
            return;
        };

        let mut multiply = 1.0;
        if !(keplerian.inclination < 0.1 && keplerian.inclination > -0.1) {
            multiply = -1.0;
        }

        // update angle of the orbit
        transform.rotation =
            Quat::from_rotation_z(multiply * keplerian.argument_of_periapsis as f32);

        // update shape
        let major = (keplerian.semi_major_axis / SPACE_SCALE) as f32;
        let minor = (keplerian.semi_minor_axis() / SPACE_SCALE) as f32;

        let ellipse = Ellipse {
            radii:  Vec2::new(major, minor),
            center: Vec2::new((-keplerian.focal_distance() / SPACE_SCALE) as f32, 0.0),
        };
        *path = GeometryBuilder::build_as(&ellipse);
    });
}

// For selecting how many orbits to update when you want to rescale the orbits' width
const ORBIT_RANGE: usize = 10;

pub fn update_requested_orbits(mut buf: ResMut<OrbitBuffer>, mut query: Query<&mut Stroke>) {
    let len = if buf.vec.len() < ORBIT_RANGE {
        buf.vec.len()
    } else {
        ORBIT_RANGE
    };

    buf.vec[..len].iter().for_each(|ent| {
        let Ok(mut stroke) = query.get_mut(*ent) else {
            error!("Entity[{:?}] not found - stroke system", ent);
            return;
        };

        stroke.options.line_width = buf.current;
    });

    buf.vec.drain(..len);
}
