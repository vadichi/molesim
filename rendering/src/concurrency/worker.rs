use std::sync::Arc;

use molesim_core::{entities::Update, math::vector2::Vector2, simulation::Simulation};

use crate::tunables;

use super::SharedState;

pub fn create_shared_state() -> SharedState {
    let simulation = Simulation::new(
        Vector2::new(tunables::SIMULATION_WIDTH, tunables::SIMULATION_HEIGHT),
        tunables::SIMULATION_DENSITY,
        tunables::FLUID_MASS,
    );

    SharedState::new(simulation)
}

pub fn simulation_worker_main(state_arc: Arc<SharedState>) {
    loop {
        let simulation = state_arc.get_mut();

        let start_time = std::time::Instant::now();

        simulation.update(tunables::SIMULATION_DELTA_TIME);

        let render_time = start_time.elapsed();
        println!("Render time: {:?}", render_time);
    }
}
