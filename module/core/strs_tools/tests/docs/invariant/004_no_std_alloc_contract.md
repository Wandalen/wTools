# Test Surface: Invariant — No-Std Alloc Contract

### Source

- **Doc Instance:** [invariant/004_no_std_alloc_contract.md](../../../docs/invariant/004_no_std_alloc_contract.md)

### Cases

| # | Status | Case |
|---|--------|------|
| IN-1 | ✅ | Core operations compile in no_std plus alloc |
| IN-2 | ✅ | Slice-returning operations do not require allocator |
| IN-3 | ✅ | ANSI and parser features excluded from no_std |

### IN-1 — Core operations compile in no_std plus alloc

- **Given:** A `no_std` environment with the `alloc` crate available
- **When:** Splitting, isolation, indentation, and number parsing are compiled
- **Then:** Compilation succeeds without standard library symbols
- **Test:** `tests/invariant_contract_test.rs` — `inv_004_in_1_core_ops_compile_no_std_plus_alloc`

### IN-2 — Slice-returning operations do not require allocator

- **Given:** A split operation configured without stripping or transformation
- **When:** Segments are yielded as borrowed slices
- **Then:** No allocator call is made during the operation
- **Test:** `tests/invariant_contract_test.rs` — `inv_004_in_2_slice_returning_ops_no_allocator`

### IN-3 — ANSI and parser features excluded from no_std

- **Given:** A `no_std` target configuration
- **When:** The ANSI utilities or parser integration features are enabled
- **Then:** Compilation fails because these features require standard library I/O
- **Test:** `tests/invariant_contract_test.rs` — `inv_004_in_3_ansi_parser_excluded_from_no_std`
