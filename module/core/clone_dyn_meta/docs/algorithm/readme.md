# Algorithm Doc Entity

### Scope

- **Purpose**: Document the internal logic of macro implementation for maintainers and contributors.
- **Responsibility**: Describe parsing, AST mutation, code generation, and the optional debug path.
- **In Scope**: `src/clone_dyn.rs` step-by-step logic, generic decomposition, where clause extension, `qt!` token generation.
- **Out of Scope**: Public API contract (`api/`), feature rationale (`feature/`), runtime clone behavior (in `clone_dyn` / `clone_dyn_types`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Macro Expansion](001_macro_expansion.md) | Full expansion pipeline from `(attr, item)` input to generated impls | ✅ |
