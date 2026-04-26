# Invariant: Error Handling

### Scope

- **Purpose**: Ensures consistent, structured error output and correct process exit codes.
- **Responsibility**: Documents the error format, exit code contract, and no-silent-failure rule.
- **In Scope**: `[ERROR] [CONTEXT]: message` format, exit codes 0/1/2, path validation, no uncaught panics.
- **Out of Scope**: Diagnostic context quality (→ genfile_core invariant/006), performance.

### Invariant Statement

All errors must use the format `[ERROR] [CONTEXT]: message`. Exit codes must follow: 0 (success), 1 (runtime error), 2 (usage/argument error). Path validation must reject `..` segments to prevent directory traversal. No operation may fail silently; every error path must produce visible output.

### Enforcement Mechanism

Integration tests assert on exit codes and error message format. The `crate::error` module enforces the format via helper functions. `genfile_core::validate_path()` is used for all user-supplied paths.

### Violation Consequences

Inconsistent exit codes break shell scripts and CI pipelines that check `$?`. Silent failures hide bugs during automation. Unvalidated paths are a security vulnerability enabling directory traversal attacks.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/error.rs` | Error formatting helpers enforcing the message format |
| doc | `docs/feature/006_template_materialization.md` | Primary user of path validation |
