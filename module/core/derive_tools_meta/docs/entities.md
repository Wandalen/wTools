# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Public interface — derive macros, applicability, and feature flags | [api/readme.md](api/readme.md) | 1 |
| `feature/` | Feature specifications — what the crate does and why | [feature/readme.md](feature/readme.md) | 1 |
| `integration/` | Integration documentation — external dependencies and consumers | [integration/readme.md](integration/readme.md) | 2 |
| `invariant/` | Behavioral contracts that must always hold | [invariant/readme.md](invariant/readme.md) | 2 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|----|------|------|
| api | 001 | Derive API | [api/001_derive_api.md](api/001_derive_api.md) |
| feature | 001 | Derive Macros | [feature/001_derive_macros.md](feature/001_derive_macros.md) |
| integration | 001 | macro_tools | [integration/001_macro_tools.md](integration/001_macro_tools.md) |
| integration | 002 | derive_tools | [integration/002_derive_tools.md](integration/002_derive_tools.md) |
| invariant | 001 | Proc-Macro Crate Separation | [invariant/001_proc_macro_separation.md](invariant/001_proc_macro_separation.md) |
| invariant | 002 | Selective Compilation | [invariant/002_selective_compilation.md](invariant/002_selective_compilation.md) |
