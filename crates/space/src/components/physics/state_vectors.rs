use bevy::{ecs::component::Component, math::DVec3, prelude::*};

use crate::{helpers::*, prelude::*, TWO_PI};

#[derive(Debug, Component, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct StateVec {
    pub position: DVec3,
    pub velocity: DVec3,
}

impl StateVec {
    pub fn new(position: DVec3, velocity: DVec3) -> Self { Self { position, velocity } }

    pub fn abs_diff(&self, other: &Self) -> f64 {
        self.position.distance(other.position) + self.velocity.distance(other.velocity)
    }

    //noinspection ALL
    pub fn to_elements(&self, mass: f64) -> Keplerian {
        // Position magnitude
        let rv = self.position;
        let r = rv.length();
        let vv = self.velocity;
        let v_mag = vv.length();

        // Orbital angular momentum
        // This vector should point in the normal direction of the orbit
        let hv = rv.cross(vv);
        let h = hv.length();

        // N vector - vector that lies on the node line in the direction of the ascending node
        let mut nv = DVec3::Z.cross(hv);

        // If inclinations is 0 this vector will be zero as well
        // since a lot of other arguments depend on this vector
        // we set it to the X axis
        if nv.length() < f64::EPSILON {
            nv = DVec3::X;
        }

        let nv = nv.normalize();

        // Eccentricity
        let μ = standard_gravitational_parameter(mass);

        let ev = (1.0 / μ) * ((v_mag.powi(2) - (μ / r)) * rv - rv.dot(vv) * vv);
        let e = ev.length();

        let is_hyperbolic = e >= 1.0; // or parabolic

        // Right ascension of the ascending node

        // Inclination
        // Equation is i = arccos(hz / h)
        let i = (hv.z / h).acos();

        // We find the angle between the node line & the X axis
        let mut Ω = (nv.x).acos();

        if nv.y < 0.0 {
            Ω = TWO_PI - Ω;
        }

        if i.abs() < f64::EPSILON {
            Ω = 0.0;
        }

        // Argument of periapsis
        let mut ω = (ev / e).dot(nv).acos();

        // An edge case for a zero inclination orbit
        // If an orbit has zero inclination,
        // the z component of the eccentricity vector
        // is zero.
        //
        // But we can still do a quadrant check using the y component
        if i.abs() < f64::EPSILON {
            if ev.y < 0.0 {
                ω = TWO_PI - ω;
            }
        } else if ev.z < 0.0 {
            ω = TWO_PI - ω;
        }

        if e == 0.0 {
            // For a circular orbit the argument of periapsis is undefined
            ω = 0.0
        }

        // Semi-major axis
        let a = if is_hyperbolic {
            (h.powi(2) / μ) / (e.powi(2) - 1.0)
        } else {
            (h.powi(2) / μ) / (1.0 - e.powi(2))
        };

        // True anomaly
        let mut v = (rv / r).dot(ev / e).acos();

        if ((rv / r).dot(vv / v_mag)) < 0.0 {
            v = TWO_PI - v;
        }

        // Mean anomaly calculation
        let M = if is_hyperbolic {
            calculate_hyperbolic_mean_anomaly(e, v)
        } else {
            calculate_elliptical_mean_anomaly(e, v)
        };

        Keplerian {
            eccentricity: e,
            semi_major_axis: a,
            inclination: i,
            right_ascension_of_the_ascending_node: Ω,
            argument_of_periapsis: ω,
            mean_anomaly_at_epoch: M,
        }
    }
}

// Hyperbolic mean anomaly calculation
fn calculate_hyperbolic_mean_anomaly(e: f64, v: f64) -> f64 {
    let term1 = (e * (e.powi(2) - 1.0).sqrt() * v.sin()) / (1.0 + e * v.cos());
    let term2_numerator = (e + 1.0).sqrt() + (e - 1.0).sqrt() * (v / 2.0).tan();
    let term2_denominator = (e + 1.0).sqrt() - (e - 1.0).sqrt() * (v / 2.0).tan();

    term1 - (term2_numerator / term2_denominator).ln()
}

// Elliptical mean anomaly calculation
fn calculate_elliptical_mean_anomaly(e: f64, v: f64) -> f64 {
    let term1 = 2.0 * (((1.0 - e) / (1.0 + e)).sqrt() * (v / 2.0).tan()).atan();
    let term2 = e * ((1.0 - e.powi(2)).sqrt() * v.sin() / (1.0 + e * v.cos()));

    term1 - term2
}
