# API: Type Wrappers

### Scope

- **Purpose**: Document the public newtype API for type-safe path handling in `pth`.
- **Responsibility**: Describe AbsolutePath, NormalizedPath, CanonicalPath, NativePath, and CurrentPath construction and available operations.
- **In Scope**: Construction, validation errors, available methods, type alias relationships.
- **Out of Scope**: Free-function API (→ `api/001`); conversion trait definitions (→ `api/003`).

### Abstract

The type wrapper API provides five path newtypes that encode path properties at the type level with zero runtime overhead. Three are true newtypes (`AbsolutePath`, `NormalizedPath`, `CurrentPath`); two are permanent type aliases (`CanonicalPath`, `NativePath`) pointing to `NormalizedPath`. Construction validates the stated property; subsequent operations preserve it.

### Operations

| Symbol | Kind | Notes |
|--------|------|-------|
| `AbsolutePath::try_from(path)` | constructor | Accepts `&str`, `&Path`, `PathBuf`; rejects paths beginning with `.` or `..` |
| `AbsolutePath::join(path)` | method | Appends a relative component; returns a new `AbsolutePath` |
| `AbsolutePath::parent()` | method | Returns parent directory as `AbsolutePath`; nothing if at root |
| `AbsolutePath::starts_with(prefix)` | method | Tests whether path begins with the given prefix (whole components only) |
| `AbsolutePath::inner()` | method | Consumes the wrapper and returns the inner path value |
| `AbsolutePath::from_iter(iter)` | constructor | Joins an iterator of path-like values; result must be absolute |
| `AbsolutePath::from_paths(paths)` | constructor | Joins a tuple of path-like values; result must be absolute |
| `NormalizedPath::try_from(path)` | constructor | Applies syntactic normalization; accepts any well-formed path |
| `NormalizedPath::join(path)` | method | Same semantics as `AbsolutePath::join` |
| `NormalizedPath::parent()` | method | Same semantics as `AbsolutePath::parent` |
| `NormalizedPath::starts_with(prefix)` | method | Same semantics as `AbsolutePath::starts_with` |
| `NormalizedPath::inner()` | method | Same semantics as `AbsolutePath::inner` |
| `CanonicalPath` | type alias | Permanent alias for `NormalizedPath` |
| `NativePath` | type alias | Permanent alias for `NormalizedPath` |
| `CurrentPath` | type (ZST) | Zero-sized marker; resolves working directory on conversion via `TryIntoPath` |

### Error Handling

All construction methods return a result. `AbsolutePath` rejects paths beginning with `.` or `..`; `NormalizedPath` accepts all well-formed paths. `CurrentPath` conversion fails if the process working directory is unavailable (permissions error or deleted directory). All errors are `io::Error`.

### Compatibility Guarantees

All newtypes and their trait implementations are stable. The type aliases `CanonicalPath` and `NativePath` are permanent — they will not be removed. The `canonicalize` free function deprecation (v0.29.0) does not affect these types.

### Example

```rust
use pth::AbsolutePath;
use pth::NormalizedPath;

// AbsolutePath: rejects relative-start paths
let abs = AbsolutePath::try_from( "/usr/share" ).expect( "absolute path" );
assert!( abs.starts_with( "/usr" ) );
let child = abs.join( "doc" ).expect( "join" );
assert_eq!( child.to_str().unwrap(), "/usr/share/doc" );

// NormalizedPath: normalizes on construction
let norm = NormalizedPath::try_from( "a/b/../c" ).expect( "normalized" );
assert_eq!( norm.to_str().unwrap(), "a/c" );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/path/absolute_path.rs](../../src/path/absolute_path.rs) | AbsolutePath implementation |
| source | [src/path/normalized_path.rs](../../src/path/normalized_path.rs) | NormalizedPath with CanonicalPath and NativePath aliases |
| source | [src/path/current_path.rs](../../src/path/current_path.rs) | CurrentPath — single filesystem-accessing type |
| test | [tests/inc/absolute_path_test/](../../tests/inc/absolute_path_test/) | AbsolutePath construction and operation tests |
| doc | [invariant/001_zero_dependencies.md](../invariant/001_zero_dependencies.md) | CurrentPath is the single exception to zero-filesystem invariant |
| doc | [api/003_conversion_traits_api.md](003_conversion_traits_api.md) | Conversion traits implemented by all path newtypes |
| doc | [feature/002_path_type_system.md](../feature/002_path_type_system.md) | Type system feature navigation |
