# Tasks

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Executor | Task | Purpose |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|----------|------|---------|
| 1 | [001](completed/001_add_colorful_text_struct.md) | 0 | 8 | 9 | 9 | 0 | ✅ (Completed) | claude | Create `color_tools` crate with `DecoratedText` | New crate: typed text-with-optional-ANSI-color abstraction for sub-row detail rendering |
| 2 | [002](completed/002_add_basic_example.md) | 0 | 7 | 9 | 2 | 0 | ✅ (Completed) | claude | Add `examples/basic.rs` — programmer-facing API demo | Runnable reference covering all three ANSI encoding schemes and all conversion paths |
| 3 | [003](completed/003_rename_to_decorated_text.md) | 0 | 7 | 5 | 2 | 0 | ✅ (Completed) | claude | Rename ColorfulText → DecoratedText across workspace | Rename struct + files in color_tools; update all data_fmt callers; prerequisite for task 022 |
| 4 | [004](completed/004_add_color_enum.md) | 0 | 8 | 7 | 2 | 0 | ✅ (Completed) | claude | Add `Color` enum and `with_color_named` method | Typed semantic colors; zero-parse HTML translation path; t22–t28 |
| 5 | [005](completed/005_add_html_rendering.md) | 0 | 8 | 7 | 2 | 0 | ✅ (Completed) | claude | Add `render_html()` and `Color::to_css()` | Browser-usable HTML output gated by `html_support` feature; t29–t34 |

## Statistics

- **Total Tasks:** 5
- **Active:** 0
- **Completed:** 5
- **Backlog:** 0
