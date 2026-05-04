# Feature: Clonable Boxed Iterators

### Scope

- **Purpose**: Enable `Clone` on `Box<dyn Iterator>` trait objects, which the standard library does not support.
- **Responsibility**: Document the design of the `_IterTrait` / `IterTrait` / `BoxedIter` system and its cloneability guarantees.
- **In Scope**: `_IterTrait`, `IterTrait`, `BoxedIter` type alias; four marker-combination `Clone` impls (`bare`, `+Send`, `+Sync`, `+Send+Sync`).
- **Out of Scope**: Iterator adapters, combinators, and extension methods — those are covered in `feature/001` and `feature/003`.

### Design

Standard Rust does not allow `Box<dyn Iterator>` to implement `Clone` because `Clone` is not object-safe (it takes `self` by value). `iter_tools` solves this via `CloneDyn` from the `clone_dyn_types` crate, which provides an object-safe clone mechanism via a vtable entry.

Two trait levels are defined to separate concerns:

- `_IterTrait<'a, T>` — the base bounds contract: `Iterator + ExactSizeIterator + DoubleEndedIterator + CloneDyn`. The underscore prefix signals this is an infrastructure trait, not intended for direct use.
- `IterTrait<'a, T>` — extends `_IterTrait` with the `Clone` bound. This is the user-facing trait.

`BoxedIter<'a, T>` is a type alias for `Box<dyn _IterTrait<'a, T> + 'a>`. It is the primary consumer-facing type.

Four `Clone` implementations cover all combinations of the `Send` and `Sync` marker traits, ensuring that `BoxedIter` can be cloned regardless of which markers the concrete type carries.

### Cross-References

| To | Type | Rationale |
|----|------|-----------|
| [src/iter.rs](../../src/iter.rs) | src | `_IterTrait`, `IterTrait`, `BoxedIter` and all four `Clone` impls. |
| [Cargo.toml](../../Cargo.toml) | config | `clone_dyn_types` dependency that enables object-safe `CloneDyn`. |
| [api/001_iter_traits.md](../api/001_iter_traits.md) | doc | Formal API contract for all three iterator trait types. |
| [invariant/001_clone_contract.md](../invariant/001_clone_contract.md) | doc | Invariant guaranteeing `Clone` across all four marker combinations. |
| [invariant/002_module_pattern.md](../invariant/002_module_pattern.md) | doc | Manual namespace chain that governs how traits are exposed. |

### Sources

| Source | Section |
|--------|---------|
| spec.md (deleted) | § Functionality § Clonable Boxed Iterators |
| spec.md (deleted) | § Usage Patterns § Pattern 2 |
