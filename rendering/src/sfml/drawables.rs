use molesim_core::entities::Entity;
use sfml::graphics::{CircleShape, Color, RenderTarget, Shape, Transformable};
use sfml::system::Vector2f;

pub trait Draw {
    fn draw(&self, target: &mut impl RenderTarget);
}

impl Draw for Entity {
    fn draw(&self, target: &mut impl RenderTarget) {
        match self {
            Entity::Circle(circle) => {
                let mut shape = CircleShape::new(5.0, 30);
                shape.set_fill_color(Color::BLACK);
                shape.set_origin(Vector2f::new(5.0, 5.0));
                shape.set_position(
                    Vector2f::new(
                        circle.kinematics().position().x as f32,
                        (crate::tunables::SIMULATION_HEIGHT - circle.kinematics().position().y) as f32,
                    )
                );
                
                target.draw(&shape);
            },

            Entity::Fence(_) => {
                // A fence is not drawn
            },
        }
    }
}
