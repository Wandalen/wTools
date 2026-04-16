# Unicode Display Width Fix — Column Calculation and Cell Padding

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)

## Goal

Column widths are currently calculated with `visual_len()` (char count, re-exported from
`strs_tools`) and cells are padded with `pad_to_width()` (also char count). For CJK characters
(2 display columns each) and emoji (variable width), char count diverges from display width,
causing misaligned columns. Introduce `unicode_visual_len()` and `pad_unicode_width()` in
`src/ansi_str.rs` and replace all four call sites in `src/formatters/table.rs`. The
`unicode-width` crate is already a direct dependency — no `Cargo.toml` changes required.
Success is measured by `w3 .test l::3` passing green with zero warnings.

## In Scope

- `src/ansi_str.rs` — add `pub(crate) fn unicode_visual_len(s: &str) -> usize` and
  `pub(crate) fn pad_unicode_width(s: &str, width: usize, align_right: bool) -> String`
- `src/formatters/table.rs` — replace 4 call sites: `visual_len` at L643 and L654
  (column width calculation), `pad_to_width` at L369 (single-line row) and L478
  (multiline row); update the import at L99 accordingly
- `tests/unicode_display_width_alignment.rs` — add T015 test cases

## Out of Scope

- `truncate_single_line()` in `ansi_str.rs` — already uses `ch.width().unwrap_or(1)`;
  no change needed
- The `visual_len` and `pad_to_width` re-exports from `strs_tools` — leave them available
  for any callers outside `table.rs` that use char-count intentionally
- `Cargo.toml` changes — `unicode-width = "0.1"` is already a direct dependency

## Description

`unicode_visual_len(s)` must strip ANSI escape sequences before measuring display width,
because ANSI codes contain non-printing bytes that inflate char count. ANSI CSI sequences
take the form `\x1b[…m`. The safe stripping algorithm:

- Track `in_esc: bool` flag
- On `\x1b`: set `in_esc = true`, skip the char
- While `in_esc`: skip chars; clear `in_esc` on any ASCII alphabetic char (sequence terminator)
- Otherwise: add `ch.width().unwrap_or(1)` to accumulated width

This mirrors the approach already used in `truncate_single_line()` (`ansi_str.rs` L156+).
The `is_ascii_alphabetic()` terminator check handles all common ANSI color sequences (`\x1b[…m`,
`\x1b[…H`, etc.) correctly for the use case here (color codes only).

`pad_unicode_width(s, width, align_right)` calls `unicode_visual_len(s)` to measure content,
computes padding as `width - content_width` (saturating), and appends or prepends the padding
spaces based on `align_right`.

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- 2-space indentation per codestyle rulebook; `cargo fmt` is forbidden
- New functions are `pub(crate)` (not `pub`) — they are internal implementation details
- Tests must be added to `tests/unicode_display_width_alignment.rs` (existing file)
- Follow TDD: write failing tests first, confirm red, implement, confirm green

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note function naming and visibility constraints.
2. **Write failing tests** — add T015-P01 through T015-N07 to
   `tests/unicode_display_width_alignment.rs`; run `w3 .test l::1`; confirm red.
3. **Add functions** — add `unicode_visual_len()` and `pad_unicode_width()` to `src/ansi_str.rs`
   after the re-export lines (L79–80).
4. **Update import** — change `table.rs` L99 import from `{ visual_len, pad_to_width }` to
   `{ unicode_visual_len, pad_unicode_width }`; confirm no other use of old names in the file.
5. **Replace call sites** — 4 replacements in `table.rs` (see locations below).
6. **Green state** — `w3 .test l::3` passes with zero failures and zero warnings.
7. **ANSI stripping pitfall check** — confirm T015-P04 asserts `unicode_visual_len` of a
   colored string equals the uncolored char count (not the total byte/char count).

## Algorithm

**`unicode_visual_len`:**
```rust
/// Returns the display width of `s`, stripping ANSI escape sequences.
/// Measures display columns using `UnicodeWidthChar::width()`.
/// Handles all common ANSI color/formatting sequences (`\x1b[...m`).
/// Note: only CSI sequences terminated by ASCII alphabetic chars are handled;
/// rare non-color sequences (e.g., cursor position codes) are best-effort.
pub( crate ) fn unicode_visual_len( s : &str ) -> usize
{
  use unicode_width::UnicodeWidthChar;
  let mut len = 0usize;
  let mut in_esc = false;
  for ch in s.chars()
  {
    if ch == '\x1b' { in_esc = true; continue; }
    if in_esc
    {
      if ch.is_ascii_alphabetic() { in_esc = false; }
      continue;
    }
    len += ch.width().unwrap_or( 1 );
  }
  len
}
```

**`pad_unicode_width`:**
```rust
/// Pads `s` to at least `width` display columns.
/// Returns `s` unchanged if already at or above `width`.
pub( crate ) fn pad_unicode_width( s : &str, width : usize, align_right : bool ) -> String
{
  let content_width = unicode_visual_len( s );
  if content_width >= width { return s.to_owned(); }
  let pad = " ".repeat( width - content_width );
  if align_right { format!( "{pad}{s}" ) } else { format!( "{s}{pad}" ) }
}
```

## Call Site Replacements

| File | Line (approx.) | Before | After |
|------|----------------|--------|-------|
| `table.rs` L99 | Import | `{ visual_len, pad_to_width }` | `{ unicode_visual_len, pad_unicode_width }` |
| `table.rs` L369 | `format_single_line_row` | `pad_to_width( &cell, width, align )` | `pad_unicode_width( &cell, width, align )` |
| `table.rs` L478 | `format_multiline_row` | `pad_to_width( &cell, width, align )` | `pad_unicode_width( &cell, width, align )` |
| `table.rs` L643 | `calculate_column_widths_for_rows` header | `visual_len( header )` | `unicode_visual_len( header )` |
| `table.rs` L654 | `calculate_column_widths_for_rows` cell | `visual_len( cell )` | `unicode_visual_len( cell )` |

## Test Matrix

*(Written before any test code.)*

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|----------------|-------------------|-------------------|
| T015-P01 | Table with CJK chars in cells | Default config | Columns align in rendered output (CJK = 2 display cols) |
| T015-P02 | Table with emoji (width=2) | Default config | Columns align in rendered output |
| T015-P03 | Unit test | `unicode_visual_len("こんにちは")` | Returns 10 (5 chars × 2 display cols each) |
| T015-P04 | Unit test | `unicode_visual_len("\x1b[31mHello\x1b[0m")` | Returns 5 (ANSI codes stripped) |
| T015-P05 | Unit test | `pad_unicode_width("A", 5, false)` | Returns `"A    "` (4 trailing spaces) |
| T015-N03 | ASCII-only table | Default config | Output identical to pre-task baseline (regression guard) |
| T015-N04 | Unit test | `unicode_visual_len("")` | Returns 0; `pad_unicode_width("", 3, false)` returns `"   "` |
| T015-N05 | Unit test | `pad_unicode_width` with content wider than width | Returns content unchanged (no truncation) |
| T015-N06 | Unit test | `unicode_visual_len` with partial/malformed ANSI | No panic; best-effort result |
| T015-N07 | CJK + min_column_width | `min_column_width(12)` on CJK content | Floor applied to unicode-measured widths correctly |

## Acceptance Criteria

- `unicode_visual_len()` exists in `src/ansi_str.rs` as `pub(crate)`, strips ANSI codes,
  measures using `UnicodeWidthChar::width()`
- `pad_unicode_width()` exists in `src/ansi_str.rs` as `pub(crate)`, uses `unicode_visual_len`
  for padding calculation
- `pad_to_width` is replaced at BOTH L369 (single-line row) AND L478 (multiline row)
- `visual_len` is replaced at BOTH L643 AND L654 in `calculate_column_widths_for_rows()`
- No remaining `visual_len` or `pad_to_width` usage in `table.rs`
- All T015-P01–P05 positive tests pass
- All T015-N03–N07 negative/edge tests pass
- T015-P04 (ANSI stripping) passes, confirming ANSI codes excluded from width
- `w3 .test l::3` exits 0 with zero failures and zero warnings

## Validation Checklist

Desired answer for every question is YES.

**`src/ansi_str.rs` — new functions**
- [ ] Does `unicode_visual_len()` exist as `pub(crate)`?
- [ ] Does `pad_unicode_width()` exist as `pub(crate)`?
- [ ] Does `unicode_visual_len()` strip ANSI codes before measuring display width?
- [ ] Does `unicode_visual_len()` use `UnicodeWidthChar::width()` (not `.chars().count()`)?

**`src/formatters/table.rs` — call site replacements**
- [ ] Is `pad_to_width` replaced at L369 (single-line row)?
- [ ] Is `pad_to_width` replaced at L478 (multiline row)?
- [ ] Is `visual_len` replaced at L643 (header width calculation)?
- [ ] Is `visual_len` replaced at L654 (cell width calculation)?
- [ ] Is the import at L99 updated to use the new function names?
- [ ] Are there zero remaining occurrences of `visual_len` or `pad_to_width` in `table.rs`?

**Test coverage**
- [ ] Does T015-P04 assert `unicode_visual_len("\x1b[31mHello\x1b[0m") == 5`?
- [ ] Does T015-N03 confirm ASCII-only output is unchanged (regression guard)?
- [ ] Do all T015-P01–P05 positive tests pass?
- [ ] Do all T015-N03–N07 negative/edge tests pass?

**Final gate**
- [ ] Does `w3 .test l::3` exit 0 with zero warnings?

## Validation Procedure

### Measurements

**M1 — Red state confirmed**
Command: `RUSTFLAGS="-D warnings" cargo nextest run --test unicode_display_width_alignment 2>&1 | grep -E "FAILED|test result"`
Before: all pass. Expected after RED step: ≥7 failures. Deviation: 0 failures = tests not written.

**M2 — New functions exist**
Command: `grep -c "pub( crate ) fn unicode_visual_len\|pub( crate ) fn pad_unicode_width" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/ansi_str.rs`
Before: 0. Expected: 2. Deviation: <2 = functions missing.

**M3 — Old call sites fully replaced**
Command: `grep -c "pad_to_width\|visual_len" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs`
Before: ≥5. Expected: 0. Deviation: >0 = replacement incomplete.

**M4 — Green state**
Command: `w3 .test l::3`
Expected: 0 failures, 0 warnings.

### Anti-faking checks

**AF1 — Multiline call site replaced**
Command: `grep -n "pad_to_width" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs`
Expected: 0 matches. A common mistake is replacing only L369 and missing L478 (multiline row).

**AF2 — ANSI stripping verified**
T015-P04 must assert `unicode_visual_len("\x1b[31mHello\x1b[0m") == 5`. If this returns 15+
(counts escape chars), the stripping algorithm is broken.

## Outcomes

*(Completed. Task delivered and verified per acceptance criteria.)*
