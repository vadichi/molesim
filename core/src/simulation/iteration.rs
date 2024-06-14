use std::ops::{Index, IndexMut};
use crate::entities::Entity;
use crate::simulation::Simulation;

// iterators; iterators w/ ::enumerate() to compare? pre-drop...? still need num to push to vector

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SimulationEntityIndex {
    Fence,
    Particle(usize),
}

impl SimulationEntityIndex {
    pub fn new_complete() -> Self {
        Self::Fence
    }

    pub fn new_particles() -> Self {
        Self::Particle(0)
    }
}

impl SimulationEntityIndex {
    pub fn next(mut self) -> Self {
        match self {
            SimulationEntityIndex::Fence => SimulationEntityIndex::Particle(0),
            SimulationEntityIndex::Particle(index) => {
                SimulationEntityIndex::Particle(index + 1)
            }
        }
    }
}

impl Simulation {
    pub fn exists(&self, index: SimulationEntityIndex) -> bool {
        match index {
            SimulationEntityIndex::Fence => true,
            SimulationEntityIndex::Particle(index) => index < self.particles.len(),
        }
    }
}

impl Index<SimulationEntityIndex> for Simulation {
    type Output = Entity;

    fn index(&self, index: SimulationEntityIndex) -> &Self::Output {
        match index {
            SimulationEntityIndex::Fence => &self.fence,
            SimulationEntityIndex::Particle(index) => unsafe { &*self.particles[index].get() },
        }
    }
}

impl IndexMut<SimulationEntityIndex> for Simulation {
    fn index_mut(&mut self, index: SimulationEntityIndex) -> &mut Self::Output {
        match index {
            SimulationEntityIndex::Fence => &mut self.fence,
            SimulationEntityIndex::Particle(index) => unsafe { &mut *self.particles[index].get() },
        }
    }
}
