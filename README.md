# Actix JWT Auth
A high-performance CRUD API built with Rust using the Actix-Web framework and PostgreSQL. This API demonstrates how to implement basic CRUD operations following REST conventions.

## Features
- Create, Read, Update, and Delete operations
- PostgreSQL database integration
- JSON request/response format
- Error handling middleware
- Added logging
  
## Features
- PostgreSQL 12+
- Cargo (Rust's package manager)
  
## Setup
1. Configure database:
   ```shell
   DATABASE_URL=postgres://username:password@localhost:5432/db_name
   ```
2. Start the application:
    ```shell 
    cd src
    ```
    ```shell
    cargo run
    ```

## Directory Structure

```text
.
├── src/
│   ├── main.rs             # Application entry point
│   ├── handlers/           # Request handlers
│   ├── migrations/         # Applying migrations
│   ├── models/             # Data models
│   ├── repositories/       # Database connection setup
│   └── errors/             # Custom error handling
├── migrations/             # Database migrations (SQL)
├── .env                    # Environment variables
├── Cargo.toml              # Project dependencies
└── README.md               # Project documentation