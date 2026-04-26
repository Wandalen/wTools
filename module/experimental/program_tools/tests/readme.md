# Tests Directory

### Scope

All tests for the `program_tools` crate, organized by domain and test type.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify crate compiles and basic imports work |
| `tests.rs` | Test suite entry point and module organization |
| `inc/` | Core builder API and edge case tests (see `inc/readme.md`) |
| `tool/asset.rs` | WIP legacy asset utilities — not compiled into any test suite |
| `asset/err_out_test/err_out_err.rs` | Error output test asset |
| `asset/err_out_test/out_err_out.rs` | Output error test asset |
| `manual/` | Manual testing plans and documentation |

### Test Organization

Tests are organized following domain-based structure:

- **`inc/`**: Core functionality tests (builder API, data structures)
- **`tool/`**: Test utilities and helpers (WIP, not yet wired)
- **`asset/`**: Test assets and fixtures
- **`manual/`**: Manual testing plans and documentation

### Running Tests

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

### Test Coverage

Current test coverage spans:

- Source builder API (empty fields, large data, special characters)
- Program builder API (zero sources, single source, multiple sources, duplicates)
- Plan builder API (minimal plan, complete nested chain)
- Debug trait implementation for all structs
- Namespace accessibility (exposed and prelude imports)
- Explicit parameter handling

See `manual/readme.md` for comprehensive manual testing plan covering all corner cases.
