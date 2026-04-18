# Source Directory

Implementation of process_tools crate providing subprocess execution, output capture, CI/CD environment detection, platform-agnostic exit status synthesis, and process lifecycle management.

## File Responsibility Table

| File | Responsibility |
|------|---------------|
| `lib.rs` | Crate entry point and module interface using `mod_interface!` pattern |
| `process.rs` | Subprocess execution with `run()`, `run_with_shell()`, `Run` builder, `Report` output capture |
| `environment.rs` | CI/CD environment detection via `is_cicd()` function |
| `exit_status.rs` | Platform-agnostic `ExitStatus` synthesis hiding Unix/Windows encoding |
| `lifecycle/` | Process lifecycle management: signal mapping, alive checks, daemonization |

## Module Organization

This crate uses the `mod_interface!` pattern with four layers:

- **process**: Core process execution functionality
- **environment**: CI/CD environment detection (feature-gated: `process_environment_is_cicd`)
- **exit_status**: Platform-agnostic `ExitStatus` construction from integer exit codes
- **lifecycle**: Process lifecycle with nested submodules (signal, check, daemon)

## Architecture

All implementation uses inline `mod private { ... }` blocks (not separate `private.rs` files) per `mod_interface` pattern requirements.
