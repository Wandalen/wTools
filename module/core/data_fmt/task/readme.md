# Tasks

### Scope

**Responsibilities:** Track all development tasks and decisions for the data_fmt crate.
**In Scope:** Active task files (NNN_*.md), actor registries, per-actor action plans, backlog, completed and cancelled tasks, and the decision log.
**Out of Scope:** Library source code (src/), tests (tests/), behavioral documentation (docs/).

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `unverified/` | New task files awaiting MAAV Verification Gate |
| `actors/` | Actors registry (executor records) |
| `action_plan/` | Per-actor action plans |
| `bug/` | Bug tracking files (BUG-NNN) for all reported and resolved defects |
| `backlog/` | Candidate tasks not yet prioritized |
| `cancelled/` | Abandoned tasks with cancellation rationale |
| `completed/` | Finished tasks with completion evidence |
| `decisions.md` | Decision log for data_fmt |

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Executor | Task | Purpose |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|----------|------|---------|
| 1 | 001 | 0 | 8 | 4 | 1 | 0 | ✅ (Completed) | dev | [Migrate data_fmt to DecoratedText strictly](completed/001_migrate_decorated_text_strict.md) | Eliminate 42 raw-ANSI-String gaps across data model, formatters, config, and theme layers |
| 2 | 002 | 210 | 7 | 6 | 1 | 5 | ✅ (Completed) | any | [Fill test coverage gaps](completed/002_fill_test_coverage_gaps.md) | Implement 56 remaining test functions covering all ⬜ cases across 6 algorithm, 4 invariant, and 5 feature specs |
| 3 | 003 | 245 | 7 | 7 | 5 | 5 | ✅ (Completed) | any | [Fix cell padding — all cells, not just outer edges](completed/003_cell_padding_all_cells.md) | bordered/grid/markdown/unicode_box must emit inner_padding spaces before and after every cell |
| 4 | 004 | 90 | 5 | 6 | 6 | 3 | ✅ (Completed) | any | [Wire ColorTheme::border_color to rendering](completed/004_wire_border_color.md) | apply theme border color to all separator/junction/corner chars during table rendering |
| 5 | 005 | 126 | 6 | 7 | 8 | 3 | ✅ (Completed) | any | [Read $COLUMNS env var as terminal-width fallback](completed/005_columns_env_fallback.md) | resolve_terminal_width must check $COLUMNS before falling back to terminal_size or 120 |
| 6 | 006 | 72 | 4 | 9 | 9 | 2 | ✅ (Completed) | any | [Bundle terminal_size into format_table_visual feature](completed/006_terminal_size_feature_bundle.md) | add dep:terminal_size to format_table_visual Cargo feature so it is self-contained |
| 7 | 007 | 168 | 7 | 6 | 8 | 4 | ✅ (Completed) | any | [Implement TableCaption titled rule above table](completed/007_table_caption.md) | add TableCaption type and TableConfig::caption() builder for titled rule before table output |
| 8 | 008 | 0 | 8 | 6 | 9 | 0 | ✅ (Completed) | dev | [Fix caption width: fill to rendered table width](completed/008_fix_caption_width.md) | render_caption_if_present must fill to compute_total_row_width(primary_widths), not resolve_terminal_width() |
| 9 | 009 | 0 | 7 | 5 | 8 | 0 | ✅ (Completed) | dev | [API consistency: with_ prefix sweep + rename Heading](completed/009_api_consistency_with_prefix.md) | add with_ prefix to all 39 config builder setters; rename TableCaption → Heading |
| 10 | 010 | 0 | 5 | 8 | 9 | 0 | ✅ (Completed) | dev | [Add Heading feature examples](completed/010_heading_examples.md) | add heading_basic.rs and heading_styles.rs example binaries (E criterion) |
| 11 | 011 | 1458 | 6 | 9 | 9 | 3 | ✅ (Completed) | any | [format_aligned display width mismatch](completed/011_format_aligned_display_width_mismatch.md) | format_aligned uses char-count visual_len for pass 1 but display-width pad_to_width for pass 2 — emoji/CJK columns misalign |
| 12 | 012 | 2205 | 7 | 7 | 9 | 5 | ✅ (Completed) | any | [Create test surface specs for 6 uncovered entity types](completed/012_create_test_surface_specs.md) | Create 43 test spec files to bring tests/docs/ coverage from 37/80 to 80/80 |
| 13 | 013 | 1728 | 8 | 6 | 9 | 4 | ✅ (Completed) | any | [Implement test code for builder, formatter, and trait specs](completed/013_implement_builder_formatter_trait_tests.md) | Implement 88 Rust test functions from 14 test spec files (builder/2, formatter/10, trait/2) |
| 14 | 014 | 1440 | 8 | 5 | 9 | 4 | ❓ (Unverified) | any | [Implement remaining 168 spec tests](unverified/014_implement_remaining_spec_tests.md) | Implement 168 test functions from 43 spec files (data_structure/1, input_model/2, input_type/2, pattern/4, trait/1, variant/33) |
| 15 | 015 | 0 | 5 | 8 | 9 | 3 | ❓ (Unverified) | any | [Add `Heading::render(width)` public method](unverified/015_add_heading_render_method.md) | Expose standalone heading render API so external callers avoid duplicating fill arithmetic |
