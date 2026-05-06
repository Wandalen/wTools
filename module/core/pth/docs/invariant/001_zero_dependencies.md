# Invariant: Zero Dependencies

### Scope

- **Purpose**: Guarantee that `pth` operations are purely syntactic — no filesystem access during normal use.
- **Responsibility**: Document the zero-filesystem-access invariant, its single exception, and enforcement mechanism.
- **In Scope**: Which operations are filesystem-free, the `CurrentPath` exception, and how the invariant is verified.
- **Out of Scope**: The normalization algorithm steps (→ `api/001`).

### Invariant Statement

All public functions in `pth` operate on path strings without accessing the filesystem. Exactly one exception exists: `CurrentPath` conversion, which calls the OS to resolve the current working directory.

Formally: for every public function `f` that is not a `CurrentPath` conversion, executing `f` on any input produces no filesystem syscalls.

### Enforcement Mechanism

The guarantee is structural. All path operations in `src/path.rs` work exclusively on `&str`, `PathBuf`, and component iterators — no call to `std::fs::*` exists in that file. Only `src/path/current_path.rs` calls `std::env::current_dir()`.

Code review and static analysis serve as enforcement gates. Any new function calling `std::fs` or `std::env::current_dir()` outside `current_path.rs` is a violation that must be caught at review time.

### Violation Consequences

Violation breaks the core design contract. Callers in constrained environments (no-std, offline containers, sandboxed processes) fail unexpectedly. Tests that verify syntactic-only behavior may flake due to environment differences. The crate's primary value proposition — filesystem-independent path manipulation — is lost.

### Example

```rust
use pth::path;

// Purely syntactic — operates on strings with no filesystem access
let result = path::normalize( "a/b/../c" );
assert_eq!( result.to_str().unwrap(), "a/c" );

// Even with a path that does not exist on disk
let result = path::normalize( "/nonexistent/path/../dir" );
assert_eq!( result.to_str().unwrap(), "/nonexistent/dir" );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/path.rs](../../src/path.rs) | Core functions — all syntactic, no filesystem calls |
| source | [src/path/current_path.rs](../../src/path/current_path.rs) | Single exception — calls `current_dir()` on conversion |
| doc | [api/001_path_functions_api.md](../api/001_path_functions_api.md) | Function contracts documenting syntactic-only behavior |
| doc | [feature/001_path_normalization.md](../feature/001_path_normalization.md) | Normalization feature — depends on this invariant |
