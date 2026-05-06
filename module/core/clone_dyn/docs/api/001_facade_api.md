# API: Facade Public Surface

### Scope

- **Purpose**: Define what `clone_dyn` exports and the conditions under which each export is available.
- **Responsibility**: Document re-exports, Cargo feature flags, and semver stability.
- **In Scope**: All items exported from `clone_dyn::*`.
- **Out of Scope**: Items only accessible by depending directly on `clone_dyn_meta` or `clone_dyn_types`.

### Abstract

`clone_dyn` is a facade crate re-exporting from `clone_dyn_types` (traits and functions) and `clone_dyn_meta` (proc-macro). Users depend only on `clone_dyn` — the sub-crates are implementation details.

### Operations

| Export | Feature Required | Source Crate | Purpose |
|--------|-----------------|--------------|---------|
| `CloneDyn` | `clone_dyn_types` | clone_dyn_types | Object-safe marker trait enabling type-erased cloning |
| `clone_into_box` | `clone_dyn_types` | clone_dyn_types | Clone a `Box<dyn Trait>` via fat pointer manipulation |
| `clone` | `clone_dyn_types` | clone_dyn_types | Convenience clone for sized types |
| `#[clone_dyn]` | `derive_clone_dyn` | clone_dyn_meta | Attribute macro generating Clone impls |

**Feature flags:**

| Feature | Enables |
|---------|---------|
| `enabled` | Core re-export wiring |
| `clone_dyn_types` | Re-exports `CloneDyn`, `clone`, `clone_into_box` |
| `derive_clone_dyn` | Re-exports `#[clone_dyn]` proc-macro |
| `full` | All features |

### Error Handling

All errors are compile-time only. No runtime error types are exported. Macro misuse produces a standard compiler error.

### Compatibility Guarantees

- SemVer 2.0.0. Breaking changes require a major version bump.
- The three crates are tightly coupled and must be published with synchronized version numbers.
- Publish order: `clone_dyn_types` → `clone_dyn_meta` → `clone_dyn`.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../feature/001_macro_usage.md` | How to use the re-exported macro |
| doc | `../invariant/001_box_only.md` | Box-only constraint on all exports |
