pub mod particle;

pub trait SimulationEntity {
    fn update(&mut self, delta: f64);
}
