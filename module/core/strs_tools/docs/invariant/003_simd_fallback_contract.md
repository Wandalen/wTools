# Invariant: SIMD Fallback Contract

### Scope

- **Purpose**: Guarantee that SIMD-accelerated and scalar code paths produce identical results, so that enabling or disabling the `simd` feature affects only throughput, not correctness.
- **Responsibility**: Defines the functional equivalence invariant between SIMD and scalar execution paths.
- **In Scope**: Result equivalence for all input shapes, byte-for-byte output identity, no correctness dependency on hardware acceleration availability.
- **Out of Scope**: Performance characteristics (documented in `algorithm/001`); feature activation mechanism (`invariant/002`); no_std compatibility (`invariant/004`).

### Invariant

For any input string and any configuration, the sequence of segments produced by the SIMD-enabled code path is identical to the sequence produced by the scalar fallback. Output equivalence is byte-for-byte: the same segment boundaries, the same content, the same classification.

This invariant holds unconditionally across all supported platforms, including platforms where the SIMD instructions are unavailable. On such platforms the SIMD path degrades to its scalar equivalent automatically; no caller action is required.

Test suites run with and without the `simd` feature to verify the invariant holds for all covered input shapes.

### Sources

- [src/simd.rs](../../src/simd.rs) — SIMD feature gating and scalar fallback dispatch
- [src/string/split/simd.rs](../../src/string/split/simd.rs) — Vectorized delimiter search with scalar fallback

### Features

- [007_simd_acceleration.md](../feature/007_simd_acceleration.md) — SIMD acceleration feature design

### Algorithms

- [001_simd_delimiter_search.md](../algorithm/001_simd_delimiter_search.md) — SIMD delimiter search algorithmic detail
