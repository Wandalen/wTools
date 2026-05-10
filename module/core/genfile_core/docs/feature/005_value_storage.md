# Feature: Value Storage

### Scope

- **Purpose**: Provides generic runtime storage for parameter values during template generation.
- **Responsibility**: Documents the value storage container and its insertion and serialization behavior.
- **In Scope**: Key-to-value mapping, conditional insertion, serialization to string map for renderers.
- **Out of Scope**: Value type definitions (→ 001, 002), parameter metadata (→ 003).

### Design

The value storage wraps a map of parameter names to optional values. The insert-if-empty method inserts a value only when the key has no existing entry, enabling default propagation without overwriting explicit values. Serialization converts all values to a string-keyed string map for consumption by template renderers. The generic value type parameter keeps the storage layer independent of specific value types.

### Features

| File | Relationship |
|------|--------------|
| [`feature/001_template_value_trait.md`](001_template_value_trait.md) | Trait bound required by the value storage container |
| [`feature/014_template_generation.md`](014_template_generation.md) | Consumes the value storage during generation |

### Sources

| File | Relationship |
|------|--------------|
| [`src/values.rs`](../../src/values.rs) | Value storage container implementation |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/inc/values_test.rs`](../../tests/inc/values_test.rs) | Value storage insertion and serialization tests |
