# Invariant: Memory Safety

### Scope

- **Purpose**: Guarantee soundness of all unsafe pointer operations in `clone_into_box`.
- **Responsibility**: Define the invariants that all callers and impls of `__clone_dyn` must uphold.
- **In Scope**: The `unsafe` block in `clone_into_box`; the `__clone_dyn` return value contract.
- **Out of Scope**: Safe `CloneDyn` impls in user code; safe wrapper functions.

### Invariant Statement

All of the following MUST hold for every call to `clone_into_box`:
1. The `*mut ()` returned by `__clone_dyn` points to a heap allocation that exactly
   matches the memory layout of the original type `T`.
2. The fat pointer metadata (vtable pointer or slice length) is preserved verbatim
   from the input reference to the output `Box<T>`.
3. `Box::from_raw` is called exactly once on the reconstructed fat pointer.
4. No double-free and no memory leak occur across any execution path.

### Enforcement Mechanism

- Miri (`cargo +nightly miri test`) detects undefined behavior and double-free at runtime.
- ASAN (`-Zsanitizer=address` under nightly) detects heap corruption.
- The sealed trait guarantees all `CloneDyn` implementations originate in this crate;
  no user impl can violate the `__clone_dyn` return value contract.

### Violation Consequences

Undefined behavior: memory corruption, segfault, or silent data corruption.
Severity: critical — any violation collapses all safety guarantees of the public API.
No recovery path; the program is in an undefined state.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../algorithm/001_fat_pointer_surgery.md` | Implementation these invariants govern |
| doc | `../pattern/001_sealed_trait.md` | Sealing mechanism that enforces the contract |
| doc | `../api/001_clone_dyn_trait.md` | `__clone_dyn` method whose return value is governed here |
