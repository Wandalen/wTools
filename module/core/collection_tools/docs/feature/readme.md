# Feature Doc Entity

### Scope

- **Purpose**: Document what collection_tools capabilities do and when to use them.
- **Responsibility**: Registry and overview of all feature doc instances.
- **In Scope**: Strict constructor macros; into-based constructor macros.
- **Out of Scope**: API signatures (see `api/`), behavioral contracts (see `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Collection Constructors](001_collection_constructors.md) | Strict variadic macros for homogeneous collection initialization | ✅ |
| 002 | [Into Constructors](002_into_constructors.md) | Into-based variadic macros for heterogeneous collection initialization | ✅ |
