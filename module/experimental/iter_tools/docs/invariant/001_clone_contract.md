# Invariant: Clone Contract for Boxed Iterators

### Scope

- **Purpose**: Guarantee that `Box<dyn _IterTrait>` is `Clone` for all four combinations of `Send` and `Sync` markers.
- **Responsibility**: State the contract, identify where it is enforced, and describe the consequence of violating it.
- **In Scope**: The four explicit `Clone` impl blocks in `src/iter.rs`; the `CloneDyn` dependency.
- **Out of Scope**: The design rationale for why clonable boxed iterators are needed — see `feature/002`.

### Invariant Statement

`Box<dyn _IterTrait<'a, T> + 'a>`, `Box<dyn _IterTrait<'a, T> + Send + 'a>`, `Box<dyn _IterTrait<'a, T> + Sync + 'a>`, and `Box<dyn _IterTrait<'a, T> + Send + Sync + 'a>` are each `Clone`. This guarantee must hold for every concrete type `T: 'a`.

### Enforcement Mechanism

Four explicit `Clone` implementation blocks in `src/iter.rs` (`mod private`) cover each marker combination. Each block calls `self.clone_box()` from the `CloneDyn` vtable entry provided by the `clone_dyn_types` dependency. Removing any impl block breaks the invariant at compile time for callers using that marker combination.

### Violation Consequences

A consumer that stores `BoxedIter` and calls `.clone()` on it receives a compile error if the corresponding marker-combination impl is missing. The failure is always caught at compile time — there are no runtime consequences.

### Cross-References

| To | Type | Rationale |
|----|------|-----------|
| [src/iter.rs](../../src/iter.rs) | src | The four `Clone` impl blocks that enforce this invariant. |
| [feature/002_clonable_boxed_iterators.md](../feature/002_clonable_boxed_iterators.md) | doc | Feature rationale explaining why clonable boxed iterators are needed. |
| [api/001_iter_traits.md](../api/001_iter_traits.md) | doc | Formal API contract for `_IterTrait`, `IterTrait`, and `BoxedIter`. |

### Sources

| Source | Section |
|--------|---------|
| spec.md (deleted) | § Design Decisions § Clonable Iterator Objects |
