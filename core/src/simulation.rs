use std::cell::RefCell;

use crate::{
    entities::{
        base::{collider::CollisionEntity, simulated::SimulationEntity},
        bbox::BoundingBox,
        particle::MassiveParticle,
    },
    math::{vector2::MVector2, Real},
};

pub struct Simulation {
    bbox: BoundingBox,
    particles: Vec<RefCell<MassiveParticle>>,
}

impl Simulation {
    pub fn new(width: Real, height: Real, particle_density: Real, mass: Real) -> Self {
        let bbox = BoundingBox::new(0.0, 0.0, width, height);

        let x_count = (width / particle_density).round() as i32;
        let y_count = (height / particle_density).round() as i32;
        let particle_count = x_count * y_count;

        let mut particles = Vec::with_capacity(particle_count as usize);
        for i in 0..x_count {
            for j in 0..y_count {
                let x = (i as Real + 0.25) * particle_density;
                let y = (j as Real + 0.25) * particle_density;

                particles.push(RefCell::new(MassiveParticle::new(
                    mass / (particle_count as Real),
                    MVector2::new(x, y),
                    MVector2::new(
                        (0.5 - rand::random::<Real>()) * 10.0,
                        (0.5 - rand::random::<Real>()) * 10.0,
                    ),
                )));
            }
        }

        Self { bbox, particles }
    }

    pub fn particles_count(&self) -> usize {
        self.particles.len()
    }

    pub fn particles_iter_mut(&mut self) -> impl Iterator<Item = &mut RefCell<MassiveParticle>> {
        self.particles.iter_mut()
    }
}

impl SimulationEntity for Simulation {
    fn update(&mut self, delta: f64) {
        self.bbox.update(delta);
        for cell in self.particles.iter() {
            let particle = &mut *cell.borrow_mut();
            particle.update(delta);
        }

        for cell in self.particles.iter() {
            let particle = &mut *cell.borrow_mut();
            self.bbox.collide(particle);
        }

        for i in 0..self.particles.len() {
            for j in (i + 1)..self.particles.len() {
                let particle_i = &mut *self.particles[i].borrow_mut();
                let particle_j = &mut *self.particles[j].borrow_mut();
                particle_i.collide(particle_j);
            }
        }
    }

    fn position(&self) -> MVector2 {
        self.bbox.position()
    }

    fn position_mut(&mut self) -> &mut MVector2 {
        self.bbox.position_mut()
    }
}
