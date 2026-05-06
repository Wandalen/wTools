# Feature: Value Storage

### Scope

- **Purpose**: Provides generic runtime storage for parameter values during template generation.
- **Responsibility**: Documents the value storage container and its insertion and serialization behavior.
- **In Scope**: Key-to-value mapping, conditional insertion, serialization to string map for renderers.
- **Out of Scope**: Value type definitions (→ 001, 002), parameter metadata (→ 003).

### Design

The value storage wraps a map of parameter names to optional values. The insert-if-empty method inserts a value only when the key has no existing entry, enabling default propagation without overwriting explicit values. Serialization converts all values to a string-keyed string map for consumption by template renderers. The generic value type parameter keeps the storage layer independent of specific value types.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/values.rs` | Value storage container implementation |
| test | `tests/` | Value storage insertion and serialization tests |
| doc | `docs/feature/001_template_value_trait.md` | Trait bound required by the value storage container |
| doc | `docs/feature/014_template_generation.md` | Consumes the value storage during generation |
