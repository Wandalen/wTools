# Invariant Doc Entity

### Scope

- **Purpose**: Document architectural constraints that `clone_dyn_meta` must maintain permanently.
- **Responsibility**: Enforce the `macro_tools`-only dependency policy for all syntax operations.
- **In Scope**: Proc-macro crate dependency policy.
- **Out of Scope**: User-facing API contracts (clone_dyn facade), runtime safety (clone_dyn_types).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | macro_tools_only | All syntax operations via macro_tools, no direct syn/quote/proc-macro2 | ✅ |
