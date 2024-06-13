use crate::entities::Update;
use crate::math::Real;
use crate::math::vector2::Vector2;

#[derive(Clone, Debug)]
pub struct KinematicsEntity {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
}

impl KinematicsEntity {
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            velocity: Vector2::zero(),
            acceleration: Vector2::zero(),
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
}

impl Update for KinematicsEntity {
    fn update(&mut self, delta_time: Real) {
        self.position += self.velocity * delta_time;
        self.velocity += self.acceleration * delta_time;
    }
}

impl KinematicsEntity {
    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn position_mut(&mut self) -> &mut Vector2 {
        &mut self.position
    }

    pub fn velocity(&self) -> Vector2 {
        self.velocity
    }

    pub fn velocity_mut(&mut self) -> &mut Vector2 {
        &mut self.velocity
    }

    pub fn acceleration(&self) -> Vector2 {
        self.acceleration
    }

    pub fn acceleration_mut(&mut self) -> &mut Vector2 {
        &mut self.acceleration
    }
}
