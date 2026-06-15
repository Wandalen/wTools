# Invariant: Heading Rendering

### Scope

- **Purpose**: Drive test coverage for the heading rendering invariants.
- **Responsibility**: Documents test cases for the three heading invariants (no-heading passthrough, width ceiling, single-line output) in `docs/invariant/005_caption.md`.
- **In Scope**: Absent heading behavior, table width ceiling enforcement, single output line guarantee.
- **Out of Scope**: Heading content format (see `feature/007`), rendering algorithm steps (see `algorithm/007`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| IN-1 | no-heading config produces byte-identical output to unconfigured baseline | ✅ |
| IN-2 | heading line never exceeds rendered table width | ✅ |
| IN-3 | heading always occupies exactly one output line | ✅ |

---

### IN-1: no-heading config produces byte-identical output to unconfigured baseline

- **Given:** Two table formatters using `TableConfig::plain()`, neither with a heading attached; both receive the same table data.
- **When:** Both formatters produce their output strings.
- **Then:** The two output strings are byte-identical; neither string begins with a rule character (`─`); no heading path executes.
- **Note:** Covered by FC-5 (`no_caption_output_unchanged_fc5`) in `tests/table_caption_test.rs`.

---

### IN-2: heading line never exceeds rendered table width

- **Given:** Three scenarios: (a) short title and one field where heading content fits within the rendered table width; (b) title whose content string exceeds the rendered table width; (c) title whose content string exactly equals the rendered table width.
- **When:** The heading line is rendered in all three scenarios.
- **Then:** (a) The total character count equals `table_width` — trailing rule fills remaining space. (b) The trailing rule is absent (clamped to zero) — the heading line starts with `─── ` and does not end with `─`. (c) No trailing rule is emitted — the heading line ends with the content, not a rule char; total char count equals `table_width`.
- **Note:** Scenario (a) covered by FC-3 (`caption_fills_to_table_width_fc3` — pending table_width implementation via TSK-008). Scenario (b) covered by FC-4 (`caption_trail_clamped_to_zero_when_content_too_wide_fc4`). Scenario (c) covered by FT-4 (`caption_content_equals_table_width_no_trailing_rule_ft4` — pending table_width implementation via TSK-008). When content alone exceeds rendered table width, the emitted line may be longer than `table_width` — the invariant guarantee is that the trailing rule is clamped to zero and content is never truncated.

---

### IN-3: heading always occupies exactly one output line

- **Given:** A heading with a title and at least one field; a table with at least one data row.
- **When:** The output is split into lines.
- **Then:** The first line is the heading line; the second line is either the header row or the top border; no heading content appears on the second or later lines; the heading line ends with exactly one newline.
- **Note:** Covered implicitly by FC-1 (`title_only_caption_renders_titled_rule_fc1`) and FC-6 (`caption_before_top_border_grid_fc6`) — both tests assert specific content for line 1 (heading) and line 2 (header or border), proving the heading is confined to one line. Also covered by FT-7 (`caption_title_exceeds_table_width_no_trailing_rule_ft7`) — title wider than table width still produces a single heading line with no truncation; and FT-8 (`caption_empty_title_lead_only_no_separator_ft8`) — empty title produces a single 20-char heading line.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/invariant/005_caption.md`](../../../docs/invariant/005_caption.md) | Source invariant spec — no-heading passthrough, width ceiling, single-line output |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../table_caption_test.rs) | Heading invariant test implementation (FC-1, FC-3, FC-4, FC-5, FC-6, FT-4, FT-7, FT-8) |
