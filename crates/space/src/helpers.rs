use crate::constants::*;

pub fn calculate_galactic_soi(m: f64) -> f64 {
    const COSMIC_A: f64 = 2.5e20;
    const M: f64 = 4.0e41;

    let mass_ratio = m / M;
    let exponent = 2.0 / 5.0;

    COSMIC_A * mass_ratio.powf(exponent)
}

pub mod elliptic;
pub mod hyperbolic;

#[inline]
pub fn standard_gravitational_parameter(mass: f64) -> f64 { G * mass }

const MAX_STEPS: usize = 100_000;

pub fn newton_approx(
    f: impl Fn(f64) -> f64,
    f_prime: impl Fn(f64) -> f64,
    x0: f64,
    epsilon: f64,
) -> f64 {
    let mut x = x0;

    for _ in 0..MAX_STEPS {
        let x_next = x - f(x) / f_prime(x);

        let error = (x_next - x).abs();

        if error < epsilon {
            return x_next;
        }

        x = x_next;
    }

    panic!(
        "Failed to converge after {} iterations (x0 = {}, x = {})",
        MAX_STEPS, x0, x
    );
}
