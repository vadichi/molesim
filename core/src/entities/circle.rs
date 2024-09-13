use crate::math::Real;
use crate::math::vector2::Vector2;
use crate::physics::kinematics::KinematicsEntity;
use crate::entities::Entity;
use crate::entities::Update;
use crate::entities::Collide;
use crate::entities::CollisionCorrection;

#[derive(Clone, Debug)]
pub struct Circle {
    kinematics: KinematicsEntity,

    mass: Real,
    radius: Real,

    collision_correction: CollisionCorrection,
}

impl Circle {
    pub fn new(
        position: Vector2,
        velocity: Vector2,
        acceleration: Vector2,
        mass: Real,
        radius: Real,
    ) -> Circle {
        Circle {
            kinematics: KinematicsEntity::new(position)
                .with_velocity(velocity)
                .with_acceleration(acceleration),
            mass,
            radius,
            collision_correction: CollisionCorrection::default(),
        }
    }
}

impl Update for Circle {
    fn update(&mut self, delta_time: Real) {
        self.kinematics.update_with(
            delta_time, 
            self.collision_correction.position,
            self.collision_correction.velocity,
        );
    }
}

impl Collide for Circle {
    fn collide(&self, other: &Entity) -> CollisionCorrection {
        CollisionCorrection {
            position: self.position_correction(other),
            velocity: self.velocity_correction(other),
        }
    }

    fn accept_correction(&mut self, correction: &CollisionCorrection) {
        self.collision_correction = correction.clone();
    }
}

impl Circle {
    fn position_correction(&self, other: &Entity) -> Vector2 {
        match other {
            Entity::Circle(other) => {
                let distance = self.kinematics.position() - other.kinematics.position();

                let overlap = self.radius + other.radius - distance.magnitude();
                if overlap < 0.0 {
                    return Vector2::zero();
                }

                let unit = distance.normalized();
                0.5 * overlap * unit
            },

            Entity::Fence(fence) => {
                let position = self.kinematics.position();

                let mut correction = Vector2::zero();

                if position.x < fence.limit_left() {
                    correction.x += fence.limit_left() - position.x;
                } else if position.x > fence.limit_right() {
                    correction.x += fence.limit_right() - position.x;
                }

                if position.y < fence.limit_bottom() {
                    correction.y += fence.limit_bottom() - position.y;
                } else if position.y > fence.limit_top() {
                    correction.y += fence.limit_top() - position.y;
                }

                correction
            }
        }
    }

    fn velocity_correction(&self, other: &Entity) -> Vector2 {
        match other {
            Entity::Circle(other) => {
                let initial_velocity = self.kinematics.velocity();

                let distance = self.kinematics.position() - other.kinematics.position();
                if distance.magnitude() > self.radius + other.radius {
                    return Vector2::zero();
                }

                let total_mass = self.mass + other.mass;
                let final_velocity = (initial_velocity * (self.mass - other.mass)
                    + 2.0 * other.mass * other.kinematics.velocity())
                    / total_mass;
                
                final_velocity - initial_velocity
            },

            Entity::Fence(fence) => {
                let position = self.kinematics.position();

                let initial_velocity = self.kinematics.velocity();
                let mut final_velocity = self.kinematics.velocity();

                // Only invert the velocity if the circle is leaving the fence
                // This prevents the circle from getting stuck in an oscillation at the edge?
                if (position.x <= fence.limit_left() && initial_velocity.x < 0.0)
                    || (position.x >= fence.limit_right() && initial_velocity.x > 0.0)
                {
                    final_velocity.x = -initial_velocity.x;
                }

                if (position.y <= fence.limit_bottom() && initial_velocity.y < 0.0)
                    || (position.y >= fence.limit_top() && initial_velocity.y > 0.0)
                {
                    final_velocity.y = -initial_velocity.y;
                }

                final_velocity - initial_velocity
            },
        }
    }
}

impl Circle {
    pub fn total_energy(&self) -> Real {
        self.kinetic_energy() + self.potential_energy()
    }

    pub fn potential_energy(&self) -> Real {
        // ToDo hack
        self.mass * -self.kinematics.acceleration().y * self.kinematics.position().y
    }

    pub fn kinetic_energy(&self) -> Real {
        0.5 * self.mass * self.kinematics.velocity().magnitude().powi(2)
    }

    pub fn momentum(&self) -> Vector2 {
        self.kinematics.velocity() * self.mass
    }

    pub fn mass(&self) -> Real {
        self.mass
    }

    pub fn kinematics(&self) -> &KinematicsEntity {
        &self.kinematics
    }

    pub fn kinematics_mut(&mut self) -> &mut KinematicsEntity {
        &mut self.kinematics
    }
}

pub struct CircleDrawable {
    pub x: Real,
    pub y: Real,
    pub radius: Real,
}

impl Circle {
    pub fn drawable(&self) -> CircleDrawable {
        CircleDrawable {
            x: self.kinematics.position().x,
            y: self.kinematics.position().y,
            radius: self.radius,
        }
    }
}
