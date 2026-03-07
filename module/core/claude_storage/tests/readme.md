# Test Suite Organization

## Overview

The claude_storage test suite uses automated tests for parameter validation and integration testing, with manual tests for exploratory and user experience validation.

## Test Structure

```
tests/
├── readme.md                              # This file - test suite organization
├── common/                                 # Shared test utilities
│   └── mod.rs                             # Pre-compiled binary helper (cargo_bin!)
├── manual/                                 # Manual testing plans and results
│   └── readme.md                          # Manual testing plan for this crate
├── cli_commands.rs                        # CLI command storage operations
├── cli_sanity.rs                          # CLI binary build and run verification
├── command_version_consistency_test.rs    # Command version consistency tests
├── content_display_integration_test.rs    # Content display behavior tests
├── count_command_bug_fix.rs               # .count context-awareness bug fix (Bug #003)
├── export_command_test.rs                 # .export parameter validation tests (Phase 1C)
├── list_smart_session_display.rs          # .list smart session display tests
├── parameter_validation_test.rs           # Multi-command parameter validation tests
├── path_resolution_integration_test.rs    # Path resolution tests
├── path_resolution_test.rs                # Path resolution unit tests
├── project_parameter_bug_fix.rs           # Project parameter parsing tests
├── project_parameter_multi_command_bug.rs # Project parameter across commands (#012)
├── project_parameter_relative_path_bug.rs # Relative path resolution (#013)
├── search_command_test.rs                 # .search parameter validation tests (Phase 1B)
├── search_special_characters_bug.rs       # Special character handling (Bug #006, #007)
├── show_project_command.rs                # .show.project command tests
├── smart_show_command.rs                  # .show smart parameter detection tests
└── status_path_test.rs                    # .status path parameter tests (Phase 1D)
```

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `common/mod.rs` | Pre-compiled binary helper for integration tests |
| `cli_commands.rs` | Test CLI command storage operations |
| `cli_sanity.rs` | Verify CLI binary builds and runs |
| `command_version_consistency_test.rs` | Validate version annotation consistency |
| `content_display_integration_test.rs` | Test content-first display (REQ-011) |
| `count_command_bug_fix.rs` | Test .count context-awareness and path projects |
| `export_command_test.rs` | Validate .export command parameters |
| `list_smart_session_display.rs` | Test smart session display in .list |
| `parameter_validation_test.rs` | Validate CLI parameter handling |
| `path_resolution_integration_test.rs` | Test path resolution in .list command |
| `path_resolution_test.rs` | Test path:: parameter smart detection |
| `project_parameter_bug_fix.rs` | Test project parameter ID resolution |
| `project_parameter_multi_command_bug.rs` | Test project parameter across commands |
| `project_parameter_relative_path_bug.rs` | Test relative path resolution (Finding #013) |
| `search_command_test.rs` | Validate .search command parameters |
| `search_special_characters_bug.rs` | Test special character handling in queries |
| `show_project_command.rs` | Test .show.project command functionality |
| `smart_show_command.rs` | Test location-aware .show command |
| `status_path_test.rs` | Test path parameter in .status command |

## Test Documentation Standards

### Feature Tests (New Commands/Parameters)

Use 4-section Purpose format:

```rust
/// Test {command} {parameter} {validation_type}
///
/// ## Purpose
/// {What this test validates and why it matters}
///
/// ## Coverage
/// {Specific corner case or requirement being tested}
///
/// ## Validation Strategy
/// {How the test verifies behavior - assertions used}
///
/// ## Related Requirements
/// {REQ-NNN from spec.md that this test validates}
#[test]
fn test_{command}_{parameter}_{case}()
```

**Examples**:
- `tests/search_command_test.rs::test_search_query_required`
- `tests/export_command_test.rs::test_export_session_id_required`
- `tests/status_path_test.rs::test_status_custom_path`

### Bug Fix Tests (Finding #NNN)

Use 5-section Root Cause format with Fix comment in source:

```rust
/// Test {command} {parameter} {issue} (Finding #NNN)
///
/// ## Root Cause
/// {Technical explanation of why bug occurred}
///
/// ## Why Not Caught
/// {Gap in existing tests that allowed bug}
///
/// ## Fix Applied
/// {What validation was added}
///
/// ## Prevention
/// {Policy to prevent similar bugs}
///
/// ## Pitfall
/// {Anti-pattern that caused bug}
#[test]
fn test_{command}_{parameter}_{issue}()
```

**Source Code Fix Comment** (3 required fields):
```rust
// Fix(issue-NNN): {One-line description}
//
// Root cause: {Why bug occurred}
//
// Pitfall: {Anti-pattern to avoid}
```

**Example**:
- Test: `tests/search_command_test.rs::test_search_verbosity_invalid`
- Fix comment: `src/cli/mod.rs:1183-1200` (Finding #010)

## Integration Test Strategy

Tests that depend on real storage state or external resources should be marked `#[ignore]`:

```rust
#[test]
#[ignore = "Integration test - depends on actual ~/.claude/ storage state"]
fn test_status_default_path()
```

**Why**:
- Prevents test failures due to environmental factors (corrupted sessions, missing directories)
- Allows tests to be run selectively with `cargo test -- --ignored`
- Separates unit/validation tests from integration tests

**Examples**:
- `tests/status_path_test.rs::test_status_default_path` - depends on ~/.claude/ state
- `tests/search_command_test.rs::test_search_entry_type_valid` - requires real session data
- `tests/export_command_test.rs::test_export_format_valid` - requires real session data

## Test Naming Conventions

```
test_{command}_{parameter}_{scenario}
```

**Examples**:
- `test_search_query_required` - .search command, query parameter, required validation
- `test_export_format_invalid` - .export command, format parameter, invalid value rejection
- `test_status_path_with_verbosity` - .status command, path+verbosity parameters, interaction

## Test Organization Principles

### Command-Specific Files

Each command gets its own test file for parameter validation:
- `search_command_test.rs` - .search parameter validation
- `export_command_test.rs` - .export parameter validation
- `status_path_test.rs` - .status path parameter tests

### Shared Validation Files

Cross-command tests in shared files:
- `parameter_validation_test.rs` - Multi-command parameter validation tests

### Integration Test Files

Feature-specific integration tests:
- `content_display_integration_test.rs` - Content display behavior
- `list_smart_session_display.rs` - Smart session display auto-enable
- `path_resolution_integration_test.rs` - Path resolution with real filesystem

## Test Quality Standards

### Documentation Quality

Test documentation must be:
- **Specific**: Technical details, not generic statements ("Fixed bug" → "search_routine missing verbosity validation")
- **Actionable**: Clear prevention steps ("Don't assume defaults prevent invalid input")
- **Traceable**: Links to requirements (REQ-012), issues (Finding #010), source locations
- **Concise**: Essential information only, no redundancy

### Test Coverage

All parameters must have validation tests:
- Required parameters → test missing parameter error
- Optional parameters → test default value behavior
- Enumerated values → test invalid value rejection
- Ranges → test boundary values and out-of-range rejection
- Booleans → test invalid value rejection (not 0 or 1)

### No Mocking

Tests must use real implementations or be marked `#[ignore]`:
- ✅ Use `TempDir` for real filesystem operations
- ✅ Mark tests requiring real storage as `#[ignore]`
- ❌ Don't mock Storage, Command, or core functionality

## Test Execution Architecture

Integration tests use a pre-compiled binary helper (`common::claude_storage_cmd()`)
instead of `cargo run` to avoid compilation during test execution.

**Why**: Each `cargo run` inside a test triggers a full cargo compilation cycle
(300s+). Under workspace-wide nextest runs, this exceeds the 300s timeout.

**Fix**: `assert_cmd::cargo::cargo_bin!("claude_storage")` resolves to the binary
path built by nextest BEFORE test execution. No recompilation at test time.

**Pattern**: All test files declare `mod common;` and use `common::claude_storage_cmd()`
instead of `Command::new("cargo").args(["run", ...])`.

## Test Verification Commands

```bash
# Run all effective tests (excludes ignored tests)
w3 .test l::3           # Default (recommended)
ctest3                  # Alias for w3 .test l::3

# Run specific test file
cargo nextest run --test search_command_test --all-features

# Run ignored tests only
cargo nextest run --all-features -- --ignored

# Run all tests including ignored
cargo nextest run --all-features -- --include-ignored
```

## Test Count Tracking

**Current Status**: 108 tests total
- 91 effective tests (passing)
- 17 ignored tests (integration tests requiring real storage state)

## Known Findings

### Finding #009: .count target parameter validation
- **Issue**: Missing validation for target parameter (accepted invalid values)
- **Tests**: 4 tests added in `parameter_validation_test.rs`
- **Fix**: Added validation at `src/cli/mod.rs:1151-1157`
- **Documentation**: Fix(issue-009) comment in source

### Finding #010: .search verbosity parameter validation
- **Issue**: search_routine missing verbosity range validation (0-5), inconsistent with other commands
- **Test**: `test_search_verbosity_invalid` in `search_command_test.rs`
- **Fix**: Added validation at `src/cli/mod.rs:1183-1200`
- **Root Cause**: Assumed default values prevent invalid input (they don't)
- **Documentation**: Fix(issue-010) comment in source + 5-section test documentation

### Finding #013: Relative path resolution in project parameter
- **Issue**: parse_project_parameter does not resolve ".", "..", "~" as paths
- **Tests**: 4 tests in `project_parameter_relative_path_bug.rs`
- **Fix**: Added relative path detection before UUID default case
- **Root Cause**: Only handled absolute paths, path-encoded, and UUID; missed relative paths
- **Documentation**: Fix(issue-013) comment in source + 5-section test documentation

### Finding #014: Path resolution in status_routine
- **Issue**: status_routine does not resolve ".", "..", "~" in path parameter
- **Tests**: 2 tests in `status_path_test.rs` (test_status_path_dot_resolves_to_cwd, test_status_path_tilde_resolves_to_home)
- **Fix**: Added resolve_path_parameter() call before Storage::with_root()
- **Root Cause**: status_routine passed path directly without resolving, unlike list_routine
- **Documentation**: Fix(issue-014) comment in source + 5-section test documentation

## Manual Testing

See `tests/manual/readme.md` for manual testing plan and procedures.

## Related Documentation

- **Specification**: `spec.md` - Command specifications and validation requirements
- **Code Design**: See applicable rulebooks via `clm .rulebooks.list`
- **Test Organization**: `test_organization.rulebook.md` - Test documentation format standards
- **Codebase Hygiene**: `codebase_hygiene.rulebook.md` - Quality standards for documentation
