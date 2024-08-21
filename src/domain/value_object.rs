// src/domain/value_object.rs
pub trait ValueObject: PartialEq + Eq + Clone + std::fmt::Debug + std::hash::Hash {
    fn equals(&self, other: &Self) -> bool;
}
