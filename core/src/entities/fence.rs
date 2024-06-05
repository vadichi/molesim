use core::panic;

use crate::entities::{CollisionEntity, SimulationEntity};
use crate::math::{vector2::Vector2, Real};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fence {
    dimensions: Vector2,

    limit_left: Real,
    limit_right: Real,
    limit_top: Real,
    limit_bottom: Real,
}

impl Fence {
    pub fn new(dimensions: Vector2) -> Self {
        Self {
            dimensions,

            limit_left: 0.0,
            limit_right: dimensions.x(),

            limit_top: 0.0,
            limit_bottom: dimensions.y(),
        }
    }

    pub fn distribute_particles(&self, particle_density: Real) -> Vec<Vector2> {
        let area = self.dimensions.x() * self.dimensions.y();
        let particle_count = (area * particle_density) as i32;

        let mut positions: Vec<Vector2> = Vec::with_capacity(particle_count as usize);
        for _ in 0..particle_count {
            let x = rand::random::<Real>() * self.dimensions.x();
            let y = rand::random::<Real>() * self.dimensions.y();

            positions.push(Vector2::new(x, y));
        }

        positions
    }
}

impl SimulationEntity for Fence {
    fn update(&mut self, _delta: f64) {}
}

impl CollisionEntity for Fence {
    fn collide(&mut self, other: &mut dyn CollisionEntity) {
        if other.position().x() < self.limit_left {
            *other.position_mut().x_mut() = self.limit_left;
            *other.velocity_mut().x_mut() = -other.velocity().x();
        } else if other.position().x() > self.limit_right {
            *other.position_mut().x_mut() = self.limit_right;
            *other.velocity_mut().x_mut() = -other.velocity().x();
        }

        if other.position().y() < self.limit_top {
            *other.position_mut().y_mut() = self.limit_top;
            *other.velocity_mut().y_mut() = -other.velocity().y();
        } else if other.position().y() > self.limit_bottom {
            *other.position_mut().y_mut() = self.limit_bottom;
            *other.velocity_mut().y_mut() = -other.velocity().y();
        }
    }

    fn position(&self) -> Vector2 {
        Vector2::zero()
    }

    fn position_mut(&mut self) -> &mut Vector2 {
        panic!("Invariant violation. Accelerating reference frames unsupported.");
    }

    fn velocity(&self) -> Vector2 {
        Vector2::zero()
    }

    fn velocity_mut(&mut self) -> &mut Vector2 {
        panic!("Invariant violation. Accelerating reference frames unsupported.");
    }

    fn mass(&self) -> Real {
        Real::INFINITY
    }

    fn mass_mut(&mut self) -> &mut Real {
        panic!("Invariant violation. Mass of hard boundary cannot be changed.")
    }
}
