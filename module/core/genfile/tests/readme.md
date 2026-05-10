# genfile Test Suite

### Responsibility Table

| File | Responsibility |
|------|----------------|
| archive_commands_test.rs | Archive lifecycle CLI command integration tests |
| file_commands_test.rs | File management CLI command integration tests |
| param_value_commands_test.rs | Parameter and value CLI command integration tests |
| content_commands_test.rs | Content transformation CLI command integration tests |
| materialization_test.rs | Template materialization and unpack integration tests |
| analysis_test.rs | Archive analysis and inspection integration tests |
| repl_exit_code_bug_test.rs | REPL exit code behavior regression tests |
| help_system_test.rs | Help system tests (FR9 — all deferred pending implementation) |
| invariant_test.rs | Behavioral invariant integration tests (error format, security, coverage) |
| cli_runner.rs | Cross-platform process execution helpers for tests |
| docs/ | Test surface spec files for doc instance coverage verification |
| manual/ | Manual testing plan and procedures |

## Overview

Current test coverage: across 9 test files covering **21 implemented commands** (3 help-system commands deferred — FR9 not yet implemented).

## Test Organization

Tests are organized by functional domain (not by methodology):

- `archive_commands_test.rs` — Archive lifecycle operations
- `file_commands_test.rs` — File management operations
- `param_value_commands_test.rs` — Parameter and value operations
- `content_commands_test.rs` — Content transformation operations
- `materialization_test.rs` — Template materialization and raw unpacking
- `analysis_test.rs` — Archive analysis and inspection
- `repl_exit_code_bug_test.rs` — REPL exit code behavior regression tests
- `invariant_test.rs` — Error format, exit codes, path traversal, coverage checks
- `help_system_test.rs` — Help system placeholder (FR9 deferred)

## Test Methodology

### Integration Testing Approach

**Current:** All tests are **integration tests** that spawn `cargo run` processes to test the complete CLI application end-to-end.

```rust
// Example pattern used throughout test suite (see cli_runner.rs)
let output = cli_runner::cargo_run_command( &[ ".archive.new", "name::test" ] )
  .output()
  .expect( "Failed to execute command" );
```

**Trade-offs:**
- ✅ Tests complete CLI behavior including argument parsing, execution, output formatting
- ✅ Validates real user workflows
- ✅ Catches integration issues between genfile and unilang framework
- ⚠️ Slower execution (spawns processes, compilation overhead)
- ⚠️ No unit test coverage of individual handler functions
- ⚠️ Cannot test error paths without triggering them via CLI
- ⚠️ Harder to test edge cases in isolation

### Missing Test Coverage

**Unit Tests:** None currently exist for:
- Individual handler functions (handlers/archive.rs, handlers/file.rs, etc.)
- Error handling paths in isolation
- State management behavior via shared_state thread-locals

**Unimplemented Commands (3):** No tests for:
- `.help`, `.` (help system - FR9)
- `.command.help` (auto-generated help - FR9)

**Recently Implemented:**
- ✅ `.pack` (FR7) - 4 comprehensive tests covering basic functionality, verbosity, dry run, and error handling
- ✅ `.materialize` (FR6) - 4 comprehensive tests covering template rendering, mandatory parameter validation, dry run, and error handling
- ✅ `.unpack` (FR6) - 3 comprehensive tests covering raw extraction, dry run, and error handling; validates {{}} placeholders preserved
- ✅ `.info` (FR8) - Displays archive metadata and statistics
- ✅ `.discover.parameters` (FR8) - Auto-detects template variables with comprehensive parameter usage analysis
- ✅ `.status` (FR8) - Shows archive readiness and completeness status
- ✅ `.analyze` (FR8) - Comprehensive archive analysis combining all insights

## Test Quality Standards

All tests follow these patterns:

1. **Cleanup First:** Remove temp files before test starts
2. **Real Filesystem:** Use `std::env::temp_dir()` for test data
3. **Explicit Assertions:** Use `assert!` with clear failure messages
4. **Cleanup After:** Remove temp files after test completes

## Running Tests

```bash
# Run all tests (integration only)
cargo test

# Run with nextest (faster)
cargo nextest run

# Run specific test file
cargo test --test archive_commands_test

# Run full test level 3 (includes clippy, doc tests)
w3 .test l::3
```

## Future Improvements

1. **Add Unit Tests:** Create handler-level unit tests for isolated testing
2. **Implement Missing Command Tests:** Add tests for unimplemented commands (use TDD)
3. **Performance Tests:** Consider adding benchmarks for command execution
4. **Manual Testing Plan:** Document manual test scenarios in `tests/manual/readme.md`

## Test File Size Guidelines

- ✅ All test files under 1500 lines
- ✅ Tests organized by domain, not methodology
- ✅ Clear naming: `*_test.rs` suffix

## Known Issues

1. **Integration-Only Coverage:** No unit tests means handler logic cant be tested in isolation
2. **Process Spawning Overhead:** Each test spawns cargo run, adding significant time
3. **Test Isolation:** Thread-local state management means tests must run sequentially in some cases
