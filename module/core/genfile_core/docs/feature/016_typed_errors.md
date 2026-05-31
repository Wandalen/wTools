# Feature: Typed Errors

### Scope

- **Purpose**: Exposes all failure modes as a typed enum for structured error handling.
- **Responsibility**: Documents the typed error variants and their intended contexts.
- **In Scope**: All error variants: render failure, missing parameters, filesystem I/O, invalid template.
- **Out of Scope**: Error formatting for CLI output (handled by the genfile crate layer).

### Design

The typed error covers all genfile_core failure modes: render failure (template engine failure), missing parameters (mandatory parameters unfilled before generation), filesystem I/O (wrapping OS-level filesystem errors), and invalid template (malformed template syntax). All variants satisfy the standard error contract. Callers can match variants to distinguish user-fixable errors (missing parameters) from system errors (filesystem I/O).

### APIs

| File | Relationship |
|------|--------------|
| [api/004_error_contract.md](../api/004_error_contract.md) | API contract for the typed error surface |

### Features

| File | Relationship |
|------|--------------|
| [feature/014_template_generation.md](014_template_generation.md) | Primary error return site |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/006_error_message_quality.md](../invariant/006_error_message_quality.md) | Quality constraint that applies to these error variants |

### Sources

| File | Relationship |
|------|--------------|
| `src/error.rs` | Typed error enum definition |

### Tests

| File | Relationship |
|------|--------------|
| `tests/inc/template_error_test.rs` | Typed error variant and display tests |
