# Invariant: same_data Validates Sizes Before Byte Comparison

### Scope

- **Purpose**: Guarantee that the byte-content comparison function never accesses memory beyond the bounds of either referenced value.
- **Responsibility**: Documents the size pre-check safety contract that makes the internal unsafe byte comparison safe under all inputs.
- **In Scope**: The same_data precondition — size equality must be confirmed before the byte comparison executes.
- **Out of Scope**: Pointer alignment, padding byte behavior during comparison, and type safety — the type-agnostic contract is covered by invariant/001.

### Invariant Statement

same_data always verifies that both referenced values occupy the same number of bytes before performing any byte comparison. If the sizes differ, same_data returns false immediately without reading any bytes from either region. The byte comparison is reached only when both sizes are confirmed equal, making out-of-bounds memory access structurally impossible regardless of the types or values compared.

### Enforcement Mechanism

The implementation calls same_size as the first operation in same_data and returns false on mismatch. The byte comparison is unreachable unless same_size returned true in the same invocation. This ordering is internal and structural — no caller can bypass the early return, and the unsafe byte comparison is never invoked on unequal-length regions.

### Violation Consequences

If the size pre-check were removed or bypassed, the byte comparison would read beyond the end of the shorter value's memory region, producing undefined behavior. The risk is compounded by the cross-type design: comparing a one-byte value against a four-byte value without the size guard would silently read three bytes past the end of the smaller region, with behavior dependent on whatever occupies that memory.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/mem.rs` | Enforces size pre-check before unsafe byte comparison |
| test | `tests/corner_cases_test.rs` | Tests size-mismatch early return and boundary conditions |
| doc | [feature/001_memory_comparison.md](../feature/001_memory_comparison.md) | Memory comparison feature that contains same_data |
| doc | [api/001_comparison_functions.md](../api/001_comparison_functions.md) | API contract for same_data — the public safety guarantee |
| doc | [invariant/001_type_agnostic_comparison.md](001_type_agnostic_comparison.md) | Companion invariant governing type-agnostic parameter design |
