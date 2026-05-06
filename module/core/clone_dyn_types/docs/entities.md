# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `algorithm/` | Implementation algorithms: how DST cloning works | [algorithm/readme.md](algorithm/readme.md) | 1 |
| `api/` | Public API contracts: traits and functions | [api/readme.md](api/readme.md) | 2 |
| `feature/` | Behavioral requirements: what the crate does | [feature/readme.md](feature/readme.md) | 3 |
| `invariant/` | Constraints: what must always hold | [invariant/readme.md](invariant/readme.md) | 3 |
| `pattern/` | Architectural design patterns | [pattern/readme.md](pattern/readme.md) | 1 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|-----|------|------|
| algorithm | 001 | Fat Pointer Surgery | [algorithm/001_fat_pointer_surgery.md](algorithm/001_fat_pointer_surgery.md) |
| api | 001 | CloneDyn Trait | [api/001_clone_dyn_trait.md](api/001_clone_dyn_trait.md) |
| api | 002 | clone_into_box and clone | [api/002_clone_into_box.md](api/002_clone_into_box.md) |
| feature | 001 | No-Std Support | [feature/001_no_std_support.md](feature/001_no_std_support.md) |
| feature | 002 | DST Cloning | [feature/002_dst_cloning.md](feature/002_dst_cloning.md) |
| feature | 003 | Type Coverage | [feature/003_type_coverage.md](feature/003_type_coverage.md) |
| invariant | 001 | Zero Dependencies | [invariant/001_zero_dependencies.md](invariant/001_zero_dependencies.md) |
| invariant | 002 | Memory Safety | [invariant/002_memory_safety.md](invariant/002_memory_safety.md) |
| invariant | 003 | Usage Constraints | [invariant/003_usage_constraints.md](invariant/003_usage_constraints.md) |
| pattern | 001 | Sealed Trait | [pattern/001_sealed_trait.md](pattern/001_sealed_trait.md) |
