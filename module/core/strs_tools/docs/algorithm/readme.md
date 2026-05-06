# Algorithm Doc Entity

### Scope

- **Purpose**: Document internal algorithmic design for maintainers and performance-conscious contributors.
- **Responsibility**: Index of algorithm doc instances; each instance explains the approach, selection criteria, and performance characteristics of one internal algorithm.
- **In Scope**: SIMD delimiter search, single-char split specialization, Boyer-Moore-inspired split for multi-char single-delimiter patterns.
- **Out of Scope**: Public API contracts (`api/`); user-facing feature descriptions (`feature/`); correctness guarantees (`invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [SIMD Delimiter Search](001_simd_delimiter_search.md) | Hardware-accelerated multi-delimiter search via vectorized byte matching | ✅ |
| 002 | [Single-Char Splitting](002_single_char_splitting.md) | Optimized split path for the common single single-byte delimiter case | ✅ |
| 003 | [Boyer-Moore Splitting](003_boyer_moore_splitting.md) | Skip-table split for multi-character single-delimiter patterns | ✅ |
