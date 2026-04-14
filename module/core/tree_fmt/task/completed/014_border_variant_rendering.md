# Border Variant Rendering — Top/Bottom Borders and Inter-Row Separators

## Goal

`TableConfig::border_variant` exists as a field and has a builder setter, but
`format_internal()` never reads it. Add top/bottom borders and inter-row separators for
`BorderVariant::AsciiGrid` and `BorderVariant::Unicode`, and fix the longstanding
`format_header_separator()` AsciiGrid corner-character bug (`|` → `+`). Success is measured
by `w3 .test l::3` passing green with zero warnings.

## In Scope

- `src/config.rs` — add `bdr_variant()` accessor in the existing `pub(crate)` accessor block
- `src/formatters/table.rs` — fix AsciiGrid header separator corner chars; add
  `format_ascii_horizontal_rule()` and `format_unicode_horizontal_rule()` helpers; add
  `format_top_border_if_needed()`, `format_bottom_border_if_needed()`, and
  `format_inter_row_sep_if_needed()` wrappers; update `format_internal()` pipeline
- `tests/table_rendering_borders.rs` — new test file; registered in `tests/readme.md`

## Out of Scope

- `BorderVariant::Ascii` and `BorderVariant::Markdown` — these variants add pipe walls in
  `format_single_line_row()` (already implemented) but do NOT produce top/bottom borders or
  inter-row separators; in scope for consistency but not for additional border lines
- Per-column border customization
- Changing the existing Unicode header separator format (`├─┼─┤`) — it is correct
- Task 013 (coloring) — border rendering establishes stable `format_internal()` structure
  that Task 013 then wraps with color codes

## Description

`format_single_line_row()` already adds leading/trailing `|` pipe characters for AsciiGrid
and Unicode variants via the `needs_border_pipes` logic. What is missing is the HORIZONTAL
rules: top border (before header), bottom border (after last data row), and inter-row
separators (between consecutive data rows for AsciiGrid/Unicode).

**AsciiGrid horizontal rule characters:**
- All rules: `left='+'`, `fill='-'`, `mid='+'`, `right='+'` → produces `+---+---+`
- Header separator (bug fix): change existing `'|'` corner to `'+'`

**Unicode horizontal rule characters:**
- Top: `left='┌'`, `fill='─'`, `mid='┬'`, `right='┐'`
- Bottom: `left='└'`, `fill='─'`, `mid='┴'`, `right='┘'`
- Header separator (already correct): `left='├'`, `fill='─'`, `mid='┼'`, `right='┤'`
- Inter-row separator: same as header separator

The parameterized `format_ascii_horizontal_rule(output, widths, left, fill, mid, right)` helper
is called three times per border type (top, bottom, inter-row) with different character sets,
avoiding code duplication.

**AsciiGrid header separator fix:** `format_header_separator()` L566–590 currently uses `'|'`
as the leading/trailing delimiter. Change the three `output.push('|')` calls to
`output.push('+')`. The internal junction `'+'` is already correct.

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- 2-space indentation per codestyle rulebook; `cargo fmt` is forbidden
- Tests must be in `tests/table_rendering_borders.rs` (new file)
- Follow TDD: write failing tests first, confirm red, implement, confirm green
- Phase dependency: Task 015 (unicode widths) must be complete before Task 014 — border
  rule widths must use display-width measurements for correct alignment with wide chars

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note function naming and line-length constraints.
2. **Write failing tests** — create `tests/table_rendering_borders.rs` with T014-P01 through
   T014-N05; run `w3 .test l::1`; confirm red.
3. **Add `bdr_variant()` accessor** — in `src/config.rs` accessor block.
4. **Fix AsciiGrid header separator** — in `format_header_separator()`, change `'|'` corners
   to `'+'`.
5. **Add horizontal rule helpers** — add `format_ascii_horizontal_rule()` and
   `format_unicode_horizontal_rule()` as private methods on `TableFormatter`.
6. **Add border wrapper helpers** — add `format_top_border_if_needed()`,
   `format_bottom_border_if_needed()`, `format_inter_row_sep_if_needed()`.
7. **Update `format_internal()`** — add top border before header row, inter-row separators
   between data rows, bottom border after last data row.
8. **Green state** — `w3 .test l::3` passes with zero failures and zero warnings.
9. **Regression check** — confirm existing `table_styles_outputs.rs` and
   `table_styles_presets.rs` tests still pass (no regressions in existing border behavior).

## Algorithm

**`format_ascii_horizontal_rule`:**
```rust
fn format_ascii_horizontal_rule(
  &self, output : &mut String, widths : &[ usize ],
  left : char, fill : char, mid : char, right : char
)
{
  output.push( left );
  for ( idx, &width ) in widths.iter().enumerate()
  {
    if idx == 0 && self.config.has_outer_padding()
    {
      output.push_str( &fill.to_string().repeat( self.config.cell_inner_padding() ) );
    }
    output.push_str( &fill.to_string().repeat( width ) );
    if idx == widths.len() - 1 && self.config.has_outer_padding()
    {
      output.push_str( &fill.to_string().repeat( self.config.cell_inner_padding() ) );
    }
    output.push( if idx < widths.len() - 1 { mid } else { right } );
  }
  output.push( '\n' );
}
```

Unicode version: same structure; note `'─'` is a multi-byte char — `fill.to_string().repeat(n)`
is correct because `String::repeat` works on `String` (char count, not bytes).

## Test Matrix

*(Written before any test code.)*

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|----------------|-------------------|-------------------|
| T014-P01 | 2 data rows | `TableConfig::grid()` | Output has `+---+` top border, header sep, bottom border |
| T014-P02 | 2 data rows | `TableConfig::unicode_box()` | Top `┌─`, bottom `└─`, header sep `├─` all present |
| T014-P03 | 2 data rows | `TableConfig::bordered()` | No top/bottom borders (BorderVariant::Ascii) |
| T014-P04 | 2 data rows | `TableConfig::plain()` | No borders at all in output |
| T014-P05 | 3 data rows | `TableConfig::grid()` | 2 inter-row separators between data rows |
| T014-N01 | AsciiGrid header sep | `TableConfig::grid()` | Header separator is `+---+`, NOT `|---|` |
| T014-N02 | Unicode top border | `TableConfig::unicode_box()` | Top border starts with `┌`, NOT `├` |
| T014-N03 | Unicode bottom border | `TableConfig::unicode_box()` | Bottom border starts with `└`, NOT `├` |
| T014-N04 | Markdown variant | `TableConfig::markdown()` | No top/bottom borders (Markdown variant) |
| T014-N05 | 0 data rows | `TableConfig::grid()` | Top border + header row + header sep + bottom border; no inter-row seps |

## Acceptance Criteria

- `bdr_variant()` accessor exists in `src/config.rs` with doc comment, returns `BorderVariant`
- `format_ascii_horizontal_rule()` and `format_unicode_horizontal_rule()` exist as private
  methods on `TableFormatter`, parameterized with left/fill/mid/right chars
- `format_internal()` calls top border, inter-row separators, and bottom border via the
  three wrapper helpers
- AsciiGrid header separator in `format_header_separator()` uses `'+'` corners (not `'|'`)
- `format_single_line_row()` row pipe characters (`'|'`) are unchanged (rows always correct)
- All T014-P01–P05 positive tests pass
- All T014-N01–N05 negative/edge tests pass
- T014-N01 specifically confirms AsciiGrid separator = `+---+` not `|---|`
- `w3 .test l::3` exits 0 with zero failures and zero warnings

## Validation Checklist

Desired answer for every question is YES.

**`src/config.rs` — accessor**
- [ ] Does `bdr_variant()` exist with a doc comment?
- [ ] Does it return `BorderVariant` (not a reference to it)?

**`src/formatters/table.rs` — border rendering**
- [ ] Does `format_ascii_horizontal_rule()` exist as a private method on `TableFormatter`?
- [ ] Does `format_unicode_horizontal_rule()` exist as a private method on `TableFormatter`?
- [ ] Does `format_internal()` call `format_top_border_if_needed` before the header row?
- [ ] Does `format_internal()` call `format_inter_row_sep_if_needed` inside the data loop?
- [ ] Does `format_internal()` call `format_bottom_border_if_needed` after the data loop?
- [ ] Does the AsciiGrid branch of `format_header_separator()` use `'+'` for corner chars?
- [ ] Is `format_single_line_row()` row pipe logic (`'|'`) unchanged?
- [ ] Is the Unicode `width + 2` in `format_header_separator()` unchanged?

**Test coverage**
- [ ] Do all 5 positive tests (T014-P01–P05) pass?
- [ ] Do all 5 negative/edge tests (T014-N01–N05) pass?
- [ ] Does T014-N01 assert header sep = `+---+` (not `|---|`)?
- [ ] Do existing `table_styles_outputs.rs` and `table_styles_presets.rs` tests still pass?

**Final gate**
- [ ] Does `w3 .test l::3` exit 0 with zero warnings?

## Validation Procedure

### Measurements

**M1 — Red state confirmed**
Command: `RUSTFLAGS="-D warnings" cargo nextest run --test table_rendering_borders 2>&1 | grep -E "FAILED|test result"`
Before: file does not exist. Expected after RED step: ≥8 failures. Deviation: 0 failures = tests not written.

**M2 — `bdr_variant()` accessor exists**
Command: `grep -c "pub( crate ) fn bdr_variant" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/config.rs`
Before: 0. Expected: 1. Deviation: 0 = accessor missing.

**M3 — AsciiGrid separator fixed**
Command: `grep -n "push( '|' )" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs | head -5`
Before: multiple hits in `format_header_separator`. Expected: 0 hits in `format_header_separator`
(only `format_single_line_row` should still have pipe pushes).

**M4 — Green state**
Command: `w3 .test l::3`
Expected: 0 failures, 0 warnings.

### Anti-faking checks

**AF1 — Row pipe chars not changed**
`grep -n "push( '|' )" table.rs` must still show `'|'` pushes in `format_single_line_row` —
these are the data row pipes and must remain. Only `format_header_separator` AsciiGrid corner
pushes should change.

**AF2 — T014-N01 directly asserts separator content**
The test must check `output.contains("+---+")` and `!output.contains("|---|")` for a
`TableConfig::grid()` table with a known data set.
