# ANSI Header and Alternating-Row Coloring in `format_internal()`

## Goal

`TableConfig` already has fields `colorize_header`, `header_color`, `alternating_rows`,
`row_color1`, and `row_color2`, plus builder setters and theme support — but `format_internal()`
never reads any of them. Wire up the coloring pipeline using a temp-buffer strategy so that:
(1) the header row is wrapped in `header_color … RESET` when `colorize_header = true`, and
(2) data rows alternate between `row_color1` and `row_color2` when `alternating_rows = true`.
Success is measured by `w3 .test l::3` passing green with zero warnings.

## In Scope

- `src/config.rs` — add 6 new `pub(crate)` accessors: `colorize_header_enabled()`,
  `header_color_str()`, `alternating_rows_enabled()`, `row_color1_str()`, `row_color2_str()`,
  and a `RESET` constant (or `reset_color_str()` helper)
- `src/formatters/table.rs` — update `format_internal()` with temp-buffer coloring; add an
  `apply_color()` helper that wraps a line string in a color code
- `tests/table_rendering_colors.rs` — new test file; registered in `tests/readme.md`

## Out of Scope

- Per-cell coloring (only row-level coloring is in scope)
- Changing `format_row()` signature — coloring wraps the row output via temp buffer
- Adding new color fields to `TableConfig`
- Theme definitions in `src/themes.rs` (themes already set the color fields correctly)

## Description

The temp-buffer strategy keeps `format_row()` unchanged. Instead, `format_internal()`
redirects `format_row()` output into a temporary `String`, strips the trailing `\n`,
wraps the content in color codes, and appends `RESET + \n`:

```
1. let mut row_buf = String::new();
2. self.format_row( &mut row_buf, row, &column_widths, is_header );
3. let content = row_buf.trim_end_matches( '\n' );
4. output.push_str( &format!( "{color}{content}{RESET}\n" ) );
```

**CRITICAL:** `RESET` must be placed BEFORE `\n`. Terminals that set background colors will
bleed the color across the rest of the line if RESET appears after or is omitted before `\n`.

The alternating pattern uses a simple index: even rows use `row_color1`, odd rows use
`row_color2`. When `alternating_rows = false` or both color strings are empty, rows are
emitted without color wrapping (the existing path).

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- 2-space indentation per codestyle rulebook; `cargo fmt` is forbidden
- Tests must be in `tests/table_rendering_colors.rs` (new file)
- Follow TDD: write failing tests first, confirm red, implement, confirm green
- Phase dependency: Task 014 (border rendering) must be complete before Task 013 — coloring
  wraps the already-bordered rows from Phase 4's `format_internal()` modifications

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note codestyle and accessor naming constraints.
2. **Write failing tests** — create `tests/table_rendering_colors.rs` with T013-P01 through
   T013-N06; run `w3 .test l::1`; confirm red.
3. **Add accessors** — add 6 new `pub(crate)` accessors in `src/config.rs`.
4. **Add `apply_color()` helper** — add a small helper in `table.rs` (or inline logic) that
   applies color wrapping to a row string.
5. **Update `format_internal()`** — add temp-buffer coloring for header and data rows.
6. **Green state** — `w3 .test l::3` passes with zero failures and zero warnings.
7. **Cross-task check** — run T013-N05 (coloring + borders combined); confirm both features
   coexist correctly.

## Test Matrix

*(Written before any test code.)*

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|----------------|-------------------|-------------------|
| T013-P01 | 2 data rows | `colorize_header(true)` + `header_color("\x1b[1m")` | Header line wrapped in `\x1b[1m…\x1b[0m` |
| T013-P02 | 3 data rows | `alternating_rows(true)` + `row_colors(color1, color2)` | Row 0 = color1, row 1 = color2, row 2 = color1 |
| T013-P03 | 1 data row | `colorize_header(true)` + `alternating_rows(true)` | Both header and data row colored independently |
| T013-P04 | Any table | `colorize_header(false)` (default) | No color codes in header output |
| T013-P05 | RESET placement | Any color config | RESET (`\x1b[0m`) appears before `\n` in every colored line |
| T013-N01 | Any table | `alternating_rows(false)` (default) | No color codes in data rows |
| T013-N02 | Any table | `row_colors("", "")` with `alternating_rows(true)` | Empty color strings: no color sequences emitted |
| T013-N03 | 1 data row | `alternating_rows(true)` + only `row_color1` set | Row 0 uses color1; single row works without color2 |
| T013-N04 | Theme applied | `Theme::color()` or equivalent | Theme-applied config produces colored header and rows |
| T013-N05 | `grid()` + colors | `TableConfig::grid().colorize_header(true)` | Borders AND coloring both appear in output |
| T013-N06 | 0 data rows | `colorize_header(true)` | Header colored, no data rows emitted, no panic |

## Acceptance Criteria

- All 6 new accessors exist in `src/config.rs` `pub(crate)` accessor block with doc comments
- `format_internal()` wraps header in `header_color…RESET` when `colorize_header = true`
- `format_internal()` wraps data rows in `row_color1`/`row_color2` (alternating) when
  `alternating_rows = true`
- RESET (`\x1b[0m`) appears BEFORE `\n` in every colored line (not after)
- When coloring is disabled (defaults), output is byte-for-byte identical to pre-task output
- All T013-P01–P05 positive tests pass
- All T013-N01–N06 negative/edge tests pass
- `w3 .test l::3` exits 0 with zero failures and zero warnings

## Validation Checklist

Desired answer for every question is YES.

**`src/config.rs` — accessors**
- [ ] Do all 6 new `pub(crate)` accessors exist?
- [ ] Do they have doc comments?
- [ ] Are color string accessors returning `&str` (not cloning)?

**`src/formatters/table.rs` — coloring logic**
- [ ] Does `format_internal()` use a temp buffer to capture each row string?
- [ ] Is RESET placed BEFORE `\n` in colored line output?
- [ ] When coloring is disabled, does the function take the unmodified path?

**Test coverage**
- [ ] Do all 5 positive tests (T013-P01–P05) pass?
- [ ] Do all 6 negative/edge tests (T013-N01–N06) pass?
- [ ] Does T013-N05 confirm borders + colors coexist correctly?

**Final gate**
- [ ] Does `w3 .test l::3` exit 0 with zero warnings?

## Validation Procedure

### Measurements

**M1 — Red state confirmed**
Command: `RUSTFLAGS="-D warnings" cargo nextest run --test table_rendering_colors 2>&1 | grep -E "FAILED|test result"`
Before: file does not exist. Expected after RED step: ≥9 failures. Deviation: 0 failures = tests not written.

**M2 — New accessors exist**
Command: `grep -c "pub( crate ) fn colorize_header_enabled\|pub( crate ) fn header_color_str\|pub( crate ) fn alternating_rows_enabled\|pub( crate ) fn row_color1_str\|pub( crate ) fn row_color2_str" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/config.rs`
Before: 0. Expected: 5. Deviation: <5 = accessors missing.

**M3 — RESET placement**
Command: Run T013-P05 in isolation; assert `\x1b[0m\n` (RESET then newline) appears in output.

**M4 — Green state**
Command: `w3 .test l::3`
Expected: 0 failures, 0 warnings.

### Anti-faking checks

**AF1 — Coloring disabled path unchanged**
T013-N01 must assert that `TableConfig::plain()` output (no coloring) is byte-for-byte
identical to the output produced before Task 013 was implemented. This prevents hiding bugs
behind "it renders something."

**AF2 — RESET before newline**
Grep rendered colored output: `contains("\x1b[0m\n")` must be true; `contains("\n\x1b[0m")` must
be false. Incorrect placement causes terminal background-color bleed.
