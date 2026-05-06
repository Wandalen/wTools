# Feature Doc Entity

### Scope

- **Purpose**: Provide navigational hubs linking all artifacts for each user-facing `pth` capability.
- **Responsibility**: Collect feature doc instances that cross-reference source files, tests, and design docs.
- **In Scope**: Feature scope, design decisions, cross-references to all related source, test, and doc artifacts.
- **Out of Scope**: Detailed API contracts (see `api/`), invariant proofs (see `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Path Normalization](001_path_normalization.md) | Syntactic path normalization — dots, parent resolution, separator conversion | ✅ |
| 002 | [Path Type System](002_path_type_system.md) | Newtype wrappers encoding path properties at compile time | ✅ |
