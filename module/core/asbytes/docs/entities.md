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
| `api/` | Registry and overview of all API doc instances | [api/readme.md](api/readme.md) | 2 |
| `feature/` | Registry and overview of all feature doc instances | [feature/readme.md](feature/readme.md) | 1 |
| `invariant/` | Registry and overview of all invariant doc instances | [invariant/readme.md](invariant/readme.md) | 2 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|----|------|------|
| api | 001 | AsBytes Trait | [api/001_as_bytes_trait.md](api/001_as_bytes_trait.md) |
| api | 002 | IntoBytes Trait | [api/002_into_bytes_trait.md](api/002_into_bytes_trait.md) |
| feature | 001 | Byte Conversion | [feature/001_byte_conversion.md](feature/001_byte_conversion.md) |
| invariant | 001 | POD Safety | [invariant/001_pod_safety.md](invariant/001_pod_safety.md) |
| invariant | 002 | Native Byte Order | [invariant/002_native_endian.md](invariant/002_native_endian.md) |
