use crate::entities::{CollisionEntity, SimulationEntity};
use crate::math::{vector2::Vector2, Real};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassiveParticle {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,

    mass: Real,
}

impl MassiveParticle {
    pub fn new(position: Vector2, velocity: Vector2, acceleration: Vector2, mass: Real) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            mass,
        }
    }
}

impl SimulationEntity for MassiveParticle {
    fn update(&mut self, delta: f64) {
        let delta_velocity = self.acceleration * delta;
        *self.velocity_mut() += delta_velocity;

        let delta_position = self.velocity() * delta;
        *self.position_mut() += delta_position;
    }
}

impl CollisionEntity for MassiveParticle {
    fn collide(&mut self, other: &mut dyn CollisionEntity) -> Vector2 {
        match std::any::TypeId::of()


        let distance = self.position().distance(other.position());
        if distance > 1.0 {
            return Vector2::zero();
        }

        let total_mass = self.mass() + other.mass();

        let new_velocity = (self.velocity() * (self.mass() - other.mass())
            + 2.0 * other.mass() * other.velocity())
            / total_mass;

        new_velocity - self.velocity()
    }

    fn position(&self) -> Vector2 {
        self.position
    }

    fn position_mut(&mut self) -> &mut Vector2 {
        &mut self.position
    }

    fn velocity(&self) -> Vector2 {
        self.velocity
    }

    fn velocity_mut(&mut self) -> &mut Vector2 {
        &mut self.velocity
    }

    fn mass(&self) -> Real {
        self.mass
    }

    fn mass_mut(&mut self) -> &mut Real {
        &mut self.mass
    }
}
