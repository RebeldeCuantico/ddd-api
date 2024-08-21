// src/domain/entity_base.rs
use uuid::Uuid;
use std::cmp::PartialEq;

pub trait EntityBase {
    fn id(&self) -> Uuid;
}

impl PartialEq for dyn EntityBase {
    fn eq(&self, other: &self) -> bool {
        self.id() == other.id();
    }
}