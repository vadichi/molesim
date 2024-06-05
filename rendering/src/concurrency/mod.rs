use std::cell::UnsafeCell;

use molesim_core::simulation::Simulation;

pub mod worker;

/**
 * Multithreading model:
 *
 * The simulation data is the shared state.
 * It is mutated by and only be the simulation worker thread, which continuously updates the simulation
 * without regard to the rendering process.
 *
 * The rendering process is the main thread.
 * It will only read the simulation data to render the current state of the simulation.
 * Rendering is not in any way synchronized with the simulation worker thread.
 *
 * This may result in particles "time traveling" slightly if the screen is re-rendered
 * out of sync with the simulation update.
 *
 * This is acceptable, as the deviations are minor, being limited to a single DELTA_TIME
 * of time difference. This does not affect the precision of the simulation, as
 * it does not read back from the rendering process.
 *
 * To achieve this in the fastest way, unsafe code is used to share the simulation data
 * The code relies on the invariants described above; behavior is undefined if these are violated.
 */

pub struct SharedState(UnsafeCell<Simulation>);

unsafe impl Send for SharedState {}
unsafe impl Sync for SharedState {}

impl SharedState {
    pub fn new(simulation: Simulation) -> Self {
        Self(UnsafeCell::new(simulation))
    }

    pub fn get(&self) -> &Simulation {
        unsafe { &*self.0.get() }
    }

    pub fn get_mut(&self) -> &mut Simulation {
        unsafe { &mut *self.0.get() }
    }
}
