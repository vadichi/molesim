use crate::math::Real;
use crate::math::vector2::Vector2;

use crate::physics::fluid::{Blob, Fluid};

use crate::entities::Entity;
use crate::entities::Update;
use crate::entities::Collide;
use crate::entities::CollisionCorrection;
use crate::entities::fence::Fence;
use crate::entities::circle::Circle;

pub struct Simulation {
    pub entities: Vec<Entity>,
}

impl Simulation {
    pub fn new(fluid: Fluid, dimensions: Vector2) -> Self {
        let fence: Fence = Fence::new(dimensions);
        
        let molecules = fluid.get_blobs()
            .iter()
            .map(|blob| {
                Entity::Circle(
                    Circle::new(
                        blob.get_position(),
                        blob.get_velocity(),
                        Vector2::new(0.0, -9.81),
                        blob.get_mass(),
                        5.0,
                    )
                )
            })
            .collect();

        Self {
            entities: [vec![Entity::Fence(fence)], molecules].concat(),
        }
    }
}

impl Update for Simulation {
    fn update(&mut self, delta: Real) {
        self.collide_all();
        self.update_all(delta);
    }
}

impl Simulation {
    pub fn update_all(&mut self, delta: Real) {
        self.entities
            .iter_mut().
            for_each(|entity| {
                entity.update(delta)
            });
    }

    pub fn collide_all(&mut self) {
        let mut corrections: Vec<CollisionCorrection> = vec![
            CollisionCorrection::zero(); 
            self.entities.len()
        ];
        
        self.entities
            .iter()
            .enumerate()
            .for_each(|(index_a, entity_a)| {
               self.entities
                   .iter()
                   .enumerate()
                   .filter(|(index_b, _)| index_a != *index_b)
                   .for_each(|(_, entity_b)| {
                       corrections[index_a] += entity_a.collide(entity_b);
                   }); 
            });

        self.entities
            .iter_mut()
            .enumerate()
            .for_each(|(index, entity)| {
                entity.accept_correction(&corrections[index]);
            });
    }
}
