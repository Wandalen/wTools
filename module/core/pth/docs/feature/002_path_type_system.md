# Feature: Path Type System

### Scope

- **Purpose**: Provide compile-time path property guarantees through newtype wrappers.
- **Responsibility**: Document the type system feature, its design rationale, and all related artifacts.
- **In Scope**: AbsolutePath, NormalizedPath, CanonicalPath alias, NativePath alias, CurrentPath, and the three conversion traits.
- **Out of Scope**: Free-function operations on raw strings (→ `feature/001`).

### Status

- **Version introduced:** 0.1.0
- **Stability:** stable
- **Module path:** `pth` (types re-exported at crate root)

### Design

The type system encodes three distinct path properties as newtypes with zero runtime overhead:

- **AbsolutePath**: A path that does not begin with `.` or `..` after normalization. Enforces that callers receive a non-relative path without requiring a filesystem check.
- **NormalizedPath**: A path that has been syntactically normalized. `CanonicalPath` and `NativePath` are permanent type aliases; callers choose whichever name best expresses intent in context.
- **CurrentPath**: A zero-sized type (ZST) that resolves to the process working directory on conversion — the only type in `pth` that performs a filesystem call.

Three conversion traits provide a tier of allocation strategies for generic functions accepting path-like inputs:
- `AsPath` — borrow a reference without allocation.
- `TryIntoPath` — convert to an owned value, always allocating.
- `TryIntoCowPath` — return a borrowed reference when possible, owned when necessary.

All path newtypes implement all three traits. A blanket `AsRef<Path>` implementation covers standard library types, ensuring generic functions work with any path-like input.

### Example

```rust
use pth::AbsolutePath;
use pth::NormalizedPath;

// AbsolutePath enforces non-relative construction
let abs = AbsolutePath::try_from( "/usr/share" ).expect( "absolute path" );
assert!( abs.starts_with( "/usr" ) );

// NormalizedPath normalizes on construction
let norm = NormalizedPath::try_from( "a/b/../c" ).expect( "normalized path" );
assert_eq!( norm.to_str().unwrap(), "a/c" );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/path/absolute_path.rs](../../src/path/absolute_path.rs) | AbsolutePath definition and TryFrom implementations |
| source | [src/path/normalized_path.rs](../../src/path/normalized_path.rs) | NormalizedPath with CanonicalPath and NativePath aliases |
| source | [src/path/current_path.rs](../../src/path/current_path.rs) | CurrentPath ZST and its conversion implementations |
| source | [src/as_path.rs](../../src/as_path.rs) | AsPath trait — borrow-only path access |
| source | [src/try_into_path.rs](../../src/try_into_path.rs) | TryIntoPath trait — owned conversion |
| source | [src/try_into_cow_path.rs](../../src/try_into_cow_path.rs) | TryIntoCowPath trait — zero-copy where possible |
| test | [tests/inc/absolute_path_test/](../../tests/inc/absolute_path_test/) | AbsolutePath construction, validation, and operation tests |
| doc | [api/002_type_wrappers_api.md](../api/002_type_wrappers_api.md) | Type wrapper API contracts |
| doc | [api/003_conversion_traits_api.md](../api/003_conversion_traits_api.md) | Conversion trait contracts |
| doc | [invariant/001_zero_dependencies.md](../invariant/001_zero_dependencies.md) | CurrentPath is the single filesystem-access exception |
