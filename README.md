# DDD API in Rust

## Overview

This project is an example of implementing Domain-Driven Design (DDD) principles in Rust. The goal is to demonstrate how to structure a Rust project following DDD patterns, including Entities, Value Objects, Aggregate Roots, and other DDD building blocks. The project includes an API that is built using Rust and is designed to be modular, extensible, and aligned with the best practices of DDD.

## Features

- **EntityBase**: A base trait that ensures all entities have a unique identifier.
- **ValueObject**: A trait for defining Value Objects that are compared by their values.
- **AggregateRoot**: A trait that marks an entity as the root of an aggregate.
- **Sequential GUID Generator**: A custom GUID generator to provide sequential unique identifiers.
- **Product Aggregate**: An example aggregate root with `Product` as the main entity.
- **Money Value Object**: An example implementation of a Value Object.
- **Logging System**: A centralized logging system for consistent log management across the application.
- **Error Handling**: A custom error type for domain-specific error handling.
- **Configuration Management**: A flexible configuration system using environment variables and configuration files.

## Project Structure

```
src/
├── crosscutting/
│   ├── guid_generator.rs   # Sequential GUID Generator
│   ├── logging.rs          # Logging system implementation
│   ├── error.rs            # Custom error types and handling
│   └── config.rs           # Configuration management
├── domain/
│   ├── aggregate_root.rs   # Definition of AggregateRoot trait
│   ├── entity_base.rs      # Definition of EntityBase trait
│   ├── product.rs          # Implementation of Product entity as an AggregateRoot
│   ├── value_object.rs     # Definition of ValueObject trait
│   └── value_objects/
│       ├── money.rs        # Implementation of Money as a ValueObject
│       └── identifier.rs   # Implementation of Identifier as a ValueObject
└── lib.rs                  # Library entry point
```

## Getting Started

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install Rust using `rustup`:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Cargo**: Rust's package manager (comes with Rust).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/ddd-api.git
   ```
2. Navigate to the project directory:
   ```bash
   cd ddd-api
   ```

### Building the Project

To build the project, run:
```bash
cargo build
```

### Running Tests

To run the tests for the project, execute:
```bash
cargo test
```

## Usage

The project is intended to be a starting point for building APIs with Rust following DDD principles. You can expand upon the `Product` aggregate, add more entities, value objects, and domain logic as needed.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any changes or additions you'd like to make.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For any inquiries or issues, please reach out to the repository owner through GitHub.


