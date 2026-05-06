# Feature: No-Std Support

### Scope

- **Purpose**: Enable use of `component_model_types` in environments without the Rust standard library.
- **Responsibility**: Documents the no_std compatibility feature, its feature flags, and the constraints on its use.
- **In Scope**: The `no_std` and `use_alloc` feature flags and their semantics; propagation to `collection_tools`.
- **Out of Scope**: The Assign trait system itself (→ `feature/001_component_assignment.md`); no_std support in `collection_tools` (upstream dependency).

### Design

The crate declares `#![cfg_attr(feature = "no_std", no_std)]` to drop the standard library link when the `no_std` feature is enabled. The `use_alloc` feature re-enables allocation-dependent functionality by enabling the `alloc` crate. Both flags propagate to the `collection_tools` dependency via the same feature names.

| Feature | Enables | Requires |
|---------|---------|----------|
| `no_std` | Drop standard library; embedded use | nothing |
| `use_alloc` | Re-enable allocation without full std | `no_std` |

The Assign trait system (`types_component_assign`) is fully no_std-compatible — it defines only trait interfaces with no heap allocation or std-specific types. Adding `no_std` support does not restrict the Assign trait API in any way.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | `cfg_attr` no_std gate at crate root |
| source | `Cargo.toml` | `no_std` and `use_alloc` feature declarations and propagation |
| doc | [feature/001_component_assignment.md](001_component_assignment.md) | The Assign trait feature that benefits from no_std support |
