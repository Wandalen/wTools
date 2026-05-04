# Invariant: no_std Core Compatibility

### Scope

- **Purpose**: Require that core iterator operations remain available regardless of whether the `no_std` feature is enabled.
- **Responsibility**: State the compatibility requirement, document the enforcement mechanism, and record the historical violation that motivated this invariant.
- **In Scope**: `zip` re-export from `core::iter::zip`; feature flag interactions for core iterator ops.
- **Out of Scope**: Allocation-dependent operations, which legitimately require the `use_alloc` feature.

### Invariant Statement

All iterator operations sourced from `core::iter` must be unconditionally re-exported. No `#[cfg]` guard may restrict their availability based on the `no_std` feature. The `zip` function is the canonical example: it must be available in all feature configurations.

### Enforcement Mechanism

`zip` is declared as `pub use core::iter::zip;` in `src/iter.rs` without any `#[cfg]` attribute. The dedicated test file `tests/inc/zip_test.rs` includes a test that explicitly enables `--all-features` (which activates `no_std`) and verifies `zip` compiles and functions correctly.

### Violation Consequences

Gating a `core::iter` re-export behind `#[cfg(not(feature = "no_std"))]` silently removes it when `no_std` is enabled. Consumers using `--all-features` (a common CI configuration) encounter a compile error at the call site with no clear indication of which feature flag removed the function.

#### Historical Violation

`src/iter.rs:267` previously had `#[cfg(not(feature = "no_std"))]` guarding the `zip` re-export. This was incorrect because `core::iter::zip` is available in both `std` and `no_std` environments. The guard was removed, `zip` became unconditionally available, and `tests/inc/zip_test.rs` was added to prevent regression.

### Cross-References

| To | Type | Rationale |
|----|------|-----------|
| [src/iter.rs](../../src/iter.rs) | src | `pub use core::iter::zip` without `#[cfg]` guard. |
| [tests/inc/zip_test.rs](../../tests/inc/zip_test.rs) | test | Regression test covering `zip` availability under `--all-features`. |
| [feature/001_itertools_reexports.md](../feature/001_itertools_reexports.md) | doc | Re-export feature that includes `zip` in its scope. |
| [api/003_reexports.md](../api/003_reexports.md) | doc | Re-exported function catalogue noting the corrected `zip` availability. |

### Sources

| Source | Section |
|--------|---------|
| spec.md (deleted) | § Architecture § no_std Support |
| tests/manual/readme.md | § Issues Log § Resolved Issues § Issue #1 |
