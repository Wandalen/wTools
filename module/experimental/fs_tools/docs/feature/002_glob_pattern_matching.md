# Feature: Glob Pattern Matching

### Scope

- **Purpose**: Give `fs_tools` consumers access to Unix shell-style file pattern matching without a separate dependency.
- **Responsibility**: Documents the re-export of the `glob` crate under `fs_tools::glob`, including feature flag requirements and access patterns.
- **In Scope**: `fs_tools::glob::glob()`, `glob_with()`, `Pattern`, `MatchOptions`, `Paths`, `GlobError`, `PatternError`; feature flag `glob`; `dependency::glob` namespace alias.
- **Out of Scope**: Custom glob implementations, wrappers around the `glob` API, behavior beyond what the upstream crate provides.

### Design

The entire `glob` crate is re-exported as a module under `fs_tools::glob` (feature-gated on `glob` which requires `enabled`). This avoids a name collision: re-exporting the crate as a module (`pub use ::glob`) instead of re-exporting its top-level function prevents `fs_tools::glob` from being ambiguous between a module and a function.

Access patterns:
- Primary: `fs_tools::glob::glob("*.rs")`
- Dependency namespace: `fs_tools::dependency::glob::glob("*.rs")`

The `glob` crate has 300M+ downloads and a stable, complete API. Re-export eliminates maintenance burden; there is no value in wrapping individual functions.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/fs/lib.rs` | `pub use ::glob` re-export declaration (feature-gated) |
| test | `../../tests/inc/glob_test.rs` | Re-export accessibility and functional coverage (9 tests) |
| doc | `../api/001_temp_dir.md` | Sibling API for contrast — TempDir vs glob access pattern |
