# Feature: Default Value Type

### Scope

- **Purpose**: Provides a built-in value enum covering common data types without requiring custom implementations.
- **Responsibility**: Documents the built-in value type and its supported variants.
- **In Scope**: String, Number, Bool, and List variants; their template string conversions.
- **Out of Scope**: Custom value types (→ 001), value storage at runtime (→ 005).

### Design

The library provides a built-in value type with four variants — String, Number (64-bit integer), Bool, and List (sequence of strings) — each implementing the template value trait. This covers the majority of use cases without requiring consumers to define their own value type. Each variant converts to a predictable string format for template substitution.

### APIs

| File | Relationship |
|------|--------------|
| [api/001_template_value_api.md](../api/001_template_value_api.md) | API contract for the built-in value type |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_template_value_trait.md](001_template_value_trait.md) | Trait that this built-in type implements |
| [feature/005_value_storage.md](005_value_storage.md) | Runtime storage for value instances |

### Sources

| File | Relationship |
|------|--------------|
| `src/value.rs` | Built-in value type definition |

### Tests

| File | Relationship |
|------|--------------|
| `tests/inc/value_test.rs` | Value type variant and conversion tests |
