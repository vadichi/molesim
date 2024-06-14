use std::cell::UnsafeCell;
use iteration::SimulationEntityIndex;

use crate::entities::{Collide, Entity, Untangle, Update};
use crate::math::Real;
use crate::math::vector2::Vector2;
use crate::entities::fence::Fence;
use crate::entities::circle::{Circle, CircleDrawable};

pub mod iteration;
pub mod particle_distribution;

pub struct Simulation {
    fence: Entity,
    particles: Vec<UnsafeCell<Entity>>, // should this be an unsafe cell <vec> instead? less indirection, same result?
}

impl Simulation {
    pub fn new(dimensions: Vector2, particle_density: Real, mass: Real) -> Self {
        let fence: Fence = Fence::new(dimensions);

        let particle_positions =
            particle_distribution::distribute_particles(dimensions, particle_density);

        let particle_count = particle_positions.len();
        let particles = particle_positions
            .into_iter()
            .map(|position| {
                UnsafeCell::new(Entity::Circle(Circle::new(
                    position,
                    Vector2::new(
                        (rand::random::<Real>() - 0.5) * 20.0,
                        (rand::random::<Real>() - 0.5) * 20.0,
                    ),
                    Vector2::new(0.0, -9.8),
                    mass / (particle_count as Real),
                    5.0,
                )))
            })
            .collect();

        Self {
            fence: Entity::Fence(fence),
            particles,
        }
    }
}

impl Update for Simulation {
    fn update(&mut self, delta: Real) {
        self.untangle_all();
        self.collide_all();
        self.update_all(delta);
    }
}

impl Simulation {
    pub fn update_all(&mut self, delta: Real) {
        let mut index = SimulationEntityIndex::new_complete();
        while self.exists(index) {
            self[index].update(delta);
            index = index.next();
        }
    }

    pub fn untangle_all(&mut self) {
        let mut position_corrections: Vec<Vector2> = vec![Vector2::zero(); self.particles.len()];

        let mut index_a = SimulationEntityIndex::new_complete();
        while self.exists(index_a) {
            let mut index_b = SimulationEntityIndex::new_complete();

            while self.exists(index_b) {
                if index_a != index_b {
                    let correction = self[index_a].untangle(&self[index_b]);

                    if let SimulationEntityIndex::Particle(index) = index_a {
                        position_corrections[index] += correction;
                    }
                }

                index_b = index_b.next();
            }

            index_a = index_a.next();
        }

        index_a = SimulationEntityIndex::new_complete();
        while self.exists(index_a) {
            if let SimulationEntityIndex::Particle(index) = index_a {
                if let Entity::Circle(circle) = &mut self[index_a] {
                    circle.accept_untangle_correction(position_corrections[index]);
                }
            }

            index_a = index_a.next();
        }
    }

    pub fn collide_all(&mut self) {
        let mut velocity_corrections: Vec<Vector2> = vec![Vector2::zero(); self.particles.len()];

        let mut index_a = SimulationEntityIndex::new_complete();
        while self.exists(index_a) {
            let mut index_b = SimulationEntityIndex::new_complete();

            while self.exists(index_b) {
                if index_a != index_b {
                    let correction = self[index_a].collide(&self[index_b]);

                    if let SimulationEntityIndex::Particle(index) = index_a {
                        velocity_corrections[index] += correction;
                    }
                }

                index_b = index_b.next();
            }

            index_a = index_a.next();
        }

        index_a = SimulationEntityIndex::new_complete();
        while self.exists(index_a) {
            if let SimulationEntityIndex::Particle(index) = index_a {
                if let Entity::Circle(circle) = &mut self[index_a] {
                    circle.accept_collision_correction(velocity_corrections[index]);
                }
            }

            index_a = index_a.next();
        }
    }
}
