# API: Conversion Traits

### Scope

- **Purpose**: Document the conversion trait contracts enabling generic path handling in `pth`.
- **Responsibility**: Describe AsPath, TryIntoPath, and TryIntoCowPath conversion traits.
- **In Scope**: Trait method contracts, implementors, allocation semantics, blanket implementation coverage.
- **Out of Scope**: Path type operations (→ `api/002`); free functions (→ `api/001`).

### Abstract

The conversion traits define three tiered interfaces for accepting diverse path-like inputs in generic functions. Each tier has a distinct allocation strategy: borrow-only (`AsPath`), always-own (`TryIntoPath`), or optimize-for-input (`TryIntoCowPath`).

### Operations

| Symbol | Kind | Notes |
|--------|------|-------|
| `AsPath::as_path()` | trait method | Borrows a path reference without allocation; blanket impl covers all `AsRef<Path>` types |
| `TryIntoPath::try_into_path()` | trait method | Converts receiver into an owned path value; always allocates for borrowed inputs |
| `TryIntoCowPath::try_into_cow_path()` | trait method | Returns borrowed reference for borrowed inputs; owned value for owned inputs |

Implementors of `AsPath`: `&str`, `String`, `&Path`, `PathBuf`, `Utf8Path` (camino), `AbsolutePath`, `NormalizedPath`, and all `T: AsRef<Path>` via blanket.

Implementors of `TryIntoPath`: `&str`, `String`, `&Path`, `PathBuf`, `Component`, `Utf8Path`, `Utf8PathBuf`, `CurrentPath`, and all path newtypes.

Implementors of `TryIntoCowPath`: same as `TryIntoPath`; borrowed inputs return a borrowed reference without allocation.

### Error Handling

All trait methods return a result. Most implementations are infallible and succeed unconditionally. `CurrentPath` can fail when the OS working directory is unavailable. Conversions from `&str` and `String` are infallible on all supported platforms.

### Compatibility Guarantees

All three traits and their existing implementations are stable. The blanket `AsRef<Path>` coverage ensures forward compatibility with new path-like types.

### Example

```rust
use pth::{ AsPath, TryIntoPath };
use std::path::Path;

// AsPath: zero-cost borrow from any path-like value
fn uses_path( p: impl AsPath ) -> bool
{
  p.as_path() == Path::new( "/tmp" )
}
assert!( uses_path( "/tmp" ) );

// TryIntoPath: own an independent path value
let owned = "/usr/bin".try_into_path().expect( "conversion" );
assert_eq!( owned.to_str().unwrap(), "/usr/bin" );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/as_path.rs](../../src/as_path.rs) | AsPath trait definition and all implementations |
| source | [src/try_into_path.rs](../../src/try_into_path.rs) | TryIntoPath trait definition and all implementations |
| source | [src/try_into_cow_path.rs](../../src/try_into_cow_path.rs) | TryIntoCowPath trait definition and all implementations |
| doc | [api/002_type_wrappers_api.md](002_type_wrappers_api.md) | Path newtypes that implement these traits |
| doc | [feature/002_path_type_system.md](../feature/002_path_type_system.md) | Type system feature including conversion traits |
