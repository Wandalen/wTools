# Algorithm Spec: CLI Help Alignment

## Source
`docs/algorithm/006_cli_help_alignment.md`

## Test Implementation
`tests/text_cli_help.rs`

## Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | key-description pairs aligned at max key width | ✅ |
| AC-2 | section header emitted with colon suffix, no indent | ✅ |
| AC-3 | blank line inserted before each section (not before first) | ✅ |
| AC-4 | simple lines indented but not aligned to description column | ✅ |
| AC-5 | empty view produces empty string | ✅ |
| AC-6 | longer key in later row expands alignment column for all | ✅ |

---

### AC-1: key-description pairs aligned at max key width

**Given:** A `CliHelp` view with one section containing three key-description
pairs where key lengths are 3, 7, and 5.
**When:** Rendered with `TextFormatter` using `TextVariant::CliHelp`.
**Then:** All description values start at the same horizontal column position;
that position equals `indent + 7 (max key width) + 2 (default gap)`.

---

### AC-2: section header emitted with colon suffix, no indent

**Given:** A section whose header row has an all-uppercase first column
(e.g. `"OPTIONS"`) and an empty second column.
**When:** Rendered.
**Then:** The header line is `"OPTIONS:"` with no leading indentation; header
text appears verbatim with a colon appended.

---

### AC-3: blank line inserted before each section (not before first)

**Given:** A `CliHelp` view with two sections separated by a blank row.
**When:** Rendered.
**Then:** A blank line appears before the second section header but not before
the first section header; the total blank-line count between sections is exactly 1.

---

### AC-4: simple lines indented but not aligned to description column

**Given:** A section containing a simple line (first column populated, second
column empty) mixed with key-description pairs.
**When:** Rendered.
**Then:** The simple line is indented by `indent` spaces but is NOT padded to the
key-width + gap column; it starts immediately after the indent without extra padding.

---

### AC-5: empty view produces empty string

**Given:** A `CliHelp` view with no sections and no rows.
**When:** Rendered.
**Then:** Output is an empty string `""`; no panic; no stray whitespace or
newlines are emitted.

---

### AC-6: longer key in later row expands alignment column for all

**Given:** A section where the first pair has key `"a"` (1 char) and the last
pair has key `"long_key"` (8 chars); all intermediate keys are shorter than 8.
**When:** Rendered.
**Then:** All description values in the section align at `indent + 8 + 2`;
even the first pair's description is at that column despite its key being short.
