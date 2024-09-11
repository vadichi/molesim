mod sfml;
mod tunables;

use std::thread;
use std::time::Duration;
use crate::sfml::SFMLRenderer;

use molesim_core::math::vector2::Vector2;
use molesim_core::physics::fluid::FluidBuilder;
use molesim_core::simulation::Simulation;
use molesim_core::entities::Update;

static mut SIMULATION: Option<Simulation> = None;

pub fn main() {
    let dimensions = Vector2::new(
        tunables::SIMULATION_WIDTH,
        tunables::SIMULATION_HEIGHT,
    );

    let fluid = FluidBuilder::default()
        .with_mass(tunables::FLUID_MASS)
        .with_blob_size(tunables::SIMULATION_DENSITY)
        .compile_for(dimensions);

    unsafe {
        SIMULATION = Some(
            Simulation::new(
                fluid,
                dimensions
            )
        );
    }

    launch_simulation_worker();

    let mut renderer = SFMLRenderer::new();
    renderer.enter_graphics_loop();
}

fn launch_simulation_worker() {
     thread::Builder::new()
        .name("MoleSim simulation worker".to_string())
        .spawn(move || {
            simulation_worker();
        })
        .expect("Failed to spawn simulation worker thread");
}

fn simulation_worker() {
    let delta_time = tunables::SIMULATION_DELTA_TIME;

    unsafe {
        loop {
            if let Some(simulation) = SIMULATION.as_mut() {
                simulation.update(delta_time);
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
}