# Tests

This directory contains all automated tests for the cli_fmt crate.

## Scope

**Responsibilities:**
Validates CLI output processing and help template rendering. Tests are organized by functional domain, with comprehensive coverage of OutputConfig behavior, process_output integration, ANSI preservation, and CliHelpTemplate column alignment and section omission. Test spec documents in `docs/` map each doc entity instance to its test cases.

**In Scope:**
- Unit tests for OutputConfig builder pattern and configuration
- Integration tests for process_output function with various configurations
- Stream selection validation (stdout, stderr, both)
- Head/tail line filtering tests with edge cases
- ANSI-aware width truncation tests
- Bug reproducer tests (e.g., stderr ordering, width boundary detection)
- CliHelpTemplate rendering tests (column alignment, ANSI suppression, section omission, desc annotation)
- Test spec documents mapping doc entity instances to test cases (`docs/`)

**Out of Scope:**
- Performance benchmarks (would belong in benches/ directory if created)
- Manual testing procedures (none required for this crate currently)
- ANSI escape code generation tests (belongs in strs_tools)
- General string manipulation tests (belongs in strs_tools)

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `readme.md` | Document test organization and navigation |
| `output.rs` | Validate CLI output processing behavior |
| `help.rs` | Validate CliHelpTemplate rendering and CliHelpStyle defaults |
| `docs/` | Test specification documents for doc entity instances |

## Organization Principles

- **Flat Structure**: Single test file until >20 test files, then one-level domain nesting
- **Domain-Based**: Tests organized by functional domain (WHAT tested), not methodology (HOW tested)
- **One Aspect Per Test**: Each test validates single specific aspect of functionality
- **Explicit Parameters**: No fragile tests relying on default values
- **Bug Documentation**: Comprehensive 5-section documentation for bug reproducer tests

## Test Coverage

`output.rs` (33 tests):
- **OutputConfig Tests**: Default configuration, has_processing detection, builder pattern
- **Stream Selection Tests**: stdout-only, stderr-only, both streams, stderr-before-stdout ordering
- **Head Tests**: Truncate to N lines, exceeds total lines behavior
- **Tail Tests**: Last N lines, exceeds total lines behavior
- **Combined Head+Tail Tests**: Combined filtering, overlap handling
- **Width Tests**: No truncation needed, truncation with suffix, zero width handling, ANSI preservation, exact boundary (`len == max_width`)
- **Integration Tests**: Combined operations testing, `lines_omitted` correctness via `process_output`

`help.rs` (9 tests):
- **T01** Column alignment: cmd/opt names padded to configured widths, no ANSI in no-TTY mode
- **T02** No ANSI codes: `tty_detect=false` suppresses all escape sequences
- **T03** Explicit `tty_detect=false`: equivalent behavior to T01
- **T04** Name not truncated: `cmd_name_width` is minimum padding, not a hard cap
- **T05** No Options section: omitted when `options` vec is empty
- **T06** No Examples section: omitted when `examples` vec is empty
- **T07** Single group: binary, group header, and command name all appear
- **T08** `CliHelpStyle::default()` field values match reference implementation layout
- **T09** `ExampleEntry.desc` rendered: `Some` appends `# text`; `None` emits no `#` (bug reproducer)

Total: 42 integration tests + 4 doc tests = 46 tests

## Test Execution

```bash
# Run all tests
cargo test

# Run with level 3 verification (recommended)
w3 .test l::3
# OR
ctest3

# Run specific test file
cargo test --test output
```

## Navigation

- CLI output processing tests: `output.rs`
- CLI help template tests: `help.rs`
- Test spec documents (doc entity → test case mapping): `docs/`
- Test matrix and bug documentation: See `output.rs` file header (lines 5-57)
- Bug reproducer documentation: See `output.rs` lines 5-35 (width truncation boundary detection)
- Help test matrix: See `help.rs` file header (lines 8-18)
