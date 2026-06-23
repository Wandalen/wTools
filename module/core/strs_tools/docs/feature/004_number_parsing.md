# Feature: Number Parsing

### Scope

- **Purpose**: Parse numeric strings into typed numbers with support for integer, floating-point, and scientific notation formats beyond the standard library's own conversion.
- **Responsibility**: Documents the number parsing capability and links to its source, tests, and API contract.
- **In Scope**: Integer and floating-point parsing, scientific notation, format leniency, the `lexical` dependency.
- **Out of Scope**: String splitting (`feature/001`); API operation signatures (`api/002`).

### Design

Number parsing wraps the `lexical` library to provide fast, format-flexible numeric conversion. The `lexical` dependency is optional and activated only when the `string_parse_number` feature is enabled; when absent, callers fall back to standard library parsing.

The operation accepts a string slice and attempts to parse it as the target numeric type. Supported formats include standard decimal integers, floating-point with decimal separator, and scientific notation. The result is an owned numeric value or a typed error indicating what went wrong.

Performance is substantially better than the standard library for large volumes of numbers due to SIMD-optimized digit scanning in the underlying `lexical` implementation.

### Sources

- [src/string/number.rs](../../src/string/number.rs) — Number parsing wrapper and format handling

### Tests

- [tests/inc/number_test.rs](../../tests/inc/number_test.rs) — Numeric parsing correctness and format tests

### APIs

- [002_string_utilities_api.md](../api/002_string_utilities_api.md) — Number parsing operation contract

### Invariants

- [004_no_std_alloc_contract.md](../invariant/004_no_std_alloc_contract.md) — No-std compatibility guarantee for core operations
