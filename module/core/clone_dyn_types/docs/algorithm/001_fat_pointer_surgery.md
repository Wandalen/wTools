# Algorithm: Fat Pointer Surgery

### Scope

**Purpose**: Enable DST cloning by selectively replacing the data component of a fat pointer.
**In Scope**: `clone_into_box` implementation for `?Sized + CloneDyn` types.
**Out of Scope**: Sized type cloning (uses direct `Clone::clone`).

### Statement

DST types (`[T]`, `str`, `dyn Trait`) are represented as fat pointers: a pair of
(data_ptr, metadata). The metadata encodes the slice length or vtable pointer.
Fat pointer surgery replaces only the data_ptr with a freshly-cloned heap allocation
while preserving the metadata verbatim, enabling correct `Box<T>` reconstruction.

### Steps

1. Cast `&T` reference to `*const T` fat pointer (preserves metadata in the pointer).
2. Call `__clone_dyn` to clone only the data; it returns a raw `*mut ()` (data pointer).
3. Overwrite the data-pointer component of the fat pointer with the returned pointer.
4. Call `Box::from_raw` on the modified fat pointer to take ownership.

### Implementation

```rust
unsafe
{
  let mut ptr = ref_dyn as *const T;
  let data_ptr = &mut ptr as *mut *const T as *mut *mut ();
  *data_ptr = < T as CloneDyn >::__clone_dyn( ref_dyn, DontCallMe );
  Box::from_raw( ptr as *mut T )
}
```

### Invariants

- Data pointer replacement MUST occur before `Box::from_raw`.
- The metadata component of the fat pointer MUST NOT be modified.
- All pointer operations MUST be confined to the `clone_into_box` function.

### Cross-References

- `invariant/002_memory_safety.md` — safety guarantees this algorithm must uphold
- `api/001_clone_dyn_trait.md` — `__clone_dyn` that supplies the cloned data pointer
- `api/002_clone_into_box.md` — the function that contains this algorithm
