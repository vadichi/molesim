use crate::math::Real;

pub mod kinematics;
pub mod collisions;

pub trait Update {
    fn update(&mut self, delta_time: Real);
}

pub trait Draw {
    fn x(&self) -> Real;
    fn y(&self) -> Real;
}

pub trait Collide {
    fn collide(&mut self, other: &dyn Collide);
}
