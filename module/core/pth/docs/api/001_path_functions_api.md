# API: Path Functions

### Scope

- **Purpose**: Document the public free-function API for path string manipulation in `pth`.
- **Responsibility**: Describe all normalization, joining, query, and transformation free functions.
- **In Scope**: normalize, normalize_unchecked, iter_join, is_glob, ext, exts, without_ext, change_ext, path_common, path_relative, rebase.
- **Out of Scope**: Type wrapper construction API (→ `api/002`); conversion trait API (→ `api/003`).

### Abstract

The path functions API provides syntactic path manipulation as free functions. All functions operate on string-like path inputs without filesystem access. The API is organized into four groups: normalization (reducing redundant components), joining (combining multiple paths), query (extracting properties), and transformation (modifying path components).

### Operations

| Symbol | Kind | Notes |
|--------|------|-------|
| `normalize(path)` | function | Removes `.`, resolves `..`, converts backslash to `/`; empty input becomes `.` |
| `normalize_unchecked(path)` | function | Same as `normalize`; panics on non-UTF-8 input instead of returning an error |
| `iter_join(paths)` | function | Joins path-like iterator with `/`; resets accumulation on any absolute path in the sequence |
| `is_glob(path)` | function | Returns `true` when path contains unescaped `*`, `?`, `[…]`, or `{…}` metacharacters |
| `ext(path)` | function | Last extension of the filename (part after the rightmost `.`); empty string if none |
| `exts(path)` | function | All extensions of the filename in left-to-right order |
| `without_ext(path)` | function | Path with the last extension removed; nothing when the path has no extension |
| `change_ext(path, ext)` | function | Path with the last extension replaced by the supplied value |
| `path_common(paths)` | function | Longest common path prefix across multiple paths; nothing when no prefix exists |
| `path_relative(from, to)` | function | Relative path from `from` to `to`; `.` when both paths are identical |
| `rebase(path, old_base, new_base)` | function | Replaces `old_base` prefix with `new_base` |
| `canonicalize(path)` | function (deprecated 0.29.0) | Renamed to `normalize_unchecked`; use `normalize` instead |

### Error Handling

Functions without a meaningful result for the given input return an empty string (e.g., `ext` on a path with no extension) or produce nothing (e.g., `without_ext` on a path with no extension). The `normalize` function is total — it handles empty input by returning `.`. Only `normalize_unchecked` can panic, and only on non-UTF-8 inputs.

The deprecated `canonicalize` function is preserved for backward compatibility but equivalent to `normalize` plus Windows verbatim prefix stripping; use `normalize` instead.

### Compatibility Guarantees

All listed functions are stable. The deprecated `canonicalize` carries `#[deprecated(since = "0.29.0")]`; it will not be removed before a major version bump. No breaking changes are planned for the remaining functions.

### Example

```rust
use pth::path;

// Normalize: resolve dots and backslash separators
let result = path::normalize( "a/b/../c/./d" );
assert_eq!( result.to_str().unwrap(), "a/c/d" );

// Glob detection
assert!( path::is_glob( "src/**/*.rs" ) );
assert!( !path::is_glob( "src/main.rs" ) );

// Extension query
assert_eq!( path::ext( "archive.tar.gz" ), "gz" );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/path.rs](../../src/path.rs) | Complete implementation of all listed free functions |
| test | [tests/inc/](../../tests/inc/) | Function-level test cases covering all listed functions |
| doc | [invariant/001_zero_dependencies.md](../invariant/001_zero_dependencies.md) | Zero-filesystem invariant satisfied by all these functions |
| doc | [invariant/002_fixed_output_format.md](../invariant/002_fixed_output_format.md) | Output format invariant satisfied by normalization functions |
| doc | [feature/001_path_normalization.md](../feature/001_path_normalization.md) | Normalization feature navigation using these functions |
