use godot::engine::{ISprite2D, Sprite2D};
use godot::prelude::*;

use molesim_core::math::vector2::Vector2 as MVector2;
use molesim_core::entities::particle::MassiveParticle;

use crate::util::Convert;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Particle {
    base: Base<Sprite2D>,

    simulation_entity: MassiveParticle,
}

#[godot_api]
impl ISprite2D for Particle {
    fn init(base: Base<Sprite2D>) -> Self {
        let initial_position = base.as_gd().get_position();
        
        Self {
            base,
            simulation_entity: MassiveParticle::new(
                MVector2::convert(initial_position),
                MVector2::new(40.0, 10.0),
            ),
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // self.simulation_entity.update(delta);
        // let position = self.simulation_entity.position();

        godot_print!("Position: {:?}", self.base().get_position());
        
        // self.base_mut()
        //     .set_position(Vector2::new(position.x() as f32, position.y() as f32));
    }
}
