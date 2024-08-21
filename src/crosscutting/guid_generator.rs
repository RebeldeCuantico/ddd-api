// src/crosscutting/guid_generator.rs
use uuid::Uuid;
use chrono::Utc;

pub trait GuidGenerator {
    fn generate(&mut self) -> Uuid;
}

pub struct SequentialGuidGenerator {
    last_timestamp: i64,
    counter: u16,
}

impl SequentialGuidGenerator {
    pub fn new() -> Self {
        SequentialGuidGenerator {
            last_timestamp: 0,
            counter: 0,
        }
    }
}

impl GuidGenerator for SequentialGuidGenerator {
    fn generate(&mut self) -> Uuid {
        let timestamp = Utc::now().timestamp_millis();
        
        if timestamp > self.last_timestamp {
            self.last_timestamp = timestamp;
            self.counter = 0;
        } else {
            self.counter += 1;
        }

        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&timestamp.to_be_bytes());
        bytes[8..10].copy_from_slice(&self.counter.to_be_bytes());
        bytes[10..16].copy_from_slice(&Uuid::new_v4().as_bytes()[10..16]);

        Uuid::from_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_sequential_guid_generator() {
        let mut generator = SequentialGuidGenerator::new();
        let mut guids = HashSet::new();

        for _ in 0..1000 {
            let guid = generator.generate();
            assert!(guids.insert(guid), "Generated GUID should be unique");
        }
    }

    #[test]
    fn test_guid_sequential_nature() {
        let mut generator = SequentialGuidGenerator::new();
        let guid1 = generator.generate();
        let guid2 = generator.generate();

        assert!(guid2 > guid1, "Second GUID should be greater than the first");
    }
}