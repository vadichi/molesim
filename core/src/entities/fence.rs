use crate::math::Real;
use crate::math::vector2::Vector2;
use crate::entities::Entity;
use crate::entities::Update;
use crate::entities::Untangle;
use crate::entities::Collide;

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

impl Untangle for Fence {
    fn untangle(&self, _other: &Entity) -> Vector2 {
        Vector2::zero()
    }
}

impl Collide for Fence {
    fn collide(&self, _other: &Entity) -> Vector2 {
        Vector2::zero()
    }
}

impl Fence {
    pub fn limit_left(&self) -> Real {
        0.0
    }

    pub fn limit_right(&self) -> Real {
        self.dimensions.x()
    }

    pub fn limit_top(&self) -> Real {
        0.0
    }

    pub fn limit_bottom(&self) -> Real {
        self.dimensions.y()
    }
}
