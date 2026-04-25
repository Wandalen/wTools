# Feature: Fallback String Conversion

### Scope

- **Purpose**: Convert arbitrary values to strings using a prioritized chain of formatter strategies, falling back automatically when the preferred strategy is unavailable.
- **Responsibility**: Documents the fallback conversion capability — its design decisions, behavioral contract, and links to all associated artifacts.
- **In Scope**: Primary strategy selection, fallback chain traversal, wrapper type dispatch, zero-copy return semantics.
- **Out of Scope**: Table or field-level formatting (→ feature/002, feature/003), text wrapping (→ feature/004).

### Design

The fallback conversion mechanism lets callers specify a prioritized list of formatting strategies. The primary strategy is attempted first; if the value does not satisfy the required interface, the chain advances to the next strategy automatically at compile time. This eliminates conditional formatting code at the call site.

The chain supports up to three levels: primary, first fallback, and second fallback. Strategy markers are zero-size types — they carry no data but direct the compile-time dispatch to the correct formatting path. The entire mechanism resolves at compile time; the marker types optimize away with no runtime overhead.

The return value is a borrowed-or-owned string container, enabling zero-copy returns when the formatted output can reference existing data without allocation.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format/to_string_with_fallback.rs` | Core fallback conversion logic and macro |
| source | `src/format/to_string_with_fallback/aref.rs` | Reference adapter for fallback chain |
| source | `src/format/to_string_with_fallback/params.rs` | Type parameter definitions |
| source | `src/format/wrapper.rs` | Wrapper type module root |
| source | `src/format/wrapper/aref.rs` | Reference wrapper implementations |
| source | `src/format/wrapper/maybe_as.rs` | Optional value wrappers |
| test | `tests/inc/to_string_with_fallback_test.rs` | Primary test suite for fallback conversion |
| test | `tests/inc/to_string_with_fallback_corner_cases_test.rs` | Edge case coverage |
| test | `tests/inc/to_string_with_fallback_extended_corner_cases_test.rs` | Extended edge case coverage |
| test | `tests/inc/to_string_example.rs` | Usage example tests |
| doc | `docs/api/001_fallback_conversion_api.md` | Public API for fallback conversion |
| doc | `docs/pattern/001_fallback_chain.md` | Fallback chain dispatch pattern |
| doc | `docs/invariant/003_synchronous_only.md` | Synchronous execution constraint |
| doc | `docs/invariant/004_declarative_macros_only.md` | Declarative macro constraint |
