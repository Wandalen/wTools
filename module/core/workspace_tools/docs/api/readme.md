# API Doc Entity

### Scope

- **Purpose**: Document the public contract of `workspace_tools` so consumers know what operations are available and under what conditions they succeed or fail.
- **Responsibility**: Define the complete public API surface — all operations, error conditions, and compatibility guarantees — as the authoritative reference for downstream crates.
- **In Scope**: The `Workspace` struct and all its methods, the `WorkspaceError` error type, public traits (`SecretInjectable`, `AsSecure`), the `testing` module, and compatibility guarantees.
- **Out of Scope**: Feature scope rationale (see `feature/`), internal helpers, private types, and test fixture details.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Workspace](001_workspace.md) | Complete public API for the Workspace type and all feature-gated method groups | ✅ |
