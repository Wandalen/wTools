# Feature: Table Caption

### Scope

- **Purpose**: Drive test coverage for the table caption feature.
- **Responsibility**: Documents test cases for `TableCaption` and `TableConfig::caption()` as specified in `docs/feature/007_table_caption.md`.
- **In Scope**: Caption rendering with title only, caption fields, terminal-width filling, width-ceiling clamping, no-caption regression, style interaction, title-exceeds-width edge case, empty-title edge case.
- **Out of Scope**: Auto-fit width algorithm (see `feature/005`); ANSI coloring (see `feature/004`); cell rendering (see `feature/001`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| FT-1 | title-only caption renders titled rule before table | ✅ |
| FT-2 | caption fields appear joined by field separator | ✅ |
| FT-3 | caption line fills to terminal width with rule chars | ✅ |
| FT-4 | caption content equals terminal width — trailing rule clamped to zero | ✅ |
| FT-5 | no-caption config produces identical output to current behavior | ✅ |
| FT-6 | caption renders before top border for grid and unicode_box styles | ✅ |
| FT-7 | title string longer than terminal width — content not truncated, no trailing rule | ✅ |
| FT-8 | empty title string — lead rule emitted, no separator, no trailing content | ✅ |

---

### FT-1: title-only caption renders titled rule before table

- **Given:** A `TableConfig::plain()` with `.caption(TableCaption::new("Results"))`.
- **When:** A two-column, one-row table is formatted.
- **Then:** The first line of the output starts with `"─── Results "` and ends with one or more `─` characters; the second line is the header row.

---

### FT-2: caption fields appear joined by field separator

- **Given:** A `TableCaption::new("Needs Review").field("28 PRs").field("15 repos")`.
- **When:** Rendered via `TableConfig::plain()`.
- **Then:** The first output line contains the substring `"Needs Review · 28 PRs · 15 repos"`.

---

### FT-3: caption line fills to terminal width with rule chars

- **Given:** A `TableConfig::plain().terminal_width(Some(60)).caption(TableCaption::new("T").field("F"))`.
- **When:** The table is formatted.
- **Then:** The first output line (the caption line) has exactly 60 display columns, i.e., `line.chars().count() == 60` — use `.chars().count()`, not `.len()`, since the rule character `─` (U+2500) is 3 UTF-8 bytes.

---

### FT-4: caption content equals terminal width — trailing rule clamped to zero

- **Given:** A `TableConfig::plain().terminal_width(Some(N))` with a caption whose
  content string (lead prefix + title + fields) is exactly N visible characters wide.
- **When:** The table is formatted.
- **Then:** No trailing rule characters are appended; the caption line is exactly the
  content string with no additional `─` chars; the total line visual width equals N;
  the content is not truncated; no panic occurs.
- **Note:** Enforces the Width Ceiling invariant from `docs/invariant/005_caption.md`
  (Invariant 2) and the Clamp-at-zero property from `docs/algorithm/007_caption_rendering.md`.
  `trail_width = max(0, terminal_width - content_len)` — when `content_len == terminal_width`,
  `trail_width` is 0 and the trailing rule loop produces no output.

---

### FT-5: no-caption config produces identical output to current behavior

- **Given:** A `TableConfig::plain()` with no `.caption()` call, and the same config built without `.caption()`.
- **When:** Both render the same table.
- **Then:** The two outputs are byte-identical; adding `caption` support does not change output for existing callers.

---

### FT-6: caption renders before top border for grid and unicode_box styles

- **Given:** A `TableConfig::grid()` (or `unicode_box()`) with `.caption(TableCaption::new("Grid Table"))`.
- **When:** The table is formatted.
- **Then:** The first line of output is the caption line (starts with `"─── Grid Table"`); the second line is the top border (`+---+` for grid, `┌───┐` for unicode_box).

---

### FT-7: title string longer than terminal width — content not truncated, no trailing rule

- **Given:** A caption whose title string alone (including the lead prefix) is wider
  than `terminal_width`; e.g. `terminal_width(Some(10))` with title `"A very long title"`.
- **When:** The table is formatted.
- **Then:** The content string is emitted verbatim without truncation; `trail_width`
  clamps to 0 (no trailing `─` characters); the caption line is wider than the
  configured terminal width; no panic occurs.
- **Note:** Enforces the no-truncation guarantee of the Width Ceiling invariant —
  when content exceeds terminal width, the trailing rule is simply omitted; content
  is never cut.

---

### FT-8: empty title string — lead rule emitted, no separator, no trailing content

- **Given:** A `TableCaption::new("")` (empty title, no fields) with
  `terminal_width(Some(20))`.
- **When:** The table is formatted.
- **Then:** The caption line begins with `"─── "` (lead prefix only); no separator
  character appears (the `·` field separator is omitted when title is empty and
  there are no fields); trailing rule fills from column 4 to column 20; no panic
  and no empty-string division-by-zero panic occurs.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/feature/007_table_caption.md`](../../../docs/feature/007_table_caption.md) | Source feature spec — TableCaption builder, rendering contract, style interaction |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../table_caption_test.rs) | Caption rendering test implementation (FT-1..FT-8) |
