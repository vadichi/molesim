use std::f64::consts::PI;

use crate::math::vector2::Vector2;
use crate::math::Real;

// Typically, approximately equal to inter-particle distance
const SMOOTHING_RADIUS: Real = 100.0;

pub fn influence_factor(source: Vector2, probe: Vector2) -> Real {
    let q = (source - probe).magnitude().abs() / SMOOTHING_RADIUS;

    let f = (3.0 / (2.0 * PI))
        * if q < 1.0 {
            (2.0 / 3.0) - q.powi(2) + 0.5 * q.powi(3)
        } else if q < 2.0 {
            (1.0 / 6.0) * (2.0 - q).powi(3)
        } else {
            0.0
        };

    // Here, the smoothing radius is raised to the power of the dimension of the space
    (1.0 / SMOOTHING_RADIUS.powi(2)) * f
}
