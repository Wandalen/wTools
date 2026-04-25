# Feature: Typed Errors

### Scope

- **Purpose**: Exposes all failure modes as a typed enum for structured error handling.
- **Responsibility**: Documents the `Error` enum variants and their intended contexts.
- **In Scope**: All error variants: render failure, missing parameters, filesystem I/O, invalid template.
- **Out of Scope**: Error formatting for CLI output (handled by the `genfile` crate layer).

### Design

The typed error covers all genfile_core failure modes: render failure (template engine failure), missing parameters (mandatory parameters unfilled before generation), filesystem I/O (wrapping OS-level filesystem errors), and invalid template (malformed template syntax). All variants satisfy the standard error contract. Callers can match variants to distinguish user-fixable errors (missing parameters) from system errors (filesystem I/O).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/error.rs` | `Error` enum definition |
| doc | `docs/feature/014_template_generation.md` | Primary error return site |
| doc | `docs/invariant/006_error_message_quality.md` | Quality constraint that applies to these error variants |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR16 in original spec; combined source migrated to feature/. spec.md has been deleted — Sources entry retained as migration record. |
