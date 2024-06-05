use std::hash::Hasher;

pub struct SpatialHasher {}

impl Hasher for SpatialHasher {
    fn finish(&self) -> u64 {
        0
    }

    fn write(&mut self, bytes: &[u8]) {
        // Do nothing
    }
}
