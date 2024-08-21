// src/domain/value_objects/identifier.rs
use uuid::Uuid;
use crate::crosscutting::guid_generator::GuidGenerator;
use crate::crosscutting::guid_generator::SequentialGuidGenerator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    id: Uuid,
}

impl Identifier {
    pub fn new() -> Self {
        Self::with_generator(&mut SequentialGuidGenerator::new())
    }

    pub fn with_generator(generator: &mut impl GuidGenerator) -> Self {
        Identifier {
            id: generator.generate(),
        }
    }

    pub fn value(&self) -> Uuid {
        self.id
    }
}
