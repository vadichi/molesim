use crate::entities::SimulationEntity;
use crate::math::Real;
use crate::math::vector2::Vector2;

pub struct MassiveParticle {
    position: Vector2,
    velocity: Vector2,
}

impl MassiveParticle {
    pub fn new(position: Vector2, velocity: Vector2) -> Self {
        Self {
            position,
            velocity,
        }
    }

    pub fn position(&self) -> Vector2 {
        self.position
    }
    pub fn velocity(&self) -> Vector2 {
        self.velocity
    }
}

impl SimulationEntity for MassiveParticle {
    fn update(&mut self, delta: f64) {
        self.position += self.velocity * delta as Real;
    }
}
