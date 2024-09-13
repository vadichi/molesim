use crate::math::Real;
use crate::math::vector2::Vector2;

pub struct Fluid {
    mass: Real,
    blob_size: Real,

    blobs: Vec<Blob>,
}

impl Fluid {
    pub fn get_mass(&self) -> Real {
        self.mass
    }

    pub fn get_blob_size(&self) -> Real {
        self.blob_size
    }

    pub fn get_blobs(&self) -> &Vec<Blob> {
        &self.blobs
    }
}

pub struct Blob {
    mass: Real,

    position: Vector2,
    velocity: Vector2,
}

impl Blob {
    pub fn get_mass(&self) -> Real {
        self.mass
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn get_velocity(&self) -> Vector2 {
        self.velocity
    }
}

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

        let blobs = (0..blob_count)
            .map(|_| {
                Blob {
                    mass: Self::calculate_blob_mass(mass, blob_count),
                    position: Self::generate_blob_position(dimensions),
                    velocity: Self::generate_blob_velocity(),
                }
            }).collect();

        Fluid {
            mass,
            blob_size,
            
            blobs,
        }
    }

    fn calculate_blob_count(blob_size: Real, world_dimensions: Vector2) -> usize {
        let area = world_dimensions.x * world_dimensions.y;
        (area / blob_size) as usize
    }

    fn calculate_blob_mass(mass: Real, blob_count: usize) -> Real {
        mass / blob_count as Real
    }

    fn generate_blob_position(world_dimensions: Vector2) -> Vector2 {
        Vector2::new(
            rand::random::<Real>() * world_dimensions.x,
            rand::random::<Real>() * world_dimensions.y
        )
    }

    fn generate_blob_velocity() -> Vector2 {
        Vector2::new(
            (rand::random::<Real>() - 0.5) * 50.0,
            (rand::random::<Real>() - 0.5) * 50.0
        )
    }
}
