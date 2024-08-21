// src/domain/product.rs
use crate::domain::aggregate_root::AggregateRoot;
use crate::domain::entity_base::EntityBase;
use crate::domain::value_objects::identifier::Identifier;
use crate::crosscutting::guid_generator::{GuidGenerator, SequentialGuidGenerator};
use uuid::Uuid;

pub struct Product {
    id: Identifier,
    name: String,
    price: f64,
    version: u64,
}

impl Product {
    pub fn new(name: String, price: f64) -> Self {
        Self::with_generator(name, price, &mut SequentialGuidGenerator::new())
    }

    pub fn with_generator(name: String, price: f64, generator: &mut impl GuidGenerator) -> Self {
        Product {
            id: Identifier::with_generator(generator),
            name,
            price,
            version: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> f64 {
        self.price
    }
}

impl EntityBase for Product {
    fn id(&self) -> Uuid {
        self.id.value()
    }
}

impl AggregateRoot for Product {
    fn version(&self) -> u64 {
        self.version
    }

    fn increment_version(&mut self) {
        self.version += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_product_creation() {
        let product = Product::new("Test Product".to_string(), 100.0);

        assert_ne!(product.id(), Uuid::nil(), "Product ID should not be nil");
        assert_eq!(product.name(), "Test Product", "Product name should be 'Test Product'");
        assert_eq!(product.price(), 100.0, "Product price should be 100.0");
    }

    #[test]
    fn test_product_is_aggregate_root() {
        let product = Product::new("Another Product".to_string(), 200.0);

        let root: &dyn AggregateRoot = &product;
        assert_eq!(root.id(), product.id(), "AggregateRoot ID should match Product ID");
        assert_eq!(root.version(), 0, "Initial version should be 0");
    }

    #[test]
    fn test_product_identifier_is_unique() {
        let mut generator = SequentialGuidGenerator::new();
        let product1 = Product::with_generator("Product 1".to_string(), 150.0, &mut generator);
        let product2 = Product::with_generator("Product 2".to_string(), 150.0, &mut generator);
    
        assert_ne!(product1.id(), product2.id(), "Each product should have a unique ID");
    }

    #[test]
    fn test_product_version_increment() {
        let mut product = Product::new("Test Product".to_string(), 100.0);
        assert_eq!(product.version(), 0, "Initial version should be 0");
        
        product.increment_version();
        assert_eq!(product.version(), 1, "Version should be incremented to 1");
    }

    #[test]
    fn test_product_with_custom_generator() {
        struct MockGenerator;
        impl GuidGenerator for MockGenerator {
            fn generate(&mut self) -> Uuid {
                Uuid::nil()
            }
        }

        let product = Product::with_generator("Custom Product".to_string(), 150.0, &mut MockGenerator);
        assert_eq!(product.id(), Uuid::nil(), "Product should have nil UUID");
    }
}
