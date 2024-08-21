// src/domain/value_objects/money.rs
use crate::domain::value_object::ValueObject;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Money {
    amount: i64,
    currency: String,
}

impl Money {
    pub fn new(amount: i64, currency: String) -> Self {
        Money { amount, currency }
    }
}

impl ValueObject for Money {
    fn equals(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency == other.currency
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_object_equality() {
        let money1 = Money::new(100, "USD".to_string());
        let money2 = Money::new(100, "USD".to_string());
        let money3 = Money::new(200, "USD".to_string());
        let money4 = Money::new(100, "EUR".to_string());

        assert!(money1.equals(&money2), "Equal Money objects should be equal");
        assert!(!money1.equals(&money3), "Money objects with different amounts should not be equal");
        assert!(!money1.equals(&money4), "Money objects with different currencies should not be equal");
    }

    #[test]
    fn test_value_object_partial_eq() {
        let money1 = Money::new(100, "USD".to_string());
        let money2 = Money::new(100, "USD".to_string());
        let money3 = Money::new(200, "USD".to_string());

        assert_eq!(money1, money2, "Equal Money objects should be equal using PartialEq");
        assert_ne!(money1, money3, "Different Money objects should not be equal using PartialEq");
    }

    #[test]
    fn test_value_object_clone() {
        let money1 = Money::new(100, "USD".to_string());
        let money2 = money1.clone();

        assert_eq!(money1, money2, "Cloned Money object should be equal to the original");
        assert!(money1.equals(&money2), "Cloned Money object should be equal using equals method");
    }

    #[test]
    fn test_value_object_debug() {
        let money = Money::new(100, "USD".to_string());
        let debug_string = format!("{:?}", money);
        assert!(debug_string.contains("100"), "Debug output should contain the amount");
        assert!(debug_string.contains("USD"), "Debug output should contain the currency");
    }

    #[test]
    fn test_value_object_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let money1 = Money::new(100, "USD".to_string());
        let money2 = Money::new(100, "USD".to_string());
        let money3 = Money::new(200, "USD".to_string());

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        let mut hasher3 = DefaultHasher::new();

        money1.hash(&mut hasher1);
        money2.hash(&mut hasher2);
        money3.hash(&mut hasher3);

        assert_eq!(hasher1.finish(), hasher2.finish(), "Equal Money objects should have the same hash");
        assert_ne!(hasher1.finish(), hasher3.finish(), "Different Money objects should have different hashes");
    }
}