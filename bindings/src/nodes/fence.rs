use godot::engine::{INode2D, Node2D};
use godot::prelude::*;

use molesim_core::entities::bbox::BoundingBox;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Fence {
    base: Base<Node2D>,

    simulation_entity: BoundingBox,
}

#[godot_api]
impl INode2D for Fence {
    fn init(base: Base<Node2D>) -> Self {
        let viewport = base.as_gd().get_viewport_rect();
        let width = viewport.size.x;
        let height = viewport.size.y;

        godot_print!("Viewport-bound fence initialized: {}x{}.", width, height);

        Self {
            base,

            simulation_entity: BoundingBox::new(0.0, 0.0, width as f64, height as f64),
        }
    }
}
