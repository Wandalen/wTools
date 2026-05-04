# Feature: Itertools Re-exports

### Scope

- **Purpose**: Provide a single versioned import point for `itertools` functionality across the wTools ecosystem.
- **Responsibility**: Document which `itertools` functions are selectively re-exported and the rationale for the selection.
- **In Scope**: 30+ re-exported combinators, comparison, and utility functions; `zip` from `core::iter`; `no_std`-compatible availability.
- **Out of Scope**: Exhaustive re-export of the full `itertools` API; parallel iteration; functions not yet adopted by the workspace.

### Design

`iter_tools` does not re-export all of `itertools`. Instead, it exposes a curated subset that covers the combinators used across the wTools workspace. This selective approach keeps the public API surface focused and avoids surfacing experimental or unstable itertools APIs.

`zip` is re-exported from `core::iter::zip` (not `itertools`) because the standard library implementation is available in all environments, including `no_std`, without any feature negotiation. Earlier code guarded the `zip` re-export behind `#[cfg(not(feature = "no_std"))]`, which was incorrect — `core::iter::zip` is always present. That guard was removed, making `zip` unconditionally available.

### Cross-References

| To | Type | Rationale |
|----|------|-----------|
| [src/iter.rs](../../src/iter.rs) | src | `orphan` module contains all re-export declarations. |
| [tests/inc/basic_test.rs](../../tests/inc/basic_test.rs) | test | Core re-export smoke tests including `multiunzip`. |
| [tests/inc/zip_test.rs](../../tests/inc/zip_test.rs) | test | Comprehensive `zip` tests including `no_std` feature coverage. |
| [examples/iter_tools_trivial.rs](../../examples/iter_tools_trivial.rs) | src | Usage demonstration of `min`, `rev`, and common combinators. |
| [api/003_reexports.md](../api/003_reexports.md) | doc | Complete catalogue of re-exported symbols with groupings. |
| [invariant/002_module_pattern.md](../invariant/002_module_pattern.md) | doc | Manual namespace chain that governs how re-exports are exposed. |
| [invariant/003_no_std_compatibility.md](../invariant/003_no_std_compatibility.md) | doc | Invariant requiring core iterator ops to be available in all configs. |

### Sources

| Source | Section |
|--------|---------|
| spec.md (deleted) | § Functionality § Re-exported Functions |
| spec.md (deleted) | § Usage Patterns § Pattern 1 |
| spec.md (deleted) | § Usage Patterns § Pattern 4 |
