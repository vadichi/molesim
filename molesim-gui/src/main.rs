use flo_canvas::*;
use flo_draw::*;
use molesim_core::{entities::base::simulated::SimulationEntity, simulation::Simulation};

const DEFAULT_FLUID_MASS: f32 = 1.0;
const DEFAULT_PARTICLE_DENSITY: f32 = 50.0;

pub fn main() {
    with_2d_graphics(|| {
        let mut simulation = Simulation::new(
            1000.0,
            1000.0,
            DEFAULT_PARTICLE_DENSITY as f64,
            DEFAULT_FLUID_MASS as f64,
        );

        let canvas = create_drawing_window("MoleSim");

        canvas.draw(|gc| {
            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            gc.clear_layer();
        });

        let mut frame = 0;
        loop {
            simulation.update(0.1);

            if frame != 4 {
                frame += 1;
                continue;
            }

            canvas.draw(|gc| {
                gc.clear_layer();
            });

            for particle in simulation.particles_iter_mut() {
                let particle = &mut *particle.borrow_mut();

                let x = particle.position().x();
                let y = particle.position().y();

                println!("Particle at ({}, {})", x, y);

                canvas.draw(|gc| {
                    gc.new_path();
                    gc.circle(x as f32, y as f32, 5.0);
                    gc.fill_color(Color::Rgba(0.69, 0.25, 0.57, 1.0));
                    gc.fill();
                });
            }

            frame = 0;
            // std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
}
