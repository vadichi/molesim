use godot::engine::{INode2D, Node2D, Texture2D};
use godot::prelude::*;

use crate::nodes::fence::Fence;
use crate::nodes::particle::Particle;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct SceneManager {
    base: Base<Node2D>,

    #[export]
    particle_density: f32,
    #[export]
    particle_scaling: Vector2,
    #[export]
    particle_texture: Gd<Texture2D>,
}

impl SceneManager {
    pub fn create_particle(&mut self, initial_position: Vector2) {
        // Un-managed allocation of a Godot node instance;
        // handed over to be managed by the engine as part of the scene tree.
        let mut sprite = Particle::new_alloc();

        sprite.set_position(initial_position);

        sprite.set_scale(self.particle_scaling);
        sprite.set_texture(self.particle_texture.clone());

        let coerced_sprite: Gd<Node> = sprite.upcast();
        self.base_mut().add_child(coerced_sprite);
    }

    pub fn create_particle_mesh(&mut self) {
        let viewport = self.base.as_gd().get_viewport_rect();
        let width = viewport.size.x;
        let height = viewport.size.y;

        godot_print!(
            "Generating particle mesh in viewport: {}x{}.",
            width,
            height
        );

        godot_print!(
            "Particle mesh: {}x{}={}.",
            x_count,
            y_count,
            x_count * y_count
        );

        for i in 0..x_count {
            for j in 0..y_count {
                let x = (i as f32 + 0.25) * self.particle_density;
                let y = (j as f32 + 0.5) * self.particle_density;

                self.create_particle(Vector2::new(x, y));
            }
        }
    }

    pub fn create_fence(&mut self) {
        let fence = Fence::new_alloc();

        let coerced_fence: Gd<Node> = fence.upcast();
        self.base_mut().add_child(coerced_fence);
    }
}

#[godot_api]
impl INode2D for SceneManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,

            particle_density: 100.0,
            particle_scaling: Vector2::new(0.1, 0.1),
            particle_texture: load("res://resources/particle.svg"),
        }
    }

    fn enter_tree(&mut self) {
        godot_print!("Scene manager initialized.");

        self.create_particle_mesh();
        self.create_fence();
    }

    fn physics_process(&mut self, _delta: f64) {
        let children = self.base_mut().get_children();
    }
}
