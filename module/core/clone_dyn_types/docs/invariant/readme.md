# Invariant Doc Entity

### Scope

- **Purpose**: Document constraints that `clone_dyn_types` must maintain at all times.
- **Responsibility**: Specify measurable invariants with enforcement and violation consequences.
- **In Scope**: Zero-dependency constraint, memory safety guarantees, caller usage constraints.
- **Out of Scope**: Feature behaviors (`feature/`), public API contracts (`api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Zero Dependencies](001_zero_dependencies.md) | Zero production dependencies in Cargo.toml | ✅ |
| 002 | [Memory Safety](002_memory_safety.md) | Soundness of all unsafe pointer operations | ✅ |
| 003 | [Usage Constraints](003_usage_constraints.md) | Caller obligations for DST coercion and arity | ✅ |
