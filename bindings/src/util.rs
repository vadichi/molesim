use godot::prelude::Vector2 as GodotVector2;
use molesim_core::math::vector2::Vector2 as MVector2;

pub trait Convert<T> {
    fn convert(value: T) -> Self;
}

impl Convert<MVector2> for GodotVector2 {
    fn convert(value: MVector2) -> GodotVector2 {
        GodotVector2::new(value.x() as f32, value.y() as f32)
    }
}

impl Convert<GodotVector2> for MVector2 {
    fn convert(value: GodotVector2) -> MVector2 {
        MVector2::new(value.x as f64, value.y as f64)
    }
}
