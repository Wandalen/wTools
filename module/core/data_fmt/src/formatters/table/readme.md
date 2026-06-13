# src/formatters/table

## Purpose
Contains the `TableFormatter` implementation split across focused source files.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | `TableFormatter` struct, `format_internal` entry point, column width calculation |
| `auto_fit.rs` | Auto-wrap and fold helpers: terminal sizing, budgets, column folding |
| `rendering.rs` | Row and border rendering primitives: single-line, multiline, separators, borders |
| `row_rendering.rs` | Header/row ANSI dispatch: `format_row`, `format_row_colored`, caption rendering |
