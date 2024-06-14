use crate::math::Real;
use crate::math::vector2::Vector2;

pub fn distribute_particles(fence_dimensions: Vector2, particle_density: Real) -> Vec<Vector2> {
    let area = fence_dimensions.x * fence_dimensions.y;
    let particle_count = (area * particle_density) as i32;

    let mut positions: Vec<Vector2> = Vec::with_capacity(particle_count as usize);
    for _ in 0..particle_count {
        let x = rand::random::<Real>() * fence_dimensions.x;
        let y = rand::random::<Real>() * fence_dimensions.y;

        positions.push(Vector2::new(x, y));
    }

    positions
}
