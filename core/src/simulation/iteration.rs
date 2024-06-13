use crate::entities::Entity;

#[derive(Debug, Clone)]
pub struct EntityIterator {
    stage: IterationStage,
}

#[derive(Debug, Clone, PartialEq)]
enum IterationStage {
    Fence,
    Particle(usize),
}

impl EntityIterator {
    pub fn new() -> EntityIterator {
        EntityIterator {
            stage: IterationStage::Fence,
        }
    }
}

impl Iterator for EntityIterator {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
