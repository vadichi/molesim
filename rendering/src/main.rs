mod concurrency;
mod flo;
mod tunables;

use std::{sync::Arc, thread};

use flo_draw::*;

use molesim_core::entities::CollisionEntity;

pub fn main() {
    with_2d_graphics(|| {
        let state = concurrency::worker::create_shared_state();
        let arc = Arc::new(state);
        let worker_arc = arc.clone();

        let (canvas, _events) = flo::create_window();
        flo::initialize_canvas(&canvas);

        thread::spawn(move || {
            concurrency::worker::simulation_worker_main(worker_arc);
        });

        loop {
            let start_time = std::time::Instant::now();

            flo::clear_canvas(&canvas);

            let simulation = arc.get();
            for particle_ref in simulation.particles_iter() {
                let particle = unsafe { *(*particle_ref).get() };

                let x = particle.position().x();
                let y = particle.position().y();

                flo::draw_particle(&canvas, x, y);
            }

            let elapsed = start_time.elapsed();
            println!("Frame time: {:?}", elapsed);
        }
    });
}