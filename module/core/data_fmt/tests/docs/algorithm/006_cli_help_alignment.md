# Algorithm: CLI Help Alignment

### Scope

- **Purpose**: Drive test coverage for the CLI help alignment algorithm.
- **Responsibility**: Documents test cases for the CLI help text alignment algorithm in `docs/algorithm/006_cli_help_alignment.md`.
- **In Scope**: Key-description alignment, section header detection and emission, blank-line separation between sections, simple line indentation, empty view behavior, ANSI exclusion from alignment width, mixed-case header non-detection, per-section alignment reset.
- **Out of Scope**: Text formatter configuration; other `TextVariant` modes (bullets, numbered, sections, keyvalue, compact).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | key-description pairs aligned at max key width | ✅ |
| AC-2 | section header emitted with colon suffix, no indent | ✅ |
| AC-3 | blank line inserted before each section (not before first) | ✅ |
| AC-4 | simple lines indented but not aligned to description column | ✅ |
| AC-5 | empty view produces empty string | ✅ |
| AC-6 | longer key in later row expands alignment column for all | ✅ |
| AC-7 | ANSI escape codes in key text excluded from alignment width calculation | ✅ |
| AC-8 | mixed-case text not detected as section header | ✅ |
| AC-9 | all-uppercase with non-empty second column not treated as header | ✅ |
| AC-10 | alignment column resets per section — long key in section 1 does not affect section 2 | ✅ |

---

### AC-1: key-description pairs aligned at max key width

- **Given:** A `CliHelp` view with one section containing three key-description
  pairs where key lengths are 3, 7, and 5.
- **When:** Rendered with `TextFormatter` using `TextVariant::CliHelp`.
- **Then:** All description values start at the same horizontal column position;
  that position equals `indent + 7 (max key width) + 2 (default gap)`.

---

### AC-2: section header emitted with colon suffix, no indent

- **Given:** A section whose header row has an all-uppercase first column
  (e.g. `"OPTIONS"`) and an empty second column.
- **When:** Rendered.
- **Then:** The header line is `"OPTIONS:"` with no leading indentation; header
  text appears verbatim with a colon appended.

---

### AC-3: blank line inserted before each section (not before first)

- **Given:** A `CliHelp` view with two sections separated by a blank row.
- **When:** Rendered.
- **Then:** A blank line appears before the second section header but not before
  the first section header; the total blank-line count between sections is exactly 1.

---

### AC-4: simple lines indented but not aligned to description column

- **Given:** A section containing a simple line (first column populated, second
  column empty) mixed with key-description pairs.
- **When:** Rendered.
- **Then:** The simple line is indented by `indent` spaces but is NOT padded to the
  key-width + gap column; it starts immediately after the indent without extra padding.

---

### AC-5: empty view produces empty string

- **Given:** A `CliHelp` view with no sections and no rows.
- **When:** Rendered.
- **Then:** Output is an empty string `""`; no panic; no stray whitespace or
  newlines are emitted.

---

### AC-6: longer key in later row expands alignment column for all

- **Given:** A section where the first pair has key `"a"` (1 char) and the last
  pair has key `"long_key"` (8 chars); all intermediate keys are shorter than 8.
- **When:** Rendered.
- **Then:** All description values in the section align at `indent + 8 + 2`;
  even the first pair's description is at that column despite its key being short.

---

### AC-7: ANSI escape codes in key text excluded from alignment width calculation

- **Given:** A section where one key contains ANSI color codes
  (e.g. `"\x1b[32m--verbose\x1b[0m"`, visual width 9) and a sibling key is plain
  ASCII `"--help"` (6 chars).
- **When:** Rendered with `TextVariant::CliHelp`.
- **Then:** The alignment column is computed from visual key width (9) not raw byte
  count; descriptions for all keys start at `indent + 9 + 2`; the ANSI codes are
  preserved verbatim in the output without double-encoding.
- **Note:** Known bug — current implementation uses byte count instead of `visual_len`
  for alignment; this case should be tagged `bug_reproducer` once the fix is in.

---

### AC-8: mixed-case text not detected as section header

- **Given:** A section where a row's first column contains mixed-case text such as
  `"Options"` (not all-uppercase) alongside a key-description pair.
- **When:** Rendered with `TextVariant::CliHelp`.
- **Then:** The mixed-case row is NOT treated as a section header; it is rendered as
  a key-description pair (with indent and alignment) or a simple indented line —
  never as an unindented header with a colon suffix; only all-uppercase first columns
  trigger the header rendering path.

---

### AC-9: all-uppercase with non-empty second column not treated as header

- **Given:** A row whose first column is all-uppercase (e.g. `"OPTIONS"`) but whose
  second column is non-empty (e.g. `"description text"`).
- **When:** Rendered with `TextVariant::CliHelp`.
- **Then:** The row is rendered as a key-description pair, not as a section header;
  the colon suffix is not appended; the row is indented; both columns appear in the output.

---

### AC-10: alignment column resets per section — long key in section 1 does not affect section 2

- **Given:** A `CliHelp` view with two sections; section 1 has a key of length 20;
  section 2 has keys of length 4.
- **When:** Rendered.
- **Then:** Section 2 descriptions align at `indent + 4 + 2`, not at `indent + 20 + 2`;
  the alignment width is computed independently per section; no bleed from one
  section's max key width to another section.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/algorithm/006_cli_help_alignment.md`](../../../docs/algorithm/006_cli_help_alignment.md) | Source algorithm spec — header detection, alignment rules, section separation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/text_cli_help.rs`](../../text_cli_help.rs) | Algorithm test implementation |
