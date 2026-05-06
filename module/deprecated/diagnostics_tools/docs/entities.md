# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Public macro interface contracts | [api/readme.md](api/readme.md) | 3 |
| `feature/` | Capability guides describing crate behavior | [feature/readme.md](feature/readme.md) | 4 |
| `invariant/` | Behavioral contracts and enforcement mechanisms | [invariant/readme.md](invariant/readme.md) | 4 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|-----|------|------|
| api | 001 | Runtime Assertion Macros | [api/001_runtime_assertion_macros.md](api/001_runtime_assertion_macros.md) |
| api | 002 | Compile-Time Assertion Macros | [api/002_compiletime_assertion_macros.md](api/002_compiletime_assertion_macros.md) |
| api | 003 | Memory Layout Assertion Macros | [api/003_memory_layout_macros.md](api/003_memory_layout_macros.md) |
| feature | 001 | Runtime Assertions | [feature/001_runtime_assertions.md](feature/001_runtime_assertions.md) |
| feature | 002 | Compile-Time Assertions | [feature/002_compiletime_assertions.md](feature/002_compiletime_assertions.md) |
| feature | 003 | Memory Layout Assertions | [feature/003_memory_layout_assertions.md](feature/003_memory_layout_assertions.md) |
| feature | 004 | No-Std Support | [feature/004_no_std_support.md](feature/004_no_std_support.md) |
| invariant | 001 | Debug Variants Are No-Ops in Release Builds | [invariant/001_debug_variants_release_noop.md](invariant/001_debug_variants_release_noop.md) |
| invariant | 002 | Equality Assertions Produce Colored Diff Output | [invariant/002_pretty_diff_output.md](invariant/002_pretty_diff_output.md) |
| invariant | 003 | Compile-Time Assertions Introduce No Runtime Overhead | [invariant/003_compiletime_zero_overhead.md](invariant/003_compiletime_zero_overhead.md) |
| invariant | 004 | Alloc Feature Requires No-Std | [invariant/004_alloc_requires_no_std.md](invariant/004_alloc_requires_no_std.md) |
