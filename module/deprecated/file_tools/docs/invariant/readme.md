# Invariant Doc Entity

### Scope

- **Purpose**: Document correctness properties of `file_tools` that must always hold.
- **Responsibility**: Registry and overview of all invariant doc instances for this crate.
- **In Scope**: Feature-flag correctness properties enforced at compile time; Drop safety guarantees.
- **Out of Scope**: Feature behavior (see `../feature/`), API contracts (see `../api/`), design decisions (see `../pattern/`).

### Files

| File | Responsibility |
|------|----------------|
| `procedure.md` | Instance creation and deprecation procedure |
| `001_std_feature_gating.md` | Std feature gating invariant doc |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Std Feature Gating](001_std_feature_gating.md) | `TempDir` requires `enabled` AND `not(no_std)` simultaneously | ✅ |
