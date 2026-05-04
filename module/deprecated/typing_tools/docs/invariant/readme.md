# Invariant Doc Entity

### Scope

- **Purpose**: Document the correctness properties that typing_tools must maintain across all versions and callers.
- **Responsibility**: Index of invariant doc instances for the typing_tools crate.
- **In Scope**: Properties that, if violated, would break callers or undermine the crate's design guarantees.
- **Out of Scope**: Feature documentation (→ feature/), API contracts (→ sub-crate docs/api/), implementation details.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Feature-Gated Sub-Crate Activation](001_feature_gated_activation.md) | Sub-crates activated only when their feature flags are set | ✅ |
