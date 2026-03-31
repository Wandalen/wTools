# Tasks

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Task | Description |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|------|-------------|
| 1 | [011](011_make_table_config_api_misuse_resistant.md) | 336 | 7 | 4 | 6 | 2 | 🔄 (Planned) | Make `TableConfig` API misuse-resistant | Make `TableConfig` fields private so struct literal misconfiguration is a compile error |
| 2 | [003](completed/003_fix_unicode_display_width_alignment.md) | 0 | 9 | 5 | 8 | 0 | ✅ (Completed) | Fix Unicode display width alignment bug | Fix character-count vs display-width mismatch causing misalignment with CJK/emoji (wide chars) |
| 3 | [001](completed/001_implement_multiline_cells_and_column_limits.md) | 0 | 8 | 7 | 5 | 0 | ✅ (Completed) | Implement multiline cell support and column size limits | Add support for multiline cells with automatic wrapping and configurable column width limits with truncation |
| 4 | [002](completed/002_fix_unicode_table_border_alignment.md) | 0 | 7 | 8 | 9 | 0 | ✅ (Completed) | Fix Unicode table border alignment bug | Add Unicode variant to needs_border_pipes logic for proper vertical pipe rendering |
| 5 | [007](completed/007_rename_helpers_to_ansi_str.md) | 0 | 7 | 8 | 3 | 7 | ✅ (Completed) | Rename helpers.rs to ansi_str.rs | Rename prohibited filename helpers.rs to ansi_str.rs with 4 reference updates |
| 6 | [009](completed/009_add_missing_readme_files.md) | 0 | 6 | 8 | 2 | 9 | ✅ (Completed) | Add missing readme.md files | Add Responsibility Table readme.md to tests/inc/, src/formatters/, src/ |
| 7 | [006](completed/006_delete_disabled_test.md) | 0 | 7 | 9 | 3 | 8 | ✅ (Completed) | Delete disabled test | Delete #[ignore] test violating no-disabled-tests rule |
| 8 | [008](completed/008_remove_inline_test_blocks.md) | 0 | 6 | 9 | 3 | 9 | ✅ (Completed) | Remove inline test blocks | Remove inline #[cfg(test)] blocks from src/formatters/sql.rs and html.rs |
| 9 | [005](completed/005_remove_integration_feature_gate.md) | 0 | 5 | 9 | 2 | 9 | ✅ (Completed) | Remove integration feature gate | Remove redundant legacy integration gate from 11 test files |
| 10 | [004](completed/004_add_word_wrapping_utility.md) | 0 | 9 | 4 | 7 | 2 | ✅ (Completed) | Add word wrapping utility | Add WrapConfig + WrapFormatter with 11 config fields, builder pattern, and full test coverage |
| 11 | [010](completed/010_update_stale_test_counts.md) | 0 | 4 | 7 | 1 | 9 | ✅ (Completed) | Update stale test counts | Update stale test counts in tests/readme.md after task 004 |

## Statistics

- **Total Tasks:** 11
- **Active:** 1
- **Completed:** 10
- **Backlog:** 0

## Issue Index

*(No issues tracked for this crate. Initialize with `task/issue/` directory if needed.)*

| Order | ID | Severity | First Seen | Regressions | Status | Issue | Summary |
|-------|----|---------:|------------|------------:|--------|-------|---------|

## Issues

- **Total Issues:** 0
- **Open:** 0
- **Closed:** 0
