# API: Fallback Conversion API

### Scope

- **Purpose**: Provide a macro and trait for converting values to strings using a prioritized formatter chain.
- **Responsibility**: Documents the public interface for fallback string conversion — available operations, error behavior, and compatibility policy.
- **In Scope**: The conversion macro, the fallback trait, and the return type semantics.
- **Out of Scope**: Wrapper type implementations (→ api/004), field-level macros (→ api/002), table formatting (→ api/003).

### Abstract

The fallback conversion API converts a value to a string by trying a primary formatting strategy, then falling back to one or two secondary strategies if the primary is unavailable. The result is a borrowed-or-owned string container returned directly to the caller. All dispatch happens at compile time.

### Operations

**Conversion macro**: Accepts a value and an ordered list of two or three strategy markers. Tries the strategies in order. Returns the result of the first applicable strategy as a string container. Takes two or three strategy markers plus the value to convert.

**Fallback trait**: Implemented automatically for any value paired with a compatible strategy chain. The trait method converts the value using the configured strategy and returns a string container. Callers typically invoke the macro rather than calling the trait method directly.

### Error Handling

No runtime errors. If no strategy in the chain applies to the value's type, the compilation fails with a type error. This ensures formatting failures are detected at compile time, not at runtime.

### Compatibility Guarantees

The macro interface is stable. Strategy markers (see api/004) are stable. The return type — a borrowed-or-owned string container — is stable. The trait is public but callers are not expected to implement it manually; the macro handles dispatch automatically.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format/to_string_with_fallback.rs` | Macro and trait implementation |
| source | `src/format/to_string_with_fallback/aref.rs` | Reference adapters |
| source | `src/format/to_string_with_fallback/params.rs` | Type parameter helpers |
| test | `tests/inc/to_string_with_fallback_test.rs` | Primary test suite |
| test | `tests/inc/to_string_with_fallback_corner_cases_test.rs` | Edge cases |
| doc | `docs/feature/001_fallback_string_conversion.md` | Feature description |
| doc | `docs/api/004_wrapper_types_api.md` | Strategy markers used with this API |
| doc | `docs/pattern/001_fallback_chain.md` | Design pattern behind this API |
| doc | `docs/invariant/004_declarative_macros_only.md` | Declarative macro constraint |
