# Tests Directory

Test suite for the `variadic_from` crate, verifying derive macro functionality, trait implementations, and compile-time error handling.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify crate compiles and basic functionality works |
| `variadic_from_tests.rs` | Aggregate comprehensive derive macro tests |
| `compile_fail.rs` | Aggregate compile-time error validation tests |
| `inc/` | Included test modules and utilities |
| `compile_fail/` | Compile-fail test cases with expected error output |

### Test Organization

This directory follows domain-based organization:
- **Smoke tests:** Basic compilation and functionality verification
- **Derive tests:** Comprehensive testing of `VariadicFrom` derive macro behavior
- **Compile-fail tests:** Validation of expected compile-time errors

All tests verify the design documented in `../docs/`.
