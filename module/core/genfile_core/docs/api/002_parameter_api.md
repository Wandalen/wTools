# API: Parameter

### Scope

- **Purpose**: Defines the contract for declaring template parameters and assembling them into a collection.
- **Responsibility**: Documents the parameter descriptor and collection API surface available to callers.
- **In Scope**: Parameter name, mandatory flag, default value, description; collection construction and mandatory listing.
- **Out of Scope**: Runtime value assignment (→ `feature/005`), missing-mandatory detection at generation time (→ `feature/015`).

### Design

Callers declare each template parameter by constructing a descriptor with a name, an optional mandatory flag, an optional default value, and an optional description. Descriptors are assembled into a parameter collection. The collection's mandatory-listing operation is used internally during generation to detect missing values; callers typically interact with it only when inspecting parameter metadata.

### Features

| File | Relationship |
|------|--------------|
| [feature/003_parameter_definition.md](../feature/003_parameter_definition.md) | Individual descriptor type being contracted here |
| [feature/004_parameter_collection.md](../feature/004_parameter_collection.md) | Collection type being contracted here |

### Sources

| File | Relationship |
|------|--------------|
| `src/parameter.rs` | Parameter descriptor and collection definitions |
