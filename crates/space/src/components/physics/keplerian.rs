use bevy::{
    math::{dvec3, DMat3, DVec3},
    prelude::*,
};

use crate::{helpers::*, prelude::StateVec, PI, TWO_PI};

pub const DEFAULT_INCLINATION: f64 = 0.0001;

#[derive(Debug, Reflect, Component, Clone, Copy)]
#[reflect(Component)]
pub struct Keplerian {
    pub eccentricity: f64,
    pub semi_major_axis: f64,
    pub inclination: f64,
    pub right_ascension_of_the_ascending_node: f64,
    pub argument_of_periapsis: f64,
    pub mean_anomaly_at_epoch: f64,
}

impl Default for Keplerian {
    fn default() -> Self {
        Self {
            eccentricity: Default::default(),
            semi_major_axis: Default::default(),
            inclination: 0.0001,
            right_ascension_of_the_ascending_node: Default::default(),
            argument_of_periapsis: Default::default(),
            mean_anomaly_at_epoch: Default::default(),
        }
    }
}

impl Keplerian {
    pub fn calculate_soi(&self, mass: f64, mass_parent: f64) -> f64 {
        self.semi_major_axis * (mass / mass_parent).powf(2.5 / 5.0)
    }

    pub fn is_reversed(&self) -> bool {
        if self.inclination < 0.1 && self.inclination > -0.1 { false } else { true }
    }
    
    pub fn semi_minor_axis(&self) -> f64 {
        // Ensure that the semi_major_axis and eccentricity are defined
        if self.eccentricity >= 0.0 && self.eccentricity < 1.0 && self.semi_major_axis > 0.0 {
            self.semi_major_axis * (1.0 - self.eccentricity.powi(2)).sqrt()
        } else {
            // Handle the error case appropriately in your application
            // For example, you could return a default value or an error
            0.0 // Placeholder value
        }
    }

    pub fn angle_abs_diff(&self, other: &Self) -> f64 {
        let mut diff = 0.0;

        diff += (self.eccentricity - other.eccentricity).abs();
        diff += (self.inclination - other.inclination).abs();
        diff += (self.right_ascension_of_the_ascending_node
            - other.right_ascension_of_the_ascending_node)
            .abs();
        diff += (self.argument_of_periapsis - other.argument_of_periapsis).abs();
        diff += (self.mean_anomaly_at_epoch - other.mean_anomaly_at_epoch).abs();

        diff
    }

    pub fn from_state_vectors(state_vectors: &StateVec, mass: f64) -> Self {
        state_vectors.to_elements(mass)
    }

    pub fn ascending_node(&self, mass: f64) -> DVec3 {
        self.position_at_true_anomaly(mass, -self.argument_of_periapsis)
    }

    pub fn descending_node(&self, mass: f64) -> DVec3 {
        self.position_at_true_anomaly(mass, PI - self.argument_of_periapsis)
    }

    pub fn periapsis(&self, mass: f64) -> DVec3 { self.position_at_true_anomaly(mass, 0.0) }

    pub fn apoapsis(&self, mass: f64) -> DVec3 { self.position_at_true_anomaly(mass, PI) }

    pub fn focal_distance(&self) -> f64 {
        if self.eccentricity >= 0.0 && self.eccentricity < 1.0 && self.semi_major_axis > 0.0 {
            self.semi_major_axis * self.eccentricity
        } else {
            // Handle the error case appropriately in your application
            // For example, you could return a default value or an error
            0.0 // Placeholder value
        }
    }

    pub fn normal(&self) -> DVec3 { self.perifocal_to_equatorial(DVec3::Z) }

    pub fn period(&self, mass: f64) -> f64 { Self::period_static(self.semi_major_axis, mass) }

    pub fn period_static(a: f64, mass: f64) -> f64 {
        TWO_PI * (a.powi(3) / standard_gravitational_parameter(mass)).sqrt()
    }

    pub fn mean_anomaly(&self, mass: f64, epoch: f64) -> f64 {
        let h = self.specific_angular_momentum(mass);
        let e = self.eccentricity;

        self.mean_anomaly_at_epoch + elliptic::mean_motion(h, e, mass) * epoch
    }

    pub fn hyperbolic_mean_anomaly(&self, mass: f64, epoch: f64) -> f64 {
        let h = self.specific_angular_momentum(mass);
        let e = self.eccentricity;

        self.mean_anomaly_at_epoch + hyperbolic::mean_motion(h, e, mass) * epoch
    }

    pub fn estimate_eccentric_anomaly(&self, mass: f64, epoch: f64, tolerance: f64) -> f64 {
        let M = self.mean_anomaly(mass, epoch);
        let e = self.eccentricity;

        elliptic::estimate_anomaly(M, e, tolerance)
    }

    pub fn estimate_hyperbolic_anomaly(&self, mass: f64, epoch: f64, tolerance: f64) -> f64 {
        let M = self.hyperbolic_mean_anomaly(mass, epoch);
        let e = self.eccentricity;

        hyperbolic::estimate_anomaly(M, e, tolerance)
    }

    pub fn state_vectors_at_epoch(&self, mass: f64, epoch: f64, tolerance: f64) -> StateVec {
        // Lowercase nu
        let v = self.true_anomaly_at_epoch(mass, epoch, tolerance);

        StateVec {
            position: self.position_at_true_anomaly(mass, v),
            velocity: self.velocity_at_true_anomaly(mass, v),
        }
    }

    //noinspection ALL
    #[inline]
    pub fn position_at_true_anomaly(&self, mass: f64, v: f64) -> DVec3 {
        let e = self.eccentricity;
        let h = self.specific_angular_momentum(mass);
        let μ = standard_gravitational_parameter(mass);

        let r = (h.powi(2) / μ) / (1.0 + e * v.cos());

        // Perifocal coordinates
        let p = r * v.cos();
        let q = r * v.sin();

        let position = dvec3(p, q, 0.0);

        self.perifocal_to_equatorial(position)
    }

    //noinspection NonAsciiCharacters
    #[inline]
    pub fn velocity_at_true_anomaly(&self, mass: f64, v: f64) -> DVec3 {
        let e = self.eccentricity;
        let h = self.specific_angular_momentum(mass);
        let μ = standard_gravitational_parameter(mass);

        let vp = -(μ / h) * v.sin();
        let vq = (μ / h) * (e + v.cos());

        self.perifocal_to_equatorial(dvec3(vp, vq, 0.0))
    }

    //noinspection NonAsciiCharacters
    #[inline(always)]
    pub fn perifocal_to_equatorial(&self, perifocal: DVec3) -> DVec3 {
        let mut m = DMat3::IDENTITY;

        let Ω = self.right_ascension_of_the_ascending_node;
        let i = self.inclination;
        let ω = self.argument_of_periapsis;

        m *= DMat3::from_rotation_z(Ω);
        m *= DMat3::from_rotation_x(i);
        m *= DMat3::from_rotation_z(ω);

        m.mul_vec3(perifocal)
    }

    //noinspection NonAsciiCharacters
    pub fn specific_angular_momentum(&self, mass: f64) -> f64 {
        let μ = standard_gravitational_parameter(mass);
        let a = self.semi_major_axis;
        let e = self.eccentricity;

        if self.is_hyperbolic() {
            (μ * a * (e.powi(2) - 1.0)).sqrt()
        } else {
            (μ * a * (1.0 - e.powi(2))).sqrt()
        }
    }

    /// Calculates true anomaly
    pub fn true_anomaly_at_epoch(&self, mass: f64, epoch: f64, tolerance: f64) -> f64 {
        let e = self.eccentricity;

        if self.is_hyperbolic() {
            let F = self.estimate_hyperbolic_anomaly(mass, epoch, tolerance);
            hyperbolic::true_anomaly(F, e)
        } else {
            let E = self.estimate_eccentric_anomaly(mass, epoch, tolerance);
            elliptic::true_anomaly(E, e)
        }
    }

    pub fn is_elliptical(&self) -> bool { self.eccentricity < 1.0 }

    pub fn is_hyperbolic(&self) -> bool { self.eccentricity > 1.0 }
}
