# Task 004: Implement pth::absolute::join Function

## Overview

Implement the `pth::absolute::join` function as specified in detailed comments at `src/lib.rs:14-29`. This function provides intelligent path joining with automatic absolutization, finding the rightmost absolute path in the input and joining from there.

## Status

🚫 ABANDONED (2025-10-29)

## Decision

This task has been abandoned after comprehensive feasibility analysis. The absolute::join function will NOT be implemented.

## Abandonment Rationale

See `-task_feasibility_analysis.md` for complete analysis. Key findings:

### 1. Zero Demonstrated Demand
- No user requests in git history
- No GitHub issues filed
- No real-world use cases found
- Just an internal qqq marker (speculation)

### 2. Confusing Behavior
- "Rightmost absolute path" rule is non-obvious
- Surprising that earlier absolute paths are discarded
- Example: `join(["/abs1", "rel", "/abs2", "rel2"])` → `"/abs2/rel2"` (loses /abs1/rel!)
- Easier to be explicit with existing APIs

### 3. Violates pth Philosophy
- pth crate is "no filesystem operations"
- Spec requires `canonicalize()` → filesystem I/O
- Can fail on non-existent paths
- Adds I/O errors to pure path manipulation library

### 4. Redundant with Existing APIs
- `pth::path::join()` already handles joining
- `PathBuf::from(base).join()` is clearer for known base
- No compelling reason for third join variant
- Users can easily implement pattern if needed:
  ```rust
  let base = paths.iter().rfind(|p| p.is_absolute()).unwrap_or(".");
  let mut result = PathBuf::from(base);
  for p in &paths { result.push(p); }
  ```

### 5. High Cost, Low Value
- 3+ hours implementation effort
- Ongoing maintenance burden (semver commitment)
- API surface expansion
- Documentation and testing requirements
- No proven user value to justify cost

## Alternative Chosen

Remove the qqq marker from `src/lib.rs:14` and document the decision.

## Original Problem Statement (Now Obsolete)

**Marker**: `qqq: xxx:` at `src/lib.rs:14`

~~Currently, users must manually handle absolute path detection and joining. The proposed `absolute::join` function automates this common pattern.~~

**REASON FOR ABANDONMENT**: No demonstrated user need, confusing semantics, violates library philosophy of pure path manipulation without filesystem I/O.

**Desired Behavior** (from spec comments):
1. Takes multiple path-like items (tuple, slice, or multiple args)
2. Finds the rightmost item that represents an absolute path
3. If absolute path found, joins all segments from that path onwards
4. If no absolute path found, joins all segments relative to CWD
5. Final joined path must be canonicalized and returned as `AbsolutePath`

## Specification

**From `src/lib.rs:14-29`**:

```rust
// qqq: xxx: implement `pth ::absolute ::join` function or add option to `pth ::path ::join`
//       Desired Signature Idea 1 : `pub fn join< T1, T2 >(p1: T1, p2: T2) -> io ::Result< AbsolutePath >`
//       Behavior :
//       1. Takes multiple path-like items (e.g., via tuple, slice, or multiple args).
//       2. Finds the rightmost item that represents an absolute path.
//       3. If an absolute path is found, it joins all path segments *from that absolute path onwards*.
//       4. If *no* absolute path is found, joins *all* segments relative to current working directory.
//       5. The final joined path must be canonicalized and returned as an `AbsolutePath`.
//
//       Example usage:
//         let p1 = pth ::absolute ::join(( "relative/path", "/absolute/base", "child" )).unwrap();
//         // Result should be: `/absolute/base/child`
//
//         let p2 = pth ::absolute ::join(( "only", "relative", "parts" )).unwrap();
//         // Result should join all parts onto the current working directory, then canonicalize.
```

## Proposed API Design

### Option 1: Multiple Implementations

```rust
// For 2 paths
pub fn join<T1, T2>(p1: T1, p2: T2) -> io::Result<AbsolutePath>
where
  T1: AsRef<Path>,
  T2: AsRef<Path>;

// For 3 paths
pub fn join3<T1, T2, T3>(p1: T1, p2: T2, p3: T3) -> io::Result<AbsolutePath>
where
  T1: AsRef<Path>,
  T2: AsRef<Path>,
  T3: AsRef<Path>;

// For iterator/slice
pub fn join_iter<I>(paths: I) -> io::Result<AbsolutePath>
where
  I: IntoIterator,
  I::Item: AsRef<Path>;
```

### Option 2: Generic over tuple length (more complex)

Use macro or trait-based approach similar to how `path::join` handles multiple arguments.

## Implementation Strategy

1. Create new module `src/path/absolute_join.rs`
2. Implement core logic:
   - Iterate paths right-to-left to find rightmost absolute
   - Build joined path from that point
   - If no absolute found, prepend `env::current_dir()?`
   - Canonicalize result
   - Wrap in `AbsolutePath`
3. Add to `mod_interface!` in `src/lib.rs`
4. Write comprehensive tests
5. Add documentation with examples

## Acceptance Criteria

- [ ] Function `absolute::join()` implemented with at least 2-argument version
- [ ] Handles rightmost absolute path detection correctly
- [ ] Falls back to CWD when no absolute path present
- [ ] Returns canonicalized `AbsolutePath`
- [ ] At least 10 tests covering:
  - All relative paths (uses CWD)
  - Rightmost absolute path (ignores earlier)
  - Middle absolute path (uses it)
  - Empty path segments
  - Windows and Unix paths
- [ ] Documentation with 3+ examples
- [ ] All existing tests still pass

## Estimated Effort

3 hours (implementation + tests + documentation)

## Target Milestone

N/A (ABANDONED)

## Related Issues

- Discovery issue 1.1: Implement pth::absolute::join Function
- Spec comment at src/lib.rs:14-29

## Implementation Notes

### Algorithm Sketch

```rust
pub fn join<I>(paths: I) -> io::Result<AbsolutePath>
where
  I: IntoIterator,
  I::Item: AsRef<Path>,
{
  let paths: Vec<_> = paths.into_iter().collect();

  // Find rightmost absolute path
  let start_idx = paths
    .iter()
    .rposition(|p| p.as_ref().is_absolute());

  let result = match start_idx {
    Some(idx) => {
      // Join from rightmost absolute path onwards
      let mut result = PathBuf::from(paths[idx].as_ref());
      for path in &paths[idx + 1..] {
        result.push(path.as_ref());
      }
      result
    }
    None => {
      // No absolute path - join all onto CWD
      let mut result = env::current_dir()?;
      for path in &paths {
        result.push(path.as_ref());
      }
      result
    }
  };

  // Canonicalize and wrap
  let canonical = result.canonicalize()?;
  Ok(AbsolutePath(canonical))
}
```

### Test Cases

1. `join(["relative", "paths"])` → canonicalized CWD + "relative/paths"
2. `join(["/abs", "rel"])` → "/abs/rel" (canonicalized)
3. `join(["rel1", "/abs", "rel2"])` → "/abs/rel2"
4. `join(["/abs1", "/abs2", "rel"])` → "/abs2/rel" (rightmost absolute)
5. Windows: `join(["C:\\abs", "rel"])` → "C:/abs/rel"
