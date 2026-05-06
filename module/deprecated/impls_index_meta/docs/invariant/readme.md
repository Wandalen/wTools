# Invariant Doc Entity

### Scope

- **Purpose**: Document behavioral invariants and contracts that `impls_index_meta` must uphold
- **Responsibility**: One instance per distinct, verifiable behavioral guarantee
- **In Scope**: Compile-time guarantees, input/output contracts, code generation invariants
- **Out of Scope**: Performance targets, runtime behavior (no runtime in a proc-macro crate)

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Compile-Time Only](001_compile_time_only.md) | All macro errors are compile-time; no runtime panics or failures | ✅ |
