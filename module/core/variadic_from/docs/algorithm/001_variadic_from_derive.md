# Algorithm: VariadicFrom Derive

### Scope

- **Purpose**: Generate FromN trait implementations for a struct based on its field count and field type homogeneity.
- **Responsibility**: Documents the code generation algorithm used by the VariadicFrom derive macro — decision logic, steps, and edge cases.
- **In Scope**: Field analysis, impl selection, convenience impl rules, generic parameter propagation.
- **Out of Scope**: Trait definitions generated → `api/001`; correctness properties → `invariant/001`, `invariant/002`.

### Abstract

The VariadicFrom derive macro processes a struct's field list and produces a set of trait implementations that allow the struct to be constructed from positional arguments. The algorithm produces different outputs depending on the number of fields and whether all or a suffix of the field types are identical. For structs with 0 or more than 3 fields, no code is generated.

### Algorithm

The algorithm proceeds in three phases:

**Phase 1 — Field count dispatch:**
Examine the number of struct fields:
- 0 fields → emit nothing
- 1 field → proceed to phase 2 with arity 1
- 2 fields → proceed to phase 2 with arity 2
- 3 fields → proceed to phase 2 with arity 3
- More than 3 fields → emit nothing

**Phase 2 — Primary impl generation:**
For N fields, generate a FromN implementation matching the field types in declaration order. Also generate a standard tuple-From implementation that delegates to the FromN method. All generic parameters from the struct — type parameters, lifetime parameters, const generics — are propagated into each generated impl block.

**Phase 3 — Convenience impl generation:**
Check for type homogeneity to decide whether to also emit lower-arity convenience impls:
- 2-field struct, both fields same type: generate a From1 that constructs both fields from one argument, cloning the argument for the second field.
- 3-field struct, all three fields same type: generate From1 (one argument cloned to all three fields).
- 3-field struct, last two fields same type: generate From2 (first argument once, second argument cloned to the last two fields).

Type identity for the convenience check uses textual comparison of the type representations. This covers the common case of identical named types but may not equate type aliases that expand to the same underlying type.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [`../../../variadic_from_meta/src/lib.rs`](../../../variadic_from_meta/src/lib.rs) | Proc-macro implementation of the derive algorithm |
| doc | [`docs/api/001_from_n_traits.md`](../api/001_from_n_traits.md) | FromN traits that this algorithm generates implementations for |
| doc | [`docs/invariant/001_field_count_boundary.md`](../invariant/001_field_count_boundary.md) | Field count boundary invariant enforced by phase 1 |
| doc | [`docs/feature/001_variadic_construction.md`](../feature/001_variadic_construction.md) | Feature hub for variadic construction |
| test | [`tests/variadic_from_tests.rs`](../../tests/variadic_from_tests.rs) | Derive behavior tests covering all field counts and type patterns |
