use crate::math::Real;
use crate::math::vector2::Vector2;
use crate::entities::Entity;
use crate::entities::Update;
use crate::entities::Untangle;
use crate::entities::Collide;
use crate::physics::kinematics::KinematicsEntity;

#[derive(Debug, Clone)]
pub struct Circle {
    kinematics: KinematicsEntity,

    mass: Real,
    radius: Real,
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
        }
    }
}

impl Update for Circle {
    fn update(&mut self, delta_time: Real) {
        self.kinematics.update(delta_time);
    }
}

impl Untangle for Circle {
    fn untangle(&self, other: &Entity) -> Vector2 {
        match other {
            Entity::Circle(circle) => {
                let distance = self.kinematics.position() - circle.kinematics.position();

                let overlap = self.radius + circle.radius - distance.magnitude();
                if overlap < 0.0 {
                    return Vector2::zero();
                }

                let unit = distance.normalized();
                0.5 * overlap * unit
            }

            Entity::Fence(fence) => {
                let position = self.kinematics.position();

                let mut correction = Vector2::zero();

                if position.x() < fence.limit_left() {
                    *correction.x_mut() += fence.limit_left() - position.x();
                } else if position.x() > fence.limit_right() {
                    *correction.x_mut() += fence.limit_right() - position.x();
                }

                if position.y() < fence.limit_bottom() {
                    *correction.y_mut() += fence.limit_bottom() - position.y();
                } else if position.y() > fence.limit_top() {
                    *correction.y_mut() += fence.limit_top() - position.y();
                }

                correction
            }
        }
    }
}

impl Collide for Circle {
    fn collide(&self, other: &Entity) -> Vector2 {
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
            }

            Entity::Fence(fence) => {
                let position = self.kinematics.position();

                let initial_velocity = self.kinematics.velocity();
                let mut final_velocity = self.kinematics.velocity();

                // Only invert the velocity if the circle is leaving the fence
                // This prevents the circle from getting stuck in an oscillation at the edge
                if (position.x() < fence.limit_left() && initial_velocity.x() < 0.0)
                    || (position.x() > fence.limit_right() && initial_velocity.x() > 0.0)
                {
                    *final_velocity.x_mut() = -initial_velocity.x();
                }

                if (position.y() < fence.limit_bottom() && initial_velocity.y() < 0.0)
                    || (position.y() > fence.limit_top() && initial_velocity.y() > 0.0)
                {
                    *final_velocity.y_mut() = -initial_velocity.y();
                }

                final_velocity - initial_velocity
            }
        }
    }
}

impl Circle {
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
            x: self.kinematics.position().x(),
            y: self.kinematics.position().y(),
            radius: self.radius,
        }
    }
}
