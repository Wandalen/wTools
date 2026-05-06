# Invariant Doc Entity

### Scope

- **Purpose**: Document the correctness properties that inspect_type must maintain across all versions and callers.
- **Responsibility**: Index of invariant doc instances for the inspect_type crate.
- **In Scope**: Properties that, if violated, would break callers or undermine the crate's design guarantees.
- **Out of Scope**: Feature documentation (→ feature/), API contracts (→ api/), implementation details.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Zero Runtime Dependencies](001_zero_dependencies.md) | No transitive runtime deps introduced into caller projects | ✅ |
| 002 | [Fixed Output Format](002_fixed_output_format.md) | Stable, predictable string format for all inspection output | ✅ |
