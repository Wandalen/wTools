# Invariant: Fixed Output Format

### Scope

- **Purpose**: Guarantee consistent path separator output across all platforms and input styles.
- **Responsibility**: Document the forward-slash output invariant, accepted inputs, and enforcement.
- **In Scope**: Output separator contract, accepted input separators, platform behavior, enforcement mechanism.
- **Out of Scope**: Internal normalization algorithm steps (→ `api/001`).

### Invariant Statement

All path strings returned by `pth` normalization functions use forward-slash (`/`) as the path separator, regardless of the platform the code runs on or the separator style used in the input.

Formally: for every normalized output string `s`, `s` contains no backslash (`\`) characters in path separator positions. Windows drive letters of the form `C:/` are preserved correctly; only the separator characters between path components are constrained.

### Enforcement Mechanism

The normalization implementation in `src/path.rs` explicitly converts all backslash separators to forward slashes as an early step during component iteration. Both `normalize_unchecked` and `normalize` apply this conversion unconditionally. The `iter_join` function produces output with the same guarantee.

Tests in `tests/inc/` include cases with Windows-style backslash inputs on all platforms, verifying that output always contains forward slashes.

### Violation Consequences

Violation breaks cross-platform portability. Code relying on normalized paths for string comparison fails on Windows when backslashes appear in output. Stored paths and log output contain platform-dependent characters, causing portability bugs across deployments. The crate's promise of unified path representation is broken.

### Example

```rust
use pth::path;

// Backslash separators are converted to forward-slash on all platforms
let result = path::normalize( "a\\b\\c" );
assert_eq!( result.to_str().unwrap(), "a/b/c" );

// Mixed separators are also normalized
let result = path::normalize( "a/b\\c/d" );
assert_eq!( result.to_str().unwrap(), "a/b/c/d" );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/path.rs](../../src/path.rs) | normalize and iter_join — convert backslash to forward-slash |
| test | [tests/inc/](../../tests/inc/) | Separator normalization test cases across platforms |
| doc | [api/001_path_functions_api.md](../api/001_path_functions_api.md) | Documents output separator contract for all functions |
| doc | [feature/001_path_normalization.md](../feature/001_path_normalization.md) | Normalization feature navigation |
