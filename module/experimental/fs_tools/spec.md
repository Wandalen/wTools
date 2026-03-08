# Specification: fs_tools

## Overview

**fs_tools** is a filesystem utilities crate providing temporary directory path management via the `TempDir` structure and Unix shell-style glob pattern matching via re-export of the `glob` crate. It serves as a foundation for filesystem operations within the wTools workspace.

**Version:** 0.1.0
**Status:** Experimental (Early Development)
**Category:** Utilities (Filesystem)
**Dependents:** Unknown (likely testing utilities)

### Scope

#### Responsibility

Provide filesystem manipulation utilities, currently limited to temporary directory path management through the `TempDir` structure, with plans for future expansion to general file and directory operations.

#### In-Scope

1. **TempDir Structure**
   - Path management for temporary directories
   - Three configurable path components: base, prefix, postfix
   - Simple constructor `new()` with empty defaults
   - Public field access for all path components
   - RAII cleanup for directories created via `create()`/`create_all()`

2. **Path Components**
   - `base_path: PathBuf` - Base directory path
   - `prefix_path: PathBuf` - Prefix path component
   - `postfix_path: PathBuf` - Postfix path component

3. **Directory Operations**
   - `TempDir::new()` - Create instance with empty paths
   - `full_path()` - Construct full path from components
   - `create()` - Create directory (parent must exist)
   - `create_all()` - Create directory and all parents
   - Automatic cleanup on Drop for created directories

4. **Feature Gating**
   - `enabled` - Master switch (default)
   - `glob` - Re-exports glob crate (requires enabled)
   - `full` - Enables all features including glob
   - Requires `not(no_std)` for TempDir functionality
   - Only TempDir requires std library; glob works in std mode

5. **Glob Pattern Matching (via re-export)**
   - `glob::glob()` - Find files matching a pattern
   - `glob::glob_with()` - Pattern matching with options
   - `glob::Pattern` - Compiled glob pattern
   - `glob::MatchOptions` - Matching configuration
   - Error types: `GlobError`, `PatternError`
   - Iterator type: `Paths`

6. **Traditional Module Organization**
   - Nested `fs` module structure
   - Standard namespaces: own, orphan, exposed, prelude
   - Not using mod_interface! (utility crate)

7. **Optional Dependencies**
   - `glob` crate (optional, via `glob` feature)
   - Uses std::path::PathBuf for TempDir
   - Minimal core implementation

#### Out-of-Scope

1. **NOT File Operations**
   - No file reading/writing
   - No file manipulation
   - **Rationale:** Future enhancement

4. **NOT Path Validation**
   - Does not validate paths exist
   - Does not check permissions
   - **Rationale:** Simple data structure only

5. **NOT Temporary File Management**
   - Only directory paths, not files
   - No temporary file creation
   - **Rationale:** Focused on directories

6. **NOT Cross-Platform Abstractions**
   - No platform-specific path handling
   - Relies on std::path::PathBuf semantics
   - **Rationale:** std provides cross-platform support

7. **NOT Atomic Operations**
   - No atomic file operations
   - No locking mechanisms
   - **Rationale:** Out of scope for current implementation

8. **NOT Recursive Operations**
   - No recursive directory traversal
   - No recursive deletion
   - **Rationale:** Not yet implemented

#### Boundaries

- **fs_tools vs std::fs**: fs_tools currently minimal wrapper; std::fs provides actual filesystem operations
- **fs_tools vs tempfile**: fs_tools is path container; tempfile creates actual temp directories with RAII
- **fs_tools vs test utilities**: fs_tools provides structure; test utilities like assert_fs provide testing facilities

## Architecture

### Dependency Structure

```
fs_tools (utilities)
├── Optional Dependencies
│   └── glob (workspace, optional via glob feature)
├── Internal Dependencies
│   └── (none - foundational utility)
└── Dev Dependencies
    └── test_tools (workspace, testing)
```

**Note:** Core functionality has zero dependencies; glob is optional.

### Module Organization

```
fs_tools
├── src/fs/lib.rs (top-level wrapper)
├── src/fs/fs.rs (implementation)
│   └── TempDir struct
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Nested `fs` module with traditional namespace organization

### Feature Architecture

```
enabled (master switch, default)
├── Requires not(no_std) for TempDir
├── Provides TempDir
│
glob (pattern matching)
├── Requires enabled
├── Re-exports glob crate
├── Provides glob(), glob_with(), Pattern, MatchOptions
│
full (all features)
├── enabled
└── glob
│
no_std (minimal mode)
└── use_alloc (unused, would enable allocation in no_std)
```

**Default Features:** `enabled`
**Full Features:** `enabled`, `glob`

**Note:** TempDir disabled in no_std mode; glob requires enabled feature

### TempDir Structure

```
TempDir {
  base_path: PathBuf       (public, empty by default)
  prefix_path: PathBuf     (public, empty by default)
  postfix_path: PathBuf    (public, empty by default)
  created_path: Option<PathBuf>  (private, tracks RAII cleanup)
}
```

**Usage Flow:**
```
TempDir::new()
  ↓
Set base_path, prefix_path, postfix_path
  ↓
Call full_path() to construct: base / prefix / postfix
  ↓
Call create() or create_all() to make directory
  ↓
Automatic cleanup on Drop (only for create()-created directories)
```

**Design Decision:** RAII cleanup only applies to directories created via `create()` or `create_all()`. Manually set paths don't trigger cleanup on Drop.

### Glob Module (Re-export)

The `glob` module is a clean re-export of the external `glob` crate:

```
fs_tools::glob (when feature = "glob")
├── glob() - Find files matching pattern
├── glob_with() - Find files with options
├── Pattern - Compiled glob pattern
├── MatchOptions - Matching configuration
├── Paths - Iterator over matching paths
├── GlobError - Error during iteration
└── PatternError - Invalid pattern error
```

**Access Patterns:**
- `fs_tools::glob::glob()` - Primary access
- `fs_tools::dependency::glob::*` - Via dependency namespace

## Public API

### Structures

```rust
/// Temporary directory management with RAII cleanup.
#[cfg(all(feature = "enabled", not(feature = "no_std")))]
#[derive(Debug)]
pub struct TempDir {
  /// Base directory path
  pub base_path: PathBuf,
  /// Prefix path component
  pub prefix_path: PathBuf,
  /// Postfix path component
  pub postfix_path: PathBuf,
  // Private: tracks path created by create()/create_all() for RAII cleanup
  created_path: Option<PathBuf>,
}
```

### Methods

```rust
impl TempDir {
  /// Creates a new TempDir with empty paths.
  /// No automatic cleanup is enabled until create()/create_all() is called.
  pub fn new() -> Self

  /// Returns full path by joining base, prefix, and postfix components.
  /// Empty components are skipped during joining.
  pub fn full_path( &self ) -> PathBuf

  /// Creates the directory at full_path().
  /// Parent directory must exist. Enables RAII cleanup on Drop.
  pub fn create( &mut self ) -> io::Result<PathBuf>

  /// Creates the directory at full_path() and all parent directories.
  /// Enables RAII cleanup on Drop.
  pub fn create_all( &mut self ) -> io::Result<PathBuf>
}

impl Drop for TempDir {
  /// Removes directory if created by create()/create_all().
  /// Errors during removal are silently ignored.
  fn drop( &mut self )
}
```

### Glob Module (feature = "glob")

```rust
/// Re-export of glob crate for pattern matching
#[cfg(feature = "glob")]
pub use ::glob;

// Key types accessible via fs_tools::glob::
// - glob() - Find files matching a pattern
// - glob_with() - Find files with MatchOptions
// - Pattern - Compiled glob pattern for matching
// - MatchOptions - Configuration for matching behavior
// - Paths - Iterator over matched paths
// - GlobError - Error during path iteration
// - PatternError - Invalid pattern syntax error
```

## Usage Patterns

### Pattern 1: Basic Creation

```rust
use fs_tools::TempDir;

let temp_dir = TempDir::new();
assert!(temp_dir.base_path.as_os_str().is_empty());
assert!(temp_dir.prefix_path.as_os_str().is_empty());
assert!(temp_dir.postfix_path.as_os_str().is_empty());
```

### Pattern 2: Setting Paths

```rust
use fs_tools::TempDir;
use std::path::PathBuf;

let mut temp_dir = TempDir::new();
temp_dir.base_path = PathBuf::from("/tmp");
temp_dir.prefix_path = PathBuf::from("test");
temp_dir.postfix_path = PathBuf::from("run_1");

// Full path would be: /tmp/test/run_1
// (but directory is not created automatically)
```

### Pattern 3: Path Access

```rust
use fs_tools::TempDir;
use std::path::PathBuf;

let mut temp_dir = TempDir::new();
temp_dir.base_path = PathBuf::from("/tmp");

// Access fields directly
let base = &temp_dir.base_path;
let prefix = &temp_dir.prefix_path;
let postfix = &temp_dir.postfix_path;

println!("Base: {:?}", base);
```

### Pattern 4: Directory Creation with RAII Cleanup

```rust
use fs_tools::TempDir;
use std::path::PathBuf;

let mut temp_dir = TempDir::new();
temp_dir.base_path = std::env::temp_dir();
temp_dir.prefix_path = PathBuf::from( "my_app" );
temp_dir.postfix_path = PathBuf::from( "session_1" );

// Create directory (enables RAII cleanup)
let path = temp_dir.create_all().expect( "failed to create" );
assert!( path.is_dir() );

// Use the directory...

// Directory is automatically removed when temp_dir goes out of scope
```

### Pattern 4b: Simple Directory Creation

```rust
use fs_tools::TempDir;
use std::path::PathBuf;

let mut temp_dir = TempDir::new();
temp_dir.base_path = std::env::temp_dir();
temp_dir.postfix_path = PathBuf::from( format!( "test_{}", std::process::id() ) );

// create() requires parent to exist (use create_all() for nested paths)
let path = temp_dir.create().expect( "failed to create" );
assert!( path.is_dir() );

// Automatically cleaned up on drop
```

### Pattern 4c: Using full_path() Without Creation

```rust
use fs_tools::TempDir;
use std::path::PathBuf;

let mut temp_dir = TempDir::new();
temp_dir.base_path = PathBuf::from( "/tmp" );
temp_dir.prefix_path = PathBuf::from( "test" );
temp_dir.postfix_path = PathBuf::from( "run_1" );

// Get composed path without creating directory
let full = temp_dir.full_path();
assert_eq!( full, PathBuf::from( "/tmp/test/run_1" ) );

// No cleanup on drop (create() was never called)
```

### Pattern 5: Basic Glob Usage

```rust
#[ cfg( feature = "glob" ) ]
{
  use fs_tools::glob::glob;

  // Find all Rust files in current directory
  for entry in glob( "*.rs" ).expect( "valid pattern" )
  {
    if let Ok( path ) = entry
    {
      println!( "{:?}", path );
    }
  }
}
```

### Pattern 6: Recursive Glob

```rust
#[ cfg( feature = "glob" ) ]
{
  use fs_tools::glob::glob;

  // Find all Rust files recursively
  for entry in glob( "src/**/*.rs" ).expect( "valid pattern" )
  {
    if let Ok( path ) = entry
    {
      println!( "{:?}", path );
    }
  }
}
```

### Pattern 7: Pattern Matching

```rust
#[ cfg( feature = "glob" ) ]
{
  use fs_tools::glob::Pattern;

  let pattern = Pattern::new( "*.rs" ).expect( "valid pattern" );

  assert!( pattern.matches( "lib.rs" ) );
  assert!( pattern.matches( "main.rs" ) );
  assert!( !pattern.matches( "Cargo.toml" ) );
}
```

### Pattern 8: Glob with Options

```rust
#[ cfg( feature = "glob" ) ]
{
  use fs_tools::glob::{ glob_with, MatchOptions };

  let options = MatchOptions
  {
    case_sensitive : true,
    require_literal_separator : false,
    require_literal_leading_dot : false,
  };

  for entry in glob_with( "/tmp/*.txt", options ).expect( "valid pattern" )
  {
    if let Ok( path ) = entry
    {
      println!( "{:?}", path );
    }
  }
}
```

## Dependencies and Consumers

### Direct Dependencies

**Production (optional):**
- `glob` (workspace, optional via `glob` feature) - Pattern matching

**Dev:**
- `test_tools` (workspace) - Testing utilities

### Consumers (Unknown)

**Likely used by:**
- Test utilities
- Build tools
- Temporary file management in workspace tools

**Usage Pattern:** Currently very limited due to minimal functionality. Consumers likely use TempDir as a path container, managing actual directory creation/deletion themselves.

## Design Rationale

### Why Only Path Management?

TempDir only stores paths without creating directories:

**Current State:** Minimal implementation
**Rationale:**
1. **Early Development**: Crate is v0.1.0, experimental status
2. **Foundation**: Establishes structure for future features
3. **Simplicity**: No dependencies or complex logic yet

**Future Enhancement:** Will likely add directory creation, cleanup, RAII

### Why Public Fields?

All TempDir fields are public:

**Benefits:**
1. **Simplicity**: Easy to set and read
2. **Flexibility**: No builder pattern overhead
3. **Transparency**: Clear what the structure contains

**Tradeoff:** No encapsulation, but acceptable for simple data container

### Why Three Separate Path Fields?

Uses base/prefix/postfix instead of single path:

**Intended Design:**
- **base_path**: Shared temp root (e.g., /tmp)
- **prefix_path**: Test-specific prefix (e.g., test_name)
- **postfix_path**: Unique identifier (e.g., timestamp, UUID)

**Benefit:** Composable paths for test isolation

**Current Issue:** No method to join these paths automatically

### Why No Cleanup?

No Drop implementation for cleanup:

**Rationale:**
1. **Not Implemented**: Early development stage
2. **Safety Concerns**: Need careful handling of filesystem errors
3. **Future Work**: Likely to be added in later versions

**Current Workaround:** Users must manually call `fs::remove_dir_all`

### Why Require std?

Disabled in no_std mode:

**Rationale:**
1. **PathBuf Requirement**: std::path::PathBuf not available in no_std
2. **Filesystem Operations**: Would require std::fs for directory creation
3. **Future Design**: Could add no_std path management with alloc

**Limitation:** Cannot use in embedded environments

### Why Minimal API?

Only provides `new()` constructor:

**Rationale:**
1. **Early Stage**: Crate is experimental, v0.1.0
2. **Iteration**: API will evolve based on usage
3. **No Premature Complexity**: Avoid overdesign

**Future:** Will likely add builder pattern, path joining, creation methods

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for testing
- 24 tests total (with `--features full`):
  - 3 basic TempDir tests
  - 10 TempDir create/RAII tests
  - 9 glob re-export tests
  - 2 smoke tests

### Test Files

```
tests/
├── inc/
│   ├── basic_test.rs - TempDir basic tests (3 tests)
│   ├── tempdir_test.rs - TempDir create/RAII tests (11 tests)
│   ├── glob_test.rs - Glob re-export tests (9 tests, requires glob feature)
│   └── mod.rs - Test module organization
├── smoke_test.rs - Smoke tests
└── tests.rs - Test aggregator
```

### Test Focus

**Test Matrix (from tests/inc/basic_test.rs):**

| ID   | Aspect Tested        | Expected Behavior                     |
|------|----------------------|---------------------------------------|
| T1.1 | TempDir creation     | TempDir::new() returns valid instance |
| T1.2 | Path access          | Can access base_path field            |
| T1.3 | Default values       | Fields initialize to empty PathBuf    |

**Test Matrix (from tests/inc/glob_test.rs):**

| ID   | Aspect Tested             | Expected Behavior                           |
|------|---------------------------|---------------------------------------------|
| G1.1 | Module accessible         | glob module accessible via fs_tools         |
| G1.2 | Types accessible          | Pattern, MatchOptions, errors are usable    |
| G2.1 | Traversal works           | glob() finds files matching pattern         |
| G2.2 | Glob with options         | glob_with() accepts MatchOptions            |
| G2.3 | Pattern matching          | Pattern::matches() validates strings        |
| G2.4 | Pattern with options      | matches_with() respects MatchOptions        |
| G2.5 | Pattern escape            | Pattern::escape() escapes special chars     |
| G2.6 | Recursive pattern         | **/*.rs finds files in subdirectories       |
| G3.1 | Dependency namespace      | glob accessible via dependency::glob        |

**Test Matrix (from tests/inc/tempdir_test.rs):**

| ID   | Aspect Tested                    | Expected Behavior                              |
|------|----------------------------------|------------------------------------------------|
| T2.1 | full_path joins components       | base/prefix/postfix joined correctly           |
| T2.2 | full_path handles empty          | Empty components skipped during join           |
| T2.3 | full_path postfix only           | Works with only postfix set                    |
| T3.1 | create creates directory         | Directory exists after create()                |
| T3.2 | create fails without parent      | Returns error if parent missing                |
| T4.1 | create_all nested directories    | Creates all parent directories                 |
| T4.2 | create_all idempotent            | Succeeds even if directory exists              |
| T5.1 | Drop cleans up created directory | Directory removed after drop                   |
| T5.2 | Drop ignores manual paths        | Manually set paths not removed                 |
| T5.3 | Drop handles already deleted     | No panic if directory already removed          |

### Known Test Limitations

1. **No Cross-Platform Tests**: Doesn't verify Windows/Unix behavior differences
2. **No Permission Tests**: Doesn't test permission denied scenarios
3. **No Concurrent Access Tests**: Doesn't test race conditions

## Future Considerations

### Potential Enhancements (High Priority)

1. ~~**Directory Creation**~~ ✅ IMPLEMENTED
   - `create()` - Create directory (parent must exist)
   - `create_all()` - Create directory with all parents

2. ~~**Automatic Cleanup (RAII)**~~ ✅ IMPLEMENTED
   - Drop implementation removes created directories
   - Only affects directories created via create()/create_all()

3. ~~**Path Construction**~~ ✅ IMPLEMENTED
   - `full_path()` - Join base/prefix/postfix into single path

4. **Builder Pattern** (Future)
   ```rust
   TempDir::builder()
     .base("/tmp")
     .prefix("test")
     .create()?
   ```

5. **UUID/Timestamp Generation**
   ```rust
   impl TempDir {
     pub fn with_timestamp() -> Self
     pub fn with_uuid() -> Self
   }
   ```

6. **File Operations**
   - Read/write files within temp directory
   - Copy files in/out
   - List contents

### Breaking Changes to Consider

1. **Make Fields Private**: Encapsulate with getters/setters
2. **Require Creation**: Make `new()` private, only allow via `create()`
3. **Result Return**: Make constructor fallible
4. **Rename**: More specific name like `TempDirPath` to clarify it's path-only

### Known Limitations

1. **No Path Validation**: Doesn't check if paths are valid before creation
2. **No Platform Handling**: Doesn't handle Windows vs Unix differences explicitly
3. **No Uniqueness**: Doesn't generate unique paths automatically (user must provide)
4. **No File Operations**: Only handles directories, not files within them
5. **No Permission Handling**: Errors on permission denied, no retry/workaround

## Adoption Guidelines

### When to Use fs_tools (Current State)

**Current Recommendation: SUITABLE FOR BASIC USE**

The crate now provides functional temporary directory management:
- TempDir with RAII cleanup
- Directory creation (`create()`, `create_all()`)
- Path composition (`full_path()`)
- Glob pattern matching (via re-export)

**Consider alternatives for:**
- **tempfile**: More mature, production-proven temp directory handling
- **assert_fs**: Testing-focused filesystem utilities
- **std::fs**: General filesystem operations beyond temp directories

**Good for:**
- Test fixtures requiring temporary directories
- Internal wTools utilities
- Simple temporary workspace needs

### When to Use (Future)

**Good Candidates (when implemented):**
- Test fixtures requiring temporary directories
- Build tools needing temporary workspaces
- CLI tools with temporary file storage
- Isolated test environments

**Poor Candidates:**
- Production temporary file management (use tempfile)
- Complex filesystem operations (use walkdir, fs_extra)
- no_std environments (requires std)

### Migration Path

```rust
// Current (working):
let mut td = TempDir::new();
td.base_path = std::env::temp_dir();
td.prefix_path = PathBuf::from( "test" );
td.postfix_path = PathBuf::from( "run_1" );

let path = td.create_all()?;
// ... use path ...
// Automatic cleanup on drop

// Future (with builder pattern):
let td = TempDir::builder()
  .base( std::env::temp_dir() )
  .prefix( "test" )
  .postfix( "run_1" )
  .create()?;
// ... use td.full_path() ...
// Automatic cleanup on drop
```

## Related Crates

- **glob** (re-exported): Unix shell-style pattern matching
- **tempfile**: Full-featured temporary file/directory management (recommended alternative)
- **assert_fs**: Filesystem testing utilities
- **std::fs**: Standard library filesystem operations
- **fs_extra**: Extended filesystem operations (copy, move, etc.)
- **walkdir**: Directory traversal

## References

- [API Documentation](https://docs.rs/fs_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/fs_tools)
- [readme.md](./readme.md)
- [glob crate](https://docs.rs/glob) - Re-exported pattern matching crate
- [tempfile crate](https://docs.rs/tempfile) - Recommended alternative for actual temp directory management
- [std::fs](https://doc.rust-lang.org/std/fs/) - Standard library filesystem operations

## Development Status Note

**Status:** This crate is in development (v0.1.0, experimental status). Current functionality includes:
- **TempDir**: Directory creation with RAII cleanup (`create()`, `create_all()`, `full_path()`)
- **glob**: Full re-export of the glob crate for pattern matching (via `glob` feature)

The specification documents both current state and anticipated future enhancements.

For production use cases requiring advanced features (unique path generation, cross-platform handling, permission management), consider the `tempfile` crate. For basic temporary directory needs within the wTools ecosystem, `fs_tools` is now functional.
