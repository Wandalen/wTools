# Feature: Typed Errors

### Scope

- **Purpose**: Exposes all failure modes as a typed enum for structured error handling.
- **Responsibility**: Documents the `Error` enum variants and their intended contexts.
- **In Scope**: All error variants: render failure, missing parameters, filesystem I/O, invalid template.
- **Out of Scope**: Error formatting for CLI output (handled by the `genfile` crate layer).

### Design

The `Error` enum covers all genfile_core failure modes: `Render` (template engine failure), `MissingParameters` (mandatory parameters unfilled before generation), `Fs` (wrapping `std::io::Error` for filesystem failures), and `InvalidTemplate` (malformed template syntax). All variants implement `std::error::Error`. The crate uses `error_tools` for error infrastructure. Callers can match variants to distinguish user-fixable errors (MissingParameters) from system errors (Fs).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/error.rs` | `Error` enum definition |
| doc | `docs/feature/014_template_generation.md` | Primary error return site |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR16 in original spec; combined source migrated to feature/ |
