# Tests Directory

Test suite for the wstring_tools alias crate.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| readme.md | Test directory overview and responsibility table |
| smoke_test.rs | Basic compilation and import smoke tests |
| wstring_tools_tests.rs | Re-exports comprehensive strs_tools test suite |
| example_compilation_test.rs | Verifies examples compile and execute correctly |
| split_corner_cases_test.rs | Comprehensive corner case validation for split functionality |
| readme_example_verification_test.rs | Verifies readme.md code examples are functional and accurate |
| manual/ | Manual testing plan and verification procedures |

## Test Strategy

The wstring_tools crate uses a lightweight testing approach:
- **Smoke tests:** Verify basic compilation and import functionality
- **Delegated tests:** Reuse the comprehensive test suite from strs_tools core crate via path import

This strategy avoids test duplication while ensuring the alias crate properly re-exports all strs_tools functionality.
