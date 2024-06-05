use crate::math::vector2::MVector2;

pub trait SimulationEntity {
    fn update(&mut self, delta: f64);

    fn position(&self) -> MVector2;
    fn position_mut(&mut self) -> &mut MVector2;
}

pub struct BaseSimulationEntity {
    position: MVector2,
}

impl BaseSimulationEntity {
    pub fn new(position: MVector2) -> Self {
        Self { position }
    }
}

impl SimulationEntity for BaseSimulationEntity {
    fn update(&mut self, _delta: f64) {}

    fn position(&self) -> MVector2 {
        self.position
    }

    fn position_mut(&mut self) -> &mut MVector2 {
        &mut self.position
    }
}
