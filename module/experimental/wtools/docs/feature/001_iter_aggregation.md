# Feature: Iter Aggregation

### Scope

- **Purpose**: Provide iterator extension utilities through the wtools iter module alias.
- **Responsibility**: Document the iter category feature flags, the aliased module, and its sub-feature structure.
- **In Scope**: Feature flags in the iter namespace, exposed module alias, default and full feature sets.
- **Out of Scope**: Iterator implementation details (see iter_tools docs/).

### Design

The iter category re-exports iter_tools under the short alias `iter`. When the base flag is enabled, the full iter_tools crate becomes accessible through the wtools namespace hierarchy (dependency, own, exposed, prelude layers).

| Flag | Enables |
|------|---------|
| `iter` | Base sub-crate inclusion |
| `iter_default` | Default feature set from iter_tools |
| `iter_full` | All iter_tools features |
| `iter_no_std` | no_std support |
| `iter_use_alloc` | Allocator support in no_std mode |

The default and full sets are identical for this category, as iter_tools exposes a single cohesive feature surface.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `../../Cargo.toml` | Feature flag definitions (lines 37-52) |
| source | `../../src/lib.rs` | Feature-gated re-export and namespace wiring |
| doc | `../api/001_namespace_hierarchy.md` | Namespace surface contract |
| doc | `../pattern/002_feature_flag_composition.md` | Feature flag design pattern |
