// src/domain/value_objects/identifier.rs
use uuid::Uuid;
use crate::crosscutting::guid_generator::SequentialGuidGenerator;
use crate::domain::value_object::ValueObject;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    id: Uuid,
}

impl Identifier {    
    pub fn new(generator: &mut SequentialGuidGenerator) -> Self {
        Identifier {
            id: generator.generate(),
        }
    }
    
    pub fn value(&self) -> Uuid {
        self.id
    }
}

impl ValueObject for Identifier {}
