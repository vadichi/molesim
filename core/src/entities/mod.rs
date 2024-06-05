use crate::math::{vector2::Vector2, Real};

pub mod fence;
pub mod particle;

pub trait SimulationEntity {
    fn update(&mut self, delta: f64);
}

pub trait CollisionEntity: SimulationEntity {
    fn collide(&mut self, other: &mut dyn CollisionEntity);

    fn position(&self) -> Vector2;
    fn position_mut(&mut self) -> &mut Vector2;

    fn velocity(&self) -> Vector2;
    fn velocity_mut(&mut self) -> &mut Vector2;

    fn mass(&self) -> Real;
    fn mass_mut(&mut self) -> &mut Real;

    fn momentum(&self) -> Vector2 {
        self.velocity() * self.mass()
    }
}
