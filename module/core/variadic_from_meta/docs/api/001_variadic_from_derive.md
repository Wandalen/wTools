# API: VariadicFrom Derive Macro

### Scope

- **Purpose**: Define the API contract of `#[derive(VariadicFrom)]` — what struct forms it accepts and what implementations it guarantees to generate.
- **Responsibility**: Specifies supported struct kinds, per-field-count generated impl sets, generic propagation rules, and generated path format.
- **In Scope**: Accepted struct forms, generated impl contract, generic propagation, hardcoded crate path requirement.
- **Out of Scope**: Code generation algorithm internals → [`variadic_from/docs/algorithm/001_variadic_from_derive.md`](../../../variadic_from/docs/algorithm/001_variadic_from_derive.md); FromN trait definitions → [`variadic_from/docs/api/001_from_n_traits.md`](../../../variadic_from/docs/api/001_from_n_traits.md).

### Activation

Apply `#[derive(VariadicFrom)]` to a struct. This crate (`variadic_from_meta`) is an internal proc-macro dependency; consumers add `variadic_from` as their direct dependency, which re-exports the derive attribute and all required traits.

### Accepted Input

**Supported struct forms:**
- Named structs with 1–3 fields
- Tuple (unnamed) structs with 1–3 fields
- Generic structs (type parameters, lifetime parameters, const generics, where clauses)

**Unsupported — no code generated, no compile error:**
- Structs with 0 fields
- Structs with 4 or more fields

**Unsupported — compile error emitted:**
- Unit structs
- Enums and all non-struct items

### Generated Impl Contract

**1-field struct** (named or tuple):
- `impl From1<FieldType> for Struct`
- `impl From<FieldType> for Struct` — delegates to `From1::from1`

**2-field struct** (named or tuple):
- `impl From2<T1, T2> for Struct`
- `impl From<(T1, T2)> for Struct` — delegates to `From2::from2`
- `impl From1<T1> for Struct` — generated **only if** both field types are identical

**3-field struct** (named or tuple):
- `impl From3<T1, T2, T3> for Struct`
- `impl From<(T1, T2, T3)> for Struct` — delegates to `From3::from3`
- `impl From1<T1> for Struct` — generated **only if** all three field types are identical
- `impl From2<T1, T2> for Struct` — generated **only if** fields 2 and 3 have identical types

### Generic Propagation

All generated impl blocks propagate the struct's generic parameters verbatim, including the where clause. For a struct `Foo<T, U> where T: Bar`, all generated impls carry `<T, U>` and `where T: Bar`.

### Generated Path Format

Generated impls reference `::variadic_from::exposed::From1`, `::variadic_from::exposed::From2`, and `::variadic_from::exposed::From3` using absolute crate paths. Renaming `variadic_from` via a `package` alias in Cargo.toml will cause the generated code to reference a non-existent path and fail to compile.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Proc-macro entry point and impl generation |
| doc | [`variadic_from/docs/algorithm/001_variadic_from_derive.md`](../../../variadic_from/docs/algorithm/001_variadic_from_derive.md) | Code generation algorithm: phases, type identity check, clone behaviour |
| doc | [`variadic_from/docs/api/001_from_n_traits.md`](../../../variadic_from/docs/api/001_from_n_traits.md) | From1, From2, From3 trait definitions |
| doc | [`variadic_from/docs/feature/001_variadic_construction.md`](../../../variadic_from/docs/feature/001_variadic_construction.md) | User-facing variadic construction feature hub |
| doc | [`variadic_from/docs/invariant/001_field_count_boundary.md`](../../../variadic_from/docs/invariant/001_field_count_boundary.md) | Field count boundary: 0 and 4+ fields generate no code |
| test | `tests/derive_test.rs` | Conformance checks for generated impl correctness |
| test | `tests/smoke_test.rs` | Basic compilation and linking checks |
