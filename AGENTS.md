# Repository Guidelines

## Project Structure & Module Organization
Nightfall_4_CE is a Rust workspace defined in `Cargo.toml`, grouping service crates like `nightfall_client`, `nightfall_proposer`, `nightfall_deployer`, sync/integration tools, and shared logic in `lib/`. Service binaries live under `src/bin` in each crate; cross-cutting docs sit in `doc/`, while blockchain artifacts are under `blockchain_assets/`. Runtime configuration defaults live in `nightfall.toml` and `configuration/toml/`, so keep environment-specific overrides isolated there.

## Build, Test, and Development Commands
Run `cargo build --workspace` for a full build and `cargo check -p <crate>` during focused work. Format and lint with `cargo +nightly fmt --all` (required because `rustfmt.toml` enables unstable features) and `cargo clippy --workspace --all-targets --all-features`. Use `cargo run -p nightfall_client` for the API and `docker compose up` to stand up the full stack defined in `docker-compose.yml` when validating multi-service flows.

## Coding Style & Naming Conventions
Rust code follows the 2024 edition defaults with four-space indentation, `snake_case` modules/functions, and `UpperCamelCase` types. Keep modules small and reuse helpers from `lib/` before duplicating logic. Maintain deterministic imports (`rustfmt` reorders automatically) and prefer explicit error types from `lib/error.rs`.

## Testing Guidelines
Unit tests live alongside sources inside each crate; integration/regression suites reside in `nightfall_test/` and `nightfall_sync_test/`. Run `cargo test --workspace` before submitting; add focused runs like `cargo test -p nightfall_test` to exercise orchestration flows. Integration tests expect Anvil and the development profile (`NF4_RUN_MODE=development`), so ensure services in `docker-compose.yml` are healthy before asserting anything CI will touch.

## Commit & Pull Request Guidelines
Model commits on the existing short, imperative style (`<component>: <action>`). Keep related work in one commit, include context in the body when changing protocol behaviour, and reference issues with `#<id>` when applicable. Pull requests should summarize the change, list manual verification (commands run, services touched), and attach logs or screenshots for user-facing differences.

## Environment & Security Notes
Never commit real keys; placeholder material lives in `configuration/trust/` and `nightfall.toml`. Use `.env` files or local secrets managers when testing, and scrub any blockchain endpoints or vault URLs before pushing public branches.
