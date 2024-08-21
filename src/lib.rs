// src/lib.rs

pub mod crosscutting {
    pub mod guid_generator;
    pub mod logging;
    pub mod error;
    pub mod config;
}

pub mod domain {
    pub mod entity_base;
    pub mod aggregate_root;
    pub mod value_object;
    pub mod value_objects {
        pub mod identifier;
        pub mod money;
    }
    pub mod product;
}
