# Task 007: Rename canonicalize() to normalize_unchecked()

## Overview

Rename the misleading `canonicalize()` function to `normalize_unchecked()` or similar to avoid confusion with `std::fs::canonicalize()`. Current function performs syntactic normalization only, not filesystem canonicalization.

## Status

🔄 Planned

## Priority Metrics

- **Value**: 7/10 (API clarity, prevents user confusion)
- **Easiness**: 5/10 (breaking change, needs careful migration)
- **Priority**: 4/5 (breaking change window closing)
- **Safety**: 3/5 (breaking change risk)
- **Advisability**: 420

## Problem Statement

**Location**: `src/path.rs:239`

**Current Documentation** (from function):
```rust
/// ⚠️ **Misleading Name**: This does NOT perform filesystem canonicalization like `std::fs::canonicalize()`.
///
/// **Actual Behavior**:
/// - Calls `normalize()` (syntactic only)
/// - Strips Windows `\\?\` verbatim prefix if present
/// - Returns `Ok(PathBuf)` always (no actual I/O despite Result type)
pub fn canonicalize( path: impl AsRef< std ::path ::Path > ) -> std ::io ::Result< std ::path ::PathBuf >
```

**From Spec** (section 10.1 - High Priority):
> **1. Rename `canonicalize()`** to `normalize_unchecked()` or similar
> - Current name conflicts with stdlib and has different semantics
> - Breaking change, must do pre-1.0.0

**Issues**:
1. Name collides with `std::fs::canonicalize()` but has different behavior
2. Returns `Result<>` but never actually returns `Err`
3. Users expect filesystem canonicalization, get syntactic normalization
4. Function has warning in docs but still confusing

## Proposed Solution

### Option 1: `normalize_unchecked()`
```rust
pub fn normalize_unchecked(path: impl AsRef<Path>) -> PathBuf
```
- Pros: Clear it's normalization without validation
- Cons: "unchecked" might imply unsafe behavior

### Option 2: `normalize_with_prefix_strip()`
```rust
pub fn normalize_with_prefix_strip(path: impl AsRef<Path>) -> PathBuf
```
- Pros: Descriptive of actual behavior
- Cons: Long name

### Option 3: `syntactic_canonicalize()`
```rust
pub fn syntactic_canonicalize(path: impl AsRef<Path>) -> PathBuf
```
- Pros: Distinguishes from filesystem canonicalization
- Cons: Still uses "canonicalize" term

### Recommendation: `normalize_unchecked()`

Most concise and clear about what it does.

## Implementation Strategy

1. **Add new function** with chosen name
2. **Deprecate old function** with clear migration message:
```rust
#[deprecated(
  since = "0.29.0",
  note = "Renamed to `normalize_unchecked()` to avoid confusion with std::fs::canonicalize(). \
          This function performs syntactic normalization only, not filesystem canonicalization."
)]
pub fn canonicalize(path: impl AsRef<Path>) -> io::Result<PathBuf> {
  Ok(normalize_unchecked(path))
}
```
3. **Update all internal uses** to new name
4. **Update documentation** and examples
5. **Add migration note** to CHANGELOG
6. **Schedule removal** for v0.30.0 or v1.0.0

## Breaking Change Migration

### Phase 1: v0.29.0 (Deprecation)
- Add `normalize_unchecked()`
- Deprecate `canonicalize()` with clear message
- Both functions available

### Phase 2: v0.30.0 or v1.0.0 (Removal)
- Remove deprecated `canonicalize()`
- Only `normalize_unchecked()` available

### User Migration
```rust
// Old (deprecated in v0.29.0, removed in v0.30.0+)
let path = pth::canonicalize("./foo")?;

// New
let path = pth::normalize_unchecked("./foo");
```

## Additional Changes

**Fix Return Type**: Make infallible since it never fails
```rust
// Before
pub fn canonicalize(path: impl AsRef<Path>) -> io::Result<PathBuf>

// After
pub fn normalize_unchecked(path: impl AsRef<Path>) -> PathBuf
```

**Update Documentation**:
```rust
/// Performs syntactic path normalization and strips Windows verbatim prefix.
///
/// This function:
/// - Resolves `.` and `..` components syntactically (no filesystem access)
/// - Strips Windows `\\?\` verbatim prefix if present
/// - Normalizes separators to `/`
///
/// **Note**: This is NOT filesystem canonicalization. It does not:
/// - Resolve symlinks
/// - Verify path existence
/// - Handle filesystem-level path resolution
///
/// For filesystem canonicalization, use `std::fs::canonicalize()` or
/// `pth::absolute::canonicalize()`.
```

## Acceptance Criteria

- [ ] New function `normalize_unchecked()` implemented
- [ ] Returns `PathBuf` directly (not `Result`)
- [ ] Old `canonicalize()` deprecated with clear message
- [ ] All internal uses updated to new name
- [ ] All tests updated
- [ ] Documentation updated with clear explanation
- [ ] CHANGELOG entry added
- [ ] Migration guide in readme or docs
- [ ] All existing tests pass

## Estimated Effort

2 hours
- Implementation: 30 min
- Update tests and internal uses: 30 min
- Documentation: 30 min
- Testing: 30 min

## Target Milestone

v0.29.0 (P1 - HIGH, breaking changes window)

## Related Issues

- Discovery issue 2.3: Misleading Function Name - canonicalize()
- Discovery issue 2.7: Inconsistent Error Return Types
- Spec section 10.1: High Priority roadmap item

## Implementation Notes

### Search for All Uses

```bash
cd /home/user1/pro/lib/willbe/module/pth
grep -r "canonicalize" src/ tests/
```

### Test Updates

Ensure all test cases updated:
```rust
// Old
assert_eq!(pth::canonicalize("./foo")?, PathBuf::from("foo"));

// New
assert_eq!(pth::normalize_unchecked("./foo"), PathBuf::from("foo"));
```

### Consider Removing Entirely

Alternative: Just remove function entirely if it's not widely used. Users can call `normalize()` directly and strip prefix if needed.
