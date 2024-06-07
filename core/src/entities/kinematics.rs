use crate::math::Real;
use crate::math::vector2::Vector2;
use crate::entities::Updatable;
use crate::entities::collisions::RigidBody;

pub trait Simulated {
    fn position(&self) -> Vector2;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimulatedEntity {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,

    rigid_body: Option<RigidBody>,
}

impl SimulatedEntity {
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            velocity: Vector2::zero(),
            acceleration: Vector2::zero(),

            rigid_body: None,
        }
    }

    pub fn with_velocity(mut self, velocity: Vector2) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_acceleration(mut self, acceleration: Vector2) -> Self {
        self.acceleration = acceleration;
        self
    }

    pub fn with_rigid_body(mut self, rigid_body: RigidBody) -> Self {
        self.rigid_body = Some(rigid_body);
        self
    }
}

impl Updatable for SimulatedEntity {
    fn update(&mut self, delta_time: Real) {
        self.position += self.velocity * delta_time;
        self.velocity += self.acceleration * delta_time;
    }
}

impl Simulated for SimulatedEntity {
    fn position(&self) -> Vector2 {
        self.position
    }
}
