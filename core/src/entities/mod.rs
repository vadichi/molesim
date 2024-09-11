use crate::math::Real;
use crate::math::vector2::Vector2;

pub mod circle;
pub mod fence;

#[derive(Clone, Debug)]
pub enum Entity {
    Fence(fence::Fence),
    Circle(circle::Circle),
}

pub trait Update {
    fn update(&mut self, delta_time: Real);
}

pub trait Collide {
    fn collide(&self, other: &Entity) -> CollisionCorrection;
    fn accept_correction(&mut self, correction: &CollisionCorrection);
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CollisionCorrection {
    position: Vector2,
    velocity: Vector2,
}

impl Entity {
    pub fn from_fence(fence: fence::Fence) -> Self {
        Entity::Fence(fence)
    }
    pub fn from_circle(circle: circle::Circle) -> Self {
        Entity::Circle(circle)
    }
}

impl Update for Entity {
    fn update(&mut self, delta_time: Real) {
        match self {
            Entity::Fence(fence) => fence.update(delta_time),
            Entity::Circle(circle) => circle.update(delta_time),
        }
    }
}

impl Collide for Entity {
    fn collide(&self, other: &Entity) -> CollisionCorrection {
        match self {
            Entity::Fence(fence) => fence.collide(other),
            Entity::Circle(circle) => circle.collide(other),
        }
    }

    fn accept_correction(&mut self, correction: &CollisionCorrection) {
        match self {
            Entity::Fence(fence) => fence.accept_correction(correction),
            Entity::Circle(circle) => circle.accept_correction(correction),
        }
    }
}

impl CollisionCorrection {
    pub fn new(position: Vector2, velocity: Vector2) -> Self {
        CollisionCorrection { position, velocity }
    }

    pub fn zero() -> Self {
        CollisionCorrection {
            position: Vector2::zero(),
            velocity: Vector2::zero(),
        }
    }
}

impl std::ops::Add for CollisionCorrection {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        CollisionCorrection {
            position: self.position + other.position,
            velocity: self.velocity + other.velocity,
        }
    }
}

impl std::ops::AddAssign for CollisionCorrection {
    fn add_assign(&mut self, other: Self) {
        self.position += other.position;
        self.velocity += other.velocity;
    }
}

