# Invariant: ANSI and Unicode

### Scope

- **Purpose**: Drive test coverage for the ANSI and Unicode handling invariants.
- **Responsibility**: Documents test cases for ANSI escape code and Unicode width invariants in `docs/invariant/002_ansi_unicode.md`.
- **In Scope**: ANSI exclusion from width measurement, East Asian Width padding, ANSI verbatim preservation, per-line reset, per-sub-line color wrapping, `DecoratedText` iteration, CJK measurement gap.
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
| IN-7 | CJK characters in cells cause visual overflow beyond allocated column width | ✅ |
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
- **Note (known limitation):** Column width *measurement* uses `visual_len` (char
  count, not East Asian Width) — CJK cells wider than their char count may overflow
  allocated column space even with correct padding. `pad_to_width` correctness
  does not compensate for the measurement gap. Root fix: Fix(issue-003) in
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

### IN-7: CJK characters in cells cause visual overflow beyond allocated column width

- **Given:** A table where one cell contains CJK characters (e.g. `"中文"`, char
  count 2 but display width 4) and column width is allocated based on char count.
- **When:** Rendered with `TableFormatter`.
- **Then:** The CJK cell visually overflows its allocated column space by the
  number of extra display columns (i.e. char_count mismatch with East Asian Width);
  this is a known limitation documented in `docs/invariant/002_ansi_unicode.md`;
  the test asserts the overflow occurs as a regression guard against unexpected
  behavior changes.
- **Note:** Known limitation — `visual_len` uses char count not East Asian Width.
  Fix tracked as issue-003 in `src/ansi_str.rs`.

---

### IN-8: ANSI codes combined with CJK characters — width measurement excludes both

- **Given:** A cell containing `"\x1b[32m中文\x1b[0m"` (ANSI codes wrapping CJK
  characters).
- **When:** Column width is measured for layout purposes.
- **Then:** ANSI bytes are excluded from the measurement; the measurement reflects
  the CJK char count (2) not the display width (4) and not the byte count; the
  combined ANSI+CJK measurement gap is additive (both deficiencies apply simultaneously).

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/invariant/002_ansi_unicode.md`](../../../docs/invariant/002_ansi_unicode.md) | Source invariant spec — ANSI exclusion, EAW padding, verbatim preservation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/unicode_display_width_alignment.rs`](../../unicode_display_width_alignment.rs) | Invariant test implementation (14 bug_reproducers, issue-003) |
