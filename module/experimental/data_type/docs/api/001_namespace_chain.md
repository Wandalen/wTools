# API: Namespace Chain

### Scope

- **Purpose**: Define the public namespace structure through which consumers access data_type re-exported items.
- **Responsibility**: Documents the layered re-export chain, the dependency module, and the item sets exposed per feature.
- **In Scope**: The own/orphan/exposed/prelude namespace hierarchy, the dependency module, and which items are gated by each feature.
- **Out of Scope**: Implementation details of the re-exported items — those belong to upstream crates.

### Cross-References

| Type   | File | Responsibility |
|--------|------|----------------|
| source | [`src/lib.rs`](../../src/lib.rs) | Top-level namespace chain and re-export entry point |
| source | [`src/dt.rs`](../../src/dt.rs) | dt sub-namespace — exposed and prelude re-export modules |
| config | [`Cargo.toml`](../../Cargo.toml) | Feature flags gating each namespace segment |
| doc    | [feature/001_either_type.md](../feature/001_either_type.md) | Either type contribution to the exposed namespace |
| doc    | [feature/002_interval_integration.md](../feature/002_interval_integration.md) | Interval types contribution to the exposed namespace |
| doc    | [feature/003_collection_integration.md](../feature/003_collection_integration.md) | Collection types contribution to the exposed namespace |
| doc    | [pattern/001_facade_reexport.md](../pattern/001_facade_reexport.md) | Structural rationale for the layered re-export approach |

### Abstract

The data_type namespace chain is a four-layer re-export hierarchy that tunnels upstream crate exports to the crate root. Items sourced from upstream crates are collected in the dt exposed sub-namespace, promoted through the top-level exposed layer, and made available at the crate root. Consumers choose their import depth based on need.

### Operations

Three access points for consumers:

**Top-level import** — all items from all enabled features. Preferred for application code using multiple data types together.

**Dependency module** — direct access to the upstream crate re-exports. Gated by the same feature flags as the top-level re-exports. Used when a consumer needs capabilities beyond what data_type re-exports at the top level.

**Items available per feature when enabled:**
- `dt_either`: sum type with two variants (left/right)
- `dt_interval`: interval types and bounds, iterable and non-iterable variants, conversion extension traits
- `dt_collection`: hash map, hash set, B-tree map, and B-tree set constructor macros

### Error Handling

No runtime error conditions. All items are unconditionally available at compile time when their feature is enabled. If a feature is disabled, the corresponding items are absent from the namespace; callers that reference them produce compile errors.

### Compatibility Guarantees

All items are direct re-exports. Compatibility is determined by the upstream crate versions declared in Cargo.toml. data_type adds no additional compatibility layer beyond what the upstream crates provide.
