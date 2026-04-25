# Invariant Doc Entity

### Scope

- **Purpose**: Document behavioral contracts — constraints on this crate that must always hold.
- **Responsibility**: Collect all invariant doc instances defining constraints that must always hold.
- **In Scope**: One instance per distinct constraint that shapes implementation decisions.
- **Out of Scope**: Desired behavior descriptions — see `feature/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Proc-Macro Crate Separation](001_proc_macro_separation.md) | Macro implementation must reside in a dedicated proc-macro crate | ✅ |
| 002 | [Feature Flag Gating](002_feature_flag_gating.md) | Each macro must be independently disableable via feature flag | ✅ |
