# Feature: Parameter Definition

### Scope

- **Purpose**: Enables defining named template parameters with metadata for validation and documentation.
- **Responsibility**: Documents the `ParameterDescriptor` type and its attributes.
- **In Scope**: Parameter name, mandatory flag, default value, and description attributes; builder pattern.
- **Out of Scope**: Parameter collections (→ 004), runtime value assignment (→ 005).

### Design

A `ParameterDescriptor` stores metadata for a single template parameter: its name, whether it is mandatory, an optional default value, and an optional description. Parameters are constructed via the `former` builder pattern. The mandatory flag drives validation in the collection layer — parameters without values at generation time cause an error when mandatory.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/parameter.rs` | `ParameterDescriptor` struct and builder |
| doc | `docs/feature/004_parameter_collection.md` | Collection containing these descriptors |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR3 in original spec; combined source migrated to feature/ |
