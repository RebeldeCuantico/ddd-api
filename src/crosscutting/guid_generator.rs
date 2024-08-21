// src/crosscutting/guid_generator.rs
use uuid::Uuid;

pub struct SequentialGuidGenerator {
    last_uuid: Uuid
}

impl SequentialGuidGenerator {
    pub fn new() -> Self {
        SequentialGuidGenerator {
            last_uuid: Uuid::nil()
        }
    }

    pub fn generate(&mut self) -> Uuid {
        let mut bytes = self.last_uuid.as_bytes().clone();

        for i in (0..16).rev() {
            if bytes[i] < 255 {
                bytes[i] += 1;
                break;
            }
            else{
                bytes[i] = 0;
            }
        }

        self.last_uuid = Uuid::from_bytes(bytes);
        self.last_uuid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_guid_generator() {
        let mut generator = SequentialGuidGenerator::new();
        let first_guid = generator.generate();
        let second_guid = generator.generate();

        assert_ne!(first_guid, Uuid::nil(), "First GUID should not be nil");
        assert_ne!(second_guid, first_guid, "Second GUID should be different from the first");
    }

    #[test]
    fn test_uuids_are_sequential() {
        let mut generator = SequentialGuidGenerator::new();
        let first_guid = generator.generate();
        let second_guid = generator.generate();
       
        let first_bytes = first_guid.as_bytes();
        let second_bytes = second_guid.as_bytes();
     
        let mut carry = true; 

        for (first_byte, second_byte) in first_bytes.iter().zip(second_bytes.iter()).rev() {
            let expected_byte = if carry {
                if *first_byte == 255 {
                    0
                } else {
                    carry = false;
                    first_byte + 1
                }
            } else {
                *first_byte
            };
            assert_eq!(
                *second_byte, expected_byte,
                "The second GUID is not the sequential successor of the first GUID"
            );
        }
      
        assert!(!carry, "The UUIDs should have differed by exactly one");
    }
}

