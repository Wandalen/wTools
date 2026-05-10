# Feature: Parameter Collection

### Scope

- **Purpose**: Aggregates all parameter descriptors for a template and exposes validation utilities.
- **Responsibility**: Documents the parameter collection type and its methods.
- **In Scope**: Storing multiple descriptors, listing mandatory parameters, validation support.
- **Out of Scope**: Individual parameter metadata (→ 003), runtime value assignment (→ 005).

### Design

The parameter collection wraps a list of parameter descriptors. It provides a method returning the names of all parameters marked mandatory, which the generation layer uses to detect missing values before rendering. The collection supports the builder pattern for construction.

### Features

| File | Relationship |
|------|--------------|
| [`feature/003_parameter_definition.md`](003_parameter_definition.md) | Individual descriptors stored in this collection |
| [`feature/015_missing_mandatory_detection.md`](015_missing_mandatory_detection.md) | Uses the list-mandatory method to detect gaps |

### Sources

| File | Relationship |
|------|--------------|
| [`src/parameter.rs`](../../src/parameter.rs) | Parameter collection implementation |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/inc/parameter_test.rs`](../../tests/inc/parameter_test.rs) | Parameter collection and mandatory listing tests |
