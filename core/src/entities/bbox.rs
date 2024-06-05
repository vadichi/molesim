use crate::math::{vector2::MVector2, Real};

use super::base::{
    collider::{BaseCollisionEntity, CollisionEntity},
    simulated::SimulationEntity,
};

pub struct BoundingBox {
    width: Real,
    height: Real,

    collision_entity: Box<dyn CollisionEntity>,
}

impl BoundingBox {
    pub fn new(x: Real, y: Real, width: Real, height: Real) -> Self {
        Self {
            width,
            height,

            collision_entity: Box::new(BaseCollisionEntity::new(
                MVector2::new(x, y),
                MVector2::zero(),
                0.0,
            )),
        }
    }
}

impl SimulationEntity for BoundingBox {
    fn update(&mut self, delta: f64) {
        self.collision_entity.update(delta);
    }

    fn position(&self) -> MVector2 {
        self.collision_entity.position()
    }

    fn position_mut(&mut self) -> &mut MVector2 {
        self.collision_entity.position_mut()
    }
}

impl CollisionEntity for BoundingBox {
    fn collide(&mut self, other: &mut dyn CollisionEntity) {
        let left_x = self.position().x();
        let right_x = left_x + self.width;

        let top_y = self.position().y();
        let bottom_y = top_y + self.height;

        if other.position().x() < left_x {
            *other.position_mut().x_mut() = left_x;
            *other.velocity_mut().x_mut() = -other.velocity().x();
        } else if other.position().x() > right_x {
            *other.position_mut().x_mut() = right_x;
            *other.velocity_mut().x_mut() = -other.velocity().x();
        }

        if other.position().y() < top_y {
            *other.position_mut().y_mut() = top_y;
            *other.velocity_mut().y_mut() = -other.velocity().y();
        } else if other.position().y() > bottom_y {
            *other.position_mut().y_mut() = bottom_y;
            *other.velocity_mut().y_mut() = -other.velocity().y();
        }
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
