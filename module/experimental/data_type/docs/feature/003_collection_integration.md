# Feature: Collection Integration

### Scope

- **Purpose**: Re-export collection types and macros from collection_tools into the data_type namespace.
- **Responsibility**: Documents the dt_collection feature flag and the collection_tools items it contributes.
- **In Scope**: All items from collection_tools' exposed and prelude namespaces (including collection_constructors), gated by dt_collection.
- **Out of Scope**: collection_tools internals, custom collection implementations, or iterator adapters.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [invariant/001_pure_aggregator.md](../invariant/001_pure_aggregator.md) | All exposed items are pass-throughs from upstream crates |

### Design

The dt_collection feature enables the collection_tools dependency with the collection_constructors sub-feature and re-exports its entire exposed and prelude namespaces into data_type. When enabled, consumers gain access to all collection_tools types, macros, and constructor utilities through data_type.

This feature is a pure pass-through: data_type adds no wrapper types and no additional behavior around the collection items.
