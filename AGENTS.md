# Development Guide

## Workflow

- Use Rust 2024 and keep the public library API documented with concise rustdoc.
- Follow red-green-refactor: write a focused failing test, implement the smallest change, then refactor.
- Put pure domain and service tests beside their modules; put CLI and filesystem workflows in `tests/`.
- Run `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-targets` before submitting changes.

## Design

- Keep CLI parsing in `src/cli.rs` and image generation in `src/services/`.
- Prefer dependency injection through the existing `ImageProcessor` and `IconGenerator` traits.
- Do not log secrets or unnecessary user file contents. Keep generated files in temporary or ignored directories.
- Validate externally supplied paths and image data before processing.

## Documentation and style

- Use `cargo fmt`; do not introduce manual formatting exceptions.
- Comments should explain non-obvious decisions, not restate code or narrate the work.
- Update `README.md` and relevant release documentation when behavior or commands change.
- Preserve the MIT license and add tests for bug fixes and new behavior.
