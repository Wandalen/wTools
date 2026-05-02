# Manual Testing Plan for data_fmt

This document describes manual testing procedures for verifying column truncation
and multiline cell features in data_fmt.

## Features Under Test

### 1. Column Truncation (`max_column_width`)
- ANSI-aware truncation (visual length calculation excluding color codes)
- Truncation marker appending
- Edge cases (marker longer than limit, exact fit, empty cells)
- Backward compatibility (disabled by default)

### 2. Multiline Cells (automatic `\n` detection)
- Automatic detection and rendering
- Row height calculation (max lines across cells)
- Column alignment preservation
- Padding shorter cells to match row height
- ANSI code support in multiline contexts

### 3. Combined Features (Multiline + Truncation)
- Per-line truncation after splitting
- ANSI code preservation through both transformations
- Visual correctness of combined output

## Test Cases

### TC-001: basic column truncation

**Given:** A table with a cell containing more than 20 characters.
**When:** `max_column_width` is set to 20 and the table is rendered in a terminal.
**Then:** Content is truncated to 17 visible characters plus `"..."` marker
(20 total); ANSI codes are not counted toward the limit; output is visually
readable.

---

### TC-002: ANSI code preservation in truncation

**Given:** A cell with ANSI colored text whose visual length exceeds the
configured `max_column_width`.
**When:** The table is rendered with truncation active.
**Then:** The ANSI color codes appear in the output before the truncated text;
the visual length (excluding codes) equals `max_column_width`; terminal color
rendering is not broken.

---

### TC-003: custom truncation marker

**Given:** `truncation_marker` is set to `"…"` (a single unicode ellipsis
character); a cell exceeds the configured limit.
**When:** The table is rendered.
**Then:** The custom marker `"…"` appears at the end of the truncated content
instead of `"..."`; visual width calculation accounts for the unicode marker's
display width.

---

### TC-004: marker longer than limit (edge case)

**Given:** `max_column_width` is 5; `truncation_marker` is `"........."` (9
characters, longer than the limit).
**When:** The table is rendered.
**Then:** Truncation is handled gracefully via `saturating_sub`; no panic
occurs; output is non-empty and the marker is emitted without corrupting
adjacent columns.

---

### TC-005: exact fit — no truncation applied

**Given:** A cell containing exactly 20 visible characters; `max_column_width`
is set to 20.
**When:** The table is rendered.
**Then:** The full cell content is displayed without any truncation marker;
visual length equals exactly 20; no extra characters appended.

---

### TC-006: basic multiline cell rendering

**Given:** A single cell containing `"Line1\nLine2\nLine3"`.
**When:** Rendered with plain table style.
**Then:** Three physical lines are emitted for that cell within the row;
column separators appear on each line; other cells in the row are padded to
match the 3-line height.

---

### TC-007: multiline row with mixed cell heights

**Given:** A row with cells `["A", "B\nC\nD", "E"]` (heights 1, 3, 1).
**When:** Rendered.
**Then:** Row height is 3 (the maximum); cells `"A"` and `"E"` are padded to
3 lines with empty continuation lines; column separators align vertically
across all 3 physical lines.

---

### TC-008: multiline cell with ANSI color codes

**Given:** A cell containing `"\x1b[31mRed\x1b[0m\n\x1b[32mGreen\x1b[0m"`.
**When:** Rendered.
**Then:** Sub-line 0 renders in red; sub-line 1 renders in green; ANSI codes
do not affect the visual column alignment; both lines align with adjacent cells.

---

### TC-009: empty logical lines within a multiline cell

**Given:** A cell containing `"Line1\n\nLine3"` (an empty line 2 between content).
**When:** Rendered.
**Then:** Three physical sub-lines are emitted; sub-line 1 is blank with correct
column-width padding; the blank line is not collapsed.

---

### TC-010: multiline combined with truncation

**Given:** A cell containing `"Very long first line that exceeds the limit\nShort"`;
`max_column_width` is 20.
**When:** Rendered.
**Then:** Sub-line 0 is truncated to 17 characters + `"..."` marker; sub-line 1
(`"Short"`) is displayed fully without truncation; each sub-line is evaluated
for truncation independently.

---

### TC-011: multiline + truncation + ANSI codes combined

**Given:** A cell containing
`"\x1b[31mVery long red text here\x1b[0m\n\x1b[32mShort green\x1b[0m"`;
`max_column_width` is 20.
**When:** Rendered.
**Then:** Sub-line 0: red color preserved; truncated with marker at visual
position 20; ANSI reset included. Sub-line 1: green color preserved; not
truncated. Visual length calculation excludes escape bytes on both lines.

---

### TC-012: multiline cells in CSV format

**Given:** A cell containing `"A\nB"` formatted with `CsvFormatter`.
**When:** Rendered.
**Then:** The `\n` is escaped as a literal `\n` sequence (not a physical line
break); the CSV output remains single-line per record; the CSV remains parseable
by standard parsers.

---

### TC-013: truncation backward compatibility

**Given:** A table with long cell content rendered without any `max_column_width`
setting.
**When:** Rendered.
**Then:** Full cell content is displayed; no truncation is applied; output is
identical to v0.4.0 behavior (truncation is disabled by default).

---

### TC-014: per-column independent truncation

**Given:** A table with 3 columns; `max_column_width` set to 15; rows with
varying cell lengths across all columns.
**When:** Rendered.
**Then:** Each column's cells are independently evaluated against the limit;
cells under 15 characters display fully; cells over 15 characters are truncated;
truncation in one column does not affect adjacent columns.

---

### TC-015: headers truncated by max_column_width

**Given:** A table with at least one header name longer than `max_column_width`;
`max_column_width` is 20.
**When:** Rendered.
**Then:** Long headers are truncated using the same rules as data cells; short
headers display fully; consistent truncation behavior for both headers and data.

---

## Visual Inspection Tests

These tests require human visual verification in a terminal:

### VI-001: terminal color display
- Run test with ANSI colored output.
- Verify colors render correctly.
- Check truncation does not break color sequences.

### VI-002: unicode truncation marker
- Use unicode markers (`…`, `→`, `⋯`).
- Verify correct display in terminal.
- Check width calculation is accurate.

### VI-003: table border alignment
- Render multiline cells with bordered style.
- Verify borders align correctly across all sub-lines.
- Check corner characters position correctly.

### VI-004: markdown table format
- Render multiline cells with markdown style.
- Verify markdown syntax is valid.
- Check alignment characters (`|`, `-`) are positioned correctly.

## Test Execution

### Prerequisites
- Terminal with ANSI color support (for color tests)
- Unicode-capable terminal (for unicode marker tests)
- data_fmt installed

### Running Tests

```bash
# Run all manual tests via the combined runner
cargo run --example verify_runner

# Or run individual manual test scenarios
cargo run --example verify_truncation
cargo run --example verify_multiline
cargo run --example verify_combined
```

### Recording Results

Update each test case status above:
- Pass — feature works as expected
- Fail — issue found (document in Issues section below)

## Issues Found

Document any issues discovered during manual testing:

```
ID: ISSUE-XXX
Test Case: TC-XXX
Severity: Critical | High | Medium | Low
Description: [What went wrong]
Steps to Reproduce: [Exact steps]
Expected: [What should happen]
Actual: [What actually happened]
Fix Applied: [Description of fix] or [Not yet fixed]
```

## Completion Criteria

All test cases pass; zero issues with severity Critical or High remaining.
