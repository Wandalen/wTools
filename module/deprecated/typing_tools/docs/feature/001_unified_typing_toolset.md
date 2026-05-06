# Feature: Unified Typing Toolset

### Scope

- **Purpose**: Provide a single import point giving access to any combination of the three typing macros — trait implementation checking, slice detection, and type inspection — without requiring separate crate dependencies.
- **Responsibility**: Documents the unified typing toolset feature — its aggregation model, feature-gate strategy, and all implementing artifacts.
- **In Scope**: Aggregated re-export of implements, is_slice, and inspect_type macros via feature-gated namespaces.
- **Out of Scope**: Individual macro behaviour (→ sub-crate docs/), crate-level integration outside the workspace.

### Design

The aggregator exposes three independently activatable typing capabilities through a unified namespace:

- **Trait implementation checking** (`typing_implements`): re-exports the implements and instance_of macros from the implements sub-crate.
- **Slice detection** (`typing_is_slice`): re-exports the is_slice macro from the is_slice sub-crate.
- **Type inspection** (`typing_inspect_type`): re-exports the inspect_to_str_type_of and inspect_type_of macros from the inspect_type sub-crate.

Each capability is independently feature-gated. Enabling the top-level `enabled` feature without any typing sub-feature produces an empty but valid crate — all macros remain absent until their respective features are activated. This preserves compile-time isolation: only the sub-crates whose features are enabled are compiled.

The namespace re-export follows the own/orphan/exposed/prelude stack at each level. Items from sub-crate orphan namespaces are merged into the typing_tools own namespace; exposed and prelude namespaces propagate accordingly.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Crate root — namespace stack and top-level re-export |
| source | `src/typing.rs` | Aggregation module — per-feature namespace merging |
| test | `tests/tests.rs` | Test root re-using sub-crate tests via path includes |
| doc | `docs/invariant/001_feature_gated_activation.md` | Feature-gate correctness invariant |
