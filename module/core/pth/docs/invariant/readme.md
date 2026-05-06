# Invariant Doc Entity

### Scope

- **Purpose**: Document correctness invariants that must always hold in the `pth` crate.
- **Responsibility**: Collect invariant doc instances for non-negotiable structural properties of the API.
- **In Scope**: Always-hold constraints, enforcement mechanisms, and violation consequences.
- **Out of Scope**: Feature behavior (see `feature/`), API contracts (see `api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Zero Dependencies](001_zero_dependencies.md) | No filesystem access except `CurrentPath` conversion | ✅ |
| 002 | [Fixed Output Format](002_fixed_output_format.md) | All normalization output uses forward-slash separator | ✅ |
