# Invariant: Error Handling

### Scope

- **Purpose**: Ensures consistent, structured error output and correct process exit codes.
- **Responsibility**: Documents the error format, exit code contract, and no-silent-failure rule.
- **In Scope**: `[ERROR] [CONTEXT]: message` format, exit codes 0/1/2, path validation, no uncaught panics.
- **Out of Scope**: Diagnostic context quality (→ genfile_core invariant/006), performance.

### Invariant Statement

All errors must use the format `[ERROR] [CONTEXT]: message`. Exit codes must follow: 0 (success), 1 (runtime error), 2 (usage/argument error). Path validation must reject `..` segments to prevent directory traversal. No operation may fail silently; every error path must produce visible output.

### Enforcement Mechanism

Integration tests assert on exit codes and error message format. A dedicated error module enforces the format via helper functions. All user-supplied paths pass through a validation function that rejects traversal patterns.

### Violation Consequences

Inconsistent exit codes break shell scripts and CI pipelines that check `$?`. Silent failures hide bugs during automation. Unvalidated paths are a security vulnerability enabling directory traversal attacks.

### Features

| File | Relationship |
|------|--------------|
| [`feature/006_template_materialization.md`](../feature/006_template_materialization.md) | Primary feature where path validation and error format are enforced |

### Sources

| File | Relationship |
|------|--------------|
| [`src/error.rs`](../../src/error.rs) | Error formatting helpers enforcing the message format |
