# API: inspect_to_str_type_of

### Scope

- **Purpose**: Provide string-mode type inspection — determine a value's type name and size at runtime, returning the formatted result without printing.
- **Responsibility**: Documents the inspect_to_str_type_of macro — its accepted input, returned output, error behaviour, and compatibility guarantees.
- **In Scope**: Single-expression inspection returning a formatted string.
- **Out of Scope**: Print side effects (→ api/002_inspect_type_of.md), batch inspection of multiple expressions.

### Abstract

A macro that accepts any single expression, determines its type name and memory size in bytes at runtime, and returns a formatted string. The expression is captured by reference so the caller retains ownership. The output format is always `sizeof( {expression_text} : {type_name} ) = {size_in_bytes}`. Stable since Rust 1.76 — no feature flags or external dependencies required.

### Operations

**Inspect single expression**: accepts any well-typed expression; evaluates it once; holds the result by reference; constructs and returns a string in the fixed format. The expression text captured in the output is the source text exactly as written at the call site. The type name is the fully qualified runtime type of the referenced value. The size is the number of bytes occupied by the value in memory.

When the expression is already a reference, a reference-to-reference is formed internally; this correctly reports the size and type of the referenced value, not the reference itself.

### Error Handling

No runtime errors. The type name and memory size are determined using stable standard library primitives that cannot panic. Any type mismatch or invalid expression produces a compile-time error — no runtime error path exists.

### Compatibility Guarantees

Stable as of Rust 1.76. No feature flags required. No external runtime dependencies (see invariant/001_zero_dependencies.md). Output format is permanently fixed (see invariant/002_fixed_output_format.md) — callers may safely assert the string format in tests.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Macro definition — single-expression arm |
| test | `tests/corner_cases_test.rs` | Full type coverage across 16 categories including format assertions |
| doc | `docs/feature/001_type_inspection.md` | End-to-end feature context |
| doc | `docs/api/002_inspect_type_of.md` | Print-mode variant wrapping this macro |
| doc | `docs/invariant/002_fixed_output_format.md` | Output format invariant |
