// src/crosscutting/error.rs
use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    ValidationError(String),
    NotFoundError(String),
    ConflictError(String),
    UnexpectedError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DomainError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            DomainError::NotFoundError(msg) => write!(f, "Not Found Error: {}", msg),
            DomainError::ConflictError(msg) => write!(f, "Conflict Error: {}", msg),
            DomainError::UnexpectedError(msg) => write!(f, "Unexpected Error: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_error_display() {
        let validation_error = DomainError::ValidationError("Invalid input".to_string());
        assert_eq!(
            validation_error.to_string(),
            "Validation Error: Invalid input"
        );

        let not_found_error = DomainError::NotFoundError("User not found".to_string());
        assert_eq!(not_found_error.to_string(), "Not Found Error: User not found");

        let conflict_error = DomainError::ConflictError("Duplicate entry".to_string());
        assert_eq!(conflict_error.to_string(), "Conflict Error: Duplicate entry");

        let unexpected_error = DomainError::UnexpectedError("Something went wrong".to_string());
        assert_eq!(
            unexpected_error.to_string(),
            "Unexpected Error: Something went wrong"
        );
    }
}