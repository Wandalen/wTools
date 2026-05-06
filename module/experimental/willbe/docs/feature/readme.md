# Feature Doc Entity

### Scope

- **Purpose**: Document what willbe capabilities do and how to use them.
- **Responsibility**: Registry and overview of all feature doc instances.
- **In Scope**: Workspace management, multi-crate publishing, CI/CD generation, enhanced publish algorithm.
- **Out of Scope**: API signatures (see `../api/`), architectural patterns (see `../pattern/`). Instance lifecycle governed by `procedure.md`.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Workspace Management](001_workspace_management.md) | Multi-crate publishing, CI/CD generation, health tables, dependency analysis | ✅ |
| 002 | [Enhanced Publish Algorithm](002_enhanced_publish_algorithm.md) | Staleness-aware cascade publish algorithm — planned, not implemented (legacy) | ❌ |
