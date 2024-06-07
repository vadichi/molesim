use crate::math::Real;

pub mod kinematics;
pub mod collisions;

pub trait Updatable {
    fn update(&mut self, delta_time: Real);
}
