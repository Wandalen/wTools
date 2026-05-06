# API: Re-exported Functions

### Scope

- **Purpose**: Catalogue all symbols re-exported by `iter_tools` with their origin and grouping.
- **Responsibility**: Serve as the authoritative list of what is available via `use iter_tools::*`.
- **In Scope**: All functions, types, and traits in the `orphan` re-export module; `zip` from `core::iter`.
- **Out of Scope**: Symbols from `_IterTrait`, `IterTrait`, `BoxedIter`, and `IterExt` — those are in `api/001` and `api/002`.

### Abstract

`iter_tools` re-exports a curated subset of `itertools` plus `zip` from the standard library. All symbols are available via `use iter_tools::*` when the `enabled` feature is active.

### Operations

**Combinators**

`interleave`, `interleave_shortest`, `intersperse`, `intersperse_with`, `chain`, `zip_longest`, `flatten`, `enumerate`, `step_by`, `take`, `take_while`, `skip`, `skip_while`, `peekable`.

**Sorting and comparison**

`min`, `max`, `min_by`, `max_by`, `min_by_key`, `max_by_key`, `sort_by`, `sort_by_key`.

**Unzipping**

`multiunzip` — unzips an iterator of tuples into a tuple of collections.

**Folding and collecting**

`fold`, `collect`, `for_each`, `count`, `sum`, `product`.

**Result processing**

`process_results` — iterates while mapping with a fallible function; stops on first error.

**Standard library re-export**

`zip` — re-exported from `core::iter::zip`. Available in all configurations including `no_std`. Note: earlier versions incorrectly gated this export behind `#[cfg(not(feature = "no_std"))]`; that guard was removed because `core::iter::zip` is always present regardless of the `no_std` feature.

**Traits**

`Itertools` — the main extension trait from the `itertools` crate.

### Error Handling

All re-exported functions follow the error semantics defined by their origin crate (`itertools` or `core::iter`). `process_results` returns `Result`; all other re-exported functions are infallible.

### Compatibility Guarantees

The re-exported set is pinned to the `itertools` version declared in the workspace `Cargo.toml`. Adding new re-exports is non-breaking. Removing existing re-exports is a breaking change.

### Cross-References

| To | Type | Rationale |
|----|------|-----------|
| [src/iter.rs](../../src/iter.rs) | src | `orphan` module containing all re-export declarations. |
| [feature/001_itertools_reexports.md](../feature/001_itertools_reexports.md) | doc | Feature rationale and design for the selective re-export approach. |
| [invariant/002_module_pattern.md](../invariant/002_module_pattern.md) | doc | Manual namespace chain that governs how re-exports flow to consumers. |
| [invariant/003_no_std_compatibility.md](../invariant/003_no_std_compatibility.md) | doc | Invariant requiring `zip` and core ops to be available in all configs. |

### Sources

| Source | Section |
|--------|---------|
| spec.md (deleted) | § API Reference § Re-exported from itertools |
| spec.md (deleted) | § Out of Scope (corrected: zip IS re-exported) |
