# Feature: Slice Detection

### Scope

- **Purpose**: Enable callers to determine at the call site whether a given expression is a reference to an unsized contiguous sequence, returning a bool without consuming the expression and without requiring the standard library.
- **Responsibility**: Documents the slice detection feature — its discrimination model, no_std guarantee, non-consuming contract, and all implementing artifacts.
- **In Scope**: Detecting whether an expression is a reference to an unsized slice as opposed to a sized array reference, a Vec, a primitive, or any other type.
- **Out of Scope**: Detecting slices by element type, string slice detection, distinguishing mutable from shared slice references, slice length inspection.

### Design

Detection works by presenting two competing methods to the compiler — one returning true for references to unsized sequences, one returning false for everything else — and relying on compile-time type resolution to select the correct one. The discrimination does not inspect the value at runtime and requires no allocator.

The structural difference that enables this: a reference to an unsized contiguous sequence carries both the data address and the element count, while a reference to a fixed-size array carries only the address. This difference in the type's representation is what compile-time type resolution uses to route to the correct result.

The mechanism requires no allocator, no collections, and no standard library runtime — only foundational type-system primitives. See [invariant/001_no_std.md](../invariant/001_no_std.md).

Note: string slices are intentionally out of scope and return false. The feature targets typed element slices only.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/lib.rs](../../src/lib.rs) | is_slice macro — type-level discrimination mechanism |
| test | [tests/is_slice_tests.rs](../../tests/is_slice_tests.rs) | Test root — delegates to slice_tests |
| test | [tests/inc/slice_tests.rs](../../tests/inc/slice_tests.rs) | 15 cases: variables, literals, arrays, Vec, primitives, Box, empty, nested, struct fields |
| doc | [api/001_is_slice.md](../api/001_is_slice.md) | is_slice macro — accepted inputs and return contract |
| doc | [invariant/001_no_std.md](../invariant/001_no_std.md) | No standard library requirement |
| doc | [invariant/002_value_not_consumed.md](../invariant/002_value_not_consumed.md) | Non-consuming evaluation guarantee |
