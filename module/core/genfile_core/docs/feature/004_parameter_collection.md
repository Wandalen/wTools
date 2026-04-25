# Feature: Parameter Collection

### Scope

- **Purpose**: Aggregates all parameter descriptors for a template and exposes validation utilities.
- **Responsibility**: Documents the parameter collection type and its methods.
- **In Scope**: Storing multiple descriptors, listing mandatory parameters, validation support.
- **Out of Scope**: Individual parameter metadata (→ 003), runtime value assignment (→ 005).

### Design

The parameter collection wraps a list of parameter descriptors. It provides a method returning the names of all parameters marked mandatory, which the generation layer uses to detect missing values before rendering. The collection supports the builder pattern for construction.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/parameter.rs` | Parameter collection implementation |
| test | `tests/` | Parameter collection and mandatory listing tests |
| doc | `docs/feature/003_parameter_definition.md` | Individual descriptors stored in this collection |
| doc | `docs/feature/015_missing_mandatory_detection.md` | Uses the list-mandatory method to detect gaps |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR4 in original spec; combined source migrated to feature/. spec.md has been deleted — Sources entry retained as migration record. |
