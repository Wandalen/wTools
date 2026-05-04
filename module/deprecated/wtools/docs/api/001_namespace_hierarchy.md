# API: Namespace Hierarchy

### Scope

- **Purpose**: Define the five-level re-export namespace that all wtools consumers depend on for predictable import paths.
- **Responsibility**: Document the dependency, own, orphan, exposed, and prelude namespace layers and their access semantics.
- **In Scope**: Five namespace layers, module alias mapping, crate root re-export contract, feature-gated module availability.
- **Out of Scope**: Individual sub-crate internal APIs (see each constituent crate's docs/api/), feature flag details (see feature/).

### Abstract

wtools organizes its public surface into five hierarchical namespace layers. Each aggregated sub-crate's exports flow through these layers from raw dependency access to prelude convenience. The crate root re-exports the own namespace, making aliased modules available directly from the crate path.

### Operations

#### Namespace Layers

| Layer | Access Pattern | Content |
|-------|---------------|---------|
| dependency | crate::dependency::{source_crate} | Raw access to the underlying crate under its original name |
| own | crate::{alias} | Short module aliases (iter, meta, mem, typing, time, string, error, derive, dt, diagnostics) |
| orphan | crate::orphan::* | Re-exports from exposed (passthrough) |
| exposed | crate::exposed::* | Combined exposed items from all enabled sub-crates |
| prelude | crate::prelude::* | Combined prelude items from all enabled sub-crates |

#### Module Alias Mapping

| Alias | Source Crate | Feature Gate |
|-------|-------------|--------------|
| iter | iter_tools | iter |
| meta | meta_tools | meta |
| mem | mem_tools | mem |
| typing | typing_tools | typing |
| time | time_tools | time |
| string | strs_tools | string |
| error | error_tools | error |
| derive | derive_tools | derive |
| dt | data_type | dt or data_type |
| diagnostics | diagnostics_tools | diagnostics |

The crate root re-exports own::*, so all aliased modules are accessible directly from the crate path without qualifying through the own layer.

#### Feature-Gated Availability

Each module in the dependency and own layers is conditionally compiled based on its corresponding feature flag. When a feature is disabled, the module does not exist in the namespace — no stub, no empty module. The exposed and prelude layers aggregate only from enabled modules.

### Error Handling

No runtime errors. Feature misconfiguration manifests as compile-time errors: attempting to use a module whose feature flag is not enabled produces an unresolved import error at compile time. This is the intended behavior — the error message indicates which feature flag to enable.

### Compatibility Guarantees

The namespace layer names (dependency, own, orphan, exposed, prelude) and the module alias names are stable public API. Adding new aggregated modules is backward-compatible. Removing or renaming an existing alias is a breaking change requiring a major version bump.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/lib.rs` | Namespace module implementations (all 225 lines) |
| config | `../../Cargo.toml` | Dependency declarations and feature-to-crate mapping |
| doc | `../pattern/001_ecosystem_aggregation.md` | Rationale for the aggregation approach |
| doc | `../pattern/002_feature_flag_composition.md` | Feature flag hierarchy pattern |
