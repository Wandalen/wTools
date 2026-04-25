# API: clone_into_box and clone

### Scope

**Purpose**: Provide the safe public API for cloning DSTs and trait objects.
**In Scope**: `clone_into_box` for unsized types; `clone` convenience function for sized types.
**Out of Scope**: `CloneDyn` trait internals; direct `__clone_dyn` invocation.

### Statement

`clone_into_box` clones any `CloneDyn` type into a correctly-typed `Box<T>`,
encapsulating all unsafe fat-pointer operations. `clone` is a convenience wrapper
around standard `Clone::clone` for sized types.

### Interface

```rust
pub fn clone_into_box< T >( ref_dyn : &T ) -> Box< T >
where
  T : ?Sized + CloneDyn
```

```rust
pub fn clone< T >( src : &T ) -> T
where
  T : Clone
```

### Usage Patterns

```rust
// Trait object cloning
impl Clone for Box< dyn MyTrait >
{
  fn clone( &self ) -> Self
  {
    clone_dyn_types::clone_into_box( &**self )
  }
}

// DST slice cloning (double-reference required for coercion)
let slice : &[ i32 ] = &[ 1, 2, 3 ];
let boxed : Box< [ i32 ] > = clone_dyn_types::clone_into_box( &slice as &dyn CloneDyn );

// Sized type convenience clone
let original = MyStruct { value : 42 };
let cloned = clone_dyn_types::clone( &original );
```

### Cross-References

- `algorithm/001_fat_pointer_surgery.md` — implementation of `clone_into_box`
- `api/001_clone_dyn_trait.md` — `CloneDyn` trait this function operates on
- `invariant/003_usage_constraints.md` — double-reference requirement for DSTs
