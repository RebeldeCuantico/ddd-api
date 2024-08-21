// src/domain/value_object.rs
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};

pub trait ValueObject: PartialEq + Eq + Hash {}

impl<T: PartialEq + Eq + Hash> ValueObject for T {}
