# API: inspect_type_of

### Scope

- **Purpose**: Provide print-mode type inspection — inspect a value's type name and size, emit the result to standard output, and also return it as a string.
- **Responsibility**: Documents the inspect_type_of macro — its relationship to string-mode inspection, print side effect, and compatibility.
- **In Scope**: Single-expression inspection with stdout output and string return.
- **Out of Scope**: Silent inspection without printing (→ api/001_inspect_to_str_type_of.md).

### Abstract

A macro that wraps string-mode inspection with a print side effect. Delegates entirely to inspect_to_str_type_of to build the result string, emits that string to standard output via println, then returns the same string. The output and return value are identical to those of inspect_to_str_type_of — the only difference is the mandatory print. Stable since Rust 1.76.

### Operations

**Inspect and print single expression**: accepts any well-typed expression; delegates evaluation and string construction to inspect_to_str_type_of; prints the resulting string to standard output; returns the string. All type, format, and ownership behaviour is inherited from inspect_to_str_type_of — see api/001_inspect_to_str_type_of.md for the full contract.

### Error Handling

Inherits the same compile-time-only error surface as inspect_to_str_type_of. The print operation uses standard output and cannot fail under normal process conditions.

### Compatibility Guarantees

Same as inspect_to_str_type_of: stable since Rust 1.76, no feature flags, no external runtime dependencies, fixed output format.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Macro definition — delegates to inspect_to_str_type_of then prints |
| test | `tests/corner_cases_test.rs` | Full type coverage across 16 categories |
| test | `tests/example_produces_output_test.rs` | Verifies examples produce expected stdout output |
| doc | `docs/feature/001_type_inspection.md` | End-to-end feature context |
| doc | `docs/api/001_inspect_to_str_type_of.md` | String-mode variant this macro wraps |
| doc | `docs/invariant/002_fixed_output_format.md` | Output format invariant |
