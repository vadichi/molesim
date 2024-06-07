use crate::math::Real;
use crate::math::vector2::Vector2;
use crate::entities::kinematics::SimulatedEntity;

pub mod fence;
pub mod circle;

pub trait Rigid {
    fn untangling_delta(&self, other: &Self) -> Vector2;
    fn collision_delta(&self, other: &Self) -> Vector2;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidBody {
    mass: Real,

    collider: Collider,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Collider {
    Circle { radius: Real },
    Fence { width: Real, height: Real },
}

impl RigidBody {
    pub fn new(mass: Real, collider: Collider) -> Self {
        Self { mass, collider }
    }
}

impl Rigid for SimulatedEntity {
    fn untangling_delta(&self, other: &Self) -> Vector2 {
        match self {
            Collider::Circle => match other {
                // Move circles along the line through their centers until they no longer overlap
                Collider::Circle => {
                    // ToDo
                    Vector2::zero()
                }

                Collider::Fence => {
                    let limit_left = other.position().x;
                    let limit_right = other.position().x + other.width();

                    let limit_top = other.position().y;
                    let limit_bottom = other.position().y + other.height();
                }
            },

            // A fence doesn't move, so it doesn't need to untangle
            Collider::Fence => Vector2::zero(),
        }
    }

    fn collision_delta(&self, other: &Self) -> Vector2 {
        match self {
            Collider::Circle => match other {
                Collider::Circle => Vector2::zero(),
                Collider::Fence => Vector2::zero(),
            },

            // A fence doesn't move, so collisions have no effect
            Collider::Fence => Vector2::zero(),
        }
    }
}
