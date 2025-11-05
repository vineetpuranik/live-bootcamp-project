
<div align="center">
  <img src="./app-service/assets/Rustgate.png" alt="Rustgate Logo" width="150"/>
</div>

# Rustgate

This repository provides a robust foundation for building scalable microservices in Rust. It features a decoupled authentication service and a user-facing application service, built with a modern technology stack that includes Axum, Tokio, SQLx, and a curated set of high-quality crates. User credentials are stored securely in PostgreSQL with Argon2 hashing. The project demonstrates best practices in web application development, including domain-driven design, containerization, and comprehensive testing.

## Table of Contents

- [Project Overview](#project-overview)
- [Architecture](#architecture)
- [Core Dependencies](#core-dependencies)
- [Technical Deep Dive](#technical-deep-dive)
- [Development Environment Setup](#development-environment-setup)
- [Running the Services](#running-the-services)
- [Task Automation with Justfile](#task-automation-with-justfile)

## Project Overview
- `app-service`: Serves the web UI and handles browser interactions using server-rendered templates. It talks to the auth service for sign-up and login flows.
- `auth-service`: Exposes APIs for user registration and authentication, showcasing domain-driven design patterns (value objects for email/password) backed by a PostgreSQL user store.

## Architecture

```mermaid
graph TD
    subgraph "User's Browser"
        A[Browser]
    end

    subgraph "Application"
        B(app-service)
    end

    subgraph "Authentication"
        C(auth-service)
    end

    subgraph "Persistence"
        D[(PostgreSQL)]
    end

    A -- HTTP Requests --> B
    B -- API Calls --> C
    C -- SQL Queries --> D
```

## Core Dependencies

This project leverages a curated set of high-quality crates to ensure robustness, performance, and developer productivity.

- **`axum`**: A powerful and ergonomic web framework for building robust APIs and web services.
- **`tokio`**: The de-facto asynchronous runtime for Rust, providing the foundation for concurrent and non-blocking I/O.
- **`tower-http`**: A collection of essential HTTP middleware for `axum`, used for features like serving static assets and CORS.
- **`serde` & `serde_json`**: For efficient and reliable serialization and deserialization of data between Rust structs and JSON.
- **`reqwest`**: A feature-rich and intuitive HTTP client for making requests to external services.

### Application Service

- **`askama`**: A type-safe and compiled template engine for server-side rendering of HTML, ensuring correctness at compile time.
- **`axum-extra`**: Provides useful extractors and utilities for `axum`, such as cookie handling.

### Authentication Service

- **`jsonwebtoken`**: Implements JSON Web Tokens (JWT) for secure, stateless authentication.
- **`validator`**: A library for data validation, ensuring the integrity of incoming requests.
- **`uuid`**: For generating and managing unique identifiers for users and other resources.
- **`sqlx`**: Async, compile-time checked Postgres access used by the production user store.
- **`async-trait`**: Enables the use of `async fn` in traits, simplifying asynchronous code.
- **`chrono`**: A comprehensive library for handling dates and times.
- **`lazy_static`**: Allows for the declaration of lazily initialized static variables.
- **`rand`**: A library for generating random numbers, used for cryptographic purposes.
- **`argon2`**: Provides the Argon2id password hashing algorithm for secure credential storage.

### Testing

- **`fake`**: A library for generating realistic fake data for testing.
- **`quickcheck` & `quickcheck_macros`**: A framework for property-based testing, which helps in finding edge cases and bugs.

## Technical Deep Dive

### Asynchronous Runtime: Tokio

This project uses the `tokio` runtime to manage asynchronous tasks. By default, Tokio spins up a multi-threaded runtime, creating a thread for each CPU core on the machine. This allows the services to handle a large number of concurrent connections efficiently. Each incoming request is spawned as a new task on the runtime, which is then executed by one of the worker threads. This model allows for a high degree of concurrency, as the threads can switch between tasks whenever one is waiting for I/O.

### Web Framework: Axum

`axum` is a web framework built on top of `tokio` and `hyper`. It uses a tower-based middleware architecture, which allows for a flexible and composable way of handling requests. When a request comes in, it is passed through a series of middleware layers, each of which can modify the request or response. For example, the `tower-http` crate provides middleware for logging, compression, and serving static files.

### Service Communication

The `app-service` and `auth-service` communicate with each other via REST APIs. The `app-service` uses the `reqwest` crate to make HTTP requests to the `auth-service`. This allows for a clean separation of concerns, as the `app-service` does not need to know about the implementation details of the `auth-service`. It also allows the two services to be deployed and scaled independently.

### Persistence Layer: PostgreSQL

The authentication service persists users in PostgreSQL through `sqlx`, using a pooled connection (`PgPool`) so concurrent requests can reuse database connections efficiently. Schema changes live under `auth-service/migrations` and are applied automatically on startup via `sqlx::migrate!`, which keeps the runtime in sync with the migration history. Passwords are encoded with Argon2id before being written to the `users` table, and verification work is pushed onto Tokio's blocking thread pool to avoid stalling async request handlers.

### Service Initialization

Both the `app-service` and `auth-service` are initialized in their respective `main.rs` files. This is where the Axum router is created and configured, and where the various components of the service are wired together.

In the `app-service`, the `main.rs` file is responsible for:

-   Creating the Axum router.
-   Adding middleware for serving static files and handling errors.
-   Defining the routes for the web interface.
-   Starting the Axum server.

In the `auth-service`, the `main.rs` file is responsible for:

-   Creating the Axum router.
-   Adding middleware for CORS and error handling.
-   Defining the routes for the authentication API.
-   Establishing the PostgreSQL connection pool (and running pending migrations) alongside the in-memory stores for banned tokens and 2FA codes.
-   Starting the Axum server.

## Development Environment Setup

To get started with this project, you will need to have the Rust toolchain and `cargo-watch` installed.

1.  **Install Rust:** If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/tools/install).

2.  **Install `cargo-watch`:** This tool is used to automatically rebuild and rerun the services when changes are made to the source code.

    ```bash
    cargo install cargo-watch
    ```

3.  **Create a `.env` file:** Place it at the project root so both services can read the shared configuration. At minimum it should include:

    ```env
    JWT_SECRET=super-secret-value
    DATABASE_URL=postgres://postgres:<password>@localhost:5432
    POSTGRES_PASSWORD=<password>
    SQLX_OFFLINE=true
    ```

    Adjust the credentials to match your local setup. `SQLX_OFFLINE=true` lets `sqlx::migrate!` compile without a live database during builds.

4.  **Start PostgreSQL:** The quickest option during development is the bundled Docker Compose service:

    ```bash
    docker compose up -d db
    ```

    You can also point `DATABASE_URL` at any existing PostgreSQL instance if you prefer a local install.

    > Tip: To create new migrations or run them manually, install the SQLx CLI with
    > `cargo install sqlx-cli --no-default-features --features postgres,rustls`.

5.  **Build the services:**

    ```bash
    # Build the application service
    cd app-service
    cargo build

    # Build the authentication service
    cd ../auth-service
    cargo build
    ```

## Running the Services

### Manual Execution

For development, you can run the services manually using `cargo-watch`. This will automatically restart the services whenever you make changes to the code. Ensure the PostgreSQL instance from the setup steps is running; the auth service will apply pending migrations on startup using the `DATABASE_URL` from your `.env` file.

#### Application Service

The application service is responsible for the user-facing web interface. To run it, execute the following command:

```bash
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

The application service will be available at `http://localhost:8000`.

#### Authentication Service

The authentication service handles user registration, login, and other authentication-related tasks. To run it, execute the following command:

```bash
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

The authentication service will be available at `http://localhost:3000`.

> The auth service loads configuration with `dotenvy`, so keeping the `.env` file at the repository root is sufficient—no extra exports are needed when running locally.

### Docker-based Execution

For a more production-like environment, you can use Docker to run the services. This ensures that the services are running in a consistent and isolated environment.

1.  **Create a `.env` file:**

    Create a `.env` file in the root of the project (you can reuse the one from the development setup):

    ```env
    JWT_SECRET=your-secret
    POSTGRES_PASSWORD=<password>
    DATABASE_URL=postgres://postgres:<password>@localhost:5432
    SQLX_OFFLINE=true
    ```

2.  **Run the services:**

    ```bash
    ./docker.sh
    ```

    This will build the Docker images and start the services.

    - The application service will be available at `http://localhost:8000`.
    - The authentication service will be available at `http://localhost:3000`.
    - PostgreSQL will be available on `localhost:5432` (mapped from the `db` container).
