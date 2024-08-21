// src/domain/product.rs
use crate::domain::aggregate_root::AggregateRoot;
use crate::domain::entity_base::EntityBase;
use crate::domain::value_objects::identifier::Identifier;
use crate::crosscutting::guid_generator::SequentialGuidGenerator;
use uuid::Uuid;

pub struct Product {
    id: Identifier,
    name: String,
    price: f64,
}

impl Product {
    pub fn new(name: String, price: f64, generator: &mut SequentialGuidGenerator) -> Self {
        Product {
            id: Identifier::new(generator), 
            name,
            price,
        }
    }
}

impl EntityBase for Product {
    fn id(&self) -> Uuid {
        self.id.value()
    }
}

impl AggregateRoot for Product {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crosscutting::guid_generator::SequentialGuidGenerator;
    use uuid::Uuid;

    #[test]
    fn test_product_creation() {
        let mut generator = SequentialGuidGenerator::new();
        let product = Product::new("Test Product".to_string(), 100.0, &mut generator);

        assert_ne!(product.id(), Uuid::nil(), "Product ID should not be nil");
       
        assert_eq!(product.name, "Test Product", "Product name should be 'Test Product'");
        assert_eq!(product.price, 100.0, "Product price should be 100.0");
    }

    #[test]
    fn test_product_is_aggregate_root() {
        let mut generator = SequentialGuidGenerator::new();
        let product = Product::new("Another Product".to_string(), 200.0, &mut generator);

        let root: &dyn AggregateRoot = &product;
        assert_eq!(root.id(), product.id(), "AggregateRoot ID should match Product ID");
    }

    #[test]
    fn test_product_identifier_is_unique() {
        let mut generator = SequentialGuidGenerator::new();
        let product1 = Product::new("Product 1".to_string(), 150.0, &mut generator);
        let product2 = Product::new("Product 2".to_string(), 150.0, &mut generator);

        assert_ne!(product1.id(), product2.id(), "Each product should have a unique ID");
    }
}
