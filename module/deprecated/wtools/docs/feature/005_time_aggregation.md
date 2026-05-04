# Feature: Time Aggregation

### Scope

- **Purpose**: Provide time and duration utilities through the wtools time module alias.
- **Responsibility**: Document the time category feature flags, the aliased module, and its sub-feature structure.
- **In Scope**: Feature flags in the time namespace, exposed module alias, the time_now sub-feature.
- **Out of Scope**: Time utility implementation details (see time_tools docs/).

### Design

The time category re-exports time_tools under the short alias `time`. It offers one granular sub-feature for current-time functions.

| Flag | Enables |
|------|---------|
| `time` | Base sub-crate inclusion |
| `time_default` | time_now |
| `time_full` | All time sub-features (same as default) |
| `time_no_std` | no_std support |
| `time_use_alloc` | Allocator support in no_std mode |
| `time_now` | Current time retrieval functions |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `../../Cargo.toml` | Feature flag definitions (lines 126-141) |
| source | `../../src/lib.rs` | Feature-gated re-export and namespace wiring |
| doc | `../api/001_namespace_hierarchy.md` | Namespace surface contract |
| doc | `../pattern/002_feature_flag_composition.md` | Feature flag design pattern |
