# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Public programmatic interface: operations, errors, compatibility | [api/readme.md](api/readme.md) | 1 |
| `feature/` | User-facing capabilities: scope, design, artifact cross-references | [feature/readme.md](feature/readme.md) | 4 |
| `invariant/` | Correctness properties that must hold unconditionally | [invariant/readme.md](invariant/readme.md) | 6 |
| `pattern/` | Architectural decisions: problem, solution, applicability, trade-offs | [pattern/readme.md](pattern/readme.md) | 2 |

## Master Doc Instances Table

| Entity    | ID  | Name                             | File                                                                               |
|-----------|-----|----------------------------------|------------------------------------------------------------------------------------|
| api       | 001 | benchkit Public API              | [api/001_benchkit_api.md](api/001_benchkit_api.md)                                 |
| feature   | 001 | Measurement and Timing           | [feature/001_measurement_timing.md](feature/001_measurement_timing.md)             |
| feature   | 002 | Data Generation                  | [feature/002_data_generation.md](feature/002_data_generation.md)                   |
| feature   | 003 | Markdown Reports                 | [feature/003_markdown_reports.md](feature/003_markdown_reports.md)                 |
| feature   | 004 | Performance Analysis             | [feature/004_performance_analysis.md](feature/004_performance_analysis.md)         |
| invariant | 001 | Benches Directory Mandate        | [invariant/001_benches_directory.md](invariant/001_benches_directory.md)           |
| invariant | 002 | Exact Section Match              | [invariant/002_exact_section_match.md](invariant/002_exact_section_match.md)       |
| invariant | 003 | Performance Overhead Constraint  | [invariant/003_performance_nfr.md](invariant/003_performance_nfr.md)               |
| invariant | 004 | Integration Ease Constraint      | [invariant/004_usability_nfr.md](invariant/004_usability_nfr.md)                   |
| invariant | 005 | Platform and Environment Compatibility | [invariant/005_compatibility_nfr.md](invariant/005_compatibility_nfr.md)     |
| invariant | 006 | Measurement Reproducibility      | [invariant/006_reliability_nfr.md](invariant/006_reliability_nfr.md)               |
| pattern   | 001 | Toolkit Not Framework            | [pattern/001_toolkit_not_framework.md](pattern/001_toolkit_not_framework.md)       |
| pattern   | 002 | Markdown-First Reporting         | [pattern/002_markdown_first_reporting.md](pattern/002_markdown_first_reporting.md) |
