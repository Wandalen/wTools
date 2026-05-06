# Feature: Either Type

### Scope

- **Purpose**: Re-export the Either sum type for use without a direct dependency on the either crate.
- **Responsibility**: Documents the dt_either feature flag and what it contributes to the data_type namespace.
- **In Scope**: The Either type exposed via the dt_either feature flag.
- **Out of Scope**: Custom sum type implementations, pattern matching ergonomics, or serialization support.

### Cross-References

| Type   | File | Responsibility |
|--------|------|----------------|
| source | [`src/dt.rs`](../../src/dt.rs) | dt_either exposed re-export — Either type pass-through |
| config | [`Cargo.toml`](../../Cargo.toml) | dt_either feature flag declaration |
| test   | [`tests/smoke_test.rs`](../../tests/smoke_test.rs) | Smoke test — Either accessibility under dt_either feature |
| test   | [`tests/inc/either_test.rs`](../../tests/inc/either_test.rs) | Either Left/Right construction and flip method tests |
| doc    | [invariant/001_pure_aggregator.md](../invariant/001_pure_aggregator.md) | All exposed items are pass-throughs from upstream crates |
| doc    | [api/001_namespace_chain.md](../api/001_namespace_chain.md) | Public namespace that exposes this feature's items |

### Design

The dt_either feature re-exports the Either type from the either crate into the data_type exposed namespace. When enabled, consumers can use Either without declaring a direct dependency on either.

This feature is a pure pass-through: data_type adds no wrapper, no additional methods, and no custom implementations around Either.
