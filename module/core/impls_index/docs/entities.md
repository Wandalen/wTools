# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Public API contracts: macros and their invocation rules | [api/readme.md](api/readme.md) | 3 |
| `feature/` | Behavioral requirements: what the crate does | [feature/readme.md](feature/readme.md) | 3 |
| `invariant/` | Constraints: what must always hold | [invariant/readme.md](invariant/readme.md) | 2 |
| `pattern/` | Architectural design patterns | [pattern/readme.md](pattern/readme.md) | 1 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|-----|------|------|
| api | 001 | Indexing Macros | [api/001_indexing_macros.md](api/001_indexing_macros.md) |
| api | 002 | Invocation Macros | [api/002_invocation_macros.md](api/002_invocation_macros.md) |
| api | 003 | Utility Macros | [api/003_utility_macros.md](api/003_utility_macros.md) |
| feature | 001 | Function Indexing | [feature/001_function_indexing.md](feature/001_function_indexing.md) |
| feature | 002 | Test Indexing | [feature/002_test_indexing.md](feature/002_test_indexing.md) |
| feature | 003 | Function Utilities | [feature/003_function_utilities.md](feature/003_function_utilities.md) |
| invariant | 001 | Unused Macro Enforcement | [invariant/001_unused_macro_enforcement.md](invariant/001_unused_macro_enforcement.md) |
| invariant | 002 | Compile-Time Resolution | [invariant/002_compile_time_resolution.md](invariant/002_compile_time_resolution.md) |
| pattern | 001 | Two-Crate Proc Macro | [pattern/001_two_crate_proc_macro.md](pattern/001_two_crate_proc_macro.md) |
