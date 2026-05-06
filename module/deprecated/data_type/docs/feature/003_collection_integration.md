# Feature: Collection Integration

### Scope

- **Purpose**: Re-export collection types and macros from collection_tools into the data_type namespace.
- **Responsibility**: Documents the dt_collection feature flag and the collection_tools items it contributes.
- **In Scope**: All items from collection_tools' exposed and prelude namespaces (including collection_constructors), gated by dt_collection.
- **Out of Scope**: collection_tools internals, custom collection implementations, or iterator adapters.

### Cross-References

| Type   | File | Responsibility |
|--------|------|----------------|
| source | [`src/dt.rs`](../../src/dt.rs) | dt_collection exposed and prelude re-export — collection_tools delegation |
| config | [`Cargo.toml`](../../Cargo.toml) | dt_collection feature flag — enables types and collection_constructors |
| test   | [`tests/smoke_test.rs`](../../tests/smoke_test.rs) | Smoke test — crate compilation with all features |
| test   | [`tests/collection_macros_feature_test.rs`](../../tests/collection_macros_feature_test.rs) | hmap!/hset!/bmap!/bset! availability under dt_collection feature |
| doc    | [invariant/001_pure_aggregator.md](../invariant/001_pure_aggregator.md) | All exposed items are pass-throughs from upstream crates |
| doc    | [api/001_namespace_chain.md](../api/001_namespace_chain.md) | Public namespace that exposes this feature's items |

### Design

The dt_collection feature enables the collection_tools dependency with the collection_constructors sub-feature and re-exports its entire exposed and prelude namespaces into data_type. When enabled, consumers gain access to all collection_tools types, macros, and constructor utilities through data_type.

This feature is a pure pass-through: data_type adds no wrapper types and no additional behavior around the collection items.
