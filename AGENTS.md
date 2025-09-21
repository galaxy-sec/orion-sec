# Repository Guidelines

## Project Structure & Module Organization
Source lives under `src/`, with `lib.rs` re-exporting the public surface. Place core domain types in `sec.rs`, loader helpers in `load.rs`, reusable errors in `error.rs`, and case-insensitive utilities in `types.rs`. Integration tests belong in `tests/` (create it when needed); keep fixture data beside the test that uses it. `_gal/` stores agent configuration and should not be modified unless you are adjusting local automation. Build artifacts appear in `target/` and should stay out of version control.

## Build, Test, and Development Commands
- `cargo fmt --all` – format Rust sources with the repo’s `rustfmt` settings.
- `cargo clippy --all-targets --all-features -- -D warnings` – lint and fail on any warning.
- `cargo test --all-features -- --test-threads=1` – run the required test suite deterministically.
- `cargo llvm-cov --all-features --workspace` – generate coverage data that matches CI.
Run the commands locally before opening a pull request to mirror the CI matrix.

## Coding Style & Naming Conventions
Follow Rust 2021 defaults with 4-space indents and `rustfmt` formatting. Use `snake_case` for modules and functions, `UpperCamelCase` for types and traits, and `SCREAMING_SNAKE_CASE` for constants. Keep public APIs funneled through `lib.rs` and document new items with `///` comments when behavior is non-obvious. Prefer `IndexMap` when order matters, matching existing patterns in `sec.rs`.

## Testing Guidelines
Add unit tests alongside implementation blocks with `#[cfg(test)]` modules, and integration tests under `tests/`. Name test functions with the behavior under test (e.g., `loads_secret_fields`). Maintain parity with CI by running `cargo test --all-features -- --test-threads=1`. Use `cargo llvm-cov` before submitting whenever coverage changes; aim to preserve or improve the Codecov trend.

## Commit & Pull Request Guidelines
Write imperative, present-tense commit subjects (`Add secret loader guard`) and keep them under ~72 characters. Squash noisy fixups before pushing. Pull requests should describe the change, list validation commands run, link related issues, and include screenshots when behavior affects outputs. Confirm formatting, clippy, tests, and coverage locally so CI stays green.

## Security & Maintenance Checks
Run `cargo audit` when touching dependencies, mirroring the CI security job. Never store secrets in the repo; keep them in environment files or secure vaults. Update `version.txt` and `Cargo.toml` together for releases, and note any breaking changes in `CHANGELOG` if one exists.
