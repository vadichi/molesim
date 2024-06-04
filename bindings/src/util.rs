use godot::prelude::Vector2;
use molesim_core::math::vector2::MVector2;

pub trait Convert<T> {
    fn convert(value: T) -> Self;
}

impl Convert<MVector2> for Vector2 {
    fn convert(value: MVector2) -> Self {
        Self::new(value.x() as f32, value.y() as f32)
    }
}

impl Convert<Vector2> for MVector2 {
    fn convert(value: Vector2) -> Self {
        Self::new(value.x as f64, value.y as f64)
    }
}
