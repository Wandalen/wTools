# Data Structure: Primitive

### Scope

- **Purpose**: Provide a discriminated union representing all supported primitive value types.
- **Responsibility**: Document the Primitive enum variants, their mapping to value types, and usage context.
- **In Scope**: 16 variant catalog, relationship to key-value descriptors, role in reflection output.
- **Out of Scope**: Reflection API operations (→ `api/001_reflection_api.md`); entity reflection design (→ `feature/001_entity_reflection.md`).

### Abstract

An enumeration of 16 variants representing the primitive scalar types that the reflection system can describe. Serves as the value discriminator in key-value descriptors returned by element iteration on reflected containers. Each variant wraps the corresponding value, enabling type-safe extraction without downcasting.

### Structure

Variants: None (absent value), signed integers (i8, i16, i32, i64, isize), unsigned integers (u8, u16, u32, u64, usize), floating point (f32, f64), string types (owned String, borrowed str reference), and binary data.

The enum participates in key-value descriptors where it holds the value half of each key-value pair yielded by container iteration. Pattern matching on the variant discriminator enables callers to extract the concrete value without type erasure overhead.

### Operations

Construction from any supported scalar type via conversion traits. Pattern matching for value extraction. Variant discrimination for type-based dispatch.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/reflect/primitive.rs` | Primitive enum definition and conversion impls |
| source | `src/reflect/axiomatic.rs` | KeyVal descriptor using Primitive as value |
| test | `tests/inc/group1/primitive_test.rs` | Primitive wrapper tests |
| test | `tests/inc/group1/common_test.rs` | Primitive reflection through entity API |
| doc | [`docs/api/001_reflection_api.md`](../api/001_reflection_api.md) | Reflection operations returning Primitive values |
| doc | [`docs/feature/001_entity_reflection.md`](../feature/001_entity_reflection.md) | Entity reflection feature scope |
