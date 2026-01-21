# impls_index Tests

Test suite for the impls_index crate, which provides macros for wrapping function definitions and creating explicit function indexes.

## Organization

Tests are organized by functional domain following domain-based organization principles.

### Root Test Files

| File | Responsibility |
|------|----------------|
| smoke_test.rs | Verify basic crate functionality for local and published builds |
| experiment.rs | Exploratory tests for new features and macro variants |
| tests.rs | Main test entry point aggregating inc/ module tests |

### inc/ Subdirectory

Domain-specific test modules organized by functionality:

| File | Responsibility |
|------|----------------|
| mod.rs | Aggregate and re-export all test modules |
| impls1_test.rs | Test basic indexing with impls1 declarative macro |
| impls2_test.rs | Test alternative indexing with impls2 and callback-based expansion |
| impls3_test.rs | Test advanced indexing with impls3 procedural macro |
| impls_basic_test.rs | Test fundamental indexing behavior across all macro variants |
| impls_optional_test.rs | Test optional function indexing (impls_optional, tests_impls_optional) |
| index_test.rs | Test index invocation macro with various syntaxes |
| tests_index_test.rs | Test automatic test attribute injection with tests_impls and tests_index |
| func_test.rs | Test function manipulation utilities (fn_name, fn_rename, fns, fns2) |

## Test Coverage

### Core Functionality
- Basic function indexing (impls1, impls2, impls3)
- Optional function indexing (impls_optional, tests_impls_optional)
- Index invocation with renaming (`as` syntax)
- Function name extraction and manipulation
- Test attribute automatic injection

### Edge Cases
- Empty index invocations (with/without commas, with/without parentheses)
- Visibility preservation (pub vs private functions)
- Function renaming during index expansion
- Optional functions with unused macros (no compile errors)
- Partial index usage (some functions indexed, some not)

### Integration
- Smoke tests verifying published and local builds
- Cross-variant compatibility (impls1, impls2, impls3)
- Experimental features and future enhancements

## Test Execution

Run tests using standard Rust test commands:

```bash
# All tests
cargo nextest run --all-features

# Specific test module
cargo test --test smoke_test
```

## Notes

- All tests use `the_module` alias to verify public API surface
- Experimental tests may be feature-gated or nightly-only
- Compile-time tests (trybuild) are nightly-only due to error message stability
