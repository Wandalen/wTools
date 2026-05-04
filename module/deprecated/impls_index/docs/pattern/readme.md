# Pattern Doc Entity

### Scope

- **Purpose**: Document architectural and design patterns applied in the `impls_index` ecosystem.
- **Responsibility**: Describe reusable design solutions, their applicability, and trade-offs.
- **In Scope**: Structural patterns governing the `impls_index` + `impls_index_meta` architecture.
- **Out of Scope**: Feature descriptions (→ `feature/`), correctness invariants (→ `invariant/`), API contracts (→ `api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Two-Crate Proc Macro](001_two_crate_proc_macro.md) | Isolate proc macro crate from runtime crate | ✅ |
