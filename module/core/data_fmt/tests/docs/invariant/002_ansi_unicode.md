# Invariant: ANSI and Unicode

### Scope

- **Purpose**: Drive test coverage for the ANSI and Unicode handling invariants.
- **Responsibility**: Documents test cases for ANSI escape code and Unicode width invariants in `docs/invariant/002_ansi_unicode.md`.
- **In Scope**: ANSI exclusion from width measurement, East Asian Width padding, ANSI verbatim preservation, per-line reset, per-sub-line color wrapping, `DecoratedText` iteration, CJK column allocation via EAW, `visual_len` char-count gap in truncation path.
- **Out of Scope**: Color theme feature behavior (see `feature/004`); auto-wrap backward compatibility (see `invariant/003`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| IN-1 | ANSI escape codes excluded from column width measurement | ✅ |
| IN-2 | pad_to_width uses East Asian Width for correct wide-character padding | ✅ |
| IN-3 | ANSI escape codes preserved verbatim in output | ✅ |
| IN-4 | every colored output line ends with ANSI reset before newline | ✅ |
| IN-5 | multiline cells receive per-sub-line color wrapping | ✅ |
| IN-6 | DecoratedText detail lines iterate raw text, not rendered output | ✅ |
| IN-7 | CJK characters allocated correct column width via East Asian Width | ✅ |
| IN-8 | ANSI codes combined with CJK characters — width measurement excludes both | ✅ |

---

### IN-1: ANSI escape codes excluded from column width measurement

- **Given:** A table where one cell contains an ANSI color-coded string such as
  `"\x1b[32mhello\x1b[0m"` (5 visible characters + escape bytes).
- **When:** The column width is computed and the cell is rendered.
- **Then:** The column width reflects the visual character count (5) not the raw
  byte count (15+); the cell is padded as if it contained 5 characters; adjacent
  columns are not shifted by the invisible escape bytes.

---

### IN-2: pad_to_width uses East Asian Width for correct wide-character padding

- **Given:** A table where one cell contains CJK characters (e.g. `"文件"`, display
  width 4 but char count 2) alongside a sibling cell with ASCII text in the same
  column.
- **When:** Rendered with `TableFormatter`.
- **Then:** `pad_to_width` computes padding using East Asian Width (via
  `unicode_width::UnicodeWidthStr::width`) so the CJK cell is padded to the
  correct terminal-column width; the column separator does not shift leftward
  relative to ASCII rows; no misalignment in the padding characters themselves.
- **Note:** Column width allocation uses `unicode_visual_len` (East Asian Width)
  via `calculate_column_widths_for_rows()`, so `pad_to_width` and column allocation
  are both EAW-aware. The `visual_len` function (char count) is used in a separate
  path (`truncate_cell`) — that is the known limitation tracked as BUG-001 in
  `src/ansi_str.rs`.

---

### IN-3: ANSI escape codes preserved verbatim in output

- **Given:** A table with an ANSI-colored cell (e.g. `"\x1b[31mred\x1b[0m"`).
- **When:** Rendered.
- **Then:** The raw output bytes contain the original ANSI escape sequences intact;
  no sequences are stripped, reordered, or escaped as literal `\x1b`; the terminal
  renders the intended color.

---

### IN-4: every colored output line ends with ANSI reset before newline

- **Given:** A table row where at least one cell contains ANSI color escape codes
  spanning the full cell value (e.g. `"\x1b[32mgreen\x1b[0m"`).
- **When:** Rendered with `TableFormatter`.
- **Then:** Each physical output line containing color sequences ends with an ANSI
  reset code (`\x1b[0m`) before the line terminator; no color bleeds from one output
  line to the next; the terminal background color is unaffected after each line.

---

### IN-5: multiline cells receive per-sub-line color wrapping

- **Given:** A cell containing `"\x1b[31mred-line1\x1b[0m\nline2"` (color on first
  sub-line only; second sub-line is plain text).
- **When:** The cell is rendered in a multiline row with `TableFormatter`.
- **Then:** Sub-line 0 contains the original ANSI sequences (`\x1b[31m` and
  `\x1b[0m`); sub-line 1 contains `"line2"` with no injected color codes; no
  sequences are copied from one sub-line to another; each sub-line is rendered
  independently.

---

### IN-6: DecoratedText detail lines iterate raw text, not rendered output

- **Given:** A `DecoratedText` value whose body contains `\n`-separated segments
  where individual segments carry ANSI formatting.
- **When:** The detail lines are iterated (e.g. for sub-row rendering).
- **Then:** Each segment is returned as its original raw text including any ANSI
  codes; no additional rendering passes are applied; the caller receives the
  unprocessed segment strings in order without modification.

---

### IN-7: CJK characters allocated correct column width via East Asian Width

- **Given:** A table with one column containing `"中文"` (char count 2, display
  width 4) in one row and `"ab"` in another row; both in the same single column.
- **When:** Rendered with `TableFormatter`.
- **Then:** Column width is allocated using `unicode_visual_len` (East Asian Width),
  so the column is 4 display columns wide; the CJK row (`"中文"`) and the padded
  ASCII row (`"ab  "`) have equal display widths — no visual overflow occurs; the
  column separator aligns correctly on both rows.
- **Note:** `calculate_column_widths_for_rows()` uses East Asian Width for column
  measurement (via `unicode_visual_len`). The `visual_len` char-count path only
  affects `truncate_cell` — tracked as BUG-001 in `src/ansi_str.rs`.

---

### IN-8: ANSI codes combined with CJK characters — width measurement excludes both

- **Given:** The `visual_len()` function (char-count measurement, used in the
  `truncate_cell` path) is called on `"\x1b[32m中文\x1b[0m"` and on `"ab"`.
- **When:** `visual_len()` is invoked directly on the ANSI+CJK string.
- **Then:** ANSI bytes are stripped before counting; the result is char count 2
  (not display width 4 and not byte count); `visual_len("ab")` equals
  `visual_len("\x1b[32m中文\x1b[0m")` — both are 2; the two deficiencies of
  `visual_len` (ANSI exclusion + EAW gap) are additive in this code path.
- **Note:** Column width *allocation* uses `unicode_visual_len` (East Asian Width)
  and is not affected by this gap. The `visual_len` char-count limitation only
  impacts `truncate_cell` — tracked as BUG-001 in `src/ansi_str.rs`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/invariant/002_ansi_unicode.md`](../../../docs/invariant/002_ansi_unicode.md) | Source invariant spec — ANSI exclusion, EAW padding, verbatim preservation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/unicode_display_width_alignment.rs`](../../unicode_display_width_alignment.rs) | Invariant test implementation (7 bug_reproducers, BUG-001) |
