use godot::engine::{INode2D, Node2D, Texture2D};
use godot::prelude::*;
use molesim_core::entities::base::simulated::SimulationEntity;
use molesim_core::simulation::Simulation;

const DEFAULT_FLUID_MASS: f32 = 1.0;
const DEFAULT_PARTICLE_DENSITY: f32 = 100.0;
const DEFAULT_PARTICLE_SCALING: Vector2 = Vector2::new(0.1, 0.1);
const DEFAULT_PARTICLE_TEXTURE: &str = "res://resources/particle.svg";

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct SceneManager {
    base: Base<Node2D>,

    #[export]
    fluid_mass: f32,

    #[export]
    particle_density: f32,
    #[export]
    particle_scaling: Vector2,
    #[export]
    particle_texture: Gd<Texture2D>,

    draw_queue: Vec<Vector2>,

    simulation_backend: Option<Simulation>,
}

#[godot_api]
impl INode2D for SceneManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,

            fluid_mass: DEFAULT_FLUID_MASS,

            particle_density: DEFAULT_PARTICLE_DENSITY,
            particle_scaling: DEFAULT_PARTICLE_SCALING,
            particle_texture: load(DEFAULT_PARTICLE_TEXTURE),

            draw_queue: Vec::new(),

            simulation_backend: None,
        }
    }

    fn enter_tree(&mut self) {
        let viewport = self.base.as_gd().get_viewport_rect();

        self.simulation_backend = Some(Simulation::new(
            viewport.size.x as f64,
            viewport.size.y as f64,
            self.particle_density as f64,
            self.fluid_mass as f64,
        ));
    }

    fn physics_process(&mut self, delta: f64) {
        if let Some(simulation) = &mut self.simulation_backend {
            simulation.update(delta);

            self.draw_queue.clear();
            for cell in simulation.particles_iter_mut() {
                let particle = &mut *cell.borrow_mut();

                self.draw_queue.push(Vector2::new(
                    particle.position().x() as f32,
                    particle.position().y() as f32,
                ));
            }

            self.base_mut().queue_redraw();
        }
    }

    fn draw(&mut self) {
        for i in 0..self.draw_queue.len() {
            let position = self.draw_queue[i];

            godot_print!("Blit at {:?}", position);
            self.base_mut()
                .draw_circle(position, 1.0, Color::from_rgb(0.69, 0.25, 0.57));
        }
    }
}
