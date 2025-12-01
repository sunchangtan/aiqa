# Repository Guidelines

## Project Structure & Module Organization
- Workspace root uses Cargo with members `crates/domain-core`, `apps/metadata`, and `apps/metadata-migration`; built artifacts land in `target/`.
- `crates/domain-core`: shared domain primitives such as `aggregate_root`, `repository`, and pagination helpers; tests live in `crates/domain-core/tests`.
- `apps/metadata`: metadata domain implementation with layers under `src/domain`, `src/application`, and `src/infrastructure` (SeaORM entities and mappers).
- `apps/metadata-migration`: SeaORM migration binary and library; migration files live under `src/m20YYMMDD_*`.

## Build, Test, and Development Commands
- `cargo check` — fast type/build validation for the full workspace.
- `cargo fmt` — format all Rust code (run before commits).
- `cargo clippy --all-targets -- -D warnings` — lint with warnings treated as errors.
- `cargo test` or `cargo test -p domain-core` — run unit tests (currently concentrated in `domain-core`).
- Migrations (run inside `apps/metadata-migration`): `cargo run -- up` to apply pending migrations, `cargo run -- down` to roll back the last batch, `cargo run -- status` to inspect state, `cargo run -- generate <name>` to scaffold a new migration.

## Coding Style & Naming Conventions
- Rust 2024 edition; use standard rustfmt defaults (4-space indent, trailing commas where allowed).
- Modules and files: `snake_case`; types and traits: `PascalCase`; functions and vars: `snake_case`; constants: `SCREAMING_SNAKE_CASE`.
- Keep domain boundaries clear: shared abstractions in `domain-core`, app-specific logic in `apps/metadata`; prefer returning `DomainError` variants from domain services.
- Place SeaORM-facing structs under `infrastructure` and isolate mapping in `infrastructure/mapper`.

## Testing Guidelines
- Use the built-in Rust test framework (`#[test]`), naming helper files `*_tests.rs` or adding cases under `tests/`.
- Cover domain rules (value object validation, pagination behavior) and mapper transformations; prefer deterministic, DB-free tests unless verifying migrations.
- If a change requires DB verification, run migrations locally first and gate tests behind env-configured connection strings.

## Commit & Pull Request Guidelines
- Follow the existing Conventional Commit style (`feat: ...`, `chore: ...`, `fix: ...`); keep subjects imperative and scoped.
- PRs should include: a short summary, affected modules, migration impacts (if any), and the commands/tests you ran (e.g., `cargo fmt`, `cargo clippy`, `cargo test`, migration command).
- Link related issues or tickets; add screenshots only when UI/UX changes are introduced (rare here).

## Security & Configuration Tips
- Keep secrets (DB URLs, credentials) in `.env` or local config; never commit them.
- Verify migration targets before running destructive commands (`down`, `reset`, `fresh`); prefer applying to local/dev databases first.
