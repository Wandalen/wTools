# Feature: Table Heading

### Scope

- **Purpose**: Drive test coverage for the table heading feature.
- **Responsibility**: Documents test cases for `Heading` and `TableConfig::with_heading()` as specified in `docs/feature/007_table_caption.md`.
- **In Scope**: Heading rendering with title only, heading fields, table-width filling, width-ceiling clamping, no-heading regression, style interaction, title-exceeds-width edge case, empty-title edge case.
- **Out of Scope**: Auto-fit width algorithm (see `feature/005`); ANSI coloring (see `feature/004`); cell rendering (see `feature/001`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| FT-1 | title-only heading renders titled rule before table | ✅ |
| FT-2 | heading fields appear joined by field separator | ✅ |
| FT-3 | heading line fills to rendered table width with rule chars | ✅ |
| FT-4 | heading content equals rendered table width — trailing rule clamped to zero | ✅ |
| FT-5 | no-heading config produces identical output to current behavior | ✅ |
| FT-6 | heading renders before top border for grid and unicode_box styles | ✅ |
| FT-7 | title string longer than rendered table width — content not truncated, no trailing rule | ✅ |
| FT-8 | empty title string — lead rule emitted, no separator, trailing rule fills to table width | ✅ |
| FT-9 | heading example binaries compile and produce visible output | ✅ |

---

### FT-1: title-only heading renders titled rule before table

- **Given:** A `TableConfig::plain()` with `.with_heading(Heading::new("Results"))`.
- **When:** A two-column, one-row table is formatted.
- **Then:** The first line of the output starts with `"─── Results "` and ends with one or more `─` characters; the second line is the header row.

---

### FT-2: heading fields appear joined by field separator

- **Given:** A `Heading::new("Needs Review").with_field("28 PRs").with_field("15 repos")`.
- **When:** Rendered via `TableConfig::plain()`.
- **Then:** The first output line contains the substring `"Needs Review · 28 PRs · 15 repos"`.

---

### FT-3: heading line fills to rendered table width with rule chars

- **Given:** A `TableConfig::plain()` with `.with_heading(Heading::new("T").with_field("F"))` applied to a table whose rendered display width is known (e.g., two columns with plain style — compute the actual `table_width` via `compute_total_row_width(primary_widths)`).
- **When:** The table is formatted.
- **Then:** The first output line (the caption line) has exactly `table_width` display columns, i.e., `line.chars().count() == table_width` — use `.chars().count()`, not `.len()`, since the rule character `─` (U+2500) is 3 UTF-8 bytes.
- **Note:** The `terminal_width` setting does not control heading line width; it only affects the auto-fit column budget. Heading fills to actual rendered table width.

---

### FT-4: heading content equals rendered table width — trailing rule clamped to zero

- **Given:** A `TableConfig::plain()` with a heading whose content string (lead prefix + title + fields) is exactly `table_width` visible characters wide, where `table_width` is the actual rendered display width of the table.
- **When:** The table is formatted.
- **Then:** No trailing rule characters are appended; the heading line is exactly the
  content string with no additional `─` chars; the total line visual width equals `table_width`;
  the content is not truncated; no panic occurs.
- **Note:** Enforces the Width Ceiling invariant from `docs/invariant/005_caption.md`
  (Invariant 2) and the Clamp-at-zero property from `docs/algorithm/007_caption_rendering.md`.
  `trail_width = saturating_sub(table_width, content_len)` — when `content_len == table_width`,
  `trail_width` is 0 and the trailing rule loop produces no output.

---

### FT-5: no-heading config produces identical output to current behavior

- **Given:** A `TableConfig::plain()` with no `.with_heading()` call, and the same config built without `.with_heading()`.
- **When:** Both render the same table.
- **Then:** The two outputs are byte-identical; adding heading support does not change output for existing callers.

---

### FT-6: heading renders before top border for grid and unicode_box styles

- **Given:** A `TableConfig::grid()` (or `unicode_box()`) with `.with_heading(Heading::new("Grid Table"))`.
- **When:** The table is formatted.
- **Then:** The first line of output is the heading line (starts with `"─── Grid Table"`); the second line is the top border (`+---+` for grid, `┌───┐` for unicode_box).

---

### FT-7: title string longer than rendered table width — content not truncated, no trailing rule

- **Given:** A heading whose title string alone (including the lead prefix) is wider
  than the rendered table width; e.g., a long title applied to a narrow table.
- **When:** The table is formatted.
- **Then:** The content string is emitted verbatim without truncation; `trail_width`
  clamps to 0 (no trailing `─` characters); the heading line is wider than the
  rendered table width; no panic occurs.
- **Note:** Enforces the no-truncation guarantee of the Width Ceiling invariant —
  when content exceeds table width, the trailing rule is simply omitted; content
  is never cut.

---

### FT-8: empty title string — lead rule emitted, no separator, trailing rule fills to table width

- **Given:** A `Heading::new("")` (empty title, no fields) applied to a table with known rendered display width (`table_width`).
- **When:** The table is formatted.
- **Then:** The heading line begins with `"─── "` (lead prefix only); no separator
  character appears (the `·` field separator is omitted when title is empty and
  there are no fields); trailing rule fills from column 4 to column `table_width`; no panic
  and no empty-string division-by-zero panic occurs; `line.chars().count() == table_width`.

---

### FT-9: heading example binaries compile and produce visible output

- **Given:** Two example binaries: `examples/heading_basic.rs` (minimal single-heading demo)
  and `examples/heading_styles.rs` (multi-style demo with at least 3 heading+table combinations).
- **When:** Each is built and run via `cargo run --example NAME --features enabled`.
- **Then:** Both exit 0; both produce non-empty stdout containing at least one `─── ` lead
  prefix (proving a heading line is rendered); `heading_styles` output contains at least 3
  heading lines; neither binary references `TableCaption` or bare `.caption()` (uses `Heading`
  and `.with_heading()` exclusively); both are registered in `Cargo.toml` with
  `required-features = ["enabled"]` and listed in `examples/readme.md`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/feature/007_table_caption.md`](../../../docs/feature/007_table_caption.md) | Source feature spec — Heading builder, rendering contract, style interaction |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../table_caption_test.rs) | Heading rendering test implementation (FT-1..FT-8) |
| `examples/heading_basic.rs` | Heading example binary (FT-9) |
| `examples/heading_styles.rs` | Heading multi-style example binary (FT-9) |
