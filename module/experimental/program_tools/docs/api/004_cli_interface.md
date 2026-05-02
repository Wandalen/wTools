# API: CLI Interface

### Scope

- **Purpose**: Provide a command-line interface equivalent to the programmatic API for interactive and shell-script use.
- **Responsibility**: Documents the CLI entry point, its subcommands, available flags, output behavior, and exit code semantics.
- **In Scope**: Available subcommands; flag names, types, and defaults; stdout/stderr forwarding behavior; exit codes.
- **Out of Scope**: Programmatic API (→ `api/001`, `api/002`, `api/003`); configuration override precedence (→ `pattern/002`).

### Abstract

The CLI wraps the programmatic runner behind a command-line interface, making every configuration parameter accessible as a flag. Every parameter available through the builder API is also available as a CLI flag, with equivalent semantics for identical input values. The CLI is the primary entry point for interactive use, shell scripts, and CI pipeline invocations that do not embed Rust code.

### Operations

**Binary name**: `program_tools`

**Subcommand: `run`**

Execute a Rust file or project as a script.

```
program_tools run [OPTIONS] <TARGET>
```

`<TARGET>` is one of:
- A path to a single Rust source file (e.g., `main.rs`, `src/bin/tool.rs`)
- A path to an existing Cargo project directory (must contain `Cargo.toml`)

| Flag | Type | Default | Purpose |
|------|------|---------|---------|
| `--profile` | `debug\|release` | `debug` | Cargo build profile |
| `--target-dir` | path | OS temp | Artifact cache directory |
| `--cargo` | path | `cargo` | Path to the Cargo binary |
| `--timeout` | milliseconds | none | Maximum execution duration |
| `--feature` | string | — | Enable a Cargo feature (repeatable) |
| `--env` | `KEY=VALUE` | — | Set a subprocess environment variable (repeatable) |
| `--edition` | `2021\|2024` | `2021` | Rust edition for generated manifests |
| `--name` | string | `script` | Package name for generated manifests |
| `--capture` | flag | false | Capture output as structured summary instead of forwarding to terminal |
| `--keep` | flag | false | Retain the temp workspace after the run |

**Exit codes**:
- `0` — execution succeeded and the target program exited zero
- `1` — infrastructure error (workspace allocation failure, Cargo not found, manifest generation failure)
- The target program's non-zero exit code is forwarded when the program itself exits non-zero

**Output behavior (default)**:
The target program's stdout and stderr are forwarded directly to the terminal in real time. The CLI does not buffer or suppress output. Pass `--capture` to suppress forwarding and instead print a structured summary after completion.

### Error Handling

Infrastructure errors print a diagnostic message to stderr and exit with code 1. Compilation errors pass Cargo's diagnostic output through to stderr and exit with Cargo's exit code.

### Compatibility Guarantees

Version 0.1.0, marked experimental. Flag names, subcommand structure, and exit codes are expected to evolve before stabilization.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/main.rs` | CLI entry point: clap parser and subcommand dispatch |
| doc | `docs/feature/005_configuration_surface.md` | Complete parameter reference with CLI flag annotations |
| doc | `docs/api/002_runner_api.md` | Programmatic runner that the CLI delegates to |
| doc | `docs/pattern/002_layered_configuration.md` | Override precedence: CLI flags at the top layer |
| test | `tests/inc/cli_test.rs` | CLI integration tests: happy path, error handling, flag validation |
| test | `tests/inc/cli_test.rs` | CLI integration tests: TC-1, TC-3, TC-4, TC-5 |
