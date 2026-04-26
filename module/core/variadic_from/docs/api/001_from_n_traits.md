# API: FromN Traits

### Scope

- **Purpose**: Define the N-argument constructor interface for types that support variadic construction.
- **Responsibility**: Documents the From1, From2, and From3 trait API — operations, error behavior, and compatibility guarantees.
- **In Scope**: From1, From2, From3 trait definitions and their single constructor method each.
- **Out of Scope**: Macro dispatch layer → `api/002`; derive automation → `algorithm/001`.

### Abstract

Three traits provide typed constructors for one, two, and three arguments respectively. Each exposes a single static method that constructs the implementing type from the given arguments. The traits are parameterized over the field types, allowing structs with heterogeneous field types to implement each trait independently.

### Operations

Three operations are defined, one per trait:
- **from1(a1)**: Construct from one argument. The type matches the first field type of the implementing struct.
- **from2(a1, a2)**: Construct from two arguments. Types match the first and second field types respectively.
- **from3(a1, a2, a3)**: Construct from three arguments. Types match the first, second, and third field types respectively.

All three operations return the implementing type by value.

### Error Handling

There is no runtime error path. Argument types are enforced at compile time by the trait's generic parameters. Calling a trait method with incompatible argument types results in a type error at compile time.

### Compatibility Guarantees

Traits are additive — a struct may implement any combination of From1, From2, and From3 independently. The standard From and tuple-From blanket implementations delegate to FromN, so adding a FromN implementation automatically provides standard conversion at no additional cost.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [`src/variadic.rs`](../../src/variadic.rs) | Trait definitions for From1, From2, From3 |
| doc | [`docs/api/002_from_macro.md`](002_from_macro.md) | from! macro that dispatches to these traits |
| doc | [`docs/algorithm/001_variadic_from_derive.md`](../algorithm/001_variadic_from_derive.md) | Derive algorithm generating FromN implementations |
| doc | [`../../../variadic_from_meta/docs/api/001_variadic_from_derive.md`](../../../variadic_from_meta/docs/api/001_variadic_from_derive.md) | Derive macro API: accepted struct forms and generated impl contract |
| doc | [`docs/feature/001_variadic_construction.md`](../feature/001_variadic_construction.md) | Feature hub for variadic construction |
| test | [`tests/variadic_from_tests.rs`](../../tests/variadic_from_tests.rs) | FromN trait usage and derive tests |
