# Feature: Typing Aggregation

### Scope

- **Purpose**: Provide type inspection and manipulation utilities through the wtools typing module alias.
- **Responsibility**: Document the typing category feature flags, the aliased module, and its granular sub-features.
- **In Scope**: Feature flags in the typing namespace, exposed module alias, three granular type-inspection sub-features.
- **Out of Scope**: Type utility implementation details (see typing_tools docs/).

### Design

The typing category re-exports typing_tools under the short alias `typing`. It offers three granular sub-features for specific type-inspection capabilities.

| Flag | Enables |
|------|---------|
| `typing` | Base sub-crate inclusion |
| `typing_default` | inspect_type, is_slice, implements |
| `typing_full` | All typing sub-features (same as default) |
| `typing_no_std` | no_std support |
| `typing_use_alloc` | Allocator support in no_std mode |
| `typing_inspect_type` | Runtime type name inspection |
| `typing_is_slice` | Slice type detection |
| `typing_implements` | Trait implementation checking |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `../../Cargo.toml` | Feature flag definitions (lines 103-124) |
| source | `../../src/lib.rs` | Feature-gated re-export and namespace wiring |
| doc | `../api/001_namespace_hierarchy.md` | Namespace surface contract |
| doc | `../pattern/002_feature_flag_composition.md` | Feature flag design pattern |
