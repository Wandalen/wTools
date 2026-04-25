# Invariant: Orphan Rule Constraint

### Scope

- **Purpose**: Document why `component_model_types` cannot provide assignment implementations for standard library types such as Duration and PathBuf.
- **Responsibility**: States the constraint, explains how the ecosystem works around it, and captures the design consequence for callers expecting out-of-the-box standard library support.
- **In Scope**: The orphan rule restriction on this crate, its cause, and how the derive macro layer resolves it.
- **Out of Scope**: The macro-side generation logic (→ `component_model_meta/docs/algorithm/001_popular_type_generation.md`).

### Invariant Statement

This crate MUST NOT implement the Assign trait for any type defined in an external crate (including the standard library). The Rust compiler enforces the orphan rule: a trait implementation is allowed only when either the trait or the type being implemented for is defined in the current crate. Since both Assign and types like Duration or PathBuf are foreign to each other in user code, no blanket implementation covering them can reside here.

### Enforcement Mechanism

The constraint is enforced at compile time by the Rust compiler. Any attempt to add an Assign implementation for a standard library type in this crate produces a compile error citing the orphan rule. No runtime check is needed or possible.

The ecosystem works around this constraint through the derive macro layer: the `component_model_meta` crate generates Assign implementations for recognized popular types (Duration, PathBuf) directly in the user's crate at derive expansion time, where those types are foreign only to the trait — which is sufficient to satisfy the orphan rule.

### Violation Consequences

Attempting to implement Assign for a standard library type in this crate fails compilation immediately, so no runtime violation is possible. The design consequence is that callers cannot receive standard library support automatically — they must either use the derive macros (`#[derive(ComponentModel)]`) or write their own Assign implementations for standard library types.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/popular_types/std_types.rs` | PopularType marker — signals to the macro layer which types need generated impls |
| api | [api/001_assign_trait.md](../api/001_assign_trait.md) | The Assign trait this constraint applies to |
| doc | `component_model_meta/docs/algorithm/001_popular_type_generation.md` | Macro-side workaround generating impls in user crates |
