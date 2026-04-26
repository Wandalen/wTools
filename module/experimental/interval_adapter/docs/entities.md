# Doc Entities

## Entity Tree

```
docs/                               Composite Entity    1st
├── api/                            Collection Entity   2nd
├── data_structure/                 Collection Entity   2nd
├── feature/                        Collection Entity   2nd
├── invariant/                      Collection Entity   2nd
└── pattern/                        Collection Entity   2nd
```

## Entities

| Entity | Type | Latent? | Purpose |
|--------|------|---------|---------|
| [docs/](readme.md) | Composite | | Documentation hierarchy for interval_adapter |
| [api/](api/readme.md) | Collection | | Public API documentation |
| [data_structure/](data_structure/readme.md) | Collection | | Canonical data type definitions |
| [feature/](feature/readme.md) | Collection | | User-visible capabilities and usage patterns |
| [invariant/](invariant/readme.md) | Collection | | Behavioral constraints and out-of-scope boundaries |
| [pattern/](pattern/readme.md) | Collection | | Architectural decisions and design rationale |

## Doc Instances

| Entity | ID | Name | File |
|--------|----|------|------|
| api | 001 | Interval Traits | [api/001_interval_traits.md](api/001_interval_traits.md) |
| api | 002 | Conversion Traits | [api/002_conversion_traits.md](api/002_conversion_traits.md) |
| data_structure | 001 | Interval | [data_structure/001_interval.md](data_structure/001_interval.md) |
| data_structure | 002 | IntervalIterator | [data_structure/002_interval_iterator.md](data_structure/002_interval_iterator.md) |
| feature | 001 | Generic Interval Parameter | [feature/001_generic_interval_parameter.md](feature/001_generic_interval_parameter.md) |
| feature | 002 | Non-Iterable Intervals | [feature/002_non_iterable_intervals.md](feature/002_non_iterable_intervals.md) |
| feature | 003 | no_std Support | [feature/003_no_std_support.md](feature/003_no_std_support.md) |
| invariant | 001 | Integer Endpoints Only | [invariant/001_integer_endpoints_only.md](invariant/001_integer_endpoints_only.md) |
| invariant | 002 | No Validation | [invariant/002_no_validation.md](invariant/002_no_validation.md) |
| invariant | 003 | No Set Operations | [invariant/003_no_set_operations.md](invariant/003_no_set_operations.md) |
| pattern | 001 | Two-Trait Hierarchy | [pattern/001_two_trait_hierarchy.md](pattern/001_two_trait_hierarchy.md) |
| pattern | 002 | Canonical Interval Type | [pattern/002_canonical_interval_type.md](pattern/002_canonical_interval_type.md) |
