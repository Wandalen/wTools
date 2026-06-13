# Algorithm: Caption Rendering

### Scope

- **Purpose**: Drive test coverage for the caption line assembly algorithm.
- **Responsibility**: Documents test cases for the caption rendering algorithm in `docs/algorithm/007_caption_rendering.md`.
- **In Scope**: Content string construction, lead prefix format, trailing rule width arithmetic, multi-byte character counting, trailing rule clamping at zero.
- **Out of Scope**: Terminal width resolution (see `feature/005_auto_fit.md`), table body rendering (see `algorithm/001`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | title-only content string contains no separator | ✅ |
| AC-2 | caption fields joined by middle-dot separator | ✅ |
| AC-3 | lead prefix is exactly three rule chars followed by a space | ✅ |
| AC-4 | trailing rule fills remaining terminal width | ✅ |
| AC-5 | trailing rule clamped to zero when content meets or exceeds terminal width | ✅ |
| AC-6 | multi-byte separator counted as one character not one byte | ✅ |
| AC-7 | empty content string: no separator emitted; trailing rule fills remaining width | ✅ |

---

### AC-1: title-only content string contains no separator

- **Given:** A `TableCaption` created with a title and no additional fields.
- **When:** The caption line is rendered.
- **Then:** The content portion of the line contains only the title text; no middle-dot separator character appears in the line; the lead prefix and trailing rule are present.
- **Note:** Covered by FC-1 (`title_only_caption_renders_titled_rule_fc1`) in `tests/table_caption_test.rs`.

---

### AC-2: caption fields joined by middle-dot separator

- **Given:** A `TableCaption` created with a title and two additional fields (e.g. `"Needs Review"`, `"28 PRs"`, `"15 repos"`).
- **When:** The caption line is rendered.
- **Then:** The content portion of the line is exactly `"Needs Review · 28 PRs · 15 repos"` — title and fields separated by ` · ` (space, U+00B7, space); the order matches the order the fields were appended.
- **Note:** Covered by FC-2 (`caption_fields_joined_by_separator_fc2`) in `tests/table_caption_test.rs`.

---

### AC-3: lead prefix is exactly three rule chars followed by a space

- **Given:** Any caption with any title.
- **When:** The caption line is rendered.
- **Then:** The line begins with exactly three U+2500 BOX DRAWINGS LIGHT HORIZONTAL characters followed by one ASCII space (`─── `); neither more nor fewer rule characters appear in the lead prefix.
- **Note:** Covered by FC-1 (`title_only_caption_renders_titled_rule_fc1`) — asserts `starts_with("─── Results ")`.

---

### AC-4: trailing rule fills remaining terminal width

- **Given:** A caption with a short title and one field; `terminal_width` set to 60 via `TableConfig`.
- **When:** The caption line is rendered.
- **Then:** The total character count of the caption line (measured by Unicode scalar values, not bytes) equals exactly 60; the trailing rule characters account for the difference between 60 and the sum of lead prefix width, content character count, and the single space that follows the content.
- **Note:** Covered by FC-3 (`caption_fills_to_terminal_width_fc3`) — asserts `.chars().count() == 60`.

---

### AC-5: trailing rule clamped to zero when content meets or exceeds terminal width

- **Given:** Three sub-cases: (a) content exactly fills terminal width (`"Caption Exactly"`, 15 chars, `terminal_width=20`: used=3+1+15+1=20); (b) content exceeds terminal width (`"LongTitleText"`, 13 chars, `terminal_width=10`); (c) title alone exceeds terminal width (`"A very long title"`, 17 chars, `terminal_width=10`).
- **When:** The caption line is rendered in each sub-case.
- **Then:** In all three sub-cases, no trailing rule character (`─`) appears at the end of the caption line; the lead prefix `─── ` is still emitted; the content is never truncated — the clamp to zero affects only the trailing rule.
- **Note:** (a) covered by `caption_content_equals_terminal_width_no_trailing_rule_ft4`; (b) covered by `caption_trail_clamped_to_zero_when_content_too_wide_fc4`; (c) covered by `caption_title_exceeds_terminal_width_no_trailing_rule_ft7` (also verifies content verbatim — no truncation).

---

### AC-6: multi-byte separator counted as one character not one byte

- **Given:** A caption with one field, using `terminal_width = 60`.
- **When:** The caption line character count is measured using Unicode scalar value count (`.chars().count()`).
- **Then:** The measured count equals 60; measuring by byte length would produce a different (larger) result because `─` (U+2500) is 3 bytes and `·` (U+00B7) is 2 bytes in UTF-8.
- **Note:** Covered by FC-3 (`caption_fills_to_terminal_width_fc3`) — the assertion comment explains "use .chars().count() — '─' is 3 UTF-8 bytes".

---

### AC-7: empty content string: no separator emitted; trailing rule fills remaining width

- **Given:** A `TableCaption::new("")` with no additional fields and `terminal_width = 20`.
- **When:** The caption line is rendered.
- **Then:** The content string is empty (zero visible characters); no middle-dot separator (`·`) appears anywhere in the output; the trailing rule fills the remaining width from column 4 to column 20 (15 rule characters); the total character count equals exactly 20; no panic occurs from empty-string arithmetic.
- **Note:** Covered by `caption_empty_title_lead_only_no_separator_ft8` in `tests/table_caption_test.rs`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/algorithm/007_caption_rendering.md`](../../../docs/algorithm/007_caption_rendering.md) | Source algorithm spec — content string, lead prefix, trailing rule computation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../table_caption_test.rs) | Caption algorithm test implementation (FC-1, FC-2, FC-3, FC-4, FT-4, FT-7, FT-8) |
