# Invariant: Feature-Gated Sub-Crate Activation

### Scope

- **Purpose**: Guarantee that each aggregated sub-crate's macros are only compiled and accessible when the corresponding feature flag is explicitly enabled, preventing unwanted symbol exposure and compile-time bloat.
- **Responsibility**: Documents the feature-gated activation invariant — its precise statement, how it is enforced, and what breaks if it is violated.
- **In Scope**: The relationship between `typing_implements`, `typing_is_slice`, and `typing_inspect_type` feature flags and the corresponding namespace re-exports in `src/typing.rs`.
- **Out of Scope**: The `enabled` feature gate itself (controls top-level module visibility), individual sub-crate feature management.

### Invariant Statement

For all build configurations: no macro from a sub-crate is reachable through typing_tools unless the sub-crate's corresponding typing feature flag is active. Enabling `enabled` alone, without any typing sub-feature, produces a crate with no public items.

### Enforcement Mechanism

Each namespace re-export in `src/typing.rs` is wrapped in a cfg attribute keyed to the respective typing feature. The Rust compiler enforces that any path through the own/orphan/exposed/prelude stack reaching a sub-crate item is blocked unless the feature is enabled. Because each sub-crate dependency is declared as optional, the linker also excludes unactivated sub-crates from the build entirely.

### Violation Consequences

If a re-export escapes its feature guard, callers requesting only a subset of typing features would unexpectedly receive macros from other sub-crates, breaking build reproducibility and potentially causing name collisions. Compile-time isolation guarantees would be lost — callers that opted out of specific sub-crates for binary size or dependency reasons would silently include them.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/typing.rs` | Feature-guarded re-exports — authoritative enforcement site |
| config | `Cargo.toml` | Optional sub-crate dependencies and feature flag declarations |
| doc | `docs/feature/001_unified_typing_toolset.md` | Feature whose isolation guarantee this invariant protects |
