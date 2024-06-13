use crate::math::{vector2::Vector2, Real};

pub mod circle;
pub mod fence;

pub trait Update {
    fn update(&mut self, delta_time: Real);
}

pub trait Untangle {
    fn untangle(&self, other: &Entity) -> Vector2;
    fn accept_untangle_correction(&mut self, correction: Vector2);
}

pub trait Collide {
    fn collide(&self, other: &Entity) -> Vector2;
    fn accept_collision_correction(&mut self, correction: Vector2);
}

#[derive(Clone, Debug)]
pub enum Entity {
    Fence(fence::Fence),
    Circle(circle::Circle),
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

impl Untangle for Entity {
    fn untangle(&self, other: &Entity) -> Vector2 {
        match self {
            Entity::Fence(fence) => fence.untangle(other),
            Entity::Circle(circle) => circle.untangle(other),
        }
    }

    fn accept_untangle_correction(&mut self, correction: Vector2) {
        match self {
            Entity::Fence(fence) => fence.accept_untangle_correction(correction),
            Entity::Circle(circle) => circle.accept_untangle_correction(correction),
        }
    }
}

impl Collide for Entity {
    fn collide(&self, other: &Entity) -> Vector2 {
        match self {
            Entity::Fence(fence) => fence.collide(other),
            Entity::Circle(circle) => circle.collide(other),
        }
    }

    fn accept_collision_correction(&mut self, correction: Vector2) {
        match self {
            Entity::Fence(fence) => fence.accept_collision_correction(correction),
            Entity::Circle(circle) => circle.accept_collision_correction(correction),
        }
    }
}
