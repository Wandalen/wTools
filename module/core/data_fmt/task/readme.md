# Tasks

### Scope

**Responsibilities:** Track all development tasks and decisions for the data_fmt crate.
**In Scope:** Active task files (NNN_*.md), actor registries, per-actor action plans, backlog, completed and cancelled tasks, and the decision log.
**Out of Scope:** Library source code (src/), tests (tests/), behavioral documentation (docs/).

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `actors/` | Actors registry (executor records) |
| `action_plan/` | Per-actor action plans |
| `backlog/` | Candidate tasks not yet prioritized |
| `cancelled/` | Abandoned tasks with cancellation rationale |
| `completed/` | Finished tasks with completion evidence |
| `decisions.md` | Decision log for data_fmt |
| `002_fill_test_coverage_gaps.md` | Active task: implement 56 remaining ⬜ test cases across all doc entity surfaces |
| `003_cell_padding_all_cells.md` | Active task: fix inner_padding to fire on every cell, not just outer table edges |
| `004_wire_border_color.md` | Active task: forward ColorTheme::border_color to rendering so separators are ANSI-decorated |
| `005_columns_env_fallback.md` | Active task: read $COLUMNS before terminal_size fallback in resolve_terminal_width |
| `006_terminal_size_feature_bundle.md` | Active task: add dep:terminal_size to format_table_visual Cargo feature |
| `007_table_caption.md` | Active task: add TableCaption struct and TableConfig::caption() builder |

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Executor | Task | Purpose |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|----------|------|---------|
| 1 | 001 | 0 | 8 | 4 | 1 | 0 | ✅ (Completed) | dev | [Migrate data_fmt to DecoratedText strictly](completed/001_migrate_decorated_text_strict.md) | Eliminate 42 raw-ANSI-String gaps across data model, formatters, config, and theme layers |
| 2 | 002 | 210 | 7 | 6 | 1 | 5 | ⏳ (In Progress) | any | [Fill test coverage gaps](002_fill_test_coverage_gaps.md) | Implement 56 remaining test functions covering all ⬜ cases across 6 algorithm, 4 invariant, and 5 feature specs |
| 3 | 003 | 245 | 7 | 7 | 5 | 5 | ❓ (Unverified) | any | [Fix cell padding — all cells, not just outer edges](003_cell_padding_all_cells.md) | bordered/grid/markdown/unicode_box must emit inner_padding spaces before and after every cell |
| 4 | 004 | 90 | 5 | 6 | 6 | 3 | ❓ (Unverified) | any | [Wire ColorTheme::border_color to rendering](004_wire_border_color.md) | apply theme border color to all separator/junction/corner chars during table rendering |
| 5 | 005 | 126 | 6 | 7 | 8 | 3 | ❓ (Unverified) | any | [Read $COLUMNS env var as terminal-width fallback](005_columns_env_fallback.md) | resolve_terminal_width must check $COLUMNS before falling back to terminal_size or 120 |
| 6 | 006 | 72 | 4 | 9 | 9 | 2 | 🎯 (Verified) | any | [Bundle terminal_size into format_table_visual feature](006_terminal_size_feature_bundle.md) | add dep:terminal_size to format_table_visual Cargo feature so it is self-contained |
| 7 | 007 | 168 | 7 | 6 | 8 | 4 | 🎯 (Verified) | any | [Implement TableCaption titled rule above table](007_table_caption.md) | add TableCaption type and TableConfig::caption() builder for titled rule before table output |
