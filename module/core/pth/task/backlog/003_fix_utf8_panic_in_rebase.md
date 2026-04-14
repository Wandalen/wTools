# Task 003: Fix UTF-8 Panic in rebase() Function

## Overview

The `rebase()` function at `src/path.rs:920` calls `.to_str().unwrap()` on paths, which panics if the path contains non-UTF-8 bytes. This violates FR-ERR003: "No Panics on Valid UTF-8 Paths" and affects public API stability.

## Status

🔄 Planned

## Priority Metrics

- **Value**: 9/10 (security/stability issue in public API)
- **Easiness**: 7/10 (straightforward fix)
- **Priority**: 5/5 (critical bug)
- **Safety**: 4/5 (needs thorough testing)
- **Advisability**: 1260

## Problem Statement

**Location**: `src/path.rs:920`

**Current Code**:
```rust
let common = path_common( vec![
  file_path.as_ref().to_str().unwrap(),  // PANIC HERE
  old_path.unwrap().as_ref().to_str().unwrap()  // AND HERE
].into_iter() )?;
```

**Impact**:
- Panics on non-UTF-8 paths (valid on Unix systems)
- Up to 5% of paths in international filesystems may contain non-UTF-8
- Violates specification requirement FR-ERR003
- User-facing panic in public API function

## Solution

Replace `.to_str().unwrap()` with `.to_string_lossy()` to handle non-UTF-8 paths gracefully.

**Proposed Fix**:
```rust
let common = path_common( vec![
  file_path.as_ref().to_string_lossy().as_ref(),
  old_path.unwrap().as_ref().to_string_lossy().as_ref()
].into_iter() )?;
```

## Acceptance Criteria

- [ ] No `.to_str().unwrap()` calls in `rebase()` function
- [ ] Function handles non-UTF-8 paths without panicking
- [ ] All existing tests pass
- [ ] New test added demonstrating non-UTF-8 path handling
- [ ] Documented behavior for lossy conversion in rustdoc

## Estimated Effort

1 hour (fix + test + documentation)

## Target Milestone

v0.29.0 (P0 - CRITICAL)

## Related Issues

- Discovery issue 2.1: UTF-8 Panic in rebase()
- Discovery issue 5.2: No Tests for UTF-8 Panic Scenarios
- Spec FR-ERR003: No Panics on Valid UTF-8 Paths
- Future Task: UTF-8 Fixes for v0.30.0 (comprehensive audit)

## Implementation Notes

### Additional Investigation Needed

Search codebase for other `.to_str().unwrap()` occurrences that may have same issue:
```bash
grep -n "to_str()\.unwrap()" src/**/*.rs
```

### Testing Strategy

Create test with non-UTF-8 path (Unix-specific):
```rust
#[cfg(unix)]
#[test]
fn test_rebase_non_utf8_path() {
  use std::os::unix::ffi::OsStrExt;
  use std::ffi::OsStr;

  // Create non-UTF-8 path
  let invalid_utf8 = OsStr::from_bytes(&[0xFF, 0xFE]);
  // Test that rebase doesn't panic
}
```

### Documentation Update

Add to `rebase()` rustdoc:
```rust
/// **UTF-8 Handling**: This function uses lossy UTF-8 conversion for paths.
/// Non-UTF-8 bytes will be replaced with � (U+FFFD REPLACEMENT CHARACTER).
```
