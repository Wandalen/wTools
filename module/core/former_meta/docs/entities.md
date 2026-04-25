# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Collect all API doc instances defining macro entry points and attribute contracts. | [api/readme.md](api/readme.md) | 1 |
| `feature/` | Collect all feature doc instances describing what this crate does and why. | [feature/readme.md](feature/readme.md) | 1 |
| `integration/` | Collect all integration doc instances for external dependencies and consumers. | [integration/readme.md](integration/readme.md) | 3 |
| `invariant/` | Collect all invariant doc instances defining constraints that must always hold. | [invariant/readme.md](invariant/readme.md) | 2 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|-----|------|------|
| api | 001 | Derive API | [api/001_derive_api.md](api/001_derive_api.md) |
| feature | 001 | Former Derive Macro | [feature/001_former_derive.md](feature/001_former_derive.md) |
| integration | 001 | macro_tools | [integration/001_macro_tools.md](integration/001_macro_tools.md) |
| integration | 002 | former_types | [integration/002_former_types.md](integration/002_former_types.md) |
| integration | 003 | former | [integration/003_former.md](integration/003_former.md) |
| invariant | 001 | Proc-Macro Crate Separation | [invariant/001_proc_macro_separation.md](invariant/001_proc_macro_separation.md) |
| invariant | 002 | Feature Flag Gating | [invariant/002_feature_flag_gating.md](invariant/002_feature_flag_gating.md) |
