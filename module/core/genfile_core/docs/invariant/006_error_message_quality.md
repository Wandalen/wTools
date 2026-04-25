# Invariant: Error Message Quality

### Scope

- **Purpose**: Ensures error messages provide enough context to diagnose failures without reading source code.
- **Responsibility**: Documents the diagnostic context requirement for all error messages.
- **In Scope**: All `Error` variants; their display messages must include file paths, parameter names, or template context.
- **Out of Scope**: CLI error formatting (→ genfile crate `error.rs`), log verbosity levels.

### Invariant Statement

All error messages must include sufficient context — relevant file paths, parameter names, or template locations — to diagnose the problem without consulting source code. Errors that omit this context are invalid.

### Enforcement Mechanism

Manual review during code review; supplemented by error message tests that assert presence of contextual information in error strings. Each `Error` variant's `Display` implementation is reviewed for completeness.

### Violation Consequences

Opaque error messages cause users to give up or file bugs that cannot be reproduced because the failure context is lost. Diagnostic context is mandatory for a library intended for automation use.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/error.rs` | `Error` enum and `Display` implementations |
| doc | `docs/feature/016_typed_errors.md` | Typed error variants documented here |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | NFR6 in original spec; combined source migrated to invariant/. spec.md has been deleted — Sources entry retained as migration record. |
