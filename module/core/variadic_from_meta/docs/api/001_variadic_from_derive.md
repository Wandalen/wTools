# API: VariadicFrom Derive Macro

### Scope

- **Purpose**: Define the API contract of `#[derive(VariadicFrom)]` — what struct forms it accepts and what implementations it guarantees to generate.
- **Responsibility**: Specifies supported struct kinds, per-field-count generated impl sets, generic propagation rules, and generated path format.
- **In Scope**: Accepted struct forms, generated impl contract, generic propagation, hardcoded crate path requirement.
- **Out of Scope**: Code generation algorithm internals → [`variadic_from/docs/algorithm/001_variadic_from_derive.md`](../../../variadic_from/docs/algorithm/001_variadic_from_derive.md); FromN trait definitions → [`variadic_from/docs/api/001_from_n_traits.md`](../../../variadic_from/docs/api/001_from_n_traits.md).

### Abstract

Derives compile-time implementations of the `From1`, `From2`, and `From3` conversion traits for structs with 1–3 fields. The macro examines field count and field types at compile time; all generated code is inlined with no runtime overhead. Consuming crates access this functionality through `variadic_from`, which re-exports the derive attribute and all required traits.

### Operations

Apply `#[derive(VariadicFrom)]` to a named or tuple struct. Consuming crates use `variadic_from` as their direct dependency; `variadic_from_meta` is an internal proc-macro dependency that consumers never reference directly.

**Accepted struct forms:**
- Named structs with 1–3 fields
- Tuple (unnamed) structs with 1–3 fields
- Generic structs (type parameters, lifetime parameters, const generics, where clauses)

**Unsupported — no implementations generated, no error:**
- Structs with 0 fields
- Structs with 4 or more fields

**Unsupported — compile error emitted:**
- Unit structs
- Enums and all non-struct items

**Generated implementations by field count:**

1-field struct (named or tuple):
- Implements `From1` for the field type
- Implements `From` (standard conversion), delegating to `From1`

2-field struct (named or tuple):
- Implements `From2` for the pair of field types
- Implements `From` for a 2-element tuple, delegating to `From2`
- Implements `From1` — only when both field types are identical

3-field struct (named or tuple):
- Implements `From3` for the triple of field types
- Implements `From` for a 3-element tuple, delegating to `From3`
- Implements `From1` — only when all three field types are identical
- Implements `From2` — only when the second and third field types are identical

### Generic Propagation

All generated implementations carry the struct's full type parameter list and where clause verbatim. No bounds are added, tightened, or removed — every generated implementation is parameterized identically to the struct definition.

### Generated Path Format

Generated implementations reference the `From1`, `From2`, and `From3` traits through hardcoded absolute paths resolving through the `variadic_from::exposed` module. Renaming the `variadic_from` crate via a Cargo.toml `package` alias makes those paths unresolvable, causing compile errors in consumer crates.

### Error Handling

All errors are compile-time diagnostics. No runtime errors are produced.

**Unsupported item kind** — applying the derive to a unit struct, an enum, or any non-struct item emits a compile error at the attribute site with a descriptive message.

**Incorrect crate path** — if `variadic_from` is renamed via a Cargo.toml `package` alias, the generated absolute crate path becomes invalid and the consumer crate fails to compile with a path-not-found error.

**Unsupported field count** — structs with zero or four-or-more fields compile without error but receive no generated implementations.

### Compatibility Guarantees

The generated impl contract (which field-count combinations produce which impl sets) is stable across patch and minor versions. Changes to generated behavior require a major version bump.

This crate is an internal proc-macro dependency. Consumers depend on `variadic_from`, which re-exports the derive and provides the stable public interface.

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
