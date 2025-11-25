# Extract Glob Pattern Utilities from wplan

**Date**: 2025-11-20
**Priority**: HIGH
**Category**: API Enhancement - Code Extraction
**Status**: ⛔ SUPERSEDED by [Task 002](./002_reexport_glob_crate.md)
**Source**: wplan_client/src/cli/batch_utils.rs
**Task ID**: 001
**Advisability**: 1920 (Value: 8, Easiness: 6, Safety: 8, Priority: 5)
**Resolution**: Clean re-export of glob crate chosen over custom wrapper extraction (2025-11-22)

**⚠️ SUPERSEDED**: After analysis, custom wrapper extraction was deemed unnecessary. The glob crate provides comprehensive functionality and fs_tools now re-exports it directly via `fs_tools::glob`. See [Task 002](./002_reexport_glob_crate.md).

**wplan_client Status (2025-11-23)**: Analyzed and found ALREADY CORRECT. wplan_client uses `glob::glob()` directly. The custom wrappers (`is_glob_pattern`, `expand_glob_to_directories`) provide legitimate wplan-specific value:
- `is_glob_pattern()` - Simple string check not in glob crate
- `expand_glob_to_directories()` - Directory filtering + sorting + error handling
No migration needed - these utilities belong in wplan_client

---

## Executive Summary

Extract glob pattern detection and expansion utilities from the wplan ecosystem to `fs_tools`, making them available to all wTools projects. These utilities detect glob patterns in strings and expand them to directory/file paths - fundamental for any CLI tool that accepts file patterns.

---

## Problem Statement

### Current Location

The wplan codebase contains glob pattern utilities:

**wplan_client/src/cli/batch_utils.rs**:
- Lines 19-23: `is_glob_pattern()` - Detects glob metacharacters
- Lines 48-76: `expand_glob_to_directories()` - Expands glob to directory paths

### Why Extract to fs_tools

1. **Universal Need**: Every CLI tool needs glob pattern handling for file arguments
2. **Foundation**: fs_tools is the natural home for filesystem pattern matching
3. **Code Reuse**: willbe, wtest, benchkit, genfile all need glob expansion
4. **Consistency**: Standardizes glob behavior across wTools
5. **Testing**: Centralizes glob edge case handling

---

## Detailed Functionality Analysis

### 1. Glob Pattern Detection

**Current Location**: `wplan_client/src/cli/batch_utils.rs:19-23`

```rust
pub fn is_glob_pattern( s : &str ) -> bool
{
  s.contains( '*' ) || s.contains( '?' ) || s.contains( '[' )
}
```

**Features**:
- Detects glob metacharacters (`*`, `?`, `[...]`)
- Simple string scanning
- Returns `true` if pattern, `false` if literal path

**Use Cases**:
- Determining whether to expand pattern or use path directly
- Validation before glob expansion
- CLI argument processing

**Enhancement Opportunities**:
- Support for `{a,b,c}` brace expansion
- Escaping detection (`\*` should not be treated as glob)
- Platform-specific path separators

### 2. Glob Expansion to Directories

**Current Location**: `wplan_client/src/cli/batch_utils.rs:48-76`

```rust
pub fn expand_glob_to_directories( pattern : &str ) -> Vec< String >
{
  let mut dirs = Vec::new();

  if let Ok( entries ) = glob::glob( pattern )
  {
    for entry in entries
    {
      if let Ok( path ) = entry
      {
        if path.is_dir()
        {
          dirs.push( path.to_string_lossy().to_string() );
        }
      }
    }
  }

  dirs
}
```

**Features**:
- Uses `glob` crate for pattern matching
- Filters results to directories only
- Returns vector of path strings

**Use Cases**:
- Batch operations on multiple directories
- Test discovery (find all test directories)
- Build systems (find all source directories)

**Enhancement Opportunities**:
- Separate expansion for files vs directories
- Follow symlinks option
- Recursive expansion option
- Sort order control

---

## Proposed API Design

### Target Location

```
fs_tools/src/glob/
  mod.rs           # Module exports
  pattern.rs       # Pattern detection and validation
  expand.rs        # Glob expansion utilities
```

### API Structure

```rust
//! Glob pattern utilities for fs_tools
//!
//! Provides:
//! - Glob pattern detection (`*`, `?`, `[...]`, `{a,b}`)
//! - Glob expansion to files/directories
//! - Pattern validation and escaping

// ============================================================================
// pattern.rs - Pattern Detection
// ============================================================================

/// Check if string contains glob pattern metacharacters.
///
/// Detects:
/// - `*` (wildcard matching zero or more characters)
/// - `?` (wildcard matching exactly one character)
/// - `[...]` (character class)
/// - `{a,b,c}` (brace expansion, if enabled)
///
/// # Example
///
/// ```rust
/// use fs_tools::glob::is_glob_pattern;
///
/// assert_eq!( is_glob_pattern( "*.rs" ), true );
/// assert_eq!( is_glob_pattern( "test_?.txt" ), true );
/// assert_eq!( is_glob_pattern( "[abc].rs" ), true );
/// assert_eq!( is_glob_pattern( "src/**/*.rs" ), true );
/// assert_eq!( is_glob_pattern( "literal.txt" ), false );
/// ```
pub fn is_glob_pattern( s : &str ) -> bool;

/// Check if string contains glob pattern, considering escapes.
///
/// # Example
///
/// ```rust
/// use fs_tools::glob::is_glob_pattern_strict;
///
/// assert_eq!( is_glob_pattern_strict( "*.rs" ), true );
/// assert_eq!( is_glob_pattern_strict( r"\*.rs" ), false );  // Escaped
/// ```
pub fn is_glob_pattern_strict( s : &str ) -> bool;

/// Validate glob pattern syntax.
///
/// # Returns
///
/// - `Ok(())` if pattern is valid
/// - `Err(PatternError)` if pattern has syntax errors
///
/// # Example
///
/// ```rust
/// use fs_tools::glob::validate_pattern;
///
/// assert!( validate_pattern( "*.rs" ).is_ok() );
/// assert!( validate_pattern( "[abc].txt" ).is_ok() );
/// assert!( validate_pattern( "[.txt" ).is_err() );  // Unclosed bracket
/// ```
pub fn validate_pattern( pattern : &str ) -> Result< (), PatternError >;

// ============================================================================
// expand.rs - Glob Expansion
// ============================================================================

use std::path::PathBuf;

/// Options for glob expansion.
#[ derive( Debug, Clone ) ]
pub struct GlobOptions
{
  /// Follow symbolic links.
  pub follow_links : bool,
  /// Case-insensitive matching (default: false).
  pub case_insensitive : bool,
  /// Sort results (default: true).
  pub sort : bool,
  /// Limit number of results (0 = unlimited).
  pub limit : usize,
}

impl Default for GlobOptions
{
  fn default() -> Self
  {
    Self
    {
      follow_links : false,
      case_insensitive : false,
      sort : true,
      limit : 0,
    }
  }
}

/// Expand glob pattern to paths.
///
/// # Returns
///
/// Vector of paths matching the pattern.
///
/// # Example
///
/// ```rust
/// use fs_tools::glob::expand;
///
/// let paths = expand( "src/**/*.rs" )?;
/// for path in paths
/// {
///   println!( "{}", path.display() );
/// }
/// ```
pub fn expand( pattern : &str ) -> Result< Vec< PathBuf >, GlobError >;

/// Expand glob with custom options.
///
/// # Example
///
/// ```rust
/// use fs_tools::glob::{ expand_with, GlobOptions };
///
/// let opts = GlobOptions
/// {
///   follow_links : true,
///   limit : 100,
///   ..Default::default()
/// };
///
/// let paths = expand_with( "**/*.rs", &opts )?;
/// ```
pub fn expand_with( pattern : &str, options : &GlobOptions ) -> Result< Vec< PathBuf >, GlobError >;

/// Expand glob pattern to directories only.
///
/// # Example
///
/// ```rust
/// use fs_tools::glob::expand_dirs;
///
/// let dirs = expand_dirs( "src/*" )?;
/// for dir in dirs
/// {
///   println!( "Directory: {}", dir.display() );
/// }
/// ```
pub fn expand_dirs( pattern : &str ) -> Result< Vec< PathBuf >, GlobError >;

/// Expand glob pattern to files only.
///
/// # Example
///
/// ```rust
/// use fs_tools::glob::expand_files;
///
/// let files = expand_files( "src/*.rs" )?;
/// for file in files
/// {
///   println!( "File: {}", file.display() );
/// }
/// ```
pub fn expand_files( pattern : &str ) -> Result< Vec< PathBuf >, GlobError >;

/// Expand multiple glob patterns.
///
/// # Example
///
/// ```rust
/// use fs_tools::glob::expand_many;
///
/// let patterns = vec![ "src/*.rs", "tests/*.rs" ];
/// let paths = expand_many( &patterns )?;
/// ```
pub fn expand_many( patterns : &[ &str ] ) -> Result< Vec< PathBuf >, GlobError >;

/// Check if glob pattern matches any files.
///
/// # Returns
///
/// - `true` if pattern matches at least one path
/// - `false` if no matches
///
/// # Example
///
/// ```rust
/// use fs_tools::glob::has_matches;
///
/// if has_matches( "*.rs" )?
/// {
///   println!( "Found Rust files" );
/// }
/// ```
pub fn has_matches( pattern : &str ) -> Result< bool, GlobError >;

/// Count number of glob matches without collecting results.
///
/// # Example
///
/// ```rust
/// use fs_tools::glob::count_matches;
///
/// let count = count_matches( "src/**/*.rs" )?;
/// println!( "Found {} Rust files", count );
/// ```
pub fn count_matches( pattern : &str ) -> Result< usize, GlobError >;

// ============================================================================
// Error Types
// ============================================================================

#[ derive( Debug ) ]
pub enum PatternError
{
  UnclosedBracket,
  UnclosedBrace,
  InvalidRange,
  EmptyPattern,
}

#[ derive( Debug ) ]
pub enum GlobError
{
  InvalidPattern( PatternError ),
  IoError( std::io::Error ),
  LimitExceeded( usize ),
}
```

---

## Implementation Phases

### Phase 1: Pattern Detection (1.5 hours)

**Tasks**:
1. Create `fs_tools/src/glob/pattern.rs`
2. Implement `is_glob_pattern()` with all metacharacters
3. Implement `is_glob_pattern_strict()` with escape handling
4. Implement `validate_pattern()` with syntax checking
5. Add comprehensive tests for edge cases
6. Document pattern syntax

**Acceptance Criteria**:
- [ ] All glob metacharacters detected (`*`, `?`, `[...]`)
- [ ] Escape sequences handled correctly (`\*`)
- [ ] Pattern validation catches syntax errors
- [ ] Tests cover edge cases (escaped chars, nested patterns)
- [ ] Documentation explains pattern syntax

### Phase 2: Basic Glob Expansion (2 hours)

**Tasks**:
1. Create `fs_tools/src/glob/expand.rs`
2. Implement `GlobOptions` configuration
3. Implement `expand()` using `glob` crate
4. Implement `expand_with()` with options
5. Implement `expand_dirs()` and `expand_files()` filters
6. Add sorting and limiting logic
7. Add error handling

**Acceptance Criteria**:
- [ ] Basic glob expansion works (`*.rs`, `**/*.txt`)
- [ ] Directory/file filtering works
- [ ] Options control behavior (follow_links, case, sort, limit)
- [ ] Errors handled gracefully (invalid pattern, IO errors)
- [ ] Tests verify all options

### Phase 3: Advanced Features (2 hours)

**Tasks**:
1. Implement `expand_many()` for multiple patterns
2. Implement `has_matches()` for existence check
3. Implement `count_matches()` for counting without collecting
4. Add deduplication for overlapping patterns
5. Add performance optimizations
6. Add integration tests

**Acceptance Criteria**:
- [ ] Multiple patterns expanded correctly
- [ ] Existence check works efficiently
- [ ] Count matches without memory overhead
- [ ] Overlapping patterns deduplicated
- [ ] Performance acceptable for large trees

### Phase 4: Integration and Migration (1 hour)

**Tasks**:
1. Update `fs_tools/src/lib.rs` to export glob module
2. Update `fs_tools/Cargo.toml` dependencies
3. Migrate wplan_client to use new API
4. Delete old implementations
5. Verify all tests pass

**Acceptance Criteria**:
- [ ] All modules exported from `fs_tools::glob`
- [ ] wplan_client uses `fs_tools::glob::*`
- [ ] Old code deleted
- [ ] All tests pass

---

## Dependencies

```toml
# fs_tools/Cargo.toml
[dependencies]
glob = { workspace = true }  # Pattern matching
```

The `glob` crate is already widely used in wTools workspace.

---

## Expected Impact

| Metric | Before | After |
|--------|--------|-------|
| wplan_client batch_utils.rs glob code | ~60 | 0 (deleted) |
| Code duplication | Isolated | Shared |
| Glob consistency | Per-project | Standardized |
| Edge case handling | Per-project | Centralized |
| Test coverage | Minimal | Comprehensive |

---

## Testing Strategy

### Unit Tests

**Pattern Detection**:
- All metacharacters (`*`, `?`, `[...]`, `{a,b}`)
- Escaped metacharacters (`\*`, `\?`)
- Invalid patterns (unclosed brackets, etc.)
- Edge cases (empty string, Unicode)

**Glob Expansion**:
- Simple patterns (`*.txt`)
- Recursive patterns (`**/*.rs`)
- Character classes (`[abc].txt`)
- Directory vs file filtering
- Sorting and limiting
- Symlink following

### Integration Tests

- Real filesystem with known structure
- wplan_client migration tests
- Performance tests on large directory trees

---

## Performance Considerations

**Pattern Detection**:
- O(n) string scan
- No allocations
- Target: <100ns per check

**Glob Expansion**:
- Depends on filesystem size
- Uses streaming iterator from `glob` crate
- Sorting adds O(n log n)
- Target: <10ms for 1000 files

---

## Documentation Requirements

Each module must include:
1. Module-level documentation with pattern syntax
2. Function documentation with examples
3. Glob pattern reference guide
4. Platform-specific behavior notes
5. Performance characteristics

---

## Acceptance Criteria

- [ ] Pattern detection module complete
- [ ] Glob expansion module complete
- [ ] Advanced features implemented
- [ ] Comprehensive test coverage (>90%)
- [ ] Edge cases handled gracefully
- [ ] Documentation complete with examples
- [ ] wplan_client successfully migrated
- [ ] Old implementations deleted
- [ ] `cargo test -p fs_tools` passes
- [ ] `cargo test -p wplan_client` passes

---

## References

**Source Files**:
- `/home/user1/pro/lib/willbe/module/wplan_client/src/cli/batch_utils.rs:19-23` (is_glob_pattern)
- `/home/user1/pro/lib/willbe/module/wplan_client/src/cli/batch_utils.rs:48-76` (expand_glob_to_directories)

**Related Projects**:
- willbe - needs glob expansion for workspace operations
- wtest - needs glob for test discovery
- benchkit - needs glob for benchmark discovery
- genfile - needs glob for template matching

**Dependencies**:
- glob (workspace) - Pattern matching engine

---

## Estimated Effort

- Phase 1: 1.5 hours (pattern detection)
- Phase 2: 2 hours (basic expansion)
- Phase 3: 2 hours (advanced features)
- Phase 4: 1 hour (migration)

**Total**: 6.5 hours

---

## Priority Justification

**HIGH Priority** because:
1. **Universal Need**: Every CLI tool accepts file patterns as arguments
2. **Simple Extraction**: Small, well-defined functionality
3. **Immediate Value**: Eliminates duplication immediately
4. **Low Risk**: Well-tested `glob` crate underneath
5. **Foundation**: Required by many other wTools projects
6. **Quick Win**: Can be completed in less than one day
