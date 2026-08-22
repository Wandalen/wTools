# Feature: Table Heading

### Scope

- **Purpose**: Drive test coverage for the heading and footer feature across every adopting formatter.
- **Responsibility**: Documents test cases for `Heading`, `TableConfig::with_heading()` / `with_footer()`, `TreeConfig::with_heading()` / `with_footer()`, `ExpandedConfig::with_heading()` / `with_footer()`, and `TextFormatter::with_heading()` / `with_footer()` as specified in `docs/feature/007_table_heading.md`.
- **In Scope**: Heading and footer rendering with title only, heading fields, target-width filling (table width for Table, widest rendered line for Tree/Expanded/Text), width-ceiling clamping, no-heading/no-footer regression, style interaction, title-exceeds-width edge case, empty-title edge case, header+footer coexistence.
- **Out of Scope**: Auto-fit width algorithm (see `feature/005`); ANSI coloring (see `feature/004`); cell rendering (see `feature/001`); Yaml/Toml/Sql heading/footer coverage (not yet implemented).

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
| FT-10 | title-only footer renders titled rule after table | ✅ |
| FT-11 | footer fields appear joined by field separator | ✅ |
| FT-12 | footer line fills to rendered table width with rule chars | ✅ |
| FT-13 | footer content equals rendered table width — trailing rule clamped to zero | ✅ |
| FT-14 | no-footer config produces identical output to current behavior | ✅ |
| FT-15 | footer renders after bottom border for grid and unicode_box styles | ✅ |
| FT-16 | title string longer than rendered table width — footer content not truncated, no trailing rule | ✅ |
| FT-17 | empty footer title string — lead rule emitted, no separator, trailing rule fills to table width | ✅ |
| FT-18 | heading and footer coexist on the same table without interfering | ✅ |
| FT-19 | title-only heading renders before tree output | ✅ |
| FT-20 | title-only footer renders after tree output | ✅ |
| FT-21 | heading/footer fill to the widest rendered tree line, not a fixed table-style width | ✅ |
| FT-22 | no heading/footer configured produces byte-identical tree output | ✅ |
| FT-23 | heading and footer coexist on tree output without interfering | ✅ |
| FT-24 | heading applies on the leaf-only-root early-return branch | ✅ |
| FT-25 | title-only heading renders before expanded output | ✅ |
| FT-26 | title-only footer renders after expanded output | ✅ |
| FT-27 | heading/footer fill to the widest rendered expanded line, not a fixed table-style width | ✅ |
| FT-28 | no heading/footer configured produces byte-identical expanded output | ✅ |
| FT-29 | heading and footer coexist on expanded output without interfering | ✅ |
| FT-30 | heading applies on the empty-headers early-return branch | ✅ |
| FT-31 | title-only heading renders before text output | ✅ |
| FT-32 | title-only footer renders after text output | ✅ |
| FT-33 | heading/footer fill to the widest rendered text line, not a fixed table-style width | ✅ |
| FT-34 | no heading/footer configured produces byte-identical text output | ✅ |
| FT-35 | heading and footer coexist on text output without interfering | ✅ |
| FT-36 | heading applies on the empty-cli-help-rows early-return branch | ✅ |

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
- **Then:** The first output line (the heading line) has exactly `table_width` display columns, i.e., `line.chars().count() == table_width` — use `.chars().count()`, not `.len()`, since the rule character `─` (U+2500) is 3 UTF-8 bytes.
- **Note:** The `terminal_width` setting does not control heading line width; it only affects the auto-fit column budget. Heading fills to actual rendered table width.

---

### FT-4: heading content equals rendered table width — trailing rule clamped to zero

- **Given:** A `TableConfig::plain()` with a heading whose content string (lead prefix + title + fields) is exactly `table_width` visible characters wide, where `table_width` is the actual rendered display width of the table.
- **When:** The table is formatted.
- **Then:** No trailing rule characters are appended; the heading line is exactly the
  content string with no additional `─` chars; the total line visual width equals `table_width`;
  the content is not truncated; no panic occurs.
- **Note:** Enforces the Width Ceiling invariant from `docs/invariant/005_heading.md`
  (Invariant 2) and the Clamp-at-zero property from `docs/algorithm/007_heading_rendering.md`.
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

### FT-10: title-only footer renders titled rule after table

- **Given:** A `TableConfig::plain()` with `.with_footer(Heading::new("Results"))`.
- **When:** A two-column, one-row table is formatted.
- **Then:** The last line of the output starts with `"─── Results "` and ends with one or more `─` characters; the line immediately before it is the last data row.

---

### FT-11: footer fields appear joined by field separator

- **Given:** A `Heading::new("52 rows").with_field("3 filtered")` attached via `.with_footer()`.
- **When:** Rendered via `TableConfig::plain()`.
- **Then:** The last output line contains the substring `"52 rows · 3 filtered"`.

---

### FT-12: footer line fills to rendered table width with rule chars

- **Given:** A `TableConfig::plain()` with `.with_footer(Heading::new("T").with_field("F"))` applied to a table whose rendered display width (`table_width`) is known.
- **When:** The table is formatted.
- **Then:** The last output line (the footer line) has exactly `table_width` display columns.

---

### FT-13: footer content equals rendered table width — trailing rule clamped to zero

- **Given:** A `TableConfig::plain()` with a footer whose content string is exactly `table_width` visible characters wide.
- **When:** The table is formatted.
- **Then:** No trailing rule characters are appended; the footer line visual width equals `table_width`; content is not truncated; no panic occurs.

---

### FT-14: no-footer config produces identical output to current behavior

- **Given:** A `TableConfig::plain()` with no `.with_footer()` call, and the same config built without `.with_footer()`.
- **When:** Both render the same table.
- **Then:** The two outputs are byte-identical.

---

### FT-15: footer renders after bottom border for grid and unicode_box styles

- **Given:** A `TableConfig::grid()` (or `unicode_box()`) with `.with_footer(Heading::new("Grid Table"))`.
- **When:** The table is formatted.
- **Then:** The last line of output is the footer line (starts with `"─── Grid Table"`); the line immediately before it is the bottom border (`+---+` for grid, `└───┘` for unicode_box).

---

### FT-16: title string longer than rendered table width — footer content not truncated, no trailing rule

- **Given:** A footer whose title string alone (including the lead prefix) is wider than the rendered table width.
- **When:** The table is formatted.
- **Then:** The content string is emitted verbatim without truncation; `trail_width` clamps to 0; the footer line is wider than the rendered table width; no panic occurs.

---

### FT-17: empty footer title string — lead rule emitted, no separator, trailing rule fills to table width

- **Given:** A `Heading::new("")` (empty title, no fields) attached via `.with_footer()` to a table with known rendered display width.
- **When:** The table is formatted.
- **Then:** The footer line begins with `"─── "`; no `·` separator appears; trailing rule fills to `table_width`; no panic.

---

### FT-18: heading and footer coexist on the same table without interfering

- **Given:** A `TableConfig::plain()` with both `.with_heading(Heading::new("Top"))` and `.with_footer(Heading::new("End"))` attached — equal-length titles (3 chars each) chosen so both individually fit within `table_width` without tripping the width-ceiling clamp (that clamp is covered separately by FT-13/FT-16).
- **When:** The table is formatted.
- **Then:** The first output line is the heading (starts with `"─── Top"`); the last output line is the footer (starts with `"─── End"`); both lines fill to the same `table_width`; the table body between them is unaffected.

---

### FT-19: title-only heading renders before tree output

- **Given:** A `TreeConfig::new()` with `.with_heading(Heading::new("Project"))`.
- **When:** A two-child tree is formatted via `TreeFormatter::with_config(config).format(&tree, render_item)`.
- **Then:** The first output line starts with `"─── Project"`; the tree body (e.g. the `"alpha"` child) still renders unaffected.

---

### FT-20: title-only footer renders after tree output

- **Given:** A `TreeConfig::new()` with `.with_footer(Heading::new("2 items"))`.
- **When:** The same two-child tree is formatted.
- **Then:** The last output line starts with `"─── 2 items"`; the tree body (e.g. the `"beta"` child) still renders unaffected.

---

### FT-21: heading/footer fill to the widest rendered tree line, not a fixed table-style width

- **Given:** A `TreeConfig::new()` with `.with_heading(Heading::new("H"))` applied to a two-child tree.
- **When:** The tree is formatted.
- **Then:** The heading line's display width equals the maximum display width across every body line (the lines after the heading) — not any config-declared or table-style width.
- **Note:** Tree lines are ragged (length varies by depth and content), unlike Table's fixed column width — this is the Tree-specific instance of the Width Ceiling invariant's target-width derivation (`docs/algorithm/007_heading_rendering.md § Target-Width Computation Per Formatter`).

---

### FT-22: no heading/footer configured produces byte-identical tree output

- **Given:** A `TreeFormatter::with_config(TreeConfig::new())` with no heading/footer, and the `TreeFormatter::new()` baseline.
- **When:** Both format the same tree.
- **Then:** The two outputs are byte-identical; the output does not start with `'─'` (no heading rule rendered) — the Tree-specific instance of Invariant 1 (No-Heading/No-Footer Passthrough).

---

### FT-23: heading and footer coexist on tree output without interfering

- **Given:** A `TreeConfig::new()` with both `.with_heading(Heading::new("Top"))` and `.with_footer(Heading::new("End"))` — equal-length titles (3 chars each), same rationale as FT-18.
- **When:** The tree is formatted.
- **Then:** The first output line is the heading (starts with `"─── Top"`); the last output line is the footer (starts with `"─── End"`); both lines fill to the same width; the tree body between them is unaffected.

---

### FT-24: heading applies on the leaf-only-root early-return branch

- **Given:** A leaf-only root `TreeNode` (has data, no children) with `TreeConfig::new().with_heading(Heading::new("Leaf"))`.
- **When:** The tree is formatted.
- **Then:** The output is exactly two lines — the heading line first (starts with `"─── Leaf"`), then the leaf line (contains both the node name and its rendered data).
- **Note:** Proves the heading/footer wrap applies uniformly across all three of `TreeFormatter::format()`'s internal branches (leaf-only-root, empty-root, standard multi-child), not only the standard path.

---

### FT-25: title-only heading renders before expanded output

- **Given:** An `ExpandedConfig::new()` with `.with_heading(Heading::new("Users"))`.
- **When:** A two-column, two-row `TableView` is formatted via `ExpandedFormatter`.
- **Then:** The first output line starts with `"─── Users"`; the expanded body (e.g. `"Alice"`) still renders unaffected.

---

### FT-26: title-only footer renders after expanded output

- **Given:** An `ExpandedConfig::new()` with `.with_footer(Heading::new("2 records"))`.
- **When:** The same `TableView` is formatted.
- **Then:** The last output line starts with `"─── 2 records"`; the expanded body (e.g. `"Bob"`) still renders unaffected.

---

### FT-27: heading/footer fill to the widest rendered expanded line, not a fixed table-style width

- **Given:** An `ExpandedConfig::new()` with `.with_heading(Heading::new("H"))` applied to a two-column, two-row view.
- **When:** The view is formatted.
- **Then:** The heading line's display width equals the maximum display width across every body line (the lines after the heading) — not any config-declared or table-style width.
- **Note:** Expanded lines are ragged (key/value width varies per record), unlike Table's fixed column width — the Expanded-specific instance of the Width Ceiling invariant's target-width derivation (`docs/algorithm/007_heading_rendering.md § Target-Width Computation Per Formatter`), same technique as Tree's FT-21.

---

### FT-28: no heading/footer configured produces byte-identical expanded output

- **Given:** An `ExpandedFormatter::with_config(ExpandedConfig::new())` with no heading/footer, and the `ExpandedFormatter::new()` baseline.
- **When:** Both format the same `TableView`.
- **Then:** The two outputs are byte-identical; the output does not start with `'─'` (no heading rule rendered) — the Expanded-specific instance of Invariant 1 (No-Heading/No-Footer Passthrough).

---

### FT-29: heading and footer coexist on expanded output without interfering

- **Given:** An `ExpandedConfig::new()` with both `.with_heading(Heading::new("Top"))` and `.with_footer(Heading::new("End"))` — equal-length titles (3 chars each), same rationale as FT-18/FT-23.
- **When:** The view is formatted.
- **Then:** The first output line is the heading (starts with `"─── Top"`); the last output line is the footer (starts with `"─── End"`); both lines fill to the same width; the expanded body between them is unaffected.

---

### FT-30: heading applies on the empty-headers early-return branch

- **Given:** A `TableView` built with zero headers (`RowBuilder::new(vec![]).build_view()`) and `ExpandedConfig::new().with_heading(Heading::new("Empty"))`.
- **When:** The view is formatted.
- **Then:** The output is exactly one line — the heading line (starts with `"─── Empty"`) — with no body content, since `format_view()` returns immediately on the `headers.is_empty()` branch.
- **Note:** Proves the heading/footer wrap applies uniformly across `ExpandedFormatter`'s internal branches, not only the populated-headers path. Mirrors Tree's FT-24 (leaf-only-root branch coverage).

---

### FT-31: title-only heading renders before text output

- **Given:** A `TextFormatter::bullets()` with `.with_heading(Heading::new("Users"))`.
- **When:** A two-row `TableView` is formatted.
- **Then:** The first output line starts with `"─── Users"`; the text body (e.g. `"Alice"`) still renders unaffected.

---

### FT-32: title-only footer renders after text output

- **Given:** A `TextFormatter::bullets()` with `.with_footer(Heading::new("2 records"))`.
- **When:** The same `TableView` is formatted.
- **Then:** The last output line starts with `"─── 2 records"`; the text body (e.g. `"Bob"`) still renders unaffected.

---

### FT-33: heading/footer fill to the widest rendered text line, not a fixed table-style width

- **Given:** A `TextFormatter::bullets()` with `.with_heading(Heading::new("H"))` applied to a two-row view.
- **When:** The view is formatted.
- **Then:** The heading line's display width equals the maximum display width across every body line (the lines after the heading) — not any config-declared or table-style width.
- **Note:** Text lines are ragged (length varies by `TextVariant` and content), unlike Table's fixed column width — the Text-specific instance of the Width Ceiling invariant's target-width derivation (`docs/algorithm/007_heading_rendering.md § Target-Width Computation Per Formatter`), same technique as Tree's FT-21 / Expanded's FT-27.

---

### FT-34: no heading/footer configured produces byte-identical text output

- **Given:** A `TextFormatter::bullets()` with no heading/footer, and the `TextFormatter::new(TextVariant::Bullets)` baseline.
- **When:** Both format the same `TableView`.
- **Then:** The two outputs are byte-identical; the output does not start with `'─'` (no heading rule rendered) — the Text-specific instance of Invariant 1 (No-Heading/No-Footer Passthrough).

---

### FT-35: heading and footer coexist on text output without interfering

- **Given:** A `TextFormatter::bullets()` with both `.with_heading(Heading::new("Top"))` and `.with_footer(Heading::new("End"))` — equal-length titles (3 chars each), same rationale as FT-18/FT-23/FT-29.
- **When:** The view is formatted.
- **Then:** The first output line is the heading (starts with `"─── Top"`); the last output line is the footer (starts with `"─── End"`); both lines fill to the same width; the text body between them is unaffected.

---

### FT-36: heading applies on the empty-cli-help-rows early-return branch

- **Given:** A `TableView` built with headers but zero rows (`RowBuilder::new(vec!["Term".into(), "Description".into()]).build_view()`) and `TextFormatter::new(TextVariant::CliHelp).with_heading(Heading::new("Empty"))`.
- **When:** The view is formatted.
- **Then:** The output is exactly one line — the heading line (starts with `"─── Empty"`) — with no body content, since `format_cli_help()` returns `String::new()` immediately on the `data.rows.is_empty()` branch, and that empty body still flows through `Format::format()`'s single `wrap_with_heading_footer()` call.
- **Note:** Proves the heading/footer wrap applies uniformly at `TextFormatter`'s single trait-level funnel point regardless of which `TextVariant` match arm (or internal early return within one) produced the body. Mirrors Tree's FT-24 and Expanded's FT-30.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/feature/007_table_heading.md`](../../../docs/feature/007_table_heading.md) | Source feature spec — Heading builder, rendering contract, style interaction, Tree/Expanded/Text usage |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_heading_test.rs`](../../table_heading_test.rs) | Heading rendering test implementation (FT-1..FT-8) |
| `examples/heading_basic.rs` | Heading example binary (FT-9) |
| `examples/heading_styles.rs` | Heading multi-style example binary (FT-9) |
| [`tests/table_footer_test.rs`](../../table_footer_test.rs) | Footer rendering test implementation (FT-10..FT-18) |
| [`tests/tree_heading_test.rs`](../../tree_heading_test.rs) | Tree heading/footer rendering test implementation (FT-19..FT-24) |
| [`tests/expanded_heading_test.rs`](../../expanded_heading_test.rs) | Expanded heading/footer rendering test implementation (FT-25..FT-30) |
| [`tests/text_heading_test.rs`](../../text_heading_test.rs) | Text heading/footer rendering test implementation (FT-31..FT-36) |
