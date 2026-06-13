# Feature: Table Caption

### Scope

- **Purpose**: Drive test coverage for the table caption feature.
- **Responsibility**: Documents test cases for `TableCaption` and `TableConfig::caption()` as specified in `docs/feature/007_table_caption.md`.
- **In Scope**: Caption rendering with title only, caption fields, terminal-width filling, no-caption regression, style interaction.
- **Out of Scope**: Auto-fit width algorithm (see `feature/005`); ANSI coloring (see `feature/004`); cell rendering (see `feature/001`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| FC-1 | title-only caption renders titled rule before table | ✅ |
| FC-2 | caption fields appear joined by field separator | ✅ |
| FC-3 | caption line fills to terminal width with rule chars | ✅ |
| FC-4 | *(not assigned — reserved ID, no case defined)* | N/A |
| FC-5 | no-caption config produces identical output to current behavior | ✅ |
| FC-6 | caption renders before top border for grid and unicode_box styles | ✅ |

---

### FC-1: title-only caption renders titled rule before table

- **Given:** A `TableConfig::plain()` with `.caption(TableCaption::new("Results"))`.
- **When:** A two-column, one-row table is formatted.
- **Then:** The first line of the output starts with `"─── Results "` and ends with one or more `─` characters; the second line is the header row.

---

### FC-2: caption fields appear joined by field separator

- **Given:** A `TableCaption::new("Needs Review").field("28 PRs").field("15 repos")`.
- **When:** Rendered via `TableConfig::plain()`.
- **Then:** The first output line contains the substring `"Needs Review · 28 PRs · 15 repos"`.

---

### FC-3: caption line fills to terminal width with rule chars

- **Given:** A `TableConfig::plain().terminal_width(Some(60)).caption(TableCaption::new("T").field("F"))`.
- **When:** The table is formatted.
- **Then:** The first output line (the caption line) has exactly 60 display columns, i.e., `line.chars().count() == 60` — use `.chars().count()`, not `.len()`, since the rule character `─` (U+2500) is 3 UTF-8 bytes.

---

### FC-5: no-caption config produces identical output to current behavior

- **Given:** A `TableConfig::plain()` with no `.caption()` call, and the same config built without `.caption()`.
- **When:** Both render the same table.
- **Then:** The two outputs are byte-identical; adding `caption` support does not change output for existing callers.

---

### FC-6: caption renders before top border for grid and unicode_box styles

- **Given:** A `TableConfig::grid()` (or `unicode_box()`) with `.caption(TableCaption::new("Grid Table"))`.
- **When:** The table is formatted.
- **Then:** The first line of output is the caption line (starts with `"─── Grid Table"`); the second line is the top border (`+---+` for grid, `┌───┐` for unicode_box).
