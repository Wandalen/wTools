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

- Source builder API: empty fields, large data, special characters
- Program builder API: zero/single/three sources, insertion order, duplicates, manifest field
- Plan builder API: minimal plan, complete nested chain, `run_options` stored vs `None`
- Debug trait, namespace accessibility, and explicit parameter handling
- `CapturedOutput` corner cases: default values, lossy UTF-8, empty needle, clone, assertion no-panics
- `RunOptions` corner cases: sentinel defaults, clone independence, Debug formatting
- `CapturedOutput` predicate methods: `exit_ok`, `stdout_eq`, `stdout_contains`, `stderr_contains`
- `run_source`: hello world, compile error, exit code forwarding, stderr capture, multiline stdout
- `run_file`: disk execution, missing source path, invalid Rust compile error
- `run_project`: error handling for missing Cargo.toml
- Timeout enforcement: capture mode and forwarding mode; zero budget fires immediately
- Env var injection: `KEY=VALUE` split, no-`=` ignored, `=` in value preserved
- Cleanup: `cleanup=false` leaves workspace on disk (PID-scoped to avoid concurrent races)
- CLI binary TC-1–TC-13: happy path, project dir, argument errors, exit code forwarding, help flags, `--capture`, `--env`

See `manual/readme.md` for the manual testing plan and `docs/cli/` for CLI test surface specs.
