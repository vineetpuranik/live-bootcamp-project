## Live Bootcamp Project

This workspace contains a pair of Rust microservices that we build and iterate on during the live bootcamp. The goal is to demonstrate end-to-end web application development with a dedicated authentication backend and a user-facing application service.

## Project Overview
- `app-service`: Serves the web UI and handles browser interactions using server-rendered templates. It talks to the auth service for sign-up and login flows.
- `auth-service`: Exposes APIs for user registration and authentication, showcasing domain-driven design patterns (value objects for email/password, in-memory stores, etc.).

## Crates in Use
- Shared foundations: `axum` for HTTP routing, `tower-http` for middleware/static assets, and `tokio` as the async runtime.
- App service tooling: `axum-extra` for cookie helpers, `askama` for templates, `reqwest` for outbound HTTP to the auth service, `serde`/`serde_json` for data serialization.
- Auth service tooling: `uuid` for user identifiers, `async-trait` to simplify async trait usage, `validator` for input validation, `serde`/`serde_json` for request/response types.
- Testing stack: `reqwest` (test client), `fake` for generating data, and `quickcheck`/`quickcheck_macros` for property tests.

## Setup & Building
```bash
cargo install cargo-watch
cd app-service
cargo build
cd ..
cd auth-service
cargo build
cd ..
```

## Run servers locally (Manually)
#### App service
```bash
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

visit http://localhost:8000

#### Auth service
```bash
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

visit http://localhost:3000

## Run servers locally (Docker)
```bash
docker compose build
docker compose up
```

visit http://localhost:8000 and http://localhost:3000
