# API Doc Entity

### Scope

- **Purpose**: Document the public interface exposed by `derive_tools`.
- **Responsibility**: Master index for all API doc instances in this crate.
- **In Scope**: Instance 001 — workspace-internal derives; Instance 002 — external package derives.
- **Out of Scope**: Behavioral rationale — see `feature/` and `invariant/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Workspace Derives](001_workspace_derives.md) | Derives from workspace-internal crates (derive_tools_meta, variadic_from, clone_dyn) | ✅ |
| 002 | [External Derives](002_external_derives.md) | Derives from external packages (derive_more, strum, parse-display) | ✅ |
