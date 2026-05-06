# API: Iterator Extensions

### Scope

- **Purpose**: Define the formal contract for the `IterExt` extension trait and its `map_result` method.
- **Responsibility**: Specify the blanket impl conditions, method signature, and stop-on-first-error collection semantics.
- **In Scope**: `IterExt` trait; `map_result` method; required bounds on the iterator and error type.
- **Out of Scope**: Trait hierarchy and `BoxedIter` — those are in `api/001`; re-exported combinators — those are in `api/003`.

### Abstract

`IterExt` is a blanket extension trait that provides `map_result` to any `Clone + Iterator`. It processes each item through a fallible mapping function and either collects all results into a `Vec` or short-circuits on the first error.

### Operations

`IterExt` is auto-implemented for all `Self` where `Self: Clone + Iterator`. No explicit implementation is required.

`map_result<F, El, RE>(self, f: F) -> Result<Vec<El>, RE>` — applies `f` to each item. If `f` returns `Ok(el)`, the element is collected. If `f` returns `Err(e)`, iteration stops immediately and `Err(e)` is returned. On success, returns `Ok(vec)` containing all mapped elements in order.

Bounds: `Self` must be `Sized + Clone`. `F` must be `FnMut(Self::Item) -> Result<El, RE>`. `RE` must be `Debug`.

### Error Handling

`map_result` returns `Err(RE)` on the first error produced by `f`. Elements processed before the error are discarded. There is no partial-success result; the return type is `Result<Vec<El>, RE>`.

### Compatibility Guarantees

The `Clone` bound on `Self` and the `Debug` bound on `RE` are stable requirements. Removing either would change observable behavior and constitutes a breaking change.

### Cross-References

| To | Type | Rationale |
|----|------|-----------|
| [src/iter.rs](../../src/iter.rs) | src | `IterExt` trait and blanket impl using `::itertools::process_results`. |
| [feature/003_iter_ext.md](../feature/003_iter_ext.md) | doc | Feature rationale, design decisions, and `Clone` requirement explanation. |
| [api/001_iter_traits.md](001_iter_traits.md) | doc | Complementary trait hierarchy API. |

### Sources

| Source | Section |
|--------|---------|
| spec.md (deleted) | § API Reference § Traits § IterExt |
| spec.md (deleted) | § Usage Patterns § Pattern 3 |
