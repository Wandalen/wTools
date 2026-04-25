# Integration: derive_tools_meta

### Scope

- **Purpose**: Document the dependency relationship between `derive_tools` and `derive_tools_meta`.
- **Responsibility**: Explain what is re-exported from `derive_tools_meta` and why the two-crate split exists.
- **In Scope**: Re-export mechanism, integration points, and compatibility requirements.
- **Out of Scope**: The `derive_tools_meta` crate itself — see its own docs/.

### System Description

`derive_tools_meta` is the workspace proc-macro companion crate. It implements the
custom derive macros that are specific to the workspace and not available from external
packages. These include delegation, newtype conversion, construction, indexing, and logical
operations on wrapper types.

Rust requires procedural macro implementations to reside in a dedicated proc-macro crate.
`derive_tools_meta` is that crate. `derive_tools` re-exports its macros as part of the
unified facade, providing the stable public interface.

### Integration Points

`derive_tools` lists `derive_tools_meta` as a dependency and re-exports its macros
under its own namespace via feature-gated `pub use` statements. Each macro in
`derive_tools_meta` corresponds to a dedicated feature flag in `derive_tools`.

Consumers depend on `derive_tools` and activate the `derive_tools_meta` macros via
the corresponding feature flags — they do not reference `derive_tools_meta` directly.

### Compatibility Requirements

The macro interface exposed by `derive_tools_meta` is the stable contract. Changes to
macro names or behavior in `derive_tools_meta` require coordinated updates to
`derive_tools`'s re-exports and a version bump.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../feature/001_aggregate_facade.md` | Aggregate facade that re-exports these macros |
| doc | `../api/001_workspace_derives.md` | Specific macros re-exported from derive_tools_meta |
| doc | `../invariant/001_pure_facade.md` | Why this crate and derive_tools are separate |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Dependencies and Consumers section; spec.md has been deleted — Sources entry retained as migration record. |
