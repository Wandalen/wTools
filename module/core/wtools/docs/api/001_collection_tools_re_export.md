# API: Collection Tools Re-export

### Scope

- **Purpose**: Document the public API surface exposed by wtools as a re-export facade over collection_tools.
- **Responsibility**: Enumerate which items are available and how feature flags govern availability.
- **In Scope**: All items re-exported from collection_tools when the `enabled` feature is active.
- **Out of Scope**: Internal implementation details of collection_tools; items not part of its public API.

### Sources

| File | Relationship |
|------|-------------|
| [src/lib.rs](../../src/lib.rs) | `pub use ::collection_tools::*` — re-exports entire public API |
| [Cargo.toml](../../Cargo.toml) | Feature flags: `enabled`, `collection_constructors`, `collection_into_constructors` |

### Features

| File | Relationship |
|------|-------------|
| [../feature/001_collection_tools_facade.md](../feature/001_collection_tools_facade.md) | Feature context for this re-export API |
