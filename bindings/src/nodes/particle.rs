use godot::engine::{ISprite2D, Sprite2D};
use godot::prelude::*;

use molesim_core::entities::particle::MassiveParticle;
use molesim_core::entities::SimulationEntity;
use molesim_core::math::vector2::MVector2;

use crate::util::Convert;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Particle {
    base: Base<Sprite2D>,

    is_initialized: bool,
    simulation_entity: MassiveParticle,
}

#[godot_api]
impl ISprite2D for Particle {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,

            is_initialized: false,
            simulation_entity: MassiveParticle::new(MVector2::zero(), MVector2::new(40.0, 10.0)),
        }
    }

    fn enter_tree(&mut self) {
        let initial_position = self.base.as_gd().get_position();
        self.simulation_entity
            .override_position(MVector2::convert(initial_position));

        self.is_initialized = true;
    }

    fn physics_process(&mut self, delta: f64) {
        if !self.is_initialized {
            return;
        }

        self.simulation_entity
            .override_acceleration(MVector2::new(0.0, 9.81));
        self.simulation_entity.update(delta);

        let position = self.simulation_entity.position();
        self.base_mut().set_position(Vector2::convert(position));
    }
}
