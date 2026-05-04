# Feature: Dt Aggregation

### Scope

- **Purpose**: Provide data type utilities through the wtools dt module alias.
- **Responsibility**: Document the dt category feature flags, the aliased module, and its data type sub-features.
- **In Scope**: Feature flags in the dt namespace, exposed module alias, either and interval sub-features.
- **Out of Scope**: Data type implementation details (see data_type docs/).

### Design

The dt category re-exports data_type under the short alias `dt`. An alternative feature name `data_type` also activates this module. It offers two granular sub-features for specific data type families.

| Flag | Enables |
|------|---------|
| `dt` | Base sub-crate inclusion |
| `dt_default` | either and interval data types |
| `dt_full` | All dt sub-features (same as default) |
| `dt_use_alloc` | Allocator support |
| `dt_either` | Either sum type |
| `dt_interval` | Interval type for range representations |

Note: the `data_type` feature flag is an alias for `dt` — both activate the same module through the namespace hierarchy.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `../../Cargo.toml` | Feature flag definitions (lines 316-348) |
| source | `../../src/lib.rs` | Feature-gated re-export and namespace wiring |
| doc | `../api/001_namespace_hierarchy.md` | Namespace surface contract |
| doc | `../pattern/002_feature_flag_composition.md` | Feature flag design pattern |
