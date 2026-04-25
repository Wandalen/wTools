# Invariant Doc Entity

### Scope

- **Purpose**: Document the behavioral constraints and correctness properties of `component_model_types` that must always hold.
- **Responsibility**: Collect one doc instance per invariant; each instance states the property, how it is enforced, and what breaks if it is violated.
- **In Scope**: Compile-time constraints, design invariants, and the limitations imposed by the Rust type system on this crate.
- **Out of Scope**: Runtime assertions or panics (this crate has none); API guarantees (→ `api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Orphan Rule Constraint](001_orphan_rule.md) | Standard library types cannot be given Assign impls in this crate | ✅ |
