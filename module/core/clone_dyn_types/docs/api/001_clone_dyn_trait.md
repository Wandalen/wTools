# API: CloneDyn Trait

### Scope

**Purpose**: Define the public `CloneDyn` trait contract for DST cloning.
**In Scope**: Trait definition, blanket implementations, sealing mechanism.
**Out of Scope**: The `clone_into_box` function; procedural macro generation.

### Statement

`CloneDyn` is a sealed, object-safe trait that enables type-erased cloning of trait
objects and DSTs without requiring `Clone` as a supertrait (which would violate
object safety).

### Interface

```rust
pub trait CloneDyn : Sealed
{
  #[ doc( hidden ) ]
  fn __clone_dyn( &self, _ : DontCallMe ) -> *mut ();
}
```

### Blanket Implementations

```rust
// All sized Clone types automatically implement CloneDyn
impl< T : Clone > CloneDyn for T { ... }

// Slices of Clone elements
impl< T : Clone > CloneDyn for [ T ] { ... }

// String slices
impl CloneDyn for str { ... }
```

### Constraints

- External crates CANNOT implement `CloneDyn` (sealed — `Sealed` supertrait is private).
- `__clone_dyn` MUST NOT be called directly; use `clone_into_box` instead.
- `CloneDyn` is NOT a subtrait of `Clone` — object safety is intentionally maintained.
- `CloneDyn` is `?Sized` — works for both sized and unsized types.

### Cross-References

- `pattern/001_sealed_trait.md` — sealing mechanism used by this trait
- `api/002_clone_into_box.md` — public function that invokes `__clone_dyn`
- `invariant/002_memory_safety.md` — `__clone_dyn` return value contract
