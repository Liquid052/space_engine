//! This plugin provides functionalities for simulating and rendering celestial mechanics in a game or application.
//!
//! ## Features
//!
//! - **Orbital Transfers**: Calculate and execute orbital transfers between celestial bodies, allowing for realistic space travel mechanics.
//!
//! - **Rendering**: Render celestial bodies and their orbits in a visually appealing manner, integrating with Bevy's rendering system.
//!
//! - **Camera Tracking**: Track and follow celestial bodies with the camera, providing an immersive experience as players navigate through space.

#![allow(non_snake_case)]
use constants::{PI, TWO_PI};

pub(crate) mod helpers;
pub(crate) mod systems;

pub mod components;

pub mod constants;
pub mod resources;
pub mod commands;
pub mod plugins;

#[doc(hidden)]
pub mod bundles;

#[doc(hidden)]
pub mod events;

// export
#[doc(hidden)]
pub mod prelude;

pub use systems::SpaceSystemSet;

#[doc(hidden)]
#[cfg(test)]
mod tests {
    use bevy::math::{dvec3, DVec3};
    use test_case::test_case;

    use super::*;

    use self::prelude::*;

    const MASS: f64 = 100_000_000_000.0;
    const EPOCH: f64 = 0.0;
    const MAX_ABS_DIFF: f64 = 0.0001;
    const TOLERANCE: f64 = 0.0001;

    fn test_back_and_forth_conversion(original: Keplerian, mass: f64, epoch: f64) {
        let sv = original.state_vectors_at_epoch(mass, epoch, TOLERANCE);

        let elements = Keplerian::from_state_vectors(&sv, mass);

        let sv_converted = elements.state_vectors_at_epoch(mass, epoch, TOLERANCE);

        let pos_diff = sv.position.distance(sv_converted.position);
        assert!(
            sv.position.abs_diff_eq(sv_converted.position, MAX_ABS_DIFF),
            "Position {:?} not equal {:?} - distance is {}",
            sv.position,
            sv_converted.position,
            pos_diff
        );
        assert!(
            sv.velocity.abs_diff_eq(sv_converted.velocity, MAX_ABS_DIFF),
            "Velocity {:?} not equal {:?}",
            sv.velocity,
            sv_converted.velocity
        );
    }

    #[test]
    fn conversion_zero_params() {
        test_back_and_forth_conversion(
            Keplerian {
                eccentricity: 0.0,
                semi_major_axis: 1.0,
                inclination: 0.0,
                right_ascension_of_the_ascending_node: 0.0,
                argument_of_periapsis: 0.0,
                mean_anomaly_at_epoch: 0.0,
            },
            MASS,
            EPOCH,
        );
    }

    #[test]
    fn conversion_zero_inclination() {
        test_back_and_forth_conversion(
            Keplerian {
                eccentricity: 0.01,
                semi_major_axis: 1.0,
                inclination: 0.0,
                right_ascension_of_the_ascending_node: 1.3,
                argument_of_periapsis: 1.2,
                mean_anomaly_at_epoch: 0.2,
            },
            MASS,
            EPOCH,
        );
    }

    #[test]
    fn conversion_highly_eccentric() {
        test_back_and_forth_conversion(
            Keplerian {
                eccentricity: 0.9,
                semi_major_axis: 1.0,
                inclination: 0.0,
                right_ascension_of_the_ascending_node: 0.0,
                argument_of_periapsis: 0.0,
                mean_anomaly_at_epoch: 0.0,
            },
            MASS,
            EPOCH,
        );
    }

    #[test]
    fn conversion_arbitrary() {
        test_back_and_forth_conversion(
            Keplerian {
                eccentricity: 0.123,
                semi_major_axis: 1.0,
                inclination: 1.2,
                right_ascension_of_the_ascending_node: 0.5,
                argument_of_periapsis: 0.3,
                mean_anomaly_at_epoch: 1.01,
            },
            MASS,
            EPOCH,
        );
    }

    #[test_case(0.0, dvec3(1.0, 0.0, 0.0))]
    #[test_case(PI / 2.0, dvec3(0.0, 1.0, 0.0))]
    #[test_case(PI, dvec3(- 1.0, 0.0, 0.0))]
    #[test_case(PI + (PI / 2.0), dvec3(0.0, - 1.0, 0.0))]
    fn elements_to_position(v: f64, exp: DVec3) {
        let elements = Keplerian {
            eccentricity: 0.0,
            semi_major_axis: 1.0,
            inclination: 0.0,
            right_ascension_of_the_ascending_node: 0.0,
            argument_of_periapsis: 0.0,
            mean_anomaly_at_epoch: 0.0,
        };

        let position = elements.position_at_true_anomaly(MASS, v);

        assert!(
            position.abs_diff_eq(exp, MAX_ABS_DIFF),
            "Position {:?} not equal {:?}",
            position,
            exp
        );
    }
}
