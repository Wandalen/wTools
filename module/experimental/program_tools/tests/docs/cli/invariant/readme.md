# Invariant Specs

### Scope

- **Purpose**: Catalog test specifications for each behavioral invariant.
- **Responsibility**: One spec file per invariant; IC- prefix; min 2 cases per spec.
- **In Scope**: Cleanup guarantee, execution isolation, output determinism, error propagation.
- **Out of Scope**: Parameter edge cases (→ `param/`); command integration (→ `command/`).

### Overview Table

| Name | Purpose | Status |
|------|---------|--------|
| `cleanup_guarantee.md` | `invariant` spec for Cleanup Guarantee | ⏳ |
| `execution_isolation.md` | `invariant` spec for Execution Isolation | ⏳ |
| `output_determinism.md` | `invariant` spec for Output Determinism | ⏳ |
| `error_propagation.md` | `invariant` spec for Error Propagation | ⏳ |
