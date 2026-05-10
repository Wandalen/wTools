# Feature: Parameter Definition

### Scope

- **Purpose**: Enables defining named template parameters with metadata for validation and documentation.
- **Responsibility**: Documents the parameter descriptor type and its attributes.
- **In Scope**: Parameter name, mandatory flag, default value, and description attributes; builder pattern.
- **Out of Scope**: Parameter collections (→ 004), runtime value assignment (→ 005).

### Design

A parameter descriptor stores metadata for a single template parameter: its name, whether it is mandatory, an optional default value, and an optional description. Parameters are constructed via the former builder pattern. The mandatory flag drives validation in the collection layer — parameters without values at generation time cause an error when mandatory.

### Features

| File | Relationship |
|------|--------------|
| [`feature/004_parameter_collection.md`](004_parameter_collection.md) | Collection containing these descriptors |

### Sources

| File | Relationship |
|------|--------------|
| [`src/parameter.rs`](../../src/parameter.rs) | Parameter descriptor struct and builder |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/inc/parameter_test.rs`](../../tests/inc/parameter_test.rs) | Parameter descriptor construction and attribute tests |
