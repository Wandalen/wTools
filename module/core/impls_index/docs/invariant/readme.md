# Invariant Doc Entity

### Scope

- **Purpose**: Document correctness properties that must hold at all times in the `impls_index` crate.
- **Responsibility**: Specify invariant statements, enforcement mechanisms, and consequences of violation.
- **In Scope**: Macro enforcement invariants and compile-time guarantees.
- **Out of Scope**: Feature descriptions (→ `feature/`), API contracts (→ `api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Unused Macro Enforcement](001_unused_macro_enforcement.md) | All strict-indexed functions must be used via `index!` | ✅ |
| 002 | [Compile-Time Resolution](002_compile_time_resolution.md) | All indexing resolves entirely at compile time | ✅ |
