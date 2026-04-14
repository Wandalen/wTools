# Source Directory

Implementation of process_tools crate providing subprocess execution and CI/CD environment detection.

## File Responsibility Table

| File | Responsibility |
|------|---------------|
| `lib.rs` | Crate entry point and module interface using `mod_interface!` pattern |
| `process.rs` | Subprocess execution with `run()`, `run_with_shell()`, `Run` builder, `Report` output capture |
| `environment.rs` | CI/CD environment detection via `is_cicd()` function |

## Module Organization

This crate uses the `mod_interface!` pattern with two layers:

- **process**: Core process execution functionality
- **environment**: CI/CD environment detection (feature-gated: `process_environment_is_cicd`)

## Architecture

All implementation uses inline `mod private { ... }` blocks (not separate `private.rs` files) per `mod_interface` pattern requirements.
