# API Doc Entity

### Scope

- **Purpose**: Document the public macro API contract of the `impls_index` crate.
- **Responsibility**: Define operations, error handling, and compatibility guarantees for each macro group.
- **In Scope**: Macro API contracts — indexing macros, invocation macros, function utility macros.
- **Out of Scope**: User-facing feature descriptions (→ `feature/`), architectural patterns (→ `pattern/`), correctness invariants (→ `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Indexing Macros](001_indexing_macros.md) | API contract for all function- and test-wrapping macros | ✅ |
| 002 | [Invocation Macros](002_invocation_macros.md) | API contract for `index!` and `tests_index!` | ✅ |
| 003 | [Utility Macros](003_utility_macros.md) | API contract for `fn_name!`, `fn_rename!`, `fns!`, `fns2!` | ✅ |
