// src/domain/aggregate_root.rs
use crate::domain::entity_base::EntityBase;

pub trait AggregateRoot: EntityBase {
   }

impl PartialEq for dyn AggregateRoot {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
