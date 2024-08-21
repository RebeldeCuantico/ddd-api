// src/crosscutting/guid_generator.rs
use uuid::Uuid;

pub trait GuidGenerator {
    fn generate(&mut self) -> Uuid;
}

pub struct SequentialGuidGenerator {
    last_uuid: Uuid,
}

impl SequentialGuidGenerator {
    pub fn new() -> Self {
        SequentialGuidGenerator {
            last_uuid: Uuid::new_v4()
        }
    }
}

impl GuidGenerator for SequentialGuidGenerator {
    fn generate(&mut self) -> Uuid {
        let mut bytes = self.last_uuid.as_bytes().clone();
        
        for i in (0..16).rev() {
            if bytes[i] < 255 {
                bytes[i] += 1;
                break;
            } else {
                bytes[i] = 0;
            }
        }

        self.last_uuid = Uuid::from_bytes(bytes);
        self.last_uuid
    }
}