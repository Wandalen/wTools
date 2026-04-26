# Invariant Doc Entity

### Scope

- **Purpose**: State correctness properties that all callers and contributors can rely on unconditionally.
- **Responsibility**: Index of invariant doc instances; each instance documents one guarantee with its enforcement mechanism and violation consequences.
- **In Scope**: Zero-copy contract, feature gating contract, SIMD fallback guarantee, no_std/alloc portability contract.
- **Out of Scope**: API operation descriptions (`api/`); algorithmic mechanics (`algorithm/`); user-facing feature descriptions (`feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Zero-Copy Contract](001_zero_copy_contract.md) | Guarantee that splitting borrows source data except when transformation is required | ✅ |
| 002 | [Feature Gating Contract](002_feature_gating_contract.md) | Guarantee that `default` is empty and all functionality is opt-in | ✅ |
| 003 | [SIMD Fallback Contract](003_simd_fallback_contract.md) | Guarantee that SIMD and scalar paths produce identical results | ✅ |
| 004 | [no_std Alloc Contract](004_no_std_alloc_contract.md) | Guarantee of no_std compatibility and alloc opt-in semantics | ✅ |
