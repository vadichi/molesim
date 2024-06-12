use std::cell::UnsafeCell;
use itertools::Itertools;
use crate::entities::{Collide, Entity, Untangle, Update};
use crate::math::Real;
use crate::math::vector2::Vector2;
use crate::entities::fence::Fence;
use crate::entities::circle::Circle;

pub mod particle_distribution;
pub mod iteration;

pub struct Simulation {
    fence: Entity,
    particles: Vec<UnsafeCell<Entity>>, // should this be an unsafe cell <vec> instead? less indirection, same result
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
    pub fn iter(&self) -> impl Iterator<Item = &Entity> + Clone {
        let particles: Vec<&Entity> = self
            .particles
            .iter()
            .map(|cell| unsafe { &*cell.get() })
            .collect();
        let fence: &Entity = &self.fence;
        particles.into_iter().chain(std::iter::once(fence))
    }

    pub fn pairs(&self) -> impl Iterator<Item = (&Entity, &Entity)> + Clone {
        self.iter()
            .cartesian_product(self.iter())
            // Drop pairs of same entity from iterator through pointer comparison
            .filter(|(a, b)| *a as *const _ != *b as *const _)
    }

    pub fn update_all(&mut self, delta: Real) {
        for cell in &self.particles {
            unsafe { &mut *cell.get() }.update(delta);
        }
    }

    pub fn untangle_all(&self) {
        let mut position_corrections: Vec<Vector2> = Vec::new();

        for (entity_a, entity_b) in self.pairs() {
            position_corrections.push(entity_a.untangle(entity_b));
        }

        for (index, cell) in self.particles.iter().enumerate() {
            if let Entity::Circle(circle) = unsafe { &mut *cell.get() } {
                if let Some(correction) = position_corrections.get(index) {
                    *circle.kinematics_mut().position_mut() += *correction;
                }
            }
        }
    }

    pub fn collide_all(&mut self) {
        let mut velocity_corrections: Vec<Vector2> = Vec::new();
        for (entity_a, entity_b) in self.pairs() {
            velocity_corrections.push(entity_a.collide(entity_b));
        }

        for (index, cell) in self.particles.iter().enumerate() {
            if let Entity::Circle(circle) = unsafe { &mut *cell.get() } {
                if let Some(correction) = velocity_corrections.get(index) {
                    *circle.kinematics_mut().velocity_mut() += *correction;
                }
            }
        }
    }
}
