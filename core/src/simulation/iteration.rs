use std::ops::Index;
use crate::entities::Entity;
use crate::simulation::Simulation;

pub struct SimulationEntityIndex {
    index: usize,
    particle_count: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SimulationEntity {
    Fence,
    Particle(usize),
}

impl SimulationEntityIndex {
    pub fn new(simulation: &Simulation) -> Self {
        Self::Fence
    }

    pub fn next(self) -> Self {
        match self {
            SimulationEntityIndex::Fence => Self::Particle(0),
            SimulationEntityIndex::Particle(index) => Self::Particle(index + 1),
        }
    }
}

impl Index<SimulationEntityIndex> for Simulation {
    type Output = Entity;

    fn index(&self, index: SimulationEntityIndex) -> &Self::Output {
        match index {
            SimulationEntityIndex::Fence => &self.fence,
            SimulationEntityIndex::Particle(index) => {
                let cell = &self.particles[index];
                unsafe { &*cell.get() }
            }
        }
    }
}
