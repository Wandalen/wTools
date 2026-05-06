# Doc Entities

## Entity Tree

```
docs/
├── feature/        Collection Entity   1st    (8 instances)
├── api/            Collection Entity   1st    (3 instances)
├── invariant/      Collection Entity   1st    (4 instances)
└── algorithm/      Collection Entity   1st    (3 instances)
```

## Entities

| Entity | Type | Mutable? | Operations | Order |
|--------|------|----------|------------|-------|
| [`feature/`](feature/) | Collection | Yes | Create, Update, Delete | 1st |
| [`api/`](api/) | Collection | Yes | Create, Update, Delete | 1st |
| [`invariant/`](invariant/) | Collection | Yes | Create, Update, Delete | 1st |
| [`algorithm/`](algorithm/) | Collection | Yes | Create, Update, Delete | 1st |

---

### Master Doc Entities Table

| Entity Type | Directory | Responsibility |
|-------------|-----------|----------------|
| feature | `feature/` | User-facing capability navigation hubs |
| api | `api/` | Public programmatic interface contracts |
| invariant | `invariant/` | Correctness properties and guarantees |
| algorithm | `algorithm/` | Internal algorithmic design for maintainers |

### Master Doc Instances Table

| ID | Entity | Name | File |
|----|--------|------|------|
| F-001 | feature | String Splitting | `feature/001_string_splitting.md` |
| F-002 | feature | Text Indentation | `feature/002_text_indentation.md` |
| F-003 | feature | String Isolation | `feature/003_string_isolation.md` |
| F-004 | feature | Number Parsing | `feature/004_number_parsing.md` |
| F-005 | feature | Command Parsing | `feature/005_command_parsing.md` |
| F-006 | feature | ANSI Utilities | `feature/006_ansi_utilities.md` |
| F-007 | feature | SIMD Acceleration | `feature/007_simd_acceleration.md` |
| F-008 | feature | Parser Integration | `feature/008_parser_integration.md` |
| A-001 | api | Split API | `api/001_split_api.md` |
| A-002 | api | String Utilities API | `api/002_string_utilities_api.md` |
| A-003 | api | Parser Integration API | `api/003_parser_integration_api.md` |
| I-001 | invariant | Zero-Copy Contract | `invariant/001_zero_copy_contract.md` |
| I-002 | invariant | Feature Gating Contract | `invariant/002_feature_gating_contract.md` |
| I-003 | invariant | SIMD Fallback Contract | `invariant/003_simd_fallback_contract.md` |
| I-004 | invariant | No-Std Alloc Contract | `invariant/004_no_std_alloc_contract.md` |
| G-001 | algorithm | SIMD Delimiter Search | `algorithm/001_simd_delimiter_search.md` |
| G-002 | algorithm | Single-Char Splitting | `algorithm/002_single_char_splitting.md` |
| G-003 | algorithm | Boyer-Moore Splitting | `algorithm/003_boyer_moore_splitting.md` |
