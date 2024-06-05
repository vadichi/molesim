use crate::math::{vector2::MVector2, Real};

use crate::entities::base::{
    collider::{BaseCollisionEntity, CollisionEntity},
    simulated::SimulationEntity,
};

pub struct MassiveParticle {
    acceleration: MVector2,

    collision_entity: Box<dyn CollisionEntity>,
}

impl MassiveParticle {
    pub fn new(mass: Real, position: MVector2, velocity: MVector2) -> Self {
        Self {
            acceleration: MVector2::zero(),
            collision_entity: Box::new(BaseCollisionEntity::new(position, velocity, mass)),
        }
    }
}

impl SimulationEntity for MassiveParticle {
    fn update(&mut self, delta: f64) {
        let dv = self.acceleration * delta;
        *self.velocity_mut() += dv;

        let dp = self.velocity() * delta;
        *self.position_mut() += dp;

        self.collision_entity.update(delta);
    }

    fn position(&self) -> MVector2 {
        self.collision_entity.position()
    }

    fn position_mut(&mut self) -> &mut MVector2 {
        self.collision_entity.position_mut()
    }
}

impl CollisionEntity for MassiveParticle {
    fn collide(&mut self, other: &mut dyn CollisionEntity) {
        let distance = self.position().distance(other.position());
        if distance > 1.0 {
            return;
        }

        let total_mass = self.mass() + other.mass();
        let distance_coefficient = 1.0 / distance.powi(2);

        let self_velocity = -(2.0 * other.mass() / total_mass)
            * distance_coefficient
            * ((self.velocity() - other.velocity()) * (self.position() - other.position()))
            * (self.position() - other.position());

        let other_velocity = -(2.0 * self.mass() / total_mass)
            * distance_coefficient
            * ((other.velocity() - self.velocity()) * (other.position() - self.position()))
            * (other.position() - self.position());

        *self.velocity_mut() = self_velocity;
        *other.velocity_mut() = other_velocity;

        self.collision_entity.collide(other);
    }

    fn velocity(&self) -> MVector2 {
        self.collision_entity.velocity()
    }

    fn velocity_mut(&mut self) -> &mut MVector2 {
        self.collision_entity.velocity_mut()
    }

    fn mass(&self) -> Real {
        self.collision_entity.mass()
    }

    fn mass_mut(&mut self) -> &mut Real {
        self.collision_entity.mass_mut()
    }
}
