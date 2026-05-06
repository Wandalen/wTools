# Feature: Path Normalization

### Scope

- **Purpose**: Provide syntactic path normalization without filesystem access.
- **Responsibility**: Document the normalization feature scope, design decisions, and all related artifacts.
- **In Scope**: normalize, normalize_unchecked, iter_join, and separator conversion behavior.
- **Out of Scope**: Filesystem-based canonicalization, glob matching, extension handling, path type construction.

### Status

- **Version introduced:** 0.1.0
- **Stability:** stable
- **Module path:** `pth::path`

### Design

Path normalization resolves redundant components from a path string using a single-pass stack algorithm:
- `.` (current-directory) components are discarded.
- `..` (parent-directory) components pop the last normal component from the stack, or are preserved if the stack is empty (relative paths cannot go above their starting point).
- Backslash separators are converted to forward-slash on all platforms.
- Empty input normalizes to `.`.
- Leading `..` in relative paths are preserved; absolute paths cannot traverse above root.

`normalize_unchecked` is a convenience wrapper that panics on non-UTF-8 input rather than returning a result. Prefer `normalize` in production code.

`iter_join` applies normalization during a joining pass: it iterates path-like values, concatenates them with `/`, and resets accumulation when an absolute path is encountered in the sequence.

### Example

```rust
use pth::path;

// Dot and double-dot resolution
let result = path::normalize( "a/b/../../c/./d" );
assert_eq!( result.to_str().unwrap(), "c/d" );

// Backslash-to-forward-slash conversion
let result = path::normalize( "x\\y\\z" );
assert_eq!( result.to_str().unwrap(), "x/y/z" );

// Relative leading double-dots are preserved
let result = path::normalize( "../a/b" );
assert_eq!( result.to_str().unwrap(), "../a/b" );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/path.rs](../../src/path.rs) | normalize, normalize_unchecked, and iter_join implementations |
| test | [tests/inc/](../../tests/inc/) | Normalization algorithm test cases |
| doc | [api/001_path_functions_api.md](../api/001_path_functions_api.md) | Normalization function contracts |
| doc | [invariant/001_zero_dependencies.md](../invariant/001_zero_dependencies.md) | Normalization is purely syntactic — no filesystem access |
| doc | [invariant/002_fixed_output_format.md](../invariant/002_fixed_output_format.md) | Normalization outputs forward-slash separator on all platforms |
