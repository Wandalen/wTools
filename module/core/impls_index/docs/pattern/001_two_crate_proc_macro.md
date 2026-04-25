# Pattern: Two-Crate Proc Macro

### Scope

- **Purpose**: Govern the structural separation between a runtime macro crate and its companion proc macro crate.
- **Responsibility**: Document the two-crate architecture applied by `impls_index` + `impls_index_meta`.
- **In Scope**: Crate boundary placement, feature flag propagation, namespace re-export from the proc macro crate.
- **Out of Scope**: Individual macro implementations (→ `api/`), user-facing features (→ `feature/`).

### Problem

Rust requires proc macros to reside in a dedicated crate with a dedicated manifest flag designating it as a proc macro crate. Mixing proc macro code with regular library code in one crate is not permitted by the toolchain. Additionally, proc macro crates carry heavy build-time dependencies that should not be forced on users who only need the simpler declarative macro variants.

### Solution

Split into two crates:

- **Runtime crate** (`impls_index`): contains declarative macros and re-exports the proc macro from the companion crate. Users add only this crate to their dependency list. Feature-gated to control conditional compilation.
- **Proc macro crate** (`impls_index_meta`): contains only the proc macro implementation (`impls3!`). Its heavy build-time proc macro framework dependencies are isolated here and do not affect compile times for users who do not enable the proc macro feature.

The runtime crate re-exports the proc macro directly into its own namespace, so callers use `impls_index::impls3!` without knowing about or depending on the companion crate. The companion crate is a transparent implementation detail managed by the workspace.

### Applicability

Apply this pattern when:
- A crate requires at least one proc macro alongside declarative macros or regular functions.
- The proc macro's build-time dependencies should not impose compilation cost on users who do not need the proc macro variant.
- All variants — declarative and proc macro — should share a single user-facing namespace without requiring users to add multiple crates to their manifest.

### Consequences

- Users depend on exactly one crate (`impls_index`) regardless of which macro variant they choose.
- `impls_index_meta` is a workspace-internal dependency — users do not declare it in their own manifests.
- Workspace `Cargo.toml` must declare both crates as members and wire the dependency.
- Feature flags in the runtime crate propagate to the companion crate through the workspace dependency declaration.
- Builds that do not enable the proc macro feature compile `impls_index_meta` only as a dependency reference; the heavy proc macro dependencies are compiled only when the feature is active.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `Cargo.toml` | Declares `impls_index_meta` as workspace dependency with feature propagation |
| source | `src/implsindex/mod.rs` | Re-exports `impls_index_meta` proc macro into the runtime namespace |
| doc | `docs/invariant/002_compile_time_resolution.md` | Compile-time guarantee upheld across both crates |
