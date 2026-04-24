# Feature: Default Value Type

### Scope

- **Purpose**: Provides a built-in value enum covering common data types without requiring custom implementations.
- **Responsibility**: Documents the `Value` enum and its supported variants.
- **In Scope**: String, Number, Bool, and List variants; their template string conversions.
- **Out of Scope**: Custom value types (→ 001), value storage at runtime (→ 005).

### Design

The library provides a `Value` enum with four variants — String, Number (i64), Bool, and List (Vec<String>) — each implementing the `TemplateValue` trait. This covers the majority of use cases without requiring consumers to define their own value type. Each variant converts to a predictable string format for template substitution.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/value.rs` | `Value` enum definition |
| doc | `docs/feature/001_template_value_trait.md` | Trait that `Value` implements |
| doc | `docs/feature/005_value_storage.md` | Runtime storage for `Value` instances |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR2 in original spec; combined source migrated to feature/ |
