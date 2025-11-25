# unilang_parser Test Suite

## Organization Principles

Tests are organized by **functional domain** (what is being tested) rather than by methodology (unit/integration). This structure mirrors the source code organization in `src/` and makes it intuitive to locate tests for specific functionality.

## Directory Structure

```
tests/
├── readme.md                           # This file
├── inc/                                # Test utilities and shared code
│   └── mod.rs
│
├── argument_parsing_tests.rs           # Argument value parsing and validation
├── command_parsing_tests.rs            # Command path parsing
├── comprehensive_tests.rs              # End-to-end parsing scenarios
├── error_reporting_tests.rs            # Error message clarity and location tracking
├── named_argument_edge_cases_test.rs   # Edge cases for named arguments
├── parser_config_entry_tests.rs        # Parser configuration options
├── spec_adherence_tests.rs             # Specification compliance verification
├── syntactic_analyzer_command_tests.rs # Syntactic analysis of commands
├── tests.rs                            # General test infrastructure
├── underscore_command_test.rs          # Underscore in command names
│
├── argv_multiword_bug_test.rs          # Argv handling edge cases (bug reproduction)
├── diagnostic_real_bug.rs              # ISSUE-CMD-PATH: Command path lookahead (diagnostic + comprehensive)
├── issue_084_mre.rs                    # Issue 084: Quote handling (MRE + comprehensive)
├── path_with_dots_regression_test.rs   # Regression: dot handling in paths
├── task_026_mre_tests.rs               # Task 026: Empty value tokenization
└── -validate_hypothesis.rs             # Hypothesis validation for investigations
```

## Domain Map

Tests are organized into these functional domains:

| Domain | Test Files | What Is Tested |
|--------|------------|----------------|
| **Argument Parsing** | `argument_parsing_tests.rs`, `named_argument_edge_cases_test.rs` | Named arguments, positional arguments, value parsing |
| **Command Path Parsing** | `command_parsing_tests.rs`, `syntactic_analyzer_command_tests.rs`, `underscore_command_test.rs` | Command path segments, dot operators, valid identifiers |
| **Error Handling** | `error_reporting_tests.rs` | Error messages, source location tracking, error clarity |
| **Parser Configuration** | `parser_config_entry_tests.rs` | Parser options, configuration settings |
| **Spec Compliance** | `spec_adherence_tests.rs` | Verification against language specification |
| **Argv Integration** | `argv_multiword_bug_test.rs` | Shell argv parsing and reconstruction |
| **Quote Handling** | `issue_084_mre.rs` | Quote escaping, inner quotes, quote workarounds |
| **Edge Cases & Regressions** | `path_with_dots_regression_test.rs`, `task_026_mre_tests.rs` | Historical bugs, corner cases, boundary conditions |
| **End-to-End** | `comprehensive_tests.rs` | Complete parsing scenarios combining multiple features |

## Test File Categories

### Feature Tests (Primary Domains)
- **argument_parsing_tests.rs:** Core argument value parsing
- **command_parsing_tests.rs:** Core command path parsing
- **error_reporting_tests.rs:** Error handling and messaging
- **spec_adherence_tests.rs:** Specification compliance

### Edge Case & Regression Tests
- **named_argument_edge_cases_test.rs:** Boundary conditions for named args
- **underscore_command_test.rs:** Specific identifier rules
- **path_with_dots_regression_test.rs:** Dot operator edge cases
- **task_026_mre_tests.rs:** Empty value handling

### Bug Reproduction Tests (MRE)
- **diagnostic_real_bug.rs:** ISSUE-CMD-PATH command path lookahead bug (12 tests covering named-only args, operator variants, API consistency)
- **issue_084_mre.rs:** Quote handling in argv (13 tests covering all quote scenarios)
- **argv_multiword_bug_test.rs:** Argv reconstruction edge cases
- **-validate_hypothesis.rs:** Hypothesis validation tests for deep investigations

### Configuration & Integration
- **parser_config_entry_tests.rs:** Parser options and settings
- **comprehensive_tests.rs:** Multi-feature integration scenarios

## Adding New Tests

### Decision Tree

**Q: Testing argument value parsing?**
→ Add to `argument_parsing_tests.rs` (general cases) or `named_argument_edge_cases_test.rs` (edge cases)

**Q: Testing command path parsing?**
→ Add to `command_parsing_tests.rs` (general cases)
→ For named-only arguments (no command path): add to `diagnostic_real_bug.rs`

**Q: Testing error messages or error reporting?**
→ Add to `error_reporting_tests.rs`

**Q: Testing quote handling (shell quotes, escaped quotes)?**
→ Add to `issue_084_mre.rs` (comprehensive quote test suite with full matrix)

**Q: Testing argv reconstruction or shell integration?**
→ Add to `argv_multiword_bug_test.rs`

**Q: Reproducing a specific bug or issue?**
→ Create new file: `issue_XXX_mre.rs` or `task_XXX_mre_tests.rs`
→ Update this readme.md with new entry in Domain Map

**Q: Testing parser configuration options?**
→ Add to `parser_config_entry_tests.rs`

**Q: Testing specification compliance?**
→ Add to `spec_adherence_tests.rs`

**Q: Complex end-to-end scenario combining multiple features?**
→ Add to `comprehensive_tests.rs`

## File Naming Conventions

- **Feature tests:** `{domain}_tests.rs` (e.g., `argument_parsing_tests.rs`)
- **Edge cases:** `{domain}_edge_cases_test.rs` or `{specific_case}_test.rs`
- **Bug reproductions:** `issue_{number}_mre.rs` or `task_{number}_mre_tests.rs`
- **Regressions:** `{description}_regression_test.rs`

**Avoid:** Generic names like `test.rs`, `all_tests.rs`, methodology-based names like `unit_tests.rs`

## Test Organization Standards

All tests follow these standards (per test_organization.rulebook.md):

- ✅ **Domain-based organization** (not methodology-based)
- ✅ **Tests in `tests/` only** (never in `src/`, `examples/`, or workspace root)
- ✅ **File size < 1500 lines** (split larger files)
- ✅ **Descriptive file and function names**
- ✅ **One aspect per test**
- ✅ **Explicit parameters** (no reliance on defaults)
- ✅ **Environmental independence** (no external dependencies)
- ✅ **Loud failures** (clear error messages)
- ✅ **No mocking** (real implementations only)
- ✅ **MRE and bug reproducer preservation** with `// test_kind:` markers

## Test Utilities

**`tests/inc/mod.rs`:** Shared test utilities and helper functions used across multiple test files.

## Manual Testing

**Not applicable** for this crate. All functionality is testable via automated tests.

## Running Tests

```bash
# Run all tests
cargo nextest run --all-features

# Run specific test file
cargo test --test issue_084_mre

# Run with full validation
RUSTFLAGS="-D warnings" cargo nextest run --all-features
```

## Test Matrix Documentation

Each test file should include file-level documentation with:
- Purpose and scope
- Test matrix covering all scenarios
- Corner cases checklist
- References to specifications and issues

See `issue_084_mre.rs` for comprehensive example of test documentation.

## Notes

- **Test count baseline:** 172 tests (as of 2025-11-01, after ISSUE-CMD-PATH fix)
- **Recent additions:**
  - 12 tests in `diagnostic_real_bug.rs` (ISSUE-CMD-PATH coverage)
  - 2 tests in `-validate_hypothesis.rs` (investigation validation)
- **No disabled tests** without explicit permission
- **Bug reproducers** marked with `// test_kind: bug_reproducer(issue-XXX)` or `// test_kind: mre`
- **Specification alignment:** `spec.md` is source of truth for expected behavior
- **Key patterns documented:** Iterator lookahead pattern in `parser_engine.rs` module docs
