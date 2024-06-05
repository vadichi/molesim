use crate::math::{vector2::MVector2, Real};

use super::simulated::{BaseSimulationEntity, SimulationEntity};

pub trait CollisionEntity: SimulationEntity {
    fn collide(&mut self, other: &mut dyn CollisionEntity);

    fn velocity(&self) -> MVector2;
    fn velocity_mut(&mut self) -> &mut MVector2;

    fn mass(&self) -> Real;
    fn mass_mut(&mut self) -> &mut Real;

    fn momentum(&self) -> MVector2 {
        self.velocity() * self.mass()
    }
}

pub struct BaseCollisionEntity {
    mass: Real,
    velocity: MVector2,

    simulation_entity: Box<dyn SimulationEntity>,
}

impl BaseCollisionEntity {
    pub fn new(position: MVector2, velocity: MVector2, mass: Real) -> Self {
        Self {
            mass,
            velocity,

            simulation_entity: Box::new(BaseSimulationEntity::new(position)),
        }
    }
}

impl SimulationEntity for BaseCollisionEntity {
    fn update(&mut self, delta: f64) {
        self.simulation_entity.update(delta);
    }

    fn position(&self) -> MVector2 {
        self.simulation_entity.position()
    }

    fn position_mut(&mut self) -> &mut MVector2 {
        self.simulation_entity.position_mut()
    }
}

impl CollisionEntity for BaseCollisionEntity {
    fn collide(&mut self, _other: &mut dyn CollisionEntity) {}

    fn velocity(&self) -> MVector2 {
        self.velocity
    }

    fn velocity_mut(&mut self) -> &mut MVector2 {
        &mut self.velocity
    }

    fn mass(&self) -> Real {
        self.mass
    }

    fn mass_mut(&mut self) -> &mut Real {
        &mut self.mass
    }
}
