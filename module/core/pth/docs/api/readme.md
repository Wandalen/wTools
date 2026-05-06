# API Doc Entity

### Scope

- **Purpose**: Document the public API contracts of the `pth` crate.
- **Responsibility**: Collect API doc instances covering free functions, path type newtypes, and conversion traits.
- **In Scope**: Operations, accepted inputs, return types, error conditions, and compatibility guarantees.
- **Out of Scope**: Internal algorithm implementations (see `src/`), user onboarding (see `readme.md`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Path Functions](001_path_functions_api.md) | Normalize, join, query, and transform path strings | ✅ |
| 002 | [Type Wrappers](002_type_wrappers_api.md) | AbsolutePath, NormalizedPath, CurrentPath newtypes | ✅ |
| 003 | [Conversion Traits](003_conversion_traits_api.md) | AsPath, TryIntoPath, TryIntoCowPath trait contracts | ✅ |
