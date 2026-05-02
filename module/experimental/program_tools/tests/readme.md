# Tests Directory

### Scope

All tests for the `program_tools` crate, organized by domain and test type.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify crate compiles and basic imports work |
| `tests.rs` | Test suite entry point and module organization |
| `inc/` | Builder API, runner integration, and output API tests (see `inc/readme.md`) |
| `manual/` | Manual testing plans and documentation |
| `docs/` | CLI test surface specs: command, param, invariant |

### Test Organization

Tests are organized following domain-based structure:

- **`inc/`**: Core functionality tests (builder API, runner integration, output API)
- **`manual/`**: Manual testing plans and documentation
- **`docs/`**: CLI test surface specs (commands, parameters, invariants)

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
- Program builder API (zero sources, single source, three sources, insertion order, duplicates)
- Plan builder API (minimal plan, complete nested chain)
- Debug trait implementation for all structs
- Namespace accessibility (exposed and prelude imports)
- Explicit parameter handling
- CapturedOutput predicate methods (exit_ok, stdout_eq, stdout_contains, stderr_contains)
- Runner integration: run_source executes inline Rust code via cargo
- Runner integration: run_file reads a file from disk and executes it
- Runner error handling: run_file on missing file returns Err
- Runner error handling: run_project on missing Cargo.toml returns Err

See `manual/readme.md` for the manual testing plan and `docs/cli/` for CLI test surface specs.
