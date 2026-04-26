# Feature: Mem Aggregation

### Scope

- **Purpose**: Provide memory manipulation utilities through the wtools mem module alias.
- **Responsibility**: Document the mem category feature flags, the aliased module, and its feature structure.
- **In Scope**: Feature flags in the mem namespace, exposed module alias, default and full feature sets.
- **Out of Scope**: Memory utility implementation details (see mem_tools docs/).

### Design

The mem category re-exports mem_tools under the short alias `mem`. This is a minimal category with no granular sub-features beyond the standard tiers.

| Flag | Enables |
|------|---------|
| `mem` | Base sub-crate inclusion |
| `mem_default` | Default feature set (base only) |
| `mem_full` | All mem_tools features (same as default) |
| `mem_no_std` | no_std support |
| `mem_use_alloc` | Allocator support in no_std mode |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `../../Cargo.toml` | Feature flag definitions (lines 90-101) |
| source | `../../src/lib.rs` | Feature-gated re-export and namespace wiring |
| doc | `../api/001_namespace_hierarchy.md` | Namespace surface contract |
| doc | `../pattern/002_feature_flag_composition.md` | Feature flag design pattern |
