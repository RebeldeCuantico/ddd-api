// src/domain/aggregate_root.rs
use crate::domain::entity_base::EntityBase;
use uuid::Uuid;

pub trait AggregateRoot: EntityBase {
   }

impl PartialEq for dyn AggregateRoot {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
