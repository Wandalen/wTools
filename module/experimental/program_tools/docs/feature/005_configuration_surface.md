# Feature: Configuration Surface

### Scope

- **Purpose**: Define all configuration parameters that govern script execution and expose them uniformly through the builder API and the CLI.
- **Responsibility**: Documents every configuration parameter — its purpose, default value, builder field name, and corresponding CLI flag.
- **In Scope**: All RunOptions parameters; their types, constraints, and defaults; their CLI flag counterparts.
- **Out of Scope**: Override precedence between layers (→ `pattern/002`); CLI subcommand structure (→ `api/004`); builder field construction syntax (→ `api/001`).

### Design

Every knob that controls execution behavior is a named parameter in the RunOptions builder and a named flag on the CLI. The two representations are semantically equivalent for identical input values — there is no configuration accessible through one channel but not the other. Defaults may differ per channel: the CLI defaults to forwarding mode for interactive use; the programmatic API defaults to captured mode for test code. This parity of semantics (not defaults) is a design invariant of the configuration surface.

**Scalar vs. collection parameters**: Scalar parameters have a single value that resolves to the highest-priority non-absent value across layers. Collection parameters (features, environment variables) accumulate values from all layers additively — no layer's entries replace another's.

### Parameters Table

| Parameter | Default | CLI Flag | Semantics | Purpose |
|-----------|---------|----------|-----------|---------|
| `build_profile` | `debug` | `--profile` | scalar | Cargo build profile: debug or release |
| `target_dir` | OS temp | `--target-dir` | scalar | Build artifact cache directory; OS temp means per-run and cleaned up |
| `cargo_path` | `cargo` | `--cargo` | scalar | Path to the Cargo binary; resolved via PATH when set to `cargo` |
| `timeout_ms` | none | `--timeout` | scalar | Maximum execution duration in milliseconds; absent means no limit |
| `features` | empty | `--feature` | collection | Cargo features to enable; each flag/call appends one entry |
| `env_vars` | empty | `--env` | collection | Subprocess environment variable assignments; each flag/call appends one entry |
| `edition` | `2021` | `--edition` | scalar | Rust edition for generated manifests only; ignored for project mode |
| `package_name` | `script` | `--name` | scalar | Package name for generated manifests only; ignored for project mode |
| `capture` | `true` (API) / forwarding (CLI) | `--capture` | scalar | Whether output is captured into the return value or forwarded to the terminal |
| `cleanup` | `true` | `--keep` | scalar | Whether to remove the temp workspace after the run; `--keep` sets to false |

**Notes**:
- `edition` and `package_name` apply only when the runner generates a manifest. They are ignored in project mode.
- `target_dir` set to a persistent path persists artifacts across runs. Unset (OS temp) means artifacts are removed with the workspace during cleanup.
- `capture` default differs by channel: the programmatic API defaults to `true` (return value populated for test assertions); the CLI defaults to forwarding (live terminal output for interactive use). In forwarding mode, stdout and stderr are written to the terminal and the captured output fields are empty.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `docs/api/001_builder_api.md` | Builder API: RunOptions field access |
| doc | `docs/api/004_cli_interface.md` | CLI flags corresponding to these parameters |
| doc | `docs/pattern/002_layered_configuration.md` | Override precedence model for resolving parameter values |
| doc | `docs/feature/003_artifact_management.md` | Artifact management parameters: target_dir and cleanup |
| test | `tests/inc/cli_test.rs` | CLI integration tests covering configuration parameter flags |
