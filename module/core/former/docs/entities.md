# Doc Entities

## Entity Tree

```
algorithm/                          Collection Entity   1st
api/                                Collection Entity   1st
feature/                            Collection Entity   1st
invariant/                          Collection Entity   1st
pattern/                            Collection Entity   1st
```

## Entities

| Entity | Type | Latent? | Purpose |
|--------|------|---------|---------|
| [algorithm/](algorithm/) | Collection | | Algorithm doc instances — computational procedures used during macro expansion |
| [api/](api/) | Collection | | API doc instances — attribute contracts for macro configuration |
| [feature/](feature/) | Collection | | Feature doc instances — user-facing macro capabilities |
| [invariant/](invariant/) | Collection | | Invariant doc instances — hard constraints on supported types |
| [pattern/](pattern/) | Collection | | Pattern doc instances — design patterns implemented or enabled by the macro |

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `algorithm/` | Computational procedures used during macro expansion | [algorithm/readme.md](algorithm/readme.md) | 1 |
| `api/` | Public attribute contracts for macro configuration | [api/readme.md](api/readme.md) | 2 |
| `feature/` | User-facing macro capabilities as navigational hubs | [feature/readme.md](feature/readme.md) | 7 |
| `invariant/` | Hard constraints on types supported by the macro | [invariant/readme.md](invariant/readme.md) | 3 |
| `pattern/` | Design patterns implemented or enabled by the macro | [pattern/readme.md](pattern/readme.md) | 2 |

## Master Doc Instances Table

| Type | ID | Name | File |
|--------|----|------|------|
| algorithm | 001 | Variant Constructor Logic | [algorithm/001_variant_constructor_logic.md](algorithm/001_variant_constructor_logic.md) |
| api | 001 | Item Attributes | [api/001_item_attributes.md](api/001_item_attributes.md) |
| api | 002 | Field Attributes | [api/002_field_attributes.md](api/002_field_attributes.md) |
| feature | 001 | Struct Former | [feature/001_struct_former.md](feature/001_struct_former.md) |
| feature | 002 | Enum Former | [feature/002_enum_former.md](feature/002_enum_former.md) |
| feature | 003 | Scalar Subformer | [feature/003_subform_scalar.md](feature/003_subform_scalar.md) |
| feature | 004 | Collection Subformer | [feature/004_subform_collection.md](feature/004_subform_collection.md) |
| feature | 005 | Entry Subformer | [feature/005_subform_entry.md](feature/005_subform_entry.md) |
| feature | 006 | Standalone Constructors | [feature/006_standalone_constructors.md](feature/006_standalone_constructors.md) |
| feature | 007 | Debug Attribute | [feature/007_debug_attribute.md](feature/007_debug_attribute.md) |
| invariant | 001 | Owned Types Only | [invariant/001_owned_types_only.md](invariant/001_owned_types_only.md) |
| invariant | 002 | No Generic Enums | [invariant/002_no_generic_enums.md](invariant/002_no_generic_enums.md) |
| invariant | 003 | Single Variant Enum | [invariant/003_single_variant_enum.md](invariant/003_single_variant_enum.md) |
| pattern | 001 | Builder Pattern | [pattern/001_builder_pattern.md](pattern/001_builder_pattern.md) |
| pattern | 002 | Subformer Composition | [pattern/002_subformer_composition.md](pattern/002_subformer_composition.md) |
