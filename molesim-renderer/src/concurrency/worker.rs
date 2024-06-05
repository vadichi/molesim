use std::sync::Arc;

use molesim_core::{entities::SimulationEntity, math::vector2::Vector2, simulation::Simulation};

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
        simulation.update(tunables::SIMULATION_DELTA_TIME);
    }
}
