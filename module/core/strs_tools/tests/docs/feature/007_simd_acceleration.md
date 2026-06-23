# SIMD Acceleration

## Edge Case Index

| ID | Short Name | Category | Status |
|----|-----------|----------|--------|
| FT-1 | Feature activation | Happy path | ✅ |
| FT-2 | Scalar fallback when disabled | Happy path | ✅ |
| FT-3 | SIMD-scalar output equivalence | Invariant | ✅ |
| FT-4 | Dependencies activated | Happy path | ✅ |

## Cases

### FT-1: Feature activation

- **Given:** Crate compiled with `simd` feature enabled
- **When:** SIMD string search functions are called
- **Then:** Functions are available and execute without error
- **Test:** `tests/simd_tests.rs` — `simd_scalar_equivalence_single_delimiter`

### FT-2: Scalar fallback when disabled

- **Given:** Crate compiled without `simd` feature
- **When:** Split operation is performed
- **Then:** Split completes using scalar code path — identical behavior
- **Test:** `tests/inc/split_test.rs` — runs without simd feature via default features

### FT-3: SIMD-scalar output equivalence

- **Given:** Same input string and delimiter configuration
- **When:** Split performed with simd enabled and with simd disabled
- **Then:** Both produce byte-for-byte identical segment sequences
- **Test:** `tests/simd_tests.rs` — `simd_scalar_equivalence_single_delimiter`, `simd_scalar_equivalence_multi_delimiter`

### FT-4: Dependencies activated

- **Given:** Crate compiled with `simd` feature
- **When:** Dependency resolution completes
- **Then:** memchr, aho-corasick, bytecount, once_cell are available
- **Test:** `tests/simd_tests.rs` — compilation with `simd` feature proves deps resolve
