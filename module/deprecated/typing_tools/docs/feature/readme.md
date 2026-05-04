# Feature Doc Entity

### Scope

- **Purpose**: Collect navigational hubs linking all artifacts for each user-facing typing capability.
- **Responsibility**: Index of feature doc instances for the typing_tools crate.
- **In Scope**: User-facing capabilities of the typing_tools aggregator.
- **Out of Scope**: API contracts (→ sub-crate docs/api/), correctness invariants (→ invariant/), internal implementation.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Unified Typing Toolset](001_unified_typing_toolset.md) | Single-import access to all typing macros via feature-gated re-exports | ✅ |
