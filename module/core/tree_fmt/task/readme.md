# Tasks

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Executor | Task | Purpose |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|----------|------|---------|
| 1 | [018](completed/018_ansi_colorize_sub_row_details.md) | 0 | 8 | 7 | 8 | 0 | ✅ (Completed) | claude | ANSI colorize sub-row detail lines | Upgrade row_details to ColorfulText and apply per-line Algorithm 3 ANSI wrapping |
| 2 | [017](completed/017_add_sub_row_to_table_formatter.md) | 0 | 7 | 8 | 4 | 0 | ✅ (Completed) | claude | Add sub-row detail lines to `TableFormatter` | Extend `TableFormatter` with optional per-row indented detail lines |
| 3 | [001](completed/001_implement_multiline_cells_and_column_limits.md) | 0 | 8 | 7 | 5 | 0 | ✅ (Completed) | any | Implement multiline cell support and column size limits | Add multiline cells with automatic wrapping and configurable column width limits |
| 4 | [002](completed/002_fix_unicode_table_border_alignment.md) | 0 | 7 | 8 | 9 | 0 | ✅ (Completed) | any | Fix Unicode table border alignment bug | Add Unicode variant to needs_border_pipes logic for proper vertical pipe rendering |
| 5 | [003](completed/003_fix_unicode_display_width_alignment.md) | 0 | 9 | 5 | 8 | 0 | ✅ (Completed) | any | Fix Unicode display width alignment bug | Fix character-count vs display-width mismatch causing misalignment with CJK/emoji |
| 6 | [004](completed/004_add_word_wrapping_utility.md) | 0 | 9 | 4 | 7 | 0 | ✅ (Completed) | any | Add word wrapping utility | Add WrapConfig + WrapFormatter with builder pattern and full test coverage |
| 7 | [005](completed/005_remove_integration_feature_gate.md) | 0 | 5 | 9 | 2 | 0 | ✅ (Completed) | any | Remove integration feature gate | Remove redundant legacy integration gate from 11 test files |
| 8 | [006](completed/006_delete_disabled_test.md) | 0 | 7 | 9 | 3 | 0 | ✅ (Completed) | any | Delete disabled test | Delete #[ignore] test violating no-disabled-tests rule |
| 9 | [007](completed/007_rename_helpers_to_ansi_str.md) | 0 | 7 | 8 | 3 | 0 | ✅ (Completed) | any | Rename helpers.rs to ansi_str.rs | Rename prohibited filename helpers.rs to ansi_str.rs with reference updates |
| 10 | [008](completed/008_remove_inline_test_blocks.md) | 0 | 6 | 9 | 3 | 0 | ✅ (Completed) | any | Remove inline test blocks | Remove inline #[cfg(test)] blocks from sql.rs and html.rs |
| 11 | [009](completed/009_add_missing_readme_files.md) | 0 | 6 | 8 | 2 | 0 | ✅ (Completed) | any | Add missing readme.md files | Add Responsibility Table readme.md to tests/inc/, src/formatters/, src/ |
| 12 | [010](completed/010_update_stale_test_counts.md) | 0 | 4 | 7 | 1 | 0 | ✅ (Completed) | any | Update stale test counts | Update stale test counts in tests/readme.md after task 004 |
| 13 | [011](completed/011_make_table_config_api_misuse_resistant.md) | 0 | 7 | 4 | 6 | 0 | ✅ (Completed) | any | Make `TableConfig` API misuse-resistant | Make `TableConfig` fields private so struct literal misconfiguration is compile error |
| 14 | [012](completed/012_enforce_min_column_width.md) | 0 | 6 | 7 | 8 | 0 | ✅ (Completed) | any | Enforce `min_column_width` floor | Floor enforcement after max cap in `calculate_column_widths_for_rows` |
| 15 | [013](completed/013_ansi_header_row_coloring.md) | 0 | 7 | 5 | 7 | 0 | ✅ (Completed) | any | ANSI header and alternating-row coloring | Temp-buffer coloring strategy in `format_internal()` |
| 16 | [014](completed/014_border_variant_rendering.md) | 0 | 8 | 5 | 7 | 0 | ✅ (Completed) | any | Border variant rendering | Top/bottom borders, inter-row separators, AsciiGrid corner fix |
| 17 | [015](completed/015_unicode_display_width.md) | 0 | 8 | 6 | 9 | 0 | ✅ (Completed) | any | Unicode display width fix | Replace char-count with display-width in column calculation and cell padding |
| 18 | [016](completed/016_add_indent_prefix_to_expanded_config.md) | 0 | 8 | 8 | 9 | 0 | ✅ (Completed) | any | Add `indent_prefix` to `ExpandedConfig` | Add indent prefix field so callers can indent key-value lines |
| 19 | [021](completed/021_terminal_width_detection_tests.md) | 0 | 7 | 8 | 9 | 0 | ✅ (Completed) | any | Terminal width detection tests | Test three-tier fallback: explicit override, terminal_size feature, 120-column default |
| 20 | [019](completed/019_cell_auto_wrapping_with_budget_allocation.md) | 0 | 8 | 4 | 5 | 0 | ✅ (Completed) | any | Cell auto-wrapping with terminal-aware budget allocation | Terminal width detection, ColumnFlex classification, budget allocation, auto-wrap cells |
| 21 | [020](completed/020_column_folding_with_auto_fold.md) | 0 | 7 | 4 | 5 | 0 | ✅ (Completed) | any | Column folding with auto-fold | FoldStyle enum, fold detection, continuation line rendering, combination with wrapping |

| 22 | [022](cancelled/022_migrate_decoratedtext_strict.md) | 192 | 8 | 4 | 1 | 6 | ❌ (Cancelled) | — | Migrate tree_fmt to DecoratedText strictly | Superseded: tree_fmt is a data_fmt shim; migration covered by data_fmt/task/001 |

## Statistics

- **Total Tasks:** 22
- **Active:** 0
- **Completed:** 21
- **Cancelled:** 1
- **Available:** 0
