# Feature: Iterator Extensions

### Scope

- **Purpose**: Provide result-oriented iterator processing that short-circuits on the first error.
- **Responsibility**: Document the `IterExt` extension trait, its blanket implementation, and the `map_result` method semantics.
- **In Scope**: `IterExt` trait; `map_result` method; stop-on-first-error collection semantics; `Clone` requirement rationale.
- **Out of Scope**: Re-exported combinators (covered in `feature/001`); boxed iterator infrastructure (covered in `feature/002`).

### Design

`IterExt` provides `map_result` as a blanket impl over all `Clone + Iterator` types. The blanket impl means any iterator that is `Clone` automatically gets this method without requiring explicit implementation.

`map_result` delegates internally to `::itertools::process_results`, which handles the stop-on-first-error semantics: the iterator is consumed until a mapping function returns `Err`, at which point iteration stops and the error is returned. On success, all mapped values are collected into a `Vec`.

The `Clone` bound on the iterator is required because `process_results` needs to be able to restart iteration from a known state. Without `Clone`, it would not be possible to inspect the iterator without consuming it.

The `Debug` bound on the error type `RE` surfaces meaningful diagnostics when `map_result` is used in test assertions.

### Cross-References

| To | Type | Rationale |
|----|------|-----------|
| [src/iter.rs](../../src/iter.rs) | src | `IterExt` trait definition and blanket impl using `process_results`. |
| [api/002_iter_ext.md](../api/002_iter_ext.md) | doc | Formal API contract for `IterExt::map_result`. |
| [invariant/002_module_pattern.md](../invariant/002_module_pattern.md) | doc | Manual namespace chain that governs how `IterExt` is exposed. |
| [feature/001_itertools_reexports.md](001_itertools_reexports.md) | doc | Complementary re-export feature that shares the same namespace chain. |

### Sources

| Source | Section |
|--------|---------|
| spec.md (deleted) | § Functionality § Iterator Extensions |
| spec.md (deleted) | § Usage Patterns § Pattern 3 |
