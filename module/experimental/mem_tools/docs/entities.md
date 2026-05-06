# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Documents public interface contracts — function signatures, error handling, and compatibility guarantees for all exported comparison functions. | [api/readme.md](api/readme.md) | 1 |
| `feature/` | Documents user-facing capabilities — scope, design rationale, and cross-references to all artifacts for each implemented feature. | [feature/readme.md](feature/readme.md) | 1 |
| `invariant/` | Documents correctness properties — invariant statements, enforcement mechanisms, and violation consequences for this crate's behavioral contracts. | [invariant/readme.md](invariant/readme.md) | 2 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|-----|------|------|
| api | 001 | Memory Comparison Functions | [api/001_comparison_functions.md](api/001_comparison_functions.md) |
| feature | 001 | Memory and Pointer Comparison | [feature/001_memory_comparison.md](feature/001_memory_comparison.md) |
| invariant | 001 | Comparison Functions Work Across Heterogeneous Types | [invariant/001_type_agnostic_comparison.md](invariant/001_type_agnostic_comparison.md) |
| invariant | 002 | same_data Validates Sizes Before Byte Comparison | [invariant/002_size_guarded_data_comparison.md](invariant/002_size_guarded_data_comparison.md) |
