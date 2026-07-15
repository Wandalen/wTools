# Completed Tasks

Tasks that passed validation and are finished.

### Scope

**Responsibilities:**
Archives finished task files for the cli_fmt crate, each retaining Work Procedure, Validation Checklist, and Outcomes evidence for completed work.

**In Scope:**
- Completed task files (001–009, 015) with appended Outcomes sections and closure History
- Validation evidence for finished work (measurements, invariants, anti-faking checks)

**Out of Scope:**
- In-progress or not-yet-started tasks (see `../unverified/`, `../verifying/`, `../verified/`, `../executing/`, `../validating/`)
- Cancelled tasks (see `../cancelled/`)
- Bug reports (see `../bug/`)

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `001_cli_help_template.md` | Implement CliHelpTemplate typed CLI help renderer |
| `002_test_coverage_gaps.md` | Fill output test coverage gaps for FT-11 and FT-12 |
| `003_test_coverage_comprehensive.md` | Close 12 comprehensive test gaps — boundary values, untested paths, spec alignment |
| `004_remaining_test_gaps.md` | Fill remaining test coverage gaps — is_default tail/width, tty_detect, data_fmt absence |
| `005_extend_cli_help_template_multi_section.md` | Extend CliHelpTemplate with option groups, usage lines, arguments section |
| `006_aspirational_test_surface.md` | Complete aspirational test surface — FT-36..FT-40 and FT-29..FT-30 |
| `007_fix_test_assertion_gaps.md` | Add FT-33/FT-17 missing assertions to `tests/output.rs` |
| `008_feature_flag_filtering_test.md` | Add FT-41 passthrough test via `output_passthrough` feature |
| `009_fill_new_test_coverage_gaps.md` | Implement 7 ⏳ spec cases — FT-42..FT-44, AP-14..AP-15, FT-31, FT-32 |
| `015_fill_col_gap_padding_contiguity_test.md` | Add FT-33 test proving padding, col_gap, and description are contiguous |
