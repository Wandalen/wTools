# Feature: Variadic Construction

### Scope

- **Purpose**: Enable structs with 1–3 fields to be instantiated from a variable number of positional arguments, reducing constructor boilerplate.
- **Responsibility**: Documents the end-to-end variadic construction capability — design decisions and artifact cross-references.
- **In Scope**: Struct instantiation from 0–3 arguments via from! and FromN; derive-based automation for 1–3 field structs.
- **Out of Scope**: Structs with 0 or >3 fields → `invariant/001`; derive algorithm internals → `algorithm/001`; trait definitions → `api/`.

### Design

Variadic construction solves the problem of repetitive constructor boilerplate. Instead of a separate constructor per argument count, a single derive annotation generates all necessary implementations automatically.

Three interacting components compose the feature:
- **Trait layer**: From1, From2, From3 define the constructor interface per argument count.
- **Macro layer**: from! dispatches to the correct FromN trait based on argument count, or to Default::default() when called with no arguments.
- **Derive layer**: VariadicFrom inspects the struct's field count and generates the appropriate FromN impls, plus convenience lower-arity impls when fields share the same type.

For structs where all fields share the same type, a convenience From1 is generated that constructs all fields from a single value. For 3-field structs where the last two fields share a type, a convenience From2 is also generated.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [`src/variadic.rs`](../../src/variadic.rs) | FromN trait definitions and from! macro |
| source | [`src/lib.rs`](../../src/lib.rs) | Module namespace and re-exports |
| doc | [`docs/api/001_from_n_traits.md`](../api/001_from_n_traits.md) | FromN trait API specification |
| doc | [`docs/api/002_from_macro.md`](../api/002_from_macro.md) | from! macro API specification |
| doc | [`docs/algorithm/001_variadic_from_derive.md`](../algorithm/001_variadic_from_derive.md) | Derive algorithm generating FromN impls |
| doc | [`../../../variadic_from_meta/docs/api/001_variadic_from_derive.md`](../../../variadic_from_meta/docs/api/001_variadic_from_derive.md) | Derive macro API: accepted struct forms and generated impl contract |
| doc | [`docs/invariant/001_field_count_boundary.md`](../invariant/001_field_count_boundary.md) | Field count boundary: 0 or >3 fields generate no code |
| doc | [`docs/invariant/002_compile_time_arg_count.md`](../invariant/002_compile_time_arg_count.md) | Argument count: >3 args produce compile error |
| test | [`tests/variadic_from_tests.rs`](../../tests/variadic_from_tests.rs) | Comprehensive derive and trait tests |
| test | [`tests/compile_fail.rs`](../../tests/compile_fail.rs) | Compile-time error validation |
| test | [`tests/smoke_test.rs`](../../tests/smoke_test.rs) | Basic compilation and functionality check |
