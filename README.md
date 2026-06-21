# Rust Dioxus Template

A template for building full-stack web applications in [Rust](https://github.com/rust-lang/rust) using [Dioxus](https://github.com/dioxuslabs/dioxus) for the frontend and [Axum](https://github.com/tokio-rs/axum) for the backend.

## Features

- **Full-Stack Rust**: Write both frontend and backend in Rust
- **Development Environment**: [devenv](https://github.com/cachix/devenv) is included for an opt-in batteries-included experience with [OpenTelemetry Collector](https://github.com/open-telemetry/opentelemetry-collector-contrib) and [Clickhouse](https://github.com/ClickHouse/ClickHouse)
- **Code Quality**: [Pre-commit](https://github.com/pre-commit/pre-commit) hooks are configured to enforce a clean and consistent coding style
- **CI**: A GitHub Actions workflow that leverages devenv for declarative and reproducible testing

## Crates

### `server`

An Axum backend with:

- [Tokio](https://github.com/tokio-rs/tokio) async runtime
- OpenTelemetry integration for tracing

### `web`

A Dioxus web frontend with:

- Reactive component-based UI
- Logging support with [dioxus-logger](https://github.com/DioxusLabs/dioxus/tree/main/packages/logger)

Compiles to WASM for deployment.

### `types`

Shared data structures with [Serde](https://github.com/serde-rs/serde) serialization:

- Type definitions used by both frontend and backend
- Ensures type safety across the full stack
