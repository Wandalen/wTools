# Feature: Aggregate Facade

### Scope

- **Purpose**: Describe the core purpose of `derive_tools` as a unified derive macro aggregator.
- **Responsibility**: Document aggregated sources, namespace organization, and feature gate architecture.
- **In Scope**: Six aggregated macro sources, namespace layer structure, and feature flag gating.
- **Out of Scope**: Individual macro behavior — see `api/001_workspace_derives.md` and `api/002_external_derives.md`.

### Design

The crate provides a single import point for derive macros from six sources: the workspace
proc-macro companion, two workspace utility crates, and three external packages. Consumers
add one dependency and select the macros they need through individual feature flags.

**Aggregated sources:**

- Workspace proc-macro implementations for delegation, conversion, construction, indexing,
  and logical operations — maintained alongside other workspace crates.
- Variadic conversion macros that generate implementations for multiple argument counts.
- Trait object cloning support for use with dynamic dispatch.
- A comprehensive external derive collection covering arithmetic, conversion, and enum utilities.
- Enum string utilities and display formatting.
- Display and string parsing via format pattern strings.

**Namespace organization** follows the workspace standard: re-exports are organized into
four layers (own, orphan, exposed, prelude) plus a dependency namespace for explicit
source access. Consumers typically use the prelude layer via a glob import.

**Feature gate architecture** allows independent activation of each macro group. An
`enabled` meta-feature activates the crate. A `full` meta-feature activates everything.
Individual derive features activate specific macros and their dependencies only. This
minimizes compile time and dependency surface for consumers that need only a subset.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../api/001_workspace_derives.md` | Workspace derive macros re-exported by this facade |
| doc | `../api/002_external_derives.md` | External derive packages re-exported by this facade |
| doc | `../invariant/001_pure_facade.md` | Why this crate must not implement its own macros |
| doc | `../integration/001_derive_tools_meta.md` | Workspace companion implementing the custom macros |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Overview, Scope, Architecture, and Design Rationale sections; spec.md has been deleted — Sources entry retained as migration record. |
