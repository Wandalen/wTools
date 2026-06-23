# Test Surface: Invariant — SIMD Fallback Contract

### Source

- **Doc Instance:** [invariant/003_simd_fallback_contract.md](../../../docs/invariant/003_simd_fallback_contract.md)

### Cases

| # | Status | Case |
|---|--------|------|
| IN-1 | ⏳ | SIMD and scalar produce identical segments |
| IN-2 | ⏳ | SIMD degrades to scalar on unsupported platforms |
| IN-3 | ⏳ | Byte-for-byte output identity across paths |

### IN-1 — SIMD and scalar produce identical segments

- **Given:** An input string and a delimiter configuration
- **When:** Split is executed via SIMD path and separately via scalar path
- **Then:** The segment sequences are identical in count, boundaries, and content
- **Test:** ⏳

### IN-2 — SIMD degrades to scalar on unsupported platforms

- **Given:** The `simd` feature is enabled at compile time
- **When:** The runtime platform does not support the required SIMD instructions
- **Then:** The split operation succeeds using the scalar fallback with no caller intervention
- **Test:** ⏳

### IN-3 — Byte-for-byte output identity across paths

- **Given:** A complex input with mixed delimiters and Unicode content
- **When:** Segments are collected from both SIMD-enabled and scalar-only builds
- **Then:** The collected segments are byte-for-byte identical
- **Test:** ⏳
