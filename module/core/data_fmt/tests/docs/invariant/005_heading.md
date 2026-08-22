# Invariant: Heading Rendering

### Scope

- **Purpose**: Drive test coverage for the heading and footer rendering invariants across every adopting formatter.
- **Responsibility**: Documents test cases for the three invariants (no-heading/no-footer passthrough, width ceiling, single-line output) in `docs/invariant/005_heading.md`, applying identically to both positions and every adopting formatter.
- **In Scope**: Absent heading/footer behavior, target-width ceiling enforcement, single output line guarantee, at both positions, across formatters.
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
- **Note:** Covered by FC-5 (`no_caption_output_unchanged_fc5`) in `tests/table_heading_test.rs`.

---

### IN-2: heading line never exceeds rendered table width

- **Given:** Three scenarios: (a) short title and one field where heading content fits within the rendered table width; (b) title whose content string exceeds the rendered table width; (c) title whose content string exactly equals the rendered table width.
- **When:** The heading line is rendered in all three scenarios.
- **Then:** (a) The total character count equals `table_width` — trailing rule fills remaining space. (b) The trailing rule is absent (clamped to zero) — the heading line starts with `─── ` and does not end with `─`. (c) No trailing rule is emitted — the heading line ends with the content, not a rule char; total char count equals `table_width`.
- **Note:** Scenario (a) covered by FC-3 (`caption_fills_to_table_width_fc3`). Scenario (b) covered by FC-4 (`caption_trail_clamped_to_zero_when_content_too_wide_fc4`). Scenario (c) covered by FT-4 (`caption_content_equals_table_width_no_trailing_rule_ft4`). All three scenarios also covered by `heading_line_never_exceeds_table_width_in2` (explicit 3-scenario invariant test). Bordered variant covered by `heading_on_bordered_table_display_width_matches` (BUG-017 reproducer). When content alone exceeds rendered table width, the emitted line may be longer than `table_width` — the invariant guarantee is that the trailing rule is clamped to zero and content is never truncated.

---

### IN-3: heading always occupies exactly one output line

- **Given:** A heading with a title and at least one field; a table with at least one data row.
- **When:** The output is split into lines.
- **Then:** The first line is the heading line; the second line is either the header row or the top border; no heading content appears on the second or later lines; the heading line ends with exactly one newline.
- **Note:** Covered implicitly by FC-1 (`title_only_caption_renders_titled_rule_fc1`) and FC-6 (`caption_before_top_border_grid_fc6`) — both tests assert specific content for line 1 (heading) and line 2 (header or border), proving the heading is confined to one line. Also covered by FT-7 (`caption_title_exceeds_table_width_no_trailing_rule_ft7`) — title wider than table width still produces a single heading line with no truncation; FT-8 (`caption_empty_title_lead_only_no_separator_ft8`) — empty title produces a single 20-char heading line. Line break sanitization verified by `heading_newline_in_title_produces_single_line`, `heading_newline_in_field_produces_single_line`, and `heading_crlf_and_cr_sanitized` (BUG-016 reproducers).

---

### Footer Coverage

Footer rendering shares `render_rule_if_present()` with heading rendering (see `docs/algorithm/007_heading_rendering.md § Position-Agnostic and Formatter-Agnostic Dispatch`), so IN-1..IN-3 apply identically at the footer position. Footer-specific instances: IN-1 covered by FT-14 (`no_footer_output_unchanged`); IN-2 covered by FT-12/FT-13/FT-16 and `footer_line_never_exceeds_table_width` (3-scenario invariant test, mirroring `heading_line_never_exceeds_table_width_in2`); IN-3 covered by FT-10 and FT-18 (heading+footer coexistence proves each occupies exactly one line without leaking into the other's line or the table body).

### Tree Coverage

IN-1..IN-3 hold identically for Tree, verified by `tests/tree_heading_test.rs`. IN-1 (no-heading/no-footer passthrough) covered by `no_heading_no_footer_output_unchanged_ft22` — byte-identical output between a plain `TreeConfig::new()` and the `TreeFormatter::new()` baseline. IN-2 (width ceiling) covered by `heading_fills_to_widest_tree_line_ft21` — the heading fills to exactly the widest rendered body line's display width, using Tree's own target-width derivation (max rendered line width) rather than Table's `compute_total_row_width`; the clamp-to-zero guarantee itself is identical since both formatters share `Heading::render_line()`. IN-3 (single output line) covered by `title_only_heading_renders_before_tree_ft19`, `title_only_footer_renders_after_tree_ft20`, and `heading_and_footer_coexist_on_tree_ft23` (heading and footer each confined to exactly one line, never leaking into the tree body); `heading_applies_to_leaf_only_root_ft24` additionally proves the single-line guarantee holds even on the leaf-only-root branch, where the entire tree body is itself a single line.

### Expanded Coverage

IN-1..IN-3 hold identically for Expanded, verified by `tests/expanded_heading_test.rs`. IN-1 (no-heading/no-footer passthrough) covered by `no_heading_no_footer_output_unchanged_ft28` — byte-identical output between a plain `ExpandedConfig::new()` and the `ExpandedFormatter::new()` baseline. IN-2 (width ceiling) covered by `heading_fills_to_widest_expanded_line_ft27` — the heading fills to exactly the widest rendered body line's display width, using Expanded's own target-width derivation (max rendered line width, same technique as Tree) rather than Table's `compute_total_row_width`; the clamp-to-zero guarantee itself is identical since both formatters share `Heading::render_line()`. IN-3 (single output line) covered by `title_only_heading_renders_before_expanded_output_ft25`, `title_only_footer_renders_after_expanded_output_ft26`, and `heading_and_footer_coexist_on_expanded_output_ft29` (heading and footer each confined to exactly one line, never leaking into the expanded body); `heading_applies_to_empty_headers_view_ft30` additionally proves the single-line guarantee holds even on the `headers.is_empty()` early-return branch, where the entire body is empty.

### Text Coverage

IN-1..IN-3 hold identically for Text, verified by `tests/text_heading_test.rs`. IN-1 (no-heading/no-footer passthrough) covered by `no_heading_no_footer_output_unchanged_ft34` — byte-identical output between `TextFormatter::bullets()` with no heading/footer and the `TextFormatter::new(TextVariant::Bullets)` baseline. IN-2 (width ceiling) covered by `heading_fills_to_widest_text_line_ft33` — the heading fills to exactly the widest rendered body line's display width, using Text's own target-width derivation (max rendered line width, same technique as Tree/Expanded) rather than Table's `compute_total_row_width`; the clamp-to-zero guarantee itself is identical since all formatters share `Heading::render_line()`. IN-3 (single output line) covered by `title_only_heading_renders_before_text_output_ft31`, `title_only_footer_renders_after_text_output_ft32`, and `heading_and_footer_coexist_on_text_output_ft35` (heading and footer each confined to exactly one line, never leaking into the text body); `heading_applies_to_empty_cli_help_rows_ft36` additionally proves the single-line guarantee holds even when `format_cli_help()`'s own `data.rows.is_empty()` early return produces an empty body, all flowing through `TextFormatter`'s single `Format::format()` funnel point.

### Yaml/Toml/Sql/Html Coverage

IN-1..IN-3 hold identically for Yaml, Toml, Sql, and Html, verified by `tests/yaml_heading_test.rs`, `tests/toml_heading_test.rs`, `tests/sql_heading_test.rs`, and `tests/html_heading_test.rs`. IN-1 (no-heading/no-footer passthrough) covered by `no_heading_no_footer_is_pure_yaml_passthrough_ft40`, `no_heading_no_footer_is_pure_toml_passthrough_ft45`, `no_heading_no_footer_is_pure_sql_passthrough_ft50`, and `no_heading_no_footer_is_pure_html_passthrough_ft56` — each asserts byte-for-byte that the unwrapped output is a plain serialized body (no leading comment line) identical to what the underlying serializer (`serde_yaml_ng`, `toml`, the hand-built `INSERT INTO` string, or the hand-built `<table>` markup) would produce alone. IN-2 (width ceiling) covered by `heading_fills_to_widest_yaml_line_ft39`, `heading_fills_to_widest_toml_line_ft44`, `heading_fills_to_widest_sql_line_ft49`, and `heading_fills_to_widest_html_line_ft55` — the *commented* heading line (comment prefix plus rule, plus comment suffix for Html) fills to exactly the widest rendered body line's display width, using each formatter's own target-width derivation (max rendered line width, same technique as Tree/Expanded/Text) with one added step: `render_commented_rule_if_present()` subtracts the comment prefix's own display width — and, for Html only, the comment suffix's own display width too — before the same clamp-to-zero arithmetic every other formatter shares via `Heading::render_line()`. IN-3 (single output line) covered by `title_only_heading_renders_before_yaml_output_ft37`/`_toml_output_ft42`/`_sql_output_ft47`/`_html_output_ft53`, the matching footer variants (`ft38`/`ft43`/`ft48`/`ft54`), and the coexistence tests `heading_and_footer_coexist_on_yaml_output_ft41`/`_toml_output_ft46`/`_sql_output_ft51`/`_html_output_ft57` (heading and footer each confined to exactly one line, never leaking into the serialized body); `heading_applies_to_empty_rows_early_return_ft52` additionally proves the single-line guarantee holds on Sql's BUG-020 empty-rows branch, where the entire body is empty — the Sql counterpart to Tree's leaf-only-root, Expanded's empty-headers, and Text's empty-cli-help-rows branch coverage. Sql's body uniquely ends in a bare `;` with no trailing newline, which `wrap_with_heading_footer()` compensates for with a conditional separating `\n` before the footer — verified indirectly by FT-48 and FT-51 asserting the footer lands on its own line rather than glued onto the closing `;`; Html's body has the identical gap (a bare `</table>` with no trailing newline), compensated for the same way and verified indirectly by FT-54 and FT-57. Html additionally proves IN-3 holds for a *delimited* comment specifically — `title_only_heading_renders_before_html_output_ft53` and `title_only_footer_renders_after_html_output_ft54` each assert the line both opens with `<!--` and closes with `-->` on that same single line, and `heading_wraps_entire_output_including_wrapper_ft58` proves the single-line heading still precedes the entire `include_wrapper` prelude rather than only the `<table>` tag.

### Sources

| File | Relationship |
|------|-------------|
| [`docs/invariant/005_heading.md`](../../../docs/invariant/005_heading.md) | Source invariant spec — no-heading/no-footer passthrough, width ceiling, single-line output |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_heading_test.rs`](../../table_heading_test.rs) | Heading invariant test implementation (FC-1, FC-3, FC-4, FC-5, FC-6, FT-4, FT-7, FT-8, heading_line_never_exceeds_table_width_in2) |
| [`tests/table_footer_test.rs`](../../table_footer_test.rs) | Footer invariant test implementation (FT-10, FT-12, FT-13, FT-14, FT-16, FT-18, footer_line_never_exceeds_table_width) |
| [`tests/tree_heading_test.rs`](../../tree_heading_test.rs) | Tree invariant test implementation (FT-19..FT-24) |
| [`tests/expanded_heading_test.rs`](../../expanded_heading_test.rs) | Expanded invariant test implementation (FT-25..FT-30) |
| [`tests/text_heading_test.rs`](../../text_heading_test.rs) | Text invariant test implementation (FT-31..FT-36) |
| [`tests/yaml_heading_test.rs`](../../yaml_heading_test.rs) | Yaml invariant test implementation (FT-37..FT-41) |
| [`tests/toml_heading_test.rs`](../../toml_heading_test.rs) | Toml invariant test implementation (FT-42..FT-46) |
| [`tests/sql_heading_test.rs`](../../sql_heading_test.rs) | Sql invariant test implementation (FT-47..FT-52) |
| [`tests/html_heading_test.rs`](../../html_heading_test.rs) | Html invariant test implementation (FT-53..FT-58) |
