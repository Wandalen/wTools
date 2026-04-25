# tests/

Comprehensive test suite for macro_tools crate validation. Tests verify procedural macro utilities, type analysis, generic parameter manipulation, and attribute parsing.

## Organization

Domain-based organization with flat structure for root tests and grouped incremental tests in `inc/` subdirectory. Tests mirror source module structure where logical.

## Test Categories

- **Root tests**: High-level integration and comprehensive coverage tests
- **inc/ tests**: Incremental unit tests per module, one test file per source module
- **Bug reproducers**: Tests marked with `bug_reproducer` attribute preserving specific issue fixes

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `tests.rs` | Aggregate and configure test suite |
| `smoke_test.rs` | Verify basic crate functionality and imports |
| `test_decompose_full_coverage.rs` | Test comprehensive generic parameter decomposition scenarios |
| `test_generic_param_utilities.rs` | Test generic parameter utility functions and edge cases |
| `test_generic_params_no_trailing_commas.rs` | Test generic parameter handling without trailing commas |
| `test_trailing_comma_issue.rs` | Test trailing comma issue reproduction and fix |
| `inc/` | Contain incremental unit tests organized by source module |
