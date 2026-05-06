# Feature: SIMD Acceleration

### Scope

- **Purpose**: Provide opt-in hardware-accelerated string processing via the `simd` feature flag, enabling vectorized delimiter search and byte counting where the target platform supports it.
- **Responsibility**: Documents the SIMD acceleration feature boundary, activation mechanism, and behavioral guarantees, linking to algorithmic detail and invariant documents.
- **In Scope**: The `simd` Cargo feature flag, SIMD-dependent optional dependencies, the vectorized delimiter search path, graceful scalar fallback.
- **Out of Scope**: Algorithm-level implementation detail (`algorithm/001`); split configuration model (`feature/001`); public API surface (`api/001`).

### Design

SIMD acceleration is entirely opt-in: callers activate it by enabling the `simd` Cargo feature. When the feature is absent the crate compiles and operates identically using scalar code paths.

When the feature is enabled, four additional dependencies become active: a byte-search library, a multi-pattern search library, a byte-counting library, and a caching primitive for compiled pattern objects. These libraries provide vectorized scanning that the splitting and delimiter-search internals can delegate to when the input meets the size threshold.

The SIMD path and the scalar path produce identical results. Correctness is not conditional on feature availability — only throughput changes. This guarantee is an invariant and is documented separately in `invariant/003_simd_fallback_contract.md`.

### Sources

- `../../architecture.md` — SIMD Optimization section; SIMD dependency list and performance characteristics migrated to this instance.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/simd.rs` | SIMD feature gating and platform dispatch |
| source | `src/string/split/simd.rs` | Vectorized delimiter search within the split path |
| doc | `docs/algorithm/001_simd_delimiter_search.md` | SIMD delimiter search algorithmic design |
| doc | `docs/invariant/002_feature_gating_contract.md` | Feature opt-in gating invariant |
| doc | `docs/invariant/003_simd_fallback_contract.md` | Scalar fallback correctness guarantee |
