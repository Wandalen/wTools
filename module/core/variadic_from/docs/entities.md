# Doc Entities

## Entity Tree

```
docs/
├── algorithm/                  Collection Entity   1st
├── api/                        Collection Entity   1st
├── feature/                    Collection Entity   1st
└── invariant/                  Collection Entity   1st
```

## Entities

| Entity | Type | Latent? | Purpose |
|--------|------|---------|---------|
| [algorithm/](algorithm/) | Collection | | Code generation procedures for FromN trait impls |
| [api/](api/) | Collection | | Public programmatic interfaces for variadic construction |
| [feature/](feature/) | Collection | | Navigational hubs for user-facing variadic capabilities |
| [invariant/](invariant/) | Collection | | Correctness properties that must always hold |

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `algorithm/` | Code generation procedures for FromN trait impls | [algorithm/readme.md](algorithm/readme.md) | 1 |
| `api/` | Public programmatic interfaces for variadic construction | [api/readme.md](api/readme.md) | 2 |
| `feature/` | Navigational hubs for user-facing variadic capabilities | [feature/readme.md](feature/readme.md) | 1 |
| `invariant/` | Correctness properties that must always hold | [invariant/readme.md](invariant/readme.md) | 2 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|----|------|------|
| algorithm | 001 | VariadicFrom Derive | [algorithm/001_variadic_from_derive.md](algorithm/001_variadic_from_derive.md) |
| api | 001 | FromN Traits | [api/001_from_n_traits.md](api/001_from_n_traits.md) |
| api | 002 | from! Macro | [api/002_from_macro.md](api/002_from_macro.md) |
| feature | 001 | Variadic Construction | [feature/001_variadic_construction.md](feature/001_variadic_construction.md) |
| invariant | 001 | Field Count Boundary | [invariant/001_field_count_boundary.md](invariant/001_field_count_boundary.md) |
| invariant | 002 | Compile-Time Arg Count | [invariant/002_compile_time_arg_count.md](invariant/002_compile_time_arg_count.md) |
