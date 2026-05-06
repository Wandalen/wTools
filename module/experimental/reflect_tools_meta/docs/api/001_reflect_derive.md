# API: Reflect Derive

### Scope

- **Purpose**: Define the invocation contract for the `#[derive(Reflect)]` proc-macro.
- **Responsibility**: Document accepted input forms, optional attributes, and compile-time error conditions.
- **In Scope**: Supported struct varieties, `#[debug]` attribute semantics, feature-gate requirements, error behavior.
- **Out of Scope**: Generated trait implementations (→ `docs/feature/001_reflect_derive.md`); runtime reflection (→ `reflect_tools`).

### Abstract

Provides the `#[derive(Reflect)]` proc-macro that generates trait implementations for struct types. Serves as the implementation backing for the `reflect_tools` frontend crate — not intended for direct consumption. Requires both `enabled` and `reflect_derive` feature flags to be active.

### Operations

**Derive application:**
Accepts any struct definition as input — unit structs, tuple structs (single and multi-field), named-field structs, generic structs with type parameters and bounds, where-clause structs, const generics, lifetime-parameterized structs, and structs with arbitrary outer attributes. Returns generated code for the struct.

**`#[debug]` attribute:**
Optional attribute placed on the struct alongside the derive. When present, outputs the struct name and derive context as diagnostic information during compilation. Does not alter the generated output.

### Error Handling

Returns a compile error at the call site if the derive is applied to a non-struct item. Parsing failure for enums, unions, functions, or any other item kind produces an immediate rejection without code generation.

### Compatibility Guarantees

Experimental status — no stability guarantees. The API surface is controlled by the `reflect_tools` frontend; direct use of this meta crate is unsupported.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Proc-macro export and feature-gating |
| source | `src/implementation/reflect.rs` | Derive logic and debug attribute handling |
| test | `tests/reflect_derive_test.rs` | Compilation tests for all supported struct forms |
| test | `tests/corner_cases_test.rs` | Edge case tests — generics, bounds, attributes, visibility |
| doc | `docs/feature/001_reflect_derive.md` | Feature scope and artifact cross-references |
| doc | `docs/invariant/001_struct_only_input.md` | Struct-only input constraint |
| config | `Cargo.toml` | Feature flags: `enabled`, `reflect_derive` |
