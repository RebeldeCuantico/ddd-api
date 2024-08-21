// src/crosscutting/logging.rs
use log::{debug, error, info, warn};

pub fn init() {
    env_logger::init();
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}

pub fn log_debug(message: &str) {
    debug!("{}", message);
}

pub fn log_warn(message: &str) {
    warn!("{}", message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            env_logger::builder().is_test(true).init();
        });
    }

    #[test]
    fn test_logging_functions() {
        setup();

        log_info("This is an info message");
        log_error("This is an error message");
        log_debug("This is a debug message");
        log_warn("This is a warning message");        
    }
}