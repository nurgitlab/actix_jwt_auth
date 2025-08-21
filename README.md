# Actix JWT Auth
A high-performance CRUD API built with Rust using the Actix-Web framework and PostgreSQL. This API demonstrates how to implement basic CRUD operations following REST conventions.

## Features
- Cookies handlers
- Error handling middleware
- Auth middleware
- JWT authentication
  
## Requirements
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

## Theory of JWT Auth

### Two Tokens

Access Token: Short-lived (e.g., 3 min). Sent in Authorization: Bearer header. Not stored in DB.

Refresh Token: Long-lived. Stored in the database.

### How it works:

- Login: Check credentials → generate both tokens → save refresh token to DB → send both to client.

- Request: Client sends access token. Server validates it without DB calls.

- Refresh: When access token expires, client sends refresh token. Server checks it in DB → issues new access and new refresh tokens → updates refresh token in DB.

- Logout: Delete refresh token from DB.

### Why rotate refresh tokens? 
It's more secure. Issuing a new one on each refresh prevents reuse and helps detect if a token was stolen.

### Benefit
The DB is only hit for login, refresh, and logout—not on every request. This makes it fast and scalable.

## Directory Structure

```text
.
├── src/
│   ├── main.rs             # Application entry point
│   ├── handlers/           # Request handlers
│   ├── migrations/         # Applying migrations
│   ├── models/             # Data models
│   ├── services/           # Layer for business logic
│   ├── middlewares/        # Custom middleware for request processing
│   ├── repositories/       # Database connection setup
│   └── errors/             # Custom error handling
├── migrations/             # Database migrations (SQL)
├── .env                    # Environment variables
├── Cargo.toml              # Project dependencies
└── README.md               # Project documentation
```

##  Todo

1. Combine Errors (I don't know the best way to do this)
2. Move constants to environment variables