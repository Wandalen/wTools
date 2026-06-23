# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Interface contracts for the output processing pipeline and CLI help template renderer. | [api/readme.md](api/readme.md) | 2 |
| `feature/` | Behavioral capabilities — output filtering, truncation, stream merging, and CLI help template rendering. | [feature/readme.md](feature/readme.md) | 2 |
| `invariant/` | Architectural boundary enforcement between cli_fmt (CLI-specific) and strs_tools (general-purpose). | [invariant/readme.md](invariant/readme.md) | 1 |
| `tests/docs/api/` | Test specifications verifying API contracts for output processing and CLI help rendering. | [tests/docs/api/readme.md](../tests/docs/api/readme.md) | 2 |
| `tests/docs/feature/` | Test specifications verifying behavioral requirements for output processing and CLI help template rendering. | [tests/docs/feature/readme.md](../tests/docs/feature/readme.md) | 2 |
| `tests/docs/invariant/` | Test specification verifying the cli_fmt/strs_tools architectural boundary invariant. | [tests/docs/invariant/readme.md](../tests/docs/invariant/readme.md) | 1 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|----|------|------|
| api | 001 | Output Module | [api/001_output_api.md](api/001_output_api.md) |
| api | 002 | Help Template Module | [api/002_help_api.md](api/002_help_api.md) |
| feature | 001 | Output Processing | [feature/001_output_processing.md](feature/001_output_processing.md) |
| feature | 002 | CLI Help Template | [feature/002_cli_help_template.md](feature/002_cli_help_template.md) |
| invariant | 001 | Architectural Boundary | [invariant/001_architectural_boundary.md](invariant/001_architectural_boundary.md) |
| tests/docs/api | 001 | Output API | [../tests/docs/api/001_output_api.md](../tests/docs/api/001_output_api.md) |
| tests/docs/api | 002 | Help Template API | [../tests/docs/api/002_help_api.md](../tests/docs/api/002_help_api.md) |
| tests/docs/feature | 001 | Output Processing | [../tests/docs/feature/001_output_processing.md](../tests/docs/feature/001_output_processing.md) |
| tests/docs/feature | 002 | CLI Help Template | [../tests/docs/feature/002_cli_help_template.md](../tests/docs/feature/002_cli_help_template.md) |
| tests/docs/invariant | 001 | Architectural Boundary | [../tests/docs/invariant/001_architectural_boundary.md](../tests/docs/invariant/001_architectural_boundary.md) |
