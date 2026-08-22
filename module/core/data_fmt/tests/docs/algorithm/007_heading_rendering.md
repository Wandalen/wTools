# Algorithm: Heading Rendering

### Scope

- **Purpose**: Drive test coverage for the titled-rule line assembly algorithm, shared by the heading and footer positions across every adopting formatter.
- **Responsibility**: Documents test cases for the rendering algorithm in `docs/algorithm/007_heading_rendering.md`.
- **In Scope**: Content string construction, lead prefix format, trailing rule width arithmetic using the calling formatter's target width, multi-byte character counting, trailing rule clamping at zero, position-agnostic and formatter-agnostic dispatch.
- **Out of Scope**: Terminal width resolution (see `feature/005_auto_fit.md`), table body rendering (see `algorithm/001`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | title-only content string contains no separator | ✅ |
| AC-2 | heading fields joined by middle-dot separator | ✅ |
| AC-3 | lead prefix is exactly three rule chars followed by a space | ✅ |
| AC-4 | trailing rule fills remaining table width | ✅ |
| AC-5 | trailing rule clamped to zero when content meets or exceeds table width | ✅ |
| AC-6 | multi-byte separator counted as one character not one byte | ✅ |
| AC-7 | empty content string: no separator emitted; trailing rule fills remaining width | ✅ |

---

### AC-1: title-only content string contains no separator

- **Given:** A `Heading` created with a title and no additional fields.
- **When:** The heading line is rendered.
- **Then:** The content portion of the line contains only the title text; no middle-dot separator character appears in the line; the lead prefix and trailing rule are present.
- **Note:** Covered by FC-1 (`title_only_caption_renders_titled_rule_fc1`) in `tests/table_heading_test.rs`.

---

### AC-2: heading fields joined by middle-dot separator

- **Given:** A `Heading` created with a title and two additional fields (e.g. `"Needs Review"`, `"28 PRs"`, `"15 repos"`).
- **When:** The heading line is rendered.
- **Then:** The content portion of the line is exactly `"Needs Review · 28 PRs · 15 repos"` — title and fields separated by ` · ` (space, U+00B7, space); the order matches the order the fields were appended.
- **Note:** Covered by FC-2 (`caption_fields_joined_by_separator_fc2`) in `tests/table_heading_test.rs`.

---

### AC-3: lead prefix is exactly three rule chars followed by a space

- **Given:** Any heading with any title.
- **When:** The heading line is rendered.
- **Then:** The line begins with exactly three U+2500 BOX DRAWINGS LIGHT HORIZONTAL characters followed by one ASCII space (`─── `); neither more nor fewer rule characters appear in the lead prefix.
- **Note:** Covered by FC-1 (`title_only_caption_renders_titled_rule_fc1`) — asserts `starts_with("─── Hi ")`.

---

### AC-4: trailing rule fills remaining table width

- **Given:** A heading with a short title and one field applied to a table whose rendered display width (`table_width`) is known.
- **When:** The heading line is rendered.
- **Then:** The total display column count of the heading line equals exactly `table_width`; the trailing rule characters account for the difference between `table_width` and the sum of lead prefix width, content display width, and the single space that follows the content.
- **Note:** Covered by FC-3 (`caption_fills_to_table_width_fc3`). CJK correctness verified by `heading_cjk_title_display_width_matches_table_body` (BUG-015 reproducer).

---

### AC-5: trailing rule clamped to zero when content meets or exceeds table width

- **Given:** Three sub-cases: (a) content exactly fills rendered table width (title sized such that lead + title + trailing space = `table_width`); (b) title whose content string exceeds the rendered table width; (c) very long title that exceeds even a wide table.
- **When:** The heading line is rendered in each sub-case.
- **Then:** In all three sub-cases, no trailing rule character (`─`) appears at the end of the heading line; the lead prefix `─── ` is still emitted; the content is never truncated — the clamp to zero affects only the trailing rule.
- **Note:** (a) covered by `caption_content_equals_table_width_no_trailing_rule_ft4`; (b) covered by `caption_trail_clamped_to_zero_when_content_too_wide_fc4`; (c) covered by `caption_title_exceeds_table_width_no_trailing_rule_ft7` (also verifies content verbatim — no truncation).

---

### AC-6: multi-byte separator counted as one character not one byte

- **Given:** A heading with one field applied to a table with known rendered width (`table_width`).
- **When:** The heading line display column count is measured.
- **Then:** The measured display column count equals `table_width`; measuring by byte length would produce a different (larger) result because `─` (U+2500) is 3 bytes and `·` (U+00B7) is 2 bytes in UTF-8. For ASCII-only content, `.chars().count()` equals display column count; for CJK content, display columns differ from character count.
- **Note:** Covered by FC-3 (`caption_fills_to_table_width_fc3`). CJK display width verified by `heading_cjk_title_display_width_matches_table_body` (BUG-015 reproducer).

---

### AC-7: empty content string: no separator emitted; trailing rule fills remaining width

- **Given:** A `Heading::new("")` with no additional fields applied to a table whose rendered display width (`table_width`) equals 10.
- **When:** The heading line is rendered.
- **Then:** The content string is empty (zero visible characters); no middle-dot separator (`·`) appears anywhere in the output; the trailing rule fills the remaining width from column 5 to column 10 (5 rule characters, since lead = 3 + space = 1 + trailing space = 1 → trail = 10 − 5 = 5); the total character count equals exactly 10; no panic occurs from empty-string arithmetic.
- **Note:** Covered by `caption_empty_title_lead_only_no_separator_ft8` in `tests/table_heading_test.rs`.

---

### Footer Coverage

Because `render_rule_if_present()` is the single implementation for both positions (see `docs/algorithm/007_heading_rendering.md § Position-Agnostic and Formatter-Agnostic Dispatch`), AC-1..AC-7 hold identically for the footer — verified by `tests/table_footer_test.rs` re-running the same content/lead/trail/clamp/multi-byte assertions against `.with_footer()` instead of `.with_heading()`.

### Tree Coverage

AC-1, AC-2, AC-3, and AC-7 (content assembly, field joining, lead prefix, empty-title handling) hold identically for Tree — the same `Heading::render_line()` implementation is invoked; only the `target_width` argument's source differs (see `docs/algorithm/007_heading_rendering.md § Target-Width Computation Per Formatter`). AC-4 and AC-5 (trailing-rule fill and zero-clamp) hold against Tree's own target width — the maximum display width across the already-rendered tree body's lines, rather than `compute_total_row_width(primary_widths)` — verified by `tests/tree_heading_test.rs`'s `heading_fills_to_widest_tree_line_ft21`. AC-6 (multi-byte-safe measurement) holds identically since `unicode_visual_len` is formatter-agnostic. Tree-specific coverage also includes: passthrough when neither heading nor footer is set (`no_heading_no_footer_output_unchanged_ft22`), heading+footer coexistence (`heading_and_footer_coexist_on_tree_ft23`), and applicability across all three of `TreeFormatter::format()`'s internal branches, including the leaf-only-root early return (`heading_applies_to_leaf_only_root_ft24`).

### Expanded Coverage

AC-1, AC-2, AC-3, and AC-7 hold identically for Expanded via the same `Heading::render_line()` implementation — only the `target_width` source differs. AC-4 and AC-5 hold against Expanded's own target width — the maximum display width across the already-rendered vertical-record body's lines, the same technique Tree uses — verified by `tests/expanded_heading_test.rs`'s `heading_fills_to_widest_expanded_line_ft27`. AC-6 holds identically since `unicode_visual_len` is formatter-agnostic. Expanded-specific coverage also includes: passthrough when neither heading nor footer is set (`no_heading_no_footer_output_unchanged_ft28`), heading+footer coexistence (`heading_and_footer_coexist_on_expanded_output_ft29`), and applicability on `ExpandedFormatter::format_view()`'s `headers.is_empty()` early-return branch (`heading_applies_to_empty_headers_view_ft30`) — the Expanded counterpart to Tree's leaf-only-root branch coverage.

### Text Coverage

AC-1, AC-2, AC-3, and AC-7 hold identically for Text via the same `Heading::render_line()` implementation — only the `target_width` source differs. AC-4 and AC-5 hold against Text's own target width — the maximum display width across the already-rendered text body's lines, the same technique Tree/Expanded use — verified by `tests/text_heading_test.rs`'s `heading_fills_to_widest_text_line_ft33`. AC-6 holds identically since `unicode_visual_len` is formatter-agnostic. Text-specific coverage also includes: passthrough when neither heading nor footer is set (`no_heading_no_footer_output_unchanged_ft34`), heading+footer coexistence (`heading_and_footer_coexist_on_text_output_ft35`), and applicability on `format_cli_help()`'s `data.rows.is_empty()` early-return branch as it flows through `TextFormatter`'s single `Format::format()` funnel point (`heading_applies_to_empty_cli_help_rows_ft36`) — the Text counterpart to Tree's leaf-only-root and Expanded's empty-headers branch coverage.

### Sources

| File | Relationship |
|------|-------------|
| [`docs/algorithm/007_heading_rendering.md`](../../../docs/algorithm/007_heading_rendering.md) | Source algorithm spec — heading/footer content string, lead prefix, trailing rule computation, per-formatter target-width derivation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_heading_test.rs`](../../table_heading_test.rs) | Heading algorithm test implementation (FC-1, FC-2, FC-3, FC-4, FT-4, FT-7, FT-8) |
| [`tests/table_footer_test.rs`](../../table_footer_test.rs) | Footer algorithm test implementation (same AC-1..AC-7 assertions, footer position) |
| [`tests/tree_heading_test.rs`](../../tree_heading_test.rs) | Tree algorithm test implementation (AC-1..AC-7 with Tree's own target-width derivation; FT-19..FT-24) |
| [`tests/expanded_heading_test.rs`](../../expanded_heading_test.rs) | Expanded algorithm test implementation (AC-1..AC-7 with Expanded's own target-width derivation; FT-25..FT-30) |
| [`tests/text_heading_test.rs`](../../text_heading_test.rs) | Text algorithm test implementation (AC-1..AC-7 with Text's own target-width derivation; FT-31..FT-36) |
