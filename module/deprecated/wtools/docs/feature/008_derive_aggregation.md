# Feature: Derive Aggregation

### Scope

- **Purpose**: Provide a comprehensive derive macro collection through the wtools derive module alias.
- **Responsibility**: Document the derive category feature flags, the aliased module, and its extensive per-macro sub-features.
- **In Scope**: Feature flags in the derive namespace, exposed module alias, 20+ granular per-macro sub-features.
- **Out of Scope**: Derive macro implementation details (see derive_tools docs/).

### Design

The derive category re-exports derive_tools under the short alias `derive`. This is the most granular category, offering individual feature flags for each derive macro family. The default and full sets are identical, enabling all available macros.

#### Standard Feature Tiers

| Flag | Enables |
|------|---------|
| `derive` | Base sub-crate inclusion |
| `derive_default` | All macros listed below |
| `derive_full` | All macros listed below (same as default) |
| `derive_no_std` | no_std support |
| `derive_use_alloc` | Allocator support in no_std mode |

#### Arithmetic Macros

| Flag | Macro |
|------|-------|
| `derive_add` | Addition operator |
| `derive_add_assign` | Addition assignment operator |
| `derive_mul` | Multiplication operator |
| `derive_mul_assign` | Multiplication assignment operator |
| `derive_sum` | Sum trait |
| `derive_not` | Not operator |

#### Reference Macros

| Flag | Macro |
|------|-------|
| `derive_as_ref` | Immutable reference conversion |
| `derive_as_mut` | Mutable reference conversion |
| `derive_deref` | Dereference operator |
| `derive_deref_mut` | Mutable dereference operator |

#### Conversion Macros

| Flag | Macro |
|------|-------|
| `derive_from` | From trait |
| `derive_inner_from` | Inner From conversion |
| `derive_try_into` | TryInto conversion |
| `derive_into_iterator` | IntoIterator conversion |
| `derive_constructor` | Constructor generation |
| `derive_index` | Index operator |
| `derive_index_mut` | Mutable index operator |
| `derive_error` | Error trait |

#### Display and Parsing

| Flag | Macro |
|------|-------|
| `derive_display` | Display formatting (uses parse-display external crate) |
| `derive_from_str` | String parsing (uses parse-display external crate) |
| `derive_strum` | Strum enum string conversions |
| `derive_strum_phf` | Strum with perfect hash function |

#### Enum and Trait Utilities

| Flag | Macro |
|------|-------|
| `derive_is_variant` | Variant testing predicates |
| `derive_unwrap` | Variant unwrapping |
| `derive_clone_dyn` | Clone for dynamic trait objects |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `../../Cargo.toml` | Feature flag definitions (lines 196-314) |
| source | `../../src/lib.rs` | Feature-gated re-export and namespace wiring |
| doc | `../api/001_namespace_hierarchy.md` | Namespace surface contract |
| doc | `../pattern/002_feature_flag_composition.md` | Feature flag design pattern |
