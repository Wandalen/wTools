# Invariant Spec: ANSI and Unicode

## Source
`docs/invariant/002_ansi_unicode.md`

## Test Implementation
`tests/unicode_display_width_alignment.rs` (14 bug_reproducers, issue-003)

## Case Index

| ID | Name | Status |
|----|------|--------|
| IC-1 | ANSI escape codes excluded from column width measurement | ✅ |
| IC-2 | CJK wide characters maintain column alignment | ✅ |
| IC-3 | ANSI escape codes preserved verbatim in output | ✅ |

---

### IC-1: ANSI escape codes excluded from column width measurement

**Given:** A table where one cell contains an ANSI color-coded string such as
`"\x1b[32mhello\x1b[0m"` (5 visible characters + escape bytes).
**When:** The column width is computed and the cell is rendered.
**Then:** The column width reflects the visual character count (5) not the raw
byte count (15+); the cell is padded as if it contained 5 characters; adjacent
columns are not shifted by the invisible escape bytes.

---

### IC-2: CJK wide characters maintain column alignment

**Given:** A table where one cell contains CJK characters (e.g. `"文件"`, display
width 4) and an adjacent cell contains ASCII text in the same column.
**When:** Rendered with `TableFormatter`.
**Then:** The column separator appears at the same visual horizontal position for
all rows; the wide-character cell does not cause subsequent columns to shift
rightward; padding accounts for the 2-visual-column width of each CJK character.

---

### IC-3: ANSI escape codes preserved verbatim in output

**Given:** A table with an ANSI-colored cell (e.g. `"\x1b[31mred\x1b[0m"`).
**When:** Rendered.
**Then:** The raw output bytes contain the original ANSI escape sequences intact;
no sequences are stripped, reordered, or escaped as literal `\x1b`; the terminal
renders the intended color.
