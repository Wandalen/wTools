# Feature: Collection Tools Facade

### Scope

- **Purpose**: Provide a single import path for all collection_tools utilities without requiring consumers to depend on collection_tools directly.
- **Responsibility**: Re-export the entire public API of collection_tools under the wtools namespace.
- **In Scope**: Full re-export of all collection_tools public items; feature-flag passthrough via `collection_constructors` and `collection_into_constructors`.
- **Out of Scope**: Additional abstractions, new macros, or modifications to the exported items.

### Sources

| File | Relationship |
|------|-------------|
| [src/lib.rs](../../src/lib.rs) | `pub use ::collection_tools::*` re-export |
| [Cargo.toml](../../Cargo.toml) | `collection_tools` dependency and feature flag declarations |

### APIs

| File | Relationship |
|------|-------------|
| [../api/001_collection_tools_re_export.md](../api/001_collection_tools_re_export.md) | Public API surface |
