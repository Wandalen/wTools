# Invariant: Error Message Quality

### Scope

- **Purpose**: Ensures error messages provide enough context to diagnose failures without reading source code.
- **Responsibility**: Documents the diagnostic context requirement for all error messages.
- **In Scope**: All error variants; their display messages must include file paths, parameter names, or template context.
- **Out of Scope**: CLI error formatting (→ genfile crate `error.rs`), log verbosity levels.

### Invariant Statement

All error messages must include sufficient context — relevant file paths, parameter names, or template locations — to diagnose the problem without consulting source code. Errors that omit this context are invalid.

### Enforcement Mechanism

Manual review during code review; supplemented by error message tests that assert presence of contextual information in error strings. Each error variant's display implementation is reviewed for completeness.

### Violation Consequences

Opaque error messages cause users to give up or file bugs that cannot be reproduced because the failure context is lost. Diagnostic context is mandatory for a library intended for automation use.

### APIs

| File | Relationship |
|------|--------------|
| [api/004_error_contract.md](../api/004_error_contract.md) | API contract governing these error messages |

### Features

| File | Relationship |
|------|--------------|
| [feature/016_typed_errors.md](../feature/016_typed_errors.md) | Typed error variants documented here |

### Sources

| File | Relationship |
|------|--------------|
| `src/error.rs` | Typed error enum and display implementations |

### Tests

| File | Relationship |
|------|--------------|
| `tests/inc/template_error_test.rs` | Error message context assertion tests |
