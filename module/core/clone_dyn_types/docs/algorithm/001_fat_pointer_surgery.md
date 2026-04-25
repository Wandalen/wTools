# Algorithm: Fat Pointer Surgery

### Scope

- **Purpose**: Enable DST cloning by selectively replacing the data component of a fat pointer.
- **Responsibility**: Specify each step of the pointer manipulation, its preconditions, and safety requirements.
- **In Scope**: `clone_into_box` implementation for `?Sized + CloneDyn` types.
- **Out of Scope**: Sized type cloning (uses direct `Clone::clone`).

### Abstract

DST types (`[T]`, `str`, `dyn Trait`) are represented as fat pointers: a pair of (data_ptr, metadata). The metadata encodes the slice length or vtable pointer. Fat pointer surgery replaces only the data_ptr with a freshly-cloned heap allocation while preserving the metadata verbatim, enabling correct `Box<T>` reconstruction.

### Algorithm

**Step 1 — Capture fat pointer.** Cast `&T` reference to `*const T` fat pointer. This preserves both components: the data address and the metadata (vtable or length).

**Step 2 — Clone the data.** Call `__clone_dyn` to produce a heap copy of the value. It returns a raw `*mut ()` carrying only the data pointer — no metadata.

**Step 3 — Swap the data component.** Overwrite the data-pointer word of the fat pointer with the address returned by `__clone_dyn`. The metadata word is left unchanged.

**Step 4 — Reconstruct ownership.** Call `Box::from_raw` on the modified fat pointer to transfer ownership of the cloned heap allocation into a correctly-typed `Box<T>`.

**Invariants during execution**

- Data pointer replacement MUST occur before `Box::from_raw`.
- The metadata component of the fat pointer MUST NOT be modified.
- All pointer operations MUST be confined to the `clone_into_box` function.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../invariant/002_memory_safety.md` | Safety guarantees this algorithm must uphold |
| doc | `../api/001_clone_dyn_trait.md` | `__clone_dyn` that supplies the cloned data pointer |
| doc | `../api/002_clone_into_box.md` | The function that contains this algorithm |
| source | `../../src/lib.rs` | Canonical implementation |
