use crate::math::{vector2::Vector2, Real};

pub mod circle;
pub mod fence;

pub trait Update {
    fn update(&mut self, delta_time: Real);
}

pub trait Untangle {
    fn untangle(&self, other: &Entity) -> Vector2;
}

pub trait Collide {
    fn collide(&self, other: &Entity) -> Vector2;
}

pub enum Entity {
    Fence(fence::Fence),
    Circle(circle::Circle),
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
}

impl Collide for Entity {
    fn collide(&self, other: &Entity) -> Vector2 {
        match self {
            Entity::Fence(fence) => fence.collide(other),
            Entity::Circle(circle) => circle.collide(other),
        }
    }
}
