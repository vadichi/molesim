use crate::entities::SimulationEntity;
use crate::math::vector2::MVector2;

pub struct MassiveParticle {
    position: MVector2,
    velocity: MVector2,
    acceleration: MVector2,
}

impl MassiveParticle {
    pub fn new(position: MVector2, velocity: MVector2) -> Self {
        Self {
            position,
            velocity,
            acceleration: MVector2::zero(),
        }
    }

    pub fn override_acceleration(&mut self, acceleration: MVector2) {
        self.acceleration = acceleration;
    }

    pub fn override_position(&mut self, position: MVector2) {
        self.position = position;
    }

    pub fn position(&self) -> MVector2 {
        self.position
    }
    pub fn velocity(&self) -> MVector2 {
        self.velocity
    }
}

impl SimulationEntity for MassiveParticle {
    fn update(&mut self, delta: f64) {
        self.velocity += self.acceleration * delta;
        self.position += self.velocity * delta;
    }
}
