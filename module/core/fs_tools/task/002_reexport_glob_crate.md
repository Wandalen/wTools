# Re-export glob Crate

**Date**: 2025-11-22
**Priority**: COMPLETED
**Category**: API Enhancement - Re-export
**Status**: ✅ COMPLETED
**Task ID**: 002
**Supersedes**: 001 (different approach taken)

---

## Executive Summary

Re-export the `glob` crate from `fs_tools` to provide Unix shell-style pattern matching functionality. This is a clean re-export approach rather than extracting custom wrapper functions (which was the original task 001 proposal).

---

## Decision Summary

### Why Re-export vs Custom Wrappers?

After analysis of Task 001 (extract glob utilities from wplan), it was determined that:

1. **wplan's wrapper functions are trivial** - `is_glob_pattern()` and `expand_glob_to_directories()` are ~32 lines of straightforward code
2. **glob crate is feature-complete** - 300M+ downloads, battle-tested, comprehensive API
3. **wtools pattern** - Re-exporting tool crates through umbrella modules is established practice in the workspace
4. **Zero maintenance burden** - Re-export requires no ongoing maintenance vs custom wrappers
5. **Full API access** - Users get complete glob functionality, not a limited subset

### Resolution

- Task 001: SUPERSEDED (custom wrappers not needed)
- Task 002: IMPLEMENTED (clean re-export)

---

## Implementation

### Cargo.toml Changes

```toml
[features]
default = [ "enabled" ]
full = [ "enabled", "glob" ]
glob = [ "enabled", "dep:glob" ]

[dependencies]
glob = { workspace = true, optional = true }
```

### lib.rs Changes

```rust
/// Re-export of the glob crate for filesystem pattern matching.
#[ cfg( feature = "glob" ) ]
#[ doc( inline ) ]
pub use ::glob;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  /// Re-export of the glob crate.
  #[ cfg( feature = "glob" ) ]
  #[ doc( inline ) ]
  pub use ::glob;
}
```

### Test Coverage

Created `tests/inc/glob_test.rs` with 9 tests covering:

| ID   | Test                           | Coverage                              |
|------|--------------------------------|---------------------------------------|
| G1.1 | glob_module_accessible         | Module accessibility via fs_tools     |
| G1.2 | glob_types_accessible          | Pattern, MatchOptions, error types    |
| G2.1 | glob_traversal_works           | glob() finds files matching pattern   |
| G2.2 | glob_with_options_works        | glob_with() accepts MatchOptions      |
| G2.3 | pattern_matching_works         | Pattern::matches() validates strings  |
| G2.4 | pattern_with_options_works     | matches_with() respects MatchOptions  |
| G2.5 | pattern_escape_works           | Pattern::escape() escapes special chars|
| G2.6 | recursive_glob_pattern_works   | **/*.rs finds files in subdirectories |
| G3.1 | dependency_namespace_accessible| glob accessible via dependency::glob  |

---

## API Access

### Primary Access

```rust
use fs_tools::glob::glob;

for entry in glob( "*.rs" ).expect( "valid pattern" )
{
  if let Ok( path ) = entry
  {
    println!( "{:?}", path );
  }
}
```

### Via Dependency Namespace

```rust
use fs_tools::dependency::glob;

let pattern = glob::Pattern::new( "*.rs" ).expect( "valid pattern" );
```

### Available Types

- `glob()` - Find files matching pattern
- `glob_with()` - Find files with MatchOptions
- `Pattern` - Compiled glob pattern
- `MatchOptions` - Matching configuration
- `Paths` - Iterator over matching paths
- `GlobError` - Error during iteration
- `PatternError` - Invalid pattern syntax

---

## Design Decisions

### Why Module-Only Re-export?

Original plan included convenience re-exports like `pub use ::glob::glob;` but this would create a name collision - `fs_tools::glob` would be ambiguous (module vs function). Module-only re-export (`pub use ::glob;`) avoids this.

### Why glob Feature Requires enabled?

Follows wtools pattern where utility features depend on the master switch. This ensures consistent behavior and simplifies feature flag logic.

### Why No Custom Wrappers?

1. **glob crate API is already excellent** - No value in wrapping it
2. **Wrapper maintenance cost** - Every glob update would require wrapper updates
3. **Full API access** - Users can use all glob features, not a limited subset
4. **wplan can use glob directly** - No need for fs_tools intermediary for custom wrappers

---

## Files Modified

- `Cargo.toml` - Added glob feature and dependency
- `src/fs/lib.rs` - Added glob re-export and dependency namespace
- `tests/inc/mod.rs` - Added glob_test module
- `tests/inc/glob_test.rs` - Created (9 tests)
- `spec.md` - Updated with glob documentation
- `readme.md` - Updated with glob examples

---

## Verification

All tests pass with `cargo nextest run --all-features`:

```
11 tests run: 11 passed, 0 skipped
```

---

## Impact on Task 001

Task 001 proposed extracting custom wrapper functions from wplan. This task supersedes that approach:

- **Task 001 Goal**: Extract `is_glob_pattern()` and `expand_glob_to_directories()` from wplan
- **Task 002 Resolution**: Provide glob crate re-export

**wplan_client Status (2025-11-23)**: After analysis, wplan_client is ALREADY CORRECT:
- Already uses `glob::glob()` directly
- Custom wrappers provide legitimate wplan-specific value (directory filtering, error handling)
- No migration needed

---

## Lessons Learned

1. **Re-export vs Wrappers**: When underlying crate has comprehensive API, re-export is preferable to custom wrappers. Saves maintenance burden.

2. **Name Collision Avoidance**: Cannot have `fs_tools::glob` as both module AND function. Module-only re-export (`pub use ::glob;`) avoids this.

3. **Feature Dependencies**: Utility features should depend on master switch (`glob = ["enabled", "dep:glob"]`) for consistent behavior.

4. **Analyze Before Migrate**: Always verify target code before proposing migration. wplan_client was already correct.

---

## References

- [glob crate documentation](https://docs.rs/glob)
- [wtools re-export pattern](../../wtools/src/lib.rs)
- [Task 001](./001_extract_glob_utilities_from_wplan.md)
