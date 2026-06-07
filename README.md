# MetaMask Backend

A lightweight Rust backend for MetaMask-related services (AI, audit, execution, permissions).

## Repository Structure

- `app/` — main Rust application
  - `Cargo.toml` — project manifest
  - `src/` — source files
    - `config.rs`, `main.rs`, `router.rs`, `state.rs`, `error.rs`
    - `handlers/`, `models/`, `services/` — domain modules

## Prerequisites

- Rust toolchain (rustc + cargo) — install via https://rustup.rs
- Git

## Environment

Copy `app/.env.example` to `app/.env` and update values as needed.

Key variables (example):

- `DATABASE_URL` — database connection string
- `OPENAI_API_KEY` — OpenAI API key (if used)

See [app/.env.example](app/.env.example) for the full sample.

## Setup

1. Install Rust toolchain:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Build dependencies:

```bash
cd app
cargo build
```

## Run

Run the server in development:

```bash
cd app
cargo run
```

By default the server will bind to the address configured in `config.rs` or via environment variables.

## Tests

Run unit tests:

```bash
cd app
cargo test
```

## Formatting & Linting

```bash
cd app
cargo fmt --all
cargo clippy --all -- -D warnings
```

## Docker (optional)

You can containerize the service by creating a `Dockerfile` at the project root. A minimal example:

```dockerfile
FROM rust:1.76 as builder
WORKDIR /usr/src/app
COPY app/ ./app/
WORKDIR /usr/src/app/app
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/app/app/target/release/app /usr/local/bin/app
CMD ["/usr/local/bin/app"]
```

## Contributing

Contributions are welcome. Please open issues or pull requests with a clear description and tests when applicable.

## License

Specify a license for this project (e.g., MIT, Apache-2.0). Add a `LICENSE` file.

## Contact

For questions, open an issue in this repository.
