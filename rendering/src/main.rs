mod concurrency;
mod flo;
mod tunables;

use std::{sync::Arc, thread};

use flo_draw::*;

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

            // use a position iterator, please!
            let simulation = arc.get();
            for particle_ref in simulation.particle_iter() {
                // let particle = unsafe { *(*particle_ref).get() };
                // let particle = match particle {
                //     molesim_core::entities::Entity::Circle(circle) => circle,
                //     _ => panic!("Expected Circle entity"),
                // };

                // let x = particle.kinematics().position().x();
                // let y = particle.kinematics().position().y();

                flo::draw_particle(&canvas, particle_ref.x, particle_ref.y);
            }

            let elapsed = start_time.elapsed();
            println!("Frame time: {:?}", elapsed);
        }
    });
}
