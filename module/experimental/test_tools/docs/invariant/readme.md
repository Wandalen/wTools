# Invariant Doc Entity

### Scope

- **Purpose**: Documents correctness properties that must always hold in `test_tools`.
- **Responsibility**: Index of all invariant doc instances with enforcement mechanisms.
- **In Scope**: Structural invariants that prevent compilation failures and runtime bugs.
- **Out of Scope**: Feature requirements; see `docs/feature/`.

### Overview Table

| ID  | Name | Purpose | Status |
|-----|------|---------|--------|
| 001 | [No cfg Gate on Namespace Modules](001_no_cfg_gate_on_namespace.md) | Prevents E0432 compilation failures | ✅ |
| 002 | [No vec! Reexport at Crate Root](002_no_vec_macro_reexport.md) | Prevents E0659 macro ambiguity | ✅ |
