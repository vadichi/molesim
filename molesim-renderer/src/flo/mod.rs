use flo_binding::*;
use flo_canvas::*;
use flo_draw::*;

use futures_core::Stream;

pub fn create_window() -> (Canvas, impl Clone + Send + Stream<Item = DrawEvent>) {
    let title = "MoleSim";
    let title_bind = bind(title.to_string());

    let size = (
        crate::tunables::SIMULATION_WIDTH as u64,
        crate::tunables::SIMULATION_HEIGHT as u64,
    );
    let size_bind = bind(size);

    let fullscreen = false;
    let fullscreen_bind = bind(fullscreen);

    let has_decorations = true;
    let has_decorations_bind = bind(has_decorations);

    let mouse_pointer = MousePointer::SystemDefault;
    let mouse_pointer_bind = bind(mouse_pointer);

    let window_properties = WindowProperties {
        title: BindRef::new(&title_bind),
        size: BindRef::new(&size_bind),
        fullscreen: BindRef::new(&fullscreen_bind),
        has_decorations: BindRef::new(&has_decorations_bind),
        mouse_pointer: BindRef::new(&mouse_pointer_bind),
    };

    create_canvas_window_with_events(window_properties)
}

pub fn initialize_canvas(canvas: &Canvas) {
    canvas.draw(|gc| {
        gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));

        gc.canvas_height(crate::tunables::SIMULATION_HEIGHT as f32);
        gc.center_region(
            0.0,
            0.0,
            crate::tunables::SIMULATION_WIDTH as f32,
            crate::tunables::SIMULATION_HEIGHT as f32,
        );

        gc.layer(LayerId(0));
    });
}

pub fn clear_canvas(canvas: &Canvas) {
    canvas.draw(|gc| {
        gc.clear_layer();
    });
}

pub fn draw_particle(canvas: &Canvas, x: f64, y: f64) {
    canvas.draw(|gc| {
        gc.new_path();
        gc.circle(x as f32, y as f32, 5.0);
        gc.fill_color(Color::Rgba(0.69, 0.25, 0.57, 1.0));
        gc.fill();
    });
}
