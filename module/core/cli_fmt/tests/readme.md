# Tests

This directory contains all automated tests for the cli_fmt crate.

### Scope

**Responsibilities:**
Validates core CLI output processing functionality including stream merging, head/tail filtering, and ANSI-aware width truncation. Tests are organized by functional domain, with comprehensive coverage of OutputConfig behavior, process_output integration, and ANSI preservation.

**In Scope:**
- Unit tests for OutputConfig builder pattern and configuration
- Integration tests for process_output function with various configurations
- Stream selection validation (stdout, stderr, both)
- Head/tail line filtering tests with edge cases
- ANSI-aware width truncation tests
- Bug reproducer tests (e.g., stderr ordering, width boundary detection)

**Out of Scope:**
- Performance benchmarks (would belong in benches/ directory if created)
- Manual testing procedures (none required for this crate currently)
- ANSI escape code generation tests (belongs in strs_tools)
- General string manipulation tests (belongs in strs_tools)

## Responsibility Table

| Entity | Responsibility | Scope | Out of Scope |
|--------|----------------|-------|--------------|
| `readme.md` | Document test organization | Test directory structure, navigation, principles | Test implementation details, actual test code |
| `output.rs` | Validate CLI output processing | OutputConfig tests, stream filtering, head/tail, width truncation, ANSI preservation, bug reproducers | Performance measurement, benchmarks, manual tests |

## Organization Principles

- **Flat Structure**: Single test file until >20 test files, then one-level domain nesting
- **Domain-Based**: Tests organized by functional domain (WHAT tested), not methodology (HOW tested)
- **One Aspect Per Test**: Each test validates single specific aspect of functionality
- **Explicit Parameters**: No fragile tests relying on default values
- **Bug Documentation**: Comprehensive 5-section documentation for bug reproducer tests

## Test Coverage

The test file includes:
- **OutputConfig Tests**: Default configuration, has_processing detection, builder pattern
- **Stream Selection Tests**: stdout-only, stderr-only, both streams, stderr-before-stdout ordering
- **Head Tests**: Truncate to N lines, exceeds total lines behavior
- **Tail Tests**: Last N lines, exceeds total lines behavior
- **Combined Head+Tail Tests**: Combined filtering, overlap handling
- **Width Tests**: No truncation needed, truncation with suffix, zero width handling, ANSI preservation
- **Integration Tests**: Combined operations testing

Total: 31 integration tests + 4 doc tests = 35 tests

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
- Test matrix and bug documentation: See `output.rs` file header (lines 5-47)
- Bug reproducer documentation: See `output.rs` lines 5-27 (width truncation boundary detection)
