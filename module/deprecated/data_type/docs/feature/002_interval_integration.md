# Feature: Interval Integration

### Scope

- **Purpose**: Re-export interval types from interval_adapter into the data_type namespace.
- **Responsibility**: Documents the dt_interval feature flag and the interval_adapter items it contributes.
- **In Scope**: All items from interval_adapter's exposed and prelude namespaces, gated by dt_interval.
- **Out of Scope**: interval_adapter internals, interval arithmetic, or custom interval implementations.

### Cross-References

| Type   | File | Responsibility |
|--------|------|----------------|
| source | [`src/dt.rs`](../../src/dt.rs) | dt_interval exposed and prelude re-export — interval_adapter delegation |
| config | [`Cargo.toml`](../../Cargo.toml) | dt_interval feature flag declaration |
| test   | [`tests/smoke_test.rs`](../../tests/smoke_test.rs) | Smoke test — crate compilation with all features |
| test   | [`tests/inc/mod.rs`](../../tests/inc/mod.rs) | Test module — includes interval_adapter suite under dt_interval feature |
| doc    | [invariant/001_pure_aggregator.md](../invariant/001_pure_aggregator.md) | All exposed items are pass-throughs from upstream crates |
| doc    | [api/001_namespace_chain.md](../api/001_namespace_chain.md) | Public namespace that exposes this feature's items |

### Design

The dt_interval feature enables the interval_adapter dependency and re-exports its entire exposed and prelude namespaces into data_type. When enabled, consumers can access all interval_adapter types and utilities through data_type without declaring a separate interval_adapter dependency.

This feature is a pure pass-through: data_type adds no wrapper types and no additional behavior around the interval types.
