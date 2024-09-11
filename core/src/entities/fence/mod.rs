use crate::math::Real;
use crate::math::vector2::Vector2;
use crate::entities::{CollisionCorrection, Entity};
use crate::entities::Update;
use crate::entities::Collide;

#[derive(Clone, Debug)]
pub struct Fence {
    dimensions: Vector2,
}

impl Fence {
    pub fn new(dimensions: Vector2) -> Fence {
        Fence { dimensions }
    }
}

impl Update for Fence {
    fn update(&mut self, _delta_time: Real) {}
}

impl Collide for Fence {
    fn collide(&self, _other: &Entity) -> CollisionCorrection {
        CollisionCorrection::zero()
    }

    fn accept_correction(&mut self, correction: &CollisionCorrection) {
        if *correction != CollisionCorrection::zero() {
            panic!(
                "Invalid collision correction {:?} for Fence entity: must be zero.",
                correction
            );
        }
    }
}

impl Fence {
    pub fn limit_left(&self) -> Real {
        0.0
    }

    pub fn limit_right(&self) -> Real {
        self.dimensions.x
    }

    pub fn limit_bottom(&self) -> Real {
        0.0
    }

    pub fn limit_top(&self) -> Real {
        self.dimensions.y
    }
}
