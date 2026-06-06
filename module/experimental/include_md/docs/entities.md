# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Macro contracts for include_md! and include_md_section! — parameters, output type, and error conditions. | [api/readme.md](api/readme.md) | 2 |
| `feature/` | Design rationale for the file inclusion and section extraction capabilities. | [feature/readme.md](feature/readme.md) | 2 |
| `invariant/` | Behavioral contracts and NFR constraints both macros must uphold unconditionally. | [invariant/readme.md](invariant/readme.md) | 4 |
| `tests/docs/api/` | Test spec files verifying include_md! and include_md_section! API contracts. | [../tests/docs/api/readme.md](../tests/docs/api/readme.md) | 2 |
| `tests/docs/feature/` | Test spec files verifying file inclusion and section extraction feature behaviors. | [../tests/docs/feature/readme.md](../tests/docs/feature/readme.md) | 2 |
| `tests/docs/invariant/` | Test spec files verifying path resolution, compile-time errors, size limit, and extraction rules. | [../tests/docs/invariant/readme.md](../tests/docs/invariant/readme.md) | 4 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|----|------|------|
| api | 001 | include_md Macro | [api/001_include_md.md](api/001_include_md.md) |
| api | 002 | include_md_section Macro | [api/002_include_md_section.md](api/002_include_md_section.md) |
| feature | 001 | File Inclusion | [feature/001_file_inclusion.md](feature/001_file_inclusion.md) |
| feature | 002 | Section Extraction | [feature/002_section_extraction.md](feature/002_section_extraction.md) |
| invariant | 001 | Path Resolution | [invariant/001_path_resolution.md](invariant/001_path_resolution.md) |
| invariant | 002 | Compile-Time Errors | [invariant/002_compile_time_errors.md](invariant/002_compile_time_errors.md) |
| invariant | 003 | Size Limit | [invariant/003_size_limit.md](invariant/003_size_limit.md) |
| invariant | 004 | Section Extraction Rules | [invariant/004_section_extraction_rules.md](invariant/004_section_extraction_rules.md) |
| tests/docs/api | 001 | include_md Macro Spec | [../tests/docs/api/001_include_md.md](../tests/docs/api/001_include_md.md) |
| tests/docs/api | 002 | include_md_section Macro Spec | [../tests/docs/api/002_include_md_section.md](../tests/docs/api/002_include_md_section.md) |
| tests/docs/feature | 001 | File Inclusion Spec | [../tests/docs/feature/001_file_inclusion.md](../tests/docs/feature/001_file_inclusion.md) |
| tests/docs/feature | 002 | Section Extraction Spec | [../tests/docs/feature/002_section_extraction.md](../tests/docs/feature/002_section_extraction.md) |
| tests/docs/invariant | 001 | Path Resolution Spec | [../tests/docs/invariant/001_path_resolution.md](../tests/docs/invariant/001_path_resolution.md) |
| tests/docs/invariant | 002 | Compile-Time Errors Spec | [../tests/docs/invariant/002_compile_time_errors.md](../tests/docs/invariant/002_compile_time_errors.md) |
| tests/docs/invariant | 003 | Size Limit Spec | [../tests/docs/invariant/003_size_limit.md](../tests/docs/invariant/003_size_limit.md) |
| tests/docs/invariant | 004 | Section Extraction Rules Spec | [../tests/docs/invariant/004_section_extraction_rules.md](../tests/docs/invariant/004_section_extraction_rules.md) |
