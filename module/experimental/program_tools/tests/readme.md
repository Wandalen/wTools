# Tests Directory

This directory contains all tests for the program_tools crate, organized by domain and test type per test_organization.rulebook.md standards.

## File Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify crate compiles and basic imports work |
| `tests.rs` | Test suite entry point and module organization |
| `inc/mod.rs` | Test module registration for inc/ directory |
| `inc/basic.rs` | Validate basic builder API with complete chain |
| `inc/corner_cases_test.rs` | Comprehensive edge case validation for all data structures |
| `tool/asset.rs` | Test asset management utilities |
| `asset/err_out_test/err_out_err.rs` | Error output test asset |
| `asset/err_out_test/out_err_out.rs` | Output error test asset |

## Test Organization

Tests are organized following domain-based structure:

- **`inc/`**: Core functionality tests (builder API, data structures)
- **`tool/`**: Test utilities and helpers
- **`asset/`**: Test assets and fixtures
- **`manual/`**: Manual testing plans and documentation

## Running Tests

Execute tests using standard commands:

```bash
# Level 3 verification (recommended)
w3 .test l::3
# or
ctest3

# Individual test file
cargo test --test tests

# Specific test function
cargo test source_empty_file_path
```

## Test Coverage

Current test coverage spans:

- Source builder API (empty fields, large data, special characters)
- Program builder API (zero sources, single source, multiple sources, duplicates)
- Plan builder API (minimal plan, complete nested chain)
- Debug trait implementation for all structs
- Namespace accessibility (exposed and prelude imports)
- Explicit parameter handling

See `manual/readme.md` for comprehensive manual testing plan covering all corner cases.
