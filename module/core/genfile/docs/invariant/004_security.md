# Invariant: Security

### Scope

- **Purpose**: Prevents path traversal attacks and injection vulnerabilities in file generation.
- **Responsibility**: Documents security constraints and the validation mechanisms enforcing them.
- **In Scope**: Path traversal prevention, no shell injection, safe binary file handling.
- **Out of Scope**: Network security (no external communication in CLI mode), authentication.

### Invariant Statement

All user-supplied file paths must be validated via `genfile_core::validate_path()` to reject `..` segments. No shell commands may be constructed from user input. Binary file content must be safely encoded without data leakage. Sensitive parameter values must not appear in logs or error messages.

### Enforcement Mechanism

`genfile_core::validate_path()` is called on all paths received from command arguments before use. No `std::process::Command` or shell execution occurs with user-supplied data. Security-focused integration tests verify traversal rejection. Code review enforces no injection patterns.

### Violation Consequences

Path traversal allows an attacker to overwrite arbitrary files outside the intended output directory during materialization, potentially overwriting system files or source code.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/handlers/materialize.rs` | Primary path validation call site |
| doc | `docs/feature/006_template_materialization.md` | Feature where path validation is critical |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | NFR4 in original spec; combined source migrated to invariant/ |
