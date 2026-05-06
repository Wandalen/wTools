# Pattern: Sealed Trait

### Scope

- **Purpose**: Prevent external implementations of `CloneDyn` to guarantee memory safety.
- **Responsibility**: Document the mechanism, applicability, and trade-offs of the sealed trait pattern as applied to `CloneDyn`.
- **In Scope**: `Sealed` supertrait; `DontCallMe` marker; `CloneDyn: Sealed` relationship.
- **Out of Scope**: Public API surface; external crate usage patterns.

### Problem

`CloneDyn::__clone_dyn` returns a raw `*mut ()`. Any unsound external implementation could return a mistyped or invalid pointer, causing undefined behavior in `clone_into_box`. The type system alone cannot prevent external `impl CloneDyn for T` blocks — `pub` traits are normally open for implementation.

### Solution

A private `Sealed` supertrait (in `mod private`) is added to `CloneDyn`. The `__clone_dyn` method takes a private `DontCallMe` marker parameter. Because `Sealed` is inaccessible outside the crate, no external type can satisfy `CloneDyn: Sealed`, making external `impl CloneDyn` impossible at compile time. All implementations are confined to this crate where they can be verified for soundness.

| Component | Responsibility |
|-----------|----------------|
| `Sealed` | Private supertrait in `mod private`; unreachable by external crates |
| `DontCallMe` | Private marker struct; prevents direct `__clone_dyn` invocation |
| `CloneDyn: Sealed` | Public trait with private supertrait; external impl impossible |

### Applicability

Use when a public trait exposes an unsafe method whose correctness invariants cannot be statically verified for arbitrary implementations. Appropriate when all valid implementations can be provided within the defining crate and external customisation must be prohibited.

### Consequences

External crates see a fully public `CloneDyn` trait but cannot implement it — the compiler error is clear (`Sealed` not in scope). Documentation must note the sealed status. The trade-off is no user customisation of the clone mechanism, which is intentional for memory safety.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../invariant/002_memory_safety.md` | Safety guarantee this pattern enforces |
| doc | `../api/001_clone_dyn_trait.md` | `CloneDyn` definition that uses this pattern |
