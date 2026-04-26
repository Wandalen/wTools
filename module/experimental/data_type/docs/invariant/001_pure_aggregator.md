# Invariant: Pure Aggregator

### Scope

- **Purpose**: Guarantee that data_type introduces no types, functions, or behavior of its own — all exported items are unmodified pass-throughs from upstream crates.
- **Responsibility**: Documents the pure aggregator contract — its statement, enforcement points, and violation consequences.
- **In Scope**: All items in the data_type exposed and prelude namespaces across all feature flags.
- **Out of Scope**: The behavior of the upstream crates themselves — those are upstream concerns.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_either_type.md](../feature/001_either_type.md) | Either type pass-through subject to this invariant |
| doc | [feature/002_interval_integration.md](../feature/002_interval_integration.md) | Interval integration pass-through subject to this invariant |
| doc | [feature/003_collection_integration.md](../feature/003_collection_integration.md) | Collection integration pass-through subject to this invariant |

### Invariant Statement

All items exported by data_type are direct re-exports of upstream items. The crate's private module is empty. No wrapper types, forwarding functions, or transformations are introduced at any layer of the namespace hierarchy.

### Enforcement Mechanism

- Source inspection: the private module in dt.rs is empty (mod private {}).
- All items in the exposed namespace are conditional pub use delegations to upstream crate namespaces.
- No type definitions, function definitions, or trait implementations exist in the crate source.

### Violation Consequences

Introducing any crate-owned type or function would add an undocumented API surface and undermine the aggregator model that makes data_type's feature flags safe to enable without side effects.
