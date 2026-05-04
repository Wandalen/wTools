# Feature: Meta Aggregation

### Scope

- **Purpose**: Provide metaprogramming macros through the wtools meta module alias.
- **Responsibility**: Document the meta category feature flags, the aliased module, and its granular sub-features.
- **In Scope**: Feature flags in the meta namespace, exposed module alias, granular macro sub-features.
- **Out of Scope**: Macro implementation details (see meta_tools docs/).

### Design

The meta category re-exports meta_tools under the short alias `meta`. This is the most granular category after derive, offering individual feature flags for each macro family.

| Flag | Enables |
|------|---------|
| `meta` | Base sub-crate inclusion |
| `meta_default` | for_each, impls_index, mod_interface, idents_concat |
| `meta_full` | All meta sub-features (same as default for this category) |
| `meta_no_std` | no_std support |
| `meta_use_alloc` | Allocator support in no_std mode |
| `meta_for_each` | for_each iteration macro |
| `meta_impls_index` | impls/index function registration macros |
| `meta_mod_interface` | Module interface organization macro |
| `meta_idents_concat` | Identifier concatenation macro |

Note: impls_index is also included as a non-optional dependency at the crate level, independent of the meta feature flag. This is a known deviation documented with a qqq marker in the manifest.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `../../Cargo.toml` | Feature flag definitions (lines 54-88) |
| source | `../../src/lib.rs` | Feature-gated re-export and namespace wiring |
| doc | `../api/001_namespace_hierarchy.md` | Namespace surface contract |
| doc | `../pattern/002_feature_flag_composition.md` | Feature flag design pattern |
