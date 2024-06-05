use std::cell::{RefCell, UnsafeCell};

use crate::{
    entities::{fence::Fence, particle::MassiveParticle, CollisionEntity, SimulationEntity},
    math::{vector2::Vector2, Real},
};

pub struct Simulation {
    fence: Fence,
    particles: Vec<UnsafeCell<MassiveParticle>>,
}

impl Simulation {
    pub fn new(dimensions: Vector2, particle_density: Real, mass: Real) -> Self {
        let fence = Fence::new(dimensions);

        let particle_positions = fence.distribute_particles(particle_density);
        let particle_count = particle_positions.len();

        let particles = particle_positions
            .into_iter()
            .map(|position| {
                UnsafeCell::new(MassiveParticle::new(
                    position,
                    Vector2::new(
                        (rand::random::<Real>() - 0.5) * 20.0,
                        (rand::random::<Real>() - 0.5) * 20.0,
                    ),
                    Vector2::new(0.0, -9.8),
                    mass / (particle_count as Real),
                ))
            })
            .collect();

        Self { fence, particles }
    }

    pub fn particles_count(&self) -> usize {
        self.particles.len()
    }

    pub fn particles_iter(&self) -> impl Iterator<Item = &UnsafeCell<MassiveParticle>> {
        self.particles.iter()
    }
}

impl SimulationEntity for Simulation {
    fn update(&mut self, delta: f64) {
        self.fence.update(delta);
        for cell in self.particles.iter_mut() {
            let particle = &mut *cell.get_mut();
            particle.update(delta);
        }

        for cell in self.particles.iter_mut() {
            let particle = &mut *cell.get_mut();
            self.fence.collide(particle);
        }

        unsafe {
            for i in 0..self.particles.len() {
                for j in (i + 1)..self.particles.len() {
                    let particle_i = &mut *self.particles[i].get();
                    let particle_j = &mut *self.particles[j].get();
                    particle_i.collide(particle_j);
                }
            }
        }
    }
}
