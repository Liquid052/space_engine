use super::{newton_approx, standard_gravitational_parameter};

/// Hyperbolic Anomaly (F) is given by the equation:
/// M = e * sinh(F) - F
/// where
/// M is the hyperbolic mean anomaly
/// e is the eccentricity
///
/// https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectories.html#equation-eq-hyperbolic-keplers-equation
pub fn estimate_anomaly(M: f64, e: f64, tolerance: f64) -> f64 {
    newton_approx(
        // f(F) = e * sinh(F) - F - M
        |F| (e * F.sinh()) - F - M,
        // f'(F) = e * cosh(F) - 1
        |F| e * F.cosh() - 1.0,
        M,
        tolerance,
    )
}

/// Hyperbolic mean motion
/// SRC: https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectories.html#equation-eq-hyperbolic-mean-anomaly
pub fn mean_motion(h: f64, e: f64, mass: f64) -> f64 {
    let micro = standard_gravitational_parameter(mass);

    (micro.powi(2) / h.powi(3)) * (e.powi(2) - 1.0).powi(3).sqrt()
}

pub fn true_anomaly(F: f64, e: f64) -> f64 {
    // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectories.html#equation-eq-eccentric-anomaly-true-anomaly-hyperbola
    2.0 * ((F / 2.0).tanh() / ((e - 1.0) / (e + 1.0)).sqrt()).atan()
}
