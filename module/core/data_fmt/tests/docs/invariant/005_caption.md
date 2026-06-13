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

- **Given:** Three scenarios: (a) short title and one field with `terminal_width = 60`; (b) title of 13 chars with `terminal_width = 10` (content exceeds terminal width); (c) title of 15 chars with `terminal_width = 20` (content exactly fills: used=3+1+15+1=20).
- **When:** The caption line is rendered in all three scenarios.
- **Then:** (a) The total character count is exactly 60 — trailing rule fills remaining space. (b) The trailing rule is absent (clamped to zero) — the caption line starts with `─── ` and does not end with `─`. (c) No trailing rule is emitted — the caption line ends with the content, not a rule char; total char count equals 20.
- **Note:** Scenario (a) covered by FC-3 (`caption_fills_to_terminal_width_fc3`). Scenario (b) covered by FC-4 (`caption_trail_clamped_to_zero_when_content_too_wide_fc4`). Scenario (c) covered by FT-4 (`caption_content_equals_terminal_width_no_trailing_rule_ft4`). When content alone exceeds terminal width, the emitted line may be longer than `terminal_width` — the invariant guarantee is that the trailing rule is clamped to zero, not that the total line length is bounded.

---

### IN-3: caption always occupies exactly one output line

- **Given:** A caption with a title and at least one field; a table with at least one data row.
- **When:** The output is split into lines.
- **Then:** The first line is the caption line; the second line is either the header row or the top border; no caption content appears on the second or later lines; the caption line ends with exactly one newline.
- **Note:** Covered implicitly by FC-1 (`title_only_caption_renders_titled_rule_fc1`) and FC-6 (`caption_before_top_border_grid_fc6`) — both tests assert specific content for line 1 (caption) and line 2 (header or border), proving the caption is confined to one line. Also covered by FT-7 (`caption_title_exceeds_terminal_width_no_trailing_rule_ft7`) — title wider than terminal width still produces a single caption line with no truncation; and FT-8 (`caption_empty_title_lead_only_no_separator_ft8`) — empty title produces a single 20-char caption line.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/invariant/005_caption.md`](../../../docs/invariant/005_caption.md) | Source invariant spec — no-caption passthrough, width ceiling, single-line output |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../table_caption_test.rs) | Caption invariant test implementation (FC-1, FC-3, FC-4, FC-5, FC-6, FT-4, FT-7, FT-8) |
