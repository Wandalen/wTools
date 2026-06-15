# Algorithm: Heading Rendering

### Scope

- **Purpose**: Drive test coverage for the heading line assembly algorithm.
- **Responsibility**: Documents test cases for the heading rendering algorithm in `docs/algorithm/007_caption_rendering.md`.
- **In Scope**: Content string construction, lead prefix format, trailing rule width arithmetic using rendered table width, multi-byte character counting, trailing rule clamping at zero.
- **Out of Scope**: Terminal width resolution (see `feature/005_auto_fit.md`), table body rendering (see `algorithm/001`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | title-only content string contains no separator | ✅ |
| AC-2 | caption fields joined by middle-dot separator | ✅ |
| AC-3 | lead prefix is exactly three rule chars followed by a space | ✅ |
| AC-4 | trailing rule fills remaining table width | ✅ |
| AC-5 | trailing rule clamped to zero when content meets or exceeds table width | ✅ |
| AC-6 | multi-byte separator counted as one character not one byte | ✅ |
| AC-7 | empty content string: no separator emitted; trailing rule fills remaining width | ✅ |

---

### AC-1: title-only content string contains no separator

- **Given:** A `Heading` created with a title and no additional fields.
- **When:** The caption line is rendered.
- **Then:** The content portion of the line contains only the title text; no middle-dot separator character appears in the line; the lead prefix and trailing rule are present.
- **Note:** Covered by FC-1 (`title_only_caption_renders_titled_rule_fc1`) in `tests/table_caption_test.rs`.

---

### AC-2: caption fields joined by middle-dot separator

- **Given:** A `Heading` created with a title and two additional fields (e.g. `"Needs Review"`, `"28 PRs"`, `"15 repos"`).
- **When:** The caption line is rendered.
- **Then:** The content portion of the line is exactly `"Needs Review · 28 PRs · 15 repos"` — title and fields separated by ` · ` (space, U+00B7, space); the order matches the order the fields were appended.
- **Note:** Covered by FC-2 (`caption_fields_joined_by_separator_fc2`) in `tests/table_caption_test.rs`.

---

### AC-3: lead prefix is exactly three rule chars followed by a space

- **Given:** Any caption with any title.
- **When:** The caption line is rendered.
- **Then:** The line begins with exactly three U+2500 BOX DRAWINGS LIGHT HORIZONTAL characters followed by one ASCII space (`─── `); neither more nor fewer rule characters appear in the lead prefix.
- **Note:** Covered by FC-1 (`title_only_caption_renders_titled_rule_fc1`) — asserts `starts_with("─── Hi ")`.

---

### AC-4: trailing rule fills remaining table width

- **Given:** A caption with a short title and one field applied to a table whose rendered display width (`table_width`) is known.
- **When:** The caption line is rendered.
- **Then:** The total display column count of the caption line equals exactly `table_width`; the trailing rule characters account for the difference between `table_width` and the sum of lead prefix width, content display width, and the single space that follows the content.
- **Note:** Covered by FC-3 (`caption_fills_to_table_width_fc3`). CJK correctness verified by `heading_cjk_title_display_width_matches_table_body` (BUG-015 reproducer).

---

### AC-5: trailing rule clamped to zero when content meets or exceeds table width

- **Given:** Three sub-cases: (a) content exactly fills rendered table width (title sized such that lead + title + trailing space = `table_width`); (b) title whose content string exceeds the rendered table width; (c) very long title that exceeds even a wide table.
- **When:** The caption line is rendered in each sub-case.
- **Then:** In all three sub-cases, no trailing rule character (`─`) appears at the end of the caption line; the lead prefix `─── ` is still emitted; the content is never truncated — the clamp to zero affects only the trailing rule.
- **Note:** (a) covered by `caption_content_equals_table_width_no_trailing_rule_ft4`; (b) covered by `caption_trail_clamped_to_zero_when_content_too_wide_fc4`; (c) covered by `caption_title_exceeds_table_width_no_trailing_rule_ft7` (also verifies content verbatim — no truncation).

---

### AC-6: multi-byte separator counted as one character not one byte

- **Given:** A caption with one field applied to a table with known rendered width (`table_width`).
- **When:** The caption line display column count is measured.
- **Then:** The measured display column count equals `table_width`; measuring by byte length would produce a different (larger) result because `─` (U+2500) is 3 bytes and `·` (U+00B7) is 2 bytes in UTF-8. For ASCII-only content, `.chars().count()` equals display column count; for CJK content, display columns differ from character count.
- **Note:** Covered by FC-3 (`caption_fills_to_table_width_fc3`). CJK display width verified by `heading_cjk_title_display_width_matches_table_body` (BUG-015 reproducer).

---

### AC-7: empty content string: no separator emitted; trailing rule fills remaining width

- **Given:** A `Heading::new("")` with no additional fields applied to a table whose rendered display width (`table_width`) equals 10.
- **When:** The caption line is rendered.
- **Then:** The content string is empty (zero visible characters); no middle-dot separator (`·`) appears anywhere in the output; the trailing rule fills the remaining width from column 5 to column 10 (5 rule characters, since lead = 3 + space = 1 + trailing space = 1 → trail = 10 − 5 = 5); the total character count equals exactly 10; no panic occurs from empty-string arithmetic.
- **Note:** Covered by `caption_empty_title_lead_only_no_separator_ft8` in `tests/table_caption_test.rs`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/algorithm/007_caption_rendering.md`](../../../docs/algorithm/007_caption_rendering.md) | Source algorithm spec — heading content string, lead prefix, trailing rule computation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../table_caption_test.rs) | Heading algorithm test implementation (FC-1, FC-2, FC-3, FC-4, FT-4, FT-7, FT-8) |
