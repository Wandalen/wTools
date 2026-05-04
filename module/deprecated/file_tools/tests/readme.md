# file_tools Test Suite

## Organization Principles

This test suite follows domain-based organization where tests are grouped by functional domain rather than methodology. Tests are organized to ensure comprehensive coverage of filesystem utilities while maintaining environmental independence and loud failure guarantees.

**Governing Standards:**
- Tests organized by functional domain (path operations, TempDir lifecycle, glob integration)
- No mocking - all tests use real filesystem operations
- Environmental independence - no reliance on external state
- Loud failures - all test failures must be explicit and clear

## Directory Structure

```
tests/
├── readme.md              # This file - test organization documentation
├── tests.rs               # Main test aggregator
├── smoke_test.rs          # Minimal functionality validation
├── path_test.rs           # Path traversal and utility tests
└── inc/                   # Core functionality test modules
    ├── mod.rs             # inc/ subdirectory aggregator
    ├── basic_test.rs      # Basic TempDir structure tests
    ├── tempdir_test.rs    # TempDir RAII lifecycle tests
    └── glob_test.rs       # Glob re-export integration tests
```

### Responsibility Table

| File/Directory | Responsibility |
|----------------|----------------|
| `inc/` | Core functionality test modules (TempDir, glob) |
| `path_test.rs` | Path traversal and utility function tests (33 tests) |
| `smoke_test.rs` | Minimal functionality validation (2 smoke tests) |
| `tests.rs` | Main test suite aggregator |

## Domain Map

### TempDir Lifecycle Domain (inc/basic_test.rs, inc/tempdir_test.rs)
**13 tests** covering TempDir structure creation, RAII cleanup semantics, and path manipulation.

**Key Test Categories:**
- Basic structure creation and field access (3 tests)
- RAII cleanup for `create()`/`create_all()` operations (10 tests)
- Path component manipulation and full_path() construction

**Critical Behaviors Validated:**
- Only directories created via `create()`/`create_all()` trigger Drop cleanup
- User-specified base paths are never deleted
- Empty TempDir instances don't attempt cleanup

### Path Module Domain (path_test.rs)
**33 tests** covering generic traversal utilities and filesystem path operations.

**Key Test Categories:**
- Generic `traverse_upward()` functionality with closures
- File discovery in ancestor directories
- Directory discovery in ancestor directories
- Helper function wrappers (file_upward_find, dir_upward_find)
- Boundary conditions (max_depth, missing files, root directory)

**Critical Behaviors Validated:**
- Generic traversal enables complex return types via closure predicates
- Depth limits prevent infinite traversal
- Graceful handling of missing targets and filesystem boundaries

### Glob Integration Domain (inc/glob_test.rs)
**9 tests** covering glob crate re-export and pattern matching functionality.

**Key Test Categories:**
- Module and type accessibility verification
- Pattern compilation and matching
- Recursive glob patterns (`**` syntax)
- Pattern options and configuration
- Glob traversal and iteration

**Critical Behaviors Validated:**
- All glob types properly re-exported (glob, Pattern, MatchOptions, etc.)
- Pattern matching follows Unix shell glob semantics
- Feature-gated compilation (requires `glob` feature)

### Smoke Testing Domain (smoke_test.rs)
**2 tests** providing minimal functionality validation for published and local builds.

### Scope

#### Responsibilities

Organizes all automated tests for filesystem utilities including temporary directory path management (TempDir structure), path traversal utilities, and glob pattern matching re-exports. Ensures comprehensive coverage of RAII lifecycle semantics, generic traversal patterns, and feature-gated conditional compilation.

#### In Scope

- **TempDir lifecycle testing**: Structure creation, RAII cleanup, path manipulation
- **Path module testing**: Generic traversal, ancestor discovery, helper functions
- **Glob integration testing**: Re-export verification, pattern matching, recursive patterns
- **Feature-gated compilation**: Testing `enabled`, `glob`, `full`, `no_std` features
- **RAII semantics validation**: Cleanup triggers, user-path preservation, empty instance handling
- **Boundary condition coverage**: Max depth limits, missing files, filesystem boundaries
- **Smoke testing**: Published and local build validation

#### Out of Scope

- **Performance benchmarking**: See `benches/` directory (when created)
- **Manual exploratory testing**: See `tests/manual/readme.md` (when created)
- **Integration with external tools**: file_tools is standalone utility crate
- **Cross-platform compatibility testing**: Relies on std::path::PathBuf portability
- **Filesystem permission testing**: TempDir is path container, not filesystem operator

## Adding New Tests

### Test Placement Guidelines

**Path Module Tests** → Add to `path_test.rs`
- Generic traversal functionality
- Ancestor discovery operations
- Helper function wrappers

**TempDir Basic Tests** → Add to `inc/basic_test.rs`
- Structure creation and initialization
- Field access and manipulation
- Non-RAII path operations

**TempDir RAII Tests** → Add to `inc/tempdir_test.rs`
- Drop cleanup semantics
- `create()`/`create_all()` operations
- Lifecycle edge cases

**Glob Integration Tests** → Add to `inc/glob_test.rs`
- Re-export verification
- Pattern matching and compilation
- Glob-specific functionality

**Smoke Tests** → Add to `smoke_test.rs`
- Minimal functionality validation
- Quick sanity checks for builds

### Test File Creation Threshold

- **Single domain, <10 tests**: Add to existing file in appropriate domain
- **Single domain, 10+ tests**: Consider splitting into focused test file
- **New domain**: Create new test file, update this readme.md

### Required Test Characteristics

1. **Environmental Independence**: No reliance on env vars, network, external services, or uncontrolled filesystem state
2. **Loud Failures**: Use `assert!`, `assert_eq!`, `panic!` - never silent failures
3. **Real Implementations**: No mocking - use actual filesystem operations via `tempfile` crate in dev-dependencies
4. **Deterministic**: Same inputs always produce same outputs
5. **Self-Contained**: Each test creates and cleans up its own resources

## File Naming Conventions

- **Domain-based naming**: `[domain]_test.rs` (e.g., `path_test.rs`, `tempdir_test.rs`)
- **Avoid methodology naming**: Never `unit_test.rs`, `integration_test.rs`, `regression_test.rs`
- **Lowercase snake_case**: All test files follow `lowercase_snake_case` convention
- **Aggregators**: `tests.rs` for main aggregator, `mod.rs` for subdirectory aggregators

## Special Considerations

### Feature-Gated Testing

Tests must account for feature-gated conditional compilation:
- `glob_test.rs` requires `#[cfg(feature = "glob")]`
- TempDir tests require `#[cfg(all(feature = "enabled", not(feature = "no_std")))]`
- Always run tests with `--all-features` flag to ensure complete coverage

### RAII Cleanup Testing

TempDir Drop implementation requires careful test design:
- Use `tempfile::tempdir()` to create isolated test directories
- Verify cleanup by checking filesystem state after Drop
- Test both successful creation and error paths
- Never rely on specific temp directory locations

### Path Traversal Testing

Path module tests require filesystem fixture setup:
- Create multi-level directory structures for traversal tests
- Test boundary conditions (root directory, max depth exceeded)
- Validate generic return types via closure predicates
- Clean up fixtures in test teardown

## Test Execution

**Standard command:**
```bash
w3 .test l::3
```

**Component commands:**
- **Unit tests**: `RUSTFLAGS="-D warnings" cargo nextest run --all-features`
- **Doc tests**: `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features`
- **Clippy**: `cargo clippy --all-targets --all-features -- -D warnings`

**Test count:** 57 total tests (as of current implementation)
- 33 path module tests (path_test.rs)
- 13 TempDir tests (inc/basic_test.rs + inc/tempdir_test.rs)
- 9 glob integration tests (inc/glob_test.rs)
- 2 smoke tests (smoke_test.rs)
