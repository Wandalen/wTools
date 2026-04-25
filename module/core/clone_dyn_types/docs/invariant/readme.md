# Invariant Doc Entity

### Scope

- **Purpose**: Document constraints that `clone_dyn_types` must maintain at all times.
- **Responsibility**: Specify measurable invariants with enforcement and violation consequences.
- **In Scope**: Zero-dependency constraint, memory safety guarantees, caller usage constraints.
- **Out of Scope**: Feature behaviors (`feature/`), public API contracts (`api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | Zero Dependencies | Zero production dependencies in Cargo.toml | ✅ |
| 002 | Memory Safety | Soundness of all unsafe pointer operations | ✅ |
| 003 | Usage Constraints | Caller obligations for DST coercion and arity | ✅ |
