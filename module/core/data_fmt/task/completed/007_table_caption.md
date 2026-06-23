# Implement `TableCaption` — titled rule printed above the table

## Execution State

- **State:** ✅ (Completed)
- **ID:** 007
- **Slug:** table_caption
- **Executor:** dev

## MOST Goal

Add a `TableCaption` type to `data_fmt` and a `TableConfig::caption()` builder so that callers can print a titled rule (e.g., `─── Needs Review · 28 PRs · 15 repos ─────`) above any table style with zero impact on tables that do not use the feature.
(Motivated: The `clr ps` command in `claude_runner` displays active Claude sessions in a `TableConfig::plain()` table and needs a titled summary line such as `─── Active Sessions · 3 running ─────` above it — currently requires a manual `println!` call that breaks visual alignment with the table's column widths; no existing API on `TableFormatter` supports attaching a labelled rule; explicitly requested 2026-06-13 for `clr ps` output; Observable: `TableConfig::plain().caption(TableCaption::new("Results").field("42 items"))` renders a line beginning with `"─── Results · 42 items "` followed by `─` chars filling the resolved terminal width, then the normal table output; Scoped: changes confined to `src/config.rs` (new struct + field + builder), `src/formatters/table/mod.rs` (emit caption before `format_top_border_if_needed`), and tests for FC-1, FC-2, FC-3, FC-5, FC-6 in `tests/table_caption_test.rs`; no changes to any other formatter; Testable: `w3 .test level::3` passes clean and all 5 FC-N cases in `tests/docs/feature/007_table_caption.md` have ✅ implementations)

## In Scope

All paths relative to the crate root (`module/core/data_fmt/`).

**`src/config.rs`:**
- Add `pub struct TableCaption` with fields: `title: String`, `fields: Vec<String>` — formatting is fixed: `field_sep = '·'` (U+00B7), `rule_char = '─'` (U+2500), `lead_width = 3` (hard-coded constants, not user-configurable in this task)
- Implement `TableCaption::new(title: impl Into<String>) -> Self`
- Implement builder method (all `#[must_use]`, return `Self`): `.field(f: impl Into<String>)`
- Add `caption: Option<TableCaption>` field to `TableConfig` (default `None`)
- Add `pub fn caption(mut self, c: TableCaption) -> Self` builder on `TableConfig`
- Add `pub(crate) fn caption_ref(&self) -> Option<&TableCaption>` accessor
- All preset constructors (`plain()`, `bordered()`, `grid()`, etc.) leave `caption: None` — no change to existing behavior

**`src/formatters/table/mod.rs`:**
- In `format_internal()`, before `format_top_border_if_needed(...)`: call `self.render_caption_if_present(&mut output)`
- Add `fn render_caption_if_present(&self, output: &mut String)`:
  - Return early if `self.config.caption_ref().is_none()`
  - Build content string: `caption.title + fields.iter().map(|f| format!(" {} {}", CAPTION_FIELD_SEP, f)).collect()`
  - Resolve terminal width via `self.resolve_terminal_width()` (existing function in `auto_fit.rs`)
  - Compute trail: `terminal_width.saturating_sub(CAPTION_LEAD_WIDTH + 1 + content.chars().count() + 1)` — use `.chars().count()`, NOT `.len()`; `·` (U+00B7) is 2 UTF-8 bytes and `─` (U+2500) is 3 bytes, so `.len()` would undercount visual columns and produce a shorter trailing rule
  - Emit: `CAPTION_RULE_CHAR × CAPTION_LEAD_WIDTH + " " + content + " " + CAPTION_RULE_CHAR × trail + "\n"`

**`tests/table_caption_test.rs`:**
- Implement FC-1, FC-2, FC-3, FC-5, FC-6 per `tests/docs/feature/007_table_caption.md` (FC-4 removed — customizable rule_char is not in scope)

## Out of Scope

- ANSI color on caption text (not requested by any caller in this task)
- Caption below the table — not requested
- All other formatters (`ExpandedFormatter`, `TreeFormatter`, etc.)
- `clr ps` migration from `unicode_box` to `plain` — separate cross-repo change in `claude_runner`
- Any change to existing `TableConfig` presets or their output

## Work Procedure

1. Read `src/config.rs`. Add `TableCaption` struct with `title: String` and `fields: Vec<String>`. Define `CAPTION_FIELD_SEP: char = '·'`, `CAPTION_RULE_CHAR: char = '─'`, `CAPTION_LEAD_WIDTH: usize = 3` as module-level constants. Implement `new()` and the single `.field()` builder method. Add `caption: Option<TableCaption>` to `TableConfig` struct. Add it to `Default` as `None`. Add `caption()` builder and `caption_ref()` accessor.
2. Read `src/formatters/table/mod.rs`. In `format_internal()`, insert `self.render_caption_if_present(&mut output);` before the `format_top_border_if_needed` call.
3. Add `fn render_caption_if_present(&self, output: &mut String)` on `TableFormatter`. Use `self.resolve_terminal_width()` from `auto_fit.rs` (already `pub(super)` within `table/`) for terminal width. Build and push the caption line.
4. Create `tests/table_caption_test.rs`. Implement FC-1, FC-2, FC-3, FC-5, FC-6.
5. Run `w3 .test level::3`. Fix any failures.
6. Verify all 5 FC-N test cases pass (FC-1, FC-2, FC-3, FC-5, FC-6); verify that `plain` table output without `.caption()` is byte-identical to the same config pre-change (FC-5 regression check).

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| `TableCaption::new("Results")`, no fields | `TableConfig::plain()` | First output line: `"─── Results "` + trailing `─` to terminal width |
| `TableCaption::new("NR").field("28 PRs").field("15 repos")` | `TableConfig::plain()` | Caption line contains `"NR · 28 PRs · 15 repos"` |
| `terminal_width(Some(60))`, `TableCaption::new("T").field("F")` | `TableConfig::plain()` | Caption line character count = 60 |
| No `.caption()` call | `TableConfig::plain()` | Output identical to current library output (FC-5 regression) |
| `TableCaption::new("G")` | `TableConfig::grid()` | Caption line appears before `+---+` top border |

## Related Documentation

- `docs/feature/007_table_caption.md` — feature spec (acceptance criteria AC-1 through AC-5)
- `tests/docs/feature/007_table_caption.md` — test spec (FC-1, FC-2, FC-3, FC-5, FC-6)
- `docs/feature/001_table_formatting.md` — existing table formatting feature
- `docs/api/003_config_types.md` — `TableConfig` API reference (update after implementation to document `TableCaption` and `.caption()`)

**Closes:** null

## History

- **[2026-06-13]** `CREATED` — Implement `TableCaption` type and `TableConfig::caption()` builder for titled rule above any table style.
- **[2026-06-13]** `REVISED` — Reduced `TableCaption` to minimum viable API (title + fields only; field_sep/rule_char/lead_width hard-coded as constants); fixed trail formula to use `.chars().count()`; added explicit `clr ps` caller citation; removed FC-4 (rule_char customization) from test spec; test spec file `tests/docs/feature/007_table_caption.md` confirmed present.

## Verification Record

All 4 MAAV dimensions PASSED on 2026-06-13 (third Verification Gate run):

| Dimension | Result |
|-----------|--------|
| Scope Coherence | ✅ PASS |
| MOST Goal Quality | ✅ PASS |
| Value / YAGNI | ✅ PASS |
| Implementation Readiness | ✅ PASS |
