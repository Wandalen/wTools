# Feature: Slice Detection

### Scope

- **Purpose**: Enable callers to determine at the call site whether a given expression is a reference to an unsized contiguous sequence, returning a bool without consuming the expression and without requiring the standard library.
- **Responsibility**: Documents the slice detection feature — its discrimination model, no_std guarantee, non-consuming contract, and all implementing artifacts.
- **In Scope**: Detecting whether an expression is a reference to an unsized slice as opposed to a sized array reference, a Vec, a primitive, or any other type.
- **Out of Scope**: Detecting slices by element type, string slice detection, distinguishing mutable from shared slice references, slice length inspection.

### Design

The detection distinguishes a reference to an unsized slice from all other types using double-ref autoref specialization: the macro wraps the expression in a non-consuming reference, constructs a phantom value from that reference, and resolves a method whose return value is true only when the phantom type matches the specific double-ref pattern formed by a slice reference. Sized array references form a different phantom type and fall through to the false branch.

The key distinction: a reference to an unsized slice is a fat pointer carrying both the data address and the element count. A reference to a sized array is a thin pointer. The phantom type construction exploits this: a slice reference produces the specific double-indirection pattern that the true branch matches, while an array reference does not.

The crate requires only the core library. No standard library allocator or collections are needed — the mechanism is purely based on zero-sized phantom type arithmetic.

Note: string slices are intentionally out of scope and return false. The feature targets typed element slices only.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | is_slice macro — autoref specialization mechanism |
| test | `tests/is_slice_tests.rs` | Test root — delegates to slice_tests |
| test | `tests/inc/slice_tests.rs` | 13 cases: variables, literals, arrays, Vec, primitives, Box, empty, nested, struct fields |
| doc | `docs/api/001_is_slice.md` | is_slice macro — accepted inputs and return contract |
| doc | `docs/invariant/001_no_std.md` | No standard library requirement |
| doc | `docs/invariant/002_value_not_consumed.md` | Non-consuming evaluation guarantee |
