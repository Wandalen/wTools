# API: Iterator Trait Contracts

### Scope

- **Purpose**: Define the formal bounds and relationships of `_IterTrait`, `IterTrait`, and `BoxedIter`.
- **Responsibility**: Specify what each type requires from implementors and what it guarantees to consumers.
- **In Scope**: Trait bound sets, lifetime parameters, `BoxedIter` alias definition, four `Clone` impl variants.
- **Out of Scope**: Re-exported combinators and extension methods — those are in `api/002` and `api/003`.

### Abstract

The iterator trait hierarchy solves the problem of cloning boxed iterator trait objects, which the standard library does not support. Three related types form the system: a base infrastructure trait, a user-facing trait that adds `Clone`, and a convenience type alias over the boxed base.

### Operations

`_IterTrait<'a, T>` requires: `Iterator<Item = T>`, `ExactSizeIterator`, `DoubleEndedIterator`, and `CloneDyn`. This is an infrastructure trait — the underscore prefix signals it is not intended for direct use by consumers.

`IterTrait<'a, T>` is a supertrait of `_IterTrait<'a, T>` and additionally requires `Clone`. This is the user-facing trait for code that needs a clonable iterator bound without boxing.

`BoxedIter<'a, T>` is a type alias for `Box<dyn _IterTrait<'a, T> + 'a>`. It is the primary heap-allocated iterator type.

Four `Clone` implementations cover `Box<dyn _IterTrait<'a, T>>` in all marker combinations: bare (no markers), `+Send`, `+Sync`, and `+Send+Sync`. Each implementation calls the `CloneDyn` vtable entry to produce a new boxed value.

### Error Handling

All errors are compile-time only. Incorrect bounds produce type errors at the call site. There are no runtime error paths in the trait system itself.

### Compatibility Guarantees

The trait bounds (`ExactSizeIterator`, `DoubleEndedIterator`, `CloneDyn`) are stable. Any concrete iterator type that satisfies these bounds is compatible with `BoxedIter`. Removing any bound would be a breaking change.

### Cross-References

| To | Type | Rationale |
|----|------|-----------|
| [src/iter.rs](../../src/iter.rs) | src | `_IterTrait`, `IterTrait`, `BoxedIter`, and all four `Clone` impls defined in `mod private`. |
| [feature/002_clonable_boxed_iterators.md](../feature/002_clonable_boxed_iterators.md) | doc | Feature rationale and design for this trait hierarchy. |
| [invariant/001_clone_contract.md](../invariant/001_clone_contract.md) | doc | Invariant guaranteeing `Clone` across all four marker combinations. |
| [api/002_iter_ext.md](002_iter_ext.md) | doc | Complementary extension API that builds on iterator types. |

### Sources

| Source | Section |
|--------|---------|
| spec.md (deleted) | § API Reference § Traits |
| spec.md (deleted) | § API Reference § Types |
