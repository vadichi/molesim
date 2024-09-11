use crate::math::Real;
use crate::math::vector2::Vector2;

#[derive(Default)]
pub struct FluidBuilder {
    mass: Option<Real>,
    blob_size: Option<Real>,
}

impl FluidBuilder {
    pub fn with_mass(mut self, mass: Real) -> Self {
        self.mass = Some(mass);
        self
    }

    pub fn with_blob_size(mut self, blob_size: Real) -> Self {
        self.blob_size = Some(blob_size);
        self
    }

    pub fn compile_for(&self, dimensions: Vector2) -> Fluid {
        let mass = self.mass.expect("Mass must be set");
        let blob_size = self.blob_size.expect("Blob size must be set");

        let blob_count = Self::calculate_blob_count(blob_size, dimensions);

        let blob_mass = Self::calculate_blob_mass(mass, blob_count);
        let blob_distribution = Self::generate_blob_distribution(blob_count, dimensions);

        Fluid {
            mass,
            blob_size,
            blob_mass,
            blob_distribution,
        }
    }

    fn calculate_blob_count(blob_size: Real, world_dimensions: Vector2) -> usize {
        let area = world_dimensions.x * world_dimensions.y;
        (area / blob_size) as usize
    }

    fn calculate_blob_mass(mass: Real, blob_count: usize) -> Real {
        mass / blob_count as Real
    }

    fn generate_blob_distribution(blob_count: usize, world_dimensions: Vector2) -> Vec<Vector2> {
        let mut blobs: Vec<Vector2> = Vec::with_capacity(blob_count);
        for _ in 0..blob_count {
            blobs.push(
                Vector2::new(
                    rand::random::<Real>() * world_dimensions.x,
                    rand::random::<Real>() * world_dimensions.y
                )
            );
        }

        blobs
    }
}

pub struct Fluid {
    mass: Real,
    blob_size: Real,

    blob_mass: Real,
    blob_distribution: Vec<Vector2>,
}

impl Fluid {
    pub fn get_blob_mass(&self) -> Real {
        self.blob_mass
    }

    pub fn get_blob_count(&self) -> usize {
        self.blob_distribution.len()
    }

    pub fn get_blob_distribution(&self) -> &Vec<Vector2> {
        &self.blob_distribution
    }
}
