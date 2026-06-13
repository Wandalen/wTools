# Invariant: Caption Rendering

### Scope

- **Purpose**: Drive test coverage for the caption rendering invariants.
- **Responsibility**: Documents test cases for the three caption invariants (no-caption passthrough, width ceiling, single-line output) in `docs/invariant/005_caption.md`.
- **In Scope**: Absent caption behavior, terminal width ceiling enforcement, single output line guarantee.
- **Out of Scope**: Caption content format (see `feature/007`), rendering algorithm steps (see `algorithm/007`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| IN-1 | no-caption config produces byte-identical output to unconfigured baseline | ✅ |
| IN-2 | caption line never exceeds resolved terminal width | ✅ |
| IN-3 | caption always occupies exactly one output line | ✅ |

---

### IN-1: no-caption config produces byte-identical output to unconfigured baseline

- **Given:** Two table formatters using `TableConfig::plain()`, neither with a caption attached; both receive the same table data.
- **When:** Both formatters produce their output strings.
- **Then:** The two output strings are byte-identical; neither string begins with a rule character (`─`); no caption path executes.
- **Note:** Covered by FC-5 (`no_caption_output_unchanged_fc5`) in `tests/table_caption_test.rs`.

---

### IN-2: caption line never exceeds resolved terminal width

- **Given:** A caption with a short title and one field; `terminal_width` set to 60 via `TableConfig`.
- **When:** The caption line is rendered.
- **Then:** The total character count of the caption line (measured by Unicode scalar values) is at most 60; specifically, when the caption content is short relative to the terminal width, the trailing rule fills exactly to 60 and the line character count equals exactly 60.
- **Note:** Covered by FC-3 (`caption_fills_to_terminal_width_fc3`) — asserts `.chars().count() == 60`. The case where content equals or exceeds terminal width (trailing rule clamped to zero) is documented as AC-5 in `tests/docs/algorithm/007_caption_rendering.md` but lacks a dedicated test (⏳).

---

### IN-3: caption always occupies exactly one output line

- **Given:** A caption with a title and at least one field; a table with at least one data row.
- **When:** The output is split into lines.
- **Then:** The first line is the caption line; the second line is either the header row or the top border; no caption content appears on the second or later lines; the caption line ends with exactly one newline.
- **Note:** Covered implicitly by FC-1 (`title_only_caption_renders_titled_rule_fc1`) and FC-6 (`caption_before_top_border_grid_fc6`) — both tests assert specific content for line 1 (caption) and line 2 (header or border), proving the caption is confined to one line.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/invariant/005_caption.md`](../../../docs/invariant/005_caption.md) | Source invariant spec — no-caption passthrough, width ceiling, single-line output |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../table_caption_test.rs) | Caption invariant test implementation (FC-1, FC-3, FC-5, FC-6) |
