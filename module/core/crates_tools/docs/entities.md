# Doc Entities

## Entity Tree

```
docs/                            structural container
├── api/                         Collection Entity   1st
├── feature/                     Collection Entity   1st
└── invariant/                   Collection Entity   1st
```

## Entities

| Entity | Type | Latent? | Purpose |
|--------|------|---------|---------|
| [api/](api/) | Collection | | Registry of API doc instances |
| [feature/](feature/) | Collection | | Registry of feature doc instances |
| [invariant/](invariant/) | Collection | | Registry of invariant doc instances |

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Registry and overview of all API doc instances | [api/readme.md](api/readme.md) | 1 |
| `feature/` | Registry and overview of all feature doc instances | [feature/readme.md](feature/readme.md) | 1 |
| `invariant/` | Registry and overview of all invariant doc instances | [invariant/readme.md](invariant/readme.md) | 2 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|----|------|------|
| api | 001 | CrateArchive | [api/001_crate_archive.md](api/001_crate_archive.md) |
| feature | 001 | Archive Inspection | [feature/001_archive_inspection.md](feature/001_archive_inspection.md) |
| invariant | 001 | In-Memory Storage | [invariant/001_in_memory_storage.md](invariant/001_in_memory_storage.md) |
| invariant | 002 | Blocking Network | [invariant/002_blocking_network.md](invariant/002_blocking_network.md) |
