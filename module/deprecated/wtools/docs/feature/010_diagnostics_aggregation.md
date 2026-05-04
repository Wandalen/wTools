# Feature: Diagnostics Aggregation

### Scope

- **Purpose**: Provide diagnostic utilities through the wtools diagnostics module alias.
- **Responsibility**: Document the diagnostics category feature flags, the aliased module, and its assertion sub-features.
- **In Scope**: Feature flags in the diagnostics namespace, exposed module alias, runtime and compile-time assertion sub-features.
- **Out of Scope**: Diagnostics implementation details (see diagnostics_tools docs/).

### Design

The diagnostics category re-exports diagnostics_tools under the alias `diagnostics`. It offers two complementary assertion mechanisms selectable via sub-features.

| Flag | Enables |
|------|---------|
| `diagnostics` | Base sub-crate inclusion |
| `diagnostics_default` | Runtime and compile-time assertions |
| `diagnostics_full` | All diagnostics sub-features (same as default) |
| `diagnostics_no_std` | no_std support |
| `diagnostics_use_alloc` | Allocator support in no_std mode |
| `diagnostics_runtime_assertions` | Runtime assertion macros and checks |
| `diagnostics_compiletime_assertions` | Compile-time assertion macros |

Note: the exposed namespace follows a different path than other categories. The diagnostics module exposes items through a `diag` sub-module rather than directly through the top-level module.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `../../Cargo.toml` | Feature flag definitions (lines 350-369) |
| source | `../../src/lib.rs` | Feature-gated re-export and namespace wiring |
| doc | `../api/001_namespace_hierarchy.md` | Namespace surface contract |
| doc | `../pattern/002_feature_flag_composition.md` | Feature flag design pattern |
