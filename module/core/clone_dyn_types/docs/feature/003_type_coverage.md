# Feature: Type Coverage

### Scope

- **Purpose**: Define which concrete types CloneDyn supports out of the box.
- **Responsibility**: Enumerate all supported type categories and their coverage status.
- **In Scope**: All `Clone` types, slices `[T]`, `str`, tuples (arity 0-12), arrays (all sizes).
- **Out of Scope**: Non-Clone types; types requiring custom unsafe impls; async/pin types.

### Design

`CloneDyn` is implemented via blanket impl for all `T: Clone + 'static` (sized types) and via explicit DST impls for `[T]` and `str`. Tuples and arrays inherit coverage through standard library `Clone` impls. The type support table below summarises the coverage.

| Category | Support | Notes |
|----------|---------|-------|
| Sized `Clone` types | Full | `i32`, `String`, `Vec<T>` — via blanket impl |
| Slices `[T]` | Full | Requires `&&[T]` double-reference for coercion |
| String slices `str` | Full | Requires `&&str` double-reference for coercion |
| Trait objects `dyn Trait` | Full | User implements `Clone for Box<dyn Trait>` |
| Tuples (arity 0-12) | Full | Rust std `Clone` arity limit |
| Arrays `[T; N]` | Full | All sizes via const generics |

The DST coercion requirement (double-reference for slices and strings) is a Rust type system constraint, not a crate limitation. It is documented as a usage constraint to prevent E0277 errors.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../invariant/003_usage_constraints.md` | Constraints on DST coercion and arity limits |
| doc | `002_dst_cloning.md` | The DST cloning capability this type coverage enables |
| source | `../../src/lib.rs` | Blanket and DST CloneDyn impls |
| test | `../../tests/clone_arrays_test.rs` | Array type coverage tests |
