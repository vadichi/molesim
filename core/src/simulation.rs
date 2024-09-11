use crate::math::Real;
use crate::math::vector2::Vector2;

use crate::physics::fluid::Fluid;

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

        let blob_mass = fluid.get_blob_mass();
        let blobs: Vec<Entity> = fluid.get_blob_distribution()
            .iter()
            .map(|position| {
                Entity::Circle(
                    Circle::new(
                        *position,
                        Vector2::new(
                            (rand::random::<Real>() - 0.5) * 50.0,
                            (rand::random::<Real>() - 0.5) * 50.0,
                        ),
                        Vector2::new(0.0, -9.81),
                        blob_mass,
                        5.0,
                    )
                )
            })
            .collect();

        Self {
            entities: [vec![Entity::Fence(fence)], blobs].concat(),
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
        for entity in &mut self.entities {
            entity.update(delta);

            if let Entity::Circle(circle) = entity {
                if circle.kinematics().position().x.is_nan() || circle.kinematics().position().y.is_nan() {
                    panic!("Particle {:?} has NaN position", circle);
                }
            }
        }
    }

    pub fn collide_all(&mut self) {
        let mut corrections: Vec<CollisionCorrection> = vec![CollisionCorrection::zero(); self.entities.len()];

        for (index_a, entity_a) in self.entities.iter().enumerate() {
            for (index_b, entity_b) in self.entities.iter().enumerate() {
                if index_a != index_b {
                    let correction = entity_a.collide(entity_b);
                    corrections[index_a] += correction;
                }
            }
        }

        for (index, entity) in self.entities.iter_mut().enumerate() {
            entity.accept_correction(&corrections[index]);
        }
    }
}
