# Invariant: Field Count Boundary

### Scope

- **Purpose**: Ensure that the VariadicFrom derive macro generates no code for structs outside the supported range of 1–3 fields.
- **Responsibility**: Documents the field count boundary invariant — its statement, enforcement mechanism, and consequences.
- **In Scope**: Zero-field and four-or-more-field struct handling by the derive macro.
- **Out of Scope**: The algorithm that enforces this boundary → `algorithm/001`; from! argument count limit → `invariant/002`.

### Invariant Statement

For any struct annotated with VariadicFrom: if the struct has zero fields or more than three fields, the derive macro emits no trait implementations. No From1, From2, From3, or standard From/Into implementations are generated.

### Enforcement Mechanism

The derive macro inspects the struct's field count before any code generation. A dispatch on field count routes to different code paths: zero and four-or-more both produce an empty output token stream. The check runs entirely at compile time within the proc-macro expansion.

### Violation Consequences

This invariant is enforced mechanically — a violation cannot occur at runtime. Without this boundary, structs with unsupported field counts would either receive incorrect implementations or cause proc-macro panics. The invariant preserves a clean, unsurprising API: any struct outside the 1–3 range has no generated FromN implementations, and callers that attempt to use them receive a trait-not-implemented compile error.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [`../../../variadic_from_meta/src/lib.rs`](../../../variadic_from_meta/src/lib.rs) | Proc-macro enforcement: field count dispatch |
| doc | [`../../../variadic_from_meta/docs/api/001_variadic_from_derive.md`](../../../variadic_from_meta/docs/api/001_variadic_from_derive.md) | Derive macro API: boundary handling stated in the accepted struct forms contract |
| doc | [`docs/algorithm/001_variadic_from_derive.md`](../algorithm/001_variadic_from_derive.md) | Derive algorithm describing the enforcement mechanism |
| doc | [`docs/feature/001_variadic_construction.md`](../feature/001_variadic_construction.md) | Feature hub for variadic construction |
| test | [`tests/compile_fail/test_4_fields.rs`](../../tests/compile_fail/test_4_fields.rs) | Confirms no From impl generated for 4-field struct |
| test | [`tests/compile_fail/test_0_fields.rs`](../../tests/compile_fail/test_0_fields.rs) | Confirms no From impl generated for 0-field struct |
