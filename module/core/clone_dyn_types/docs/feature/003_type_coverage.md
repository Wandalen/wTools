# Feature: Type Coverage

### Scope

**Purpose**: Define which concrete types CloneDyn supports out of the box.
**In Scope**: All `Clone` types, slices `[T]`, `str`, tuples (arity 0-12), arrays (all sizes).
**Out of Scope**: Non-Clone types; types requiring custom unsafe impls; async/pin types.

### Statement

All types implementing `Clone` automatically implement `CloneDyn` via blanket impl.
Slices `[T]` and `str` have explicit DST implementations. Tuples up to arity 12 and
arrays of all sizes are supported via Rust std blanket `Clone` impls.

### Type Support Table

| Category | Support | Notes |
|----------|---------|-------|
| Sized `Clone` types | Full | `i32`, `String`, `Vec<T>` — via blanket impl |
| Slices `[T]` | Full | Requires `&&[T]` double-reference for coercion |
| String slices `str` | Full | Requires `&&str` double-reference for coercion |
| Trait objects `dyn Trait` | Full | User implements `Clone for Box<dyn Trait>` |
| Tuples (arity 0-12) | Full | Rust std `Clone` arity limit |
| Arrays `[T; N]` | Full | All sizes via const generics |

### Acceptance Criteria

- AC-1: All primitive types (`i32`, `bool`, `char`, `u64`, `f64`) implement `CloneDyn`
- AC-2: `String` and `Vec<T: Clone>` implement `CloneDyn`
- AC-3: `[T: Clone]` implements `CloneDyn`; `str` implements `CloneDyn`
- AC-4: Tuples of arity 0 through 12 implement `CloneDyn`
- AC-5: Arrays `[T: Clone; N]` of any size implement `CloneDyn`

### Cross-References

- `invariant/003_usage_constraints.md` — constraints on DST coercion and arity limits
- `feature/002_dst_cloning.md` — the DST cloning capability this type coverage enables
