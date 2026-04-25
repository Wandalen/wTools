# Pattern: Sealed Trait

### Scope

**Purpose**: Prevent external implementations of `CloneDyn` to guarantee memory safety.
**In Scope**: `Sealed` supertrait; `DontCallMe` marker; `CloneDyn: Sealed` relationship.
**Out of Scope**: Public API surface; external crate usage patterns.

### Statement

The sealed trait pattern uses a private supertrait (`Sealed`) and a private marker
type (`DontCallMe`) to prevent external crates from implementing `CloneDyn`, while
keeping the trait itself `pub`. All `CloneDyn` implementations are confined to
this crate where they can be verified for soundness.

### Components

| Component | Responsibility |
|-----------|----------------|
| `Sealed` | Private supertrait in `mod private`; unreachable by external crates |
| `DontCallMe` | Private marker struct; prevents direct `__clone_dyn` invocation |
| `CloneDyn: Sealed` | Public trait with private supertrait; external impl impossible |

### Why This Pattern

`CloneDyn::__clone_dyn` returns a raw `*mut ()`. Any unsound external implementation
could return a mistyped or invalid pointer, causing undefined behavior in
`clone_into_box`. Sealing eliminates this attack surface entirely — only
crate-internal implementations exist, all verified for correctness.

### Cross-References

- `invariant/002_memory_safety.md` — safety guarantee this pattern enforces
- `api/001_clone_dyn_trait.md` — `CloneDyn` definition that uses this pattern
