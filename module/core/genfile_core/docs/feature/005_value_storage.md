# Feature: Value Storage

### Scope

- **Purpose**: Provides generic runtime storage for parameter values during template generation.
- **Responsibility**: Documents the `Values<V>` container and its insertion and serialization behavior.
- **In Scope**: Key-to-value mapping, conditional insertion, serialization to string map for renderers.
- **Out of Scope**: Value type definitions (→ 001, 002), parameter metadata (→ 003).

### Design

The value storage wraps a map of parameter names to optional values. The insert-if-empty method inserts a value only when the key has no existing entry, enabling default propagation without overwriting explicit values. Serialization converts all values to a string-keyed string map for consumption by template renderers. The generic value type parameter keeps the storage layer independent of specific value types.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/values.rs` | `Values<V>` implementation |
| doc | `docs/feature/001_template_value_trait.md` | Trait bound required by `Values<V>` |
| doc | `docs/feature/014_template_generation.md` | Consumes `Values` during generation |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR5 in original spec; combined source migrated to feature/. spec.md has been deleted — Sources entry retained as migration record. |
