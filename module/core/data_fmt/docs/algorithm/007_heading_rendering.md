# Algorithm: Heading Rendering

### Scope

- **Purpose**: Compute and emit a titled rule line that matches the calling formatter's rendered width, above its output (heading) or below it (footer).
- **Responsibility**: Documents the titled-rule line assembly, lead/trail rule computation, and target-width integration, shared identically by the heading and footer positions across every adopting formatter.
- **In Scope**: Content string construction, lead prefix, trailing rule width calculation, multi-byte character handling, target-width computation (per-formatter), position-agnostic and formatter-agnostic dispatch (heading vs footer; table vs tree vs expanded vs text vs comment-wrapped yaml/toml/sql/html).
- **Out of Scope**: Terminal width resolution (see `feature/005_auto_fit.md § Terminal Width Detection`), table body rendering (see `algorithm/001_multiline_cell_rendering.md`).

### Features

| File | Relationship |
|------|-------------|
| [007_table_heading.md](../feature/007_table_heading.md) | Feature that this algorithm implements |

### Sources

| File | Relationship |
|------|-------------|
| [`src/config/table_heading.rs`](../../src/config/table_heading.rs) | `Heading` struct; `Heading::render_line()` — titled-rule line assembly (relocated from `formatters/table/row_rendering.rs` so every formatter can share one implementation); `render_rule_if_present()` — `Option`-handling wrapper used at each bare-rule formatter's call site; `render_commented_rule_if_present()` — comment-prefix-and-suffix wrapper used by Yaml/Toml/Sql/Html, reusing `Heading::render_line()` with only its trailing newline swapped for the caller's suffix |
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | Table call sites in `format_internal()` — supply `compute_total_row_width(primary_widths)` as the target width |
| [`src/formatters/tree/mod.rs`](../../src/formatters/tree/mod.rs) | Tree call site in `wrap_with_heading_footer()` — supplies the widest rendered body line's display width as the target width |
| [`src/formatters/expanded.rs`](../../src/formatters/expanded.rs) | Expanded call site in `wrap_with_heading_footer()` — supplies the widest rendered body line's display width as the target width, same technique as Tree |
| [`src/formatters/text.rs`](../../src/formatters/text.rs) | Text call site in `wrap_with_heading_footer()`, invoked once from the shared `Format::format()` funnel point — supplies the widest rendered body line's display width as the target width, same technique as Tree/Expanded |
| [`src/formatters/yaml.rs`](../../src/formatters/yaml.rs) | Yaml call site in `wrap_with_heading_footer()`, invoked from `Format::format()` — supplies the widest rendered body line's display width as `target_width` to `render_commented_rule_if_present()` with a `"# "` comment prefix |
| [`src/formatters/toml_fmt.rs`](../../src/formatters/toml_fmt.rs) | Toml call site — identical technique to Yaml, same `"# "` comment prefix (TOML also uses `#` for comments) |
| [`src/formatters/sql.rs`](../../src/formatters/sql.rs) | Sql call site — identical technique, `"-- "` comment prefix (SQL line-comment syntax); wired into both of `Format::format()`'s return points (the BUG-020 empty-rows early return and the final populated-rows return) |
| [`src/formatters/html.rs`](../../src/formatters/html.rs) | Html call site in `wrap_with_heading_footer()`, invoked once from the single `Format::format()` return point — supplies the widest rendered line's display width as `target_width` to `render_commented_rule_if_present()` with `"<!-- "` prefix AND `" -->"` suffix — the only adopter to pass a non-empty suffix, since `<!-- -->` needs an explicit closing delimiter |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_heading_test.rs`](../../tests/table_heading_test.rs) | Table heading rendering tests |
| [`tests/table_footer_test.rs`](../../tests/table_footer_test.rs) | Table footer rendering tests |
| [`tests/tree_heading_test.rs`](../../tests/tree_heading_test.rs) | Tree heading/footer rendering tests |
| [`tests/expanded_heading_test.rs`](../../tests/expanded_heading_test.rs) | Expanded heading/footer rendering tests |
| [`tests/text_heading_test.rs`](../../tests/text_heading_test.rs) | Text heading/footer rendering tests |
| [`tests/yaml_heading_test.rs`](../../tests/yaml_heading_test.rs) | Yaml comment-wrapped heading/footer rendering tests |
| [`tests/toml_heading_test.rs`](../../tests/toml_heading_test.rs) | Toml comment-wrapped heading/footer rendering tests |
| [`tests/sql_heading_test.rs`](../../tests/sql_heading_test.rs) | Sql comment-wrapped heading/footer rendering tests, including the BUG-020 empty-rows branch |
| [`tests/html_heading_test.rs`](../../tests/html_heading_test.rs) | Html comment-delimited heading/footer rendering tests, including the `include_wrapper` interaction |

### Abstract

A four-step algorithm that assembles a titled rule line from a title, zero or more heading fields, a fixed lead prefix, and a trailing rule that fills to a caller-supplied target width. The field separator and rule characters are multi-byte in UTF-8, so width is measured by display column count (not byte length or character count — CJK characters occupy 2 display columns each). Line breaks in title and fields are sanitized to spaces before assembly. The trailing rule is clamped to zero when heading content already meets or exceeds the target width. The same four steps assemble both the heading line (rendered above the formatted output) and the footer line (rendered below it) — position is determined entirely by which call site invokes the algorithm, never by the algorithm itself; likewise, the same four steps serve every formatter that adopts `Heading` — which formatter is calling is never visible to the algorithm either.

### Algorithm

1. **Build content string**: sanitize line breaks in title and each field (replace `\r\n`, `\r`, `\n` with space), then concatenate the title followed by `" {field_separator} {field_value}"` for each field. The field separator is a fixed middle dot character (U+00B7).
2. **Build lead prefix**: repeat the rule character (U+2500 BOX DRAWINGS LIGHT HORIZONTAL) × lead_width (fixed at 3), then append one space, producing `"─── "`.
3. **Compute trailing rule width**: `trail_width = target_width − lead_width − 1 − content_display_width − 1`, where `target_width` is a `usize` the calling formatter computes and passes to `Heading::render_line(target_width)` — the algorithm itself has no opinion on how `target_width` is derived (see § Target-Width Computation Per Formatter below). The subtractions account for the lead chars, the space after the lead, the content display column count, and one trailing space. Clamp to 0 if negative. Use display column count (`unicode_visual_len`), not byte length or character count — CJK characters are 1 char but 2 display columns; both the field separator (U+00B7) and rule character (U+2500) are multi-byte in UTF-8.
4. **Emit**: lead + content + " " + rule_char × trail_width + newline.

The `terminal_width` setting continues to influence Table's auto-fit column budget but does not itself affect heading or footer line width at any formatter — line width always tracks the formatter's own rendered output, never the terminal.

### Target-Width Computation Per Formatter

`target_width` is computed once per `format()` call and reused for both the heading and footer call site within that call — but *how* it is computed is formatter-specific:

- **Table**: `compute_total_row_width(primary_widths)` — the rendered display width of the table, accounting for column widths, separators, per-column padding, and border pipes. Column widths (`primary_widths`) are already fixed by the auto-fit pipeline before either call site runs, so both the heading and footer line fill to the identical width as every table row.
- **Tree**: the maximum display width across every line of the already-rendered tree body (`body.lines().map(unicode_visual_len).max()`). Tree output has no fixed column width the way table rows do — line length varies with depth and content — so there is no `compute_total_row_width` equivalent; the widest actual line stands in for it. This means the body must be fully rendered *before* `target_width` can be computed, which is why `TreeFormatter::format()` computes the body first and wraps heading/footer around it afterward, rather than emitting the heading inline as `format_internal()` does for Table.
- **Expanded**: identical technique to Tree — the maximum display width across every line of the already-rendered vertical key-value body. Expanded lines are ragged (key width, value width, and padding vary per record), so there is likewise no fixed-width equivalent; `ExpandedFormatter`'s internal `format_view()` builds the full body first, then calls the same `wrap_with_heading_footer()`-shaped helper to bracket it.
- **Text**: identical technique to Tree/Expanded — the maximum display width across every line of the already-rendered text body. Text lines are ragged in every variant (`Bullets`, `Numbered`, `Sections`, `KeyValue`, `Compact`, `CliHelp` each produce different line shapes), so there is likewise no fixed-width equivalent. Unlike Tree/Expanded, `TextFormatter` has no separate config type and no internal branch to converge — its `Format::format()` implementation is the single point that both produces the body (via a `match` over `TextVariant`) and calls `wrap_with_heading_footer()` once on the result before returning, so every variant (including `CliHelp`'s own `data.rows.is_empty()` early return inside `format_cli_help`) is wrapped uniformly with no per-branch wiring needed.
- **Yaml / Toml / Sql / Html**: identical `target_width` derivation to Tree/Expanded/Text — the maximum display width across every line of the already-rendered structured-data body (`serde_yaml_ng`/`toml`/hand-built SQL/HTML output). This `target_width` is the width of the final commented line (comment prefix + rule + comment suffix), not the rule alone — see § Comment-Wrapped Rendering below for how the prefix and suffix are folded in before `Heading::render_line()` is called.

### Comment-Wrapped Rendering (Yaml/Toml/Sql/Html)

Yaml, Toml, Sql, and Html output each has its own comment syntax (`#` for Yaml/Toml, `--` for Sql, `<!-- -->` for Html), so a bare rule line like the one Table/Tree/Expanded/Text emit would not be valid syntax in those formats — it would either be misparsed, mean something else (Yaml/Toml), or — for Html specifically — swallow everything up to the next `-->` if left unclosed. `render_commented_rule_if_present(output, rule, target_width, comment_prefix, comment_suffix)` handles this without modifying `Heading::render_line()`'s own content-and-width logic:

1. Compute `prefix_display_width = unicode_visual_len(comment_prefix)` and `suffix_display_width = unicode_visual_len(comment_suffix)`.
2. Compute `inner_width = target_width.saturating_sub(prefix_display_width).saturating_sub(suffix_display_width)` — the rule portion narrows to leave room for both the prefix and the suffix rather than being added on top of them.
3. Render `heading.render_line(inner_width)`, strip its trailing `\n`, then push `comment_prefix`, the stripped line, `comment_suffix`, and a fresh trailing `\n` — in that order.

Because `prefix_display_width + inner_width + suffix_display_width == target_width` (when `target_width >= prefix_display_width + suffix_display_width`), the resulting commented line's total display width equals `target_width` exactly — the same width invariant Tree/Expanded/Text already establish for their own (uncommented) heading/footer lines. `Heading::render_line()` itself has no awareness that a prefix or suffix was ever involved; every historical bug fix inside it (BUG-015 CJK width, BUG-016 newline sanitization) therefore applies to every comment-wrapped formatter automatically, same as every other adopter. Yaml/Toml/Sql pass `comment_suffix: ""` — a line comment has no closing delimiter, so step 3's suffix push is a no-op and step 1/2's suffix-width subtraction is always 0, leaving their behavior byte-for-byte unchanged from before `comment_suffix` was introduced. Html is the only adopter that passes a non-empty suffix (`" -->"`).

### Position-Agnostic and Formatter-Agnostic Dispatch

The rendering primitives — `Heading::render_line(target_width)` and the `Option`-handling wrapper `render_rule_if_present(output, rule, target_width)` — take the `Heading` value and the target width as explicit parameters rather than reading any specific config field or formatter state internally. Every call site across every formatter supplies both parameters independently:

- **Table heading call site** (`formatters/table/mod.rs`, in `format_internal()`): passes `self.config.heading_ref()` and `compute_total_row_width(primary_widths)`, positioned immediately before `format_top_border_if_needed`.
- **Table footer call site** (same function): passes `self.config.footer_ref()` and the same `compute_total_row_width(primary_widths)`, positioned immediately after `format_bottom_border_if_needed`.
- **Tree heading/footer call sites** (`formatters/tree/mod.rs`, in `wrap_with_heading_footer()`): passes `self.config.heading_ref()` / `self.config.footer_ref()` and the max-rendered-line-width computed from the already-built tree body.
- **Expanded heading/footer call sites** (`formatters/expanded.rs`, in `wrap_with_heading_footer()`): passes `self.config.heading.as_ref()` / `self.config.footer.as_ref()` (direct field access — `ExpandedConfig`'s fields are all `pub`, unlike `TableConfig`/`TreeConfig`'s private-field-plus-accessor convention; see `api/003_config_types.md`) and the max-rendered-line-width computed from the already-built expanded body.
- **Text heading/footer call sites** (`formatters/text.rs`, in `wrap_with_heading_footer()`, invoked once from `Format::format()`): passes `self.heading.as_ref()` / `self.footer.as_ref()` (direct field access on the formatter itself — no separate config type) and the max-rendered-line-width computed from the already-built text body.
- **Yaml / Toml heading/footer call sites** (`formatters/yaml.rs`, `formatters/toml_fmt.rs`, in `wrap_with_heading_footer()`): passes `self.heading.as_ref()` / `self.footer.as_ref()` and the max-rendered-line-width, but through `render_commented_rule_if_present()` with a `"# "` comment prefix and an empty `""` comment suffix, rather than the bare `render_rule_if_present()` every earlier formatter uses (§ Comment-Wrapped Rendering above).
- **Sql heading/footer call sites** (`formatters/sql.rs`, in `wrap_with_heading_footer()`, called from both of `Format::format()`'s return points): identical to Yaml/Toml but with a `"-- "` comment prefix (SQL line-comment syntax) and the same empty `""` suffix.
- **Html heading/footer call site** (`formatters/html.rs`, in `wrap_with_heading_footer()`, invoked once from the single `Format::format()` return point): passes `self.heading.as_ref()` / `self.footer.as_ref()` and the max-rendered-line-width through `render_commented_rule_if_present()` with a `"<!-- "` comment prefix AND a non-empty `" -->"` comment suffix — the only call site that needs the suffix, since HTML's `<!-- -->` is a delimited comment rather than a line comment.

Because the primitives have no internal awareness of which position or which formatter is calling, every call site shares the exact same four-step algorithm and — by construction — every bug fix applied to one position or formatter (BUG-015 CJK width, BUG-016 newline sanitization, BUG-017 bordered-padding width) applies identically to every other with no duplicated logic. Every planned adopter (Table, Tree, Expanded, Text, Yaml, Toml, Sql, Html — see `feature/007_table_heading.md`) is now wired; each needed only its own `target_width` derivation and a call to the shared primitives, never a re-implementation of the four-step algorithm.

### Key Properties

- **Multi-byte safety**: the field separator and rule character are each two or more bytes in UTF-8. Width is measured in display columns (via `unicode_visual_len`), not bytes or character count — CJK characters occupy 2 display columns.
- **Clamp at zero**: when heading content alone equals or exceeds `target_width`, trail_width becomes 0 — the trailing rule is omitted; content is never truncated.
- **Render position**: the heading line is emitted before the table top border (or before the header row when no top border exists for the selected style; before the root/first line for Tree; before the first record line for Expanded; before the first rendered line for Text/Yaml/Toml/Sql/Html — including before the optional `<!DOCTYPE>` wrapper prelude when Html's `include_wrapper` is set); the footer line is emitted after the table bottom border (or after the last row when no bottom border exists for the selected style; after the last rendered line for Tree/Text/Yaml/Toml/Sql/Html; after the last record line for Expanded).
- **Style-agnostic**: the algorithm is identical across all 9 table style presets, at both positions, and across every adopting formatter.
- **Independence**: heading and footer rendering are entirely independent of auto-wrap and auto-fold (Table), and of `record_separator` (Expanded's own per-record divider, repeated once per row rather than once per output). Neither line is subject to column folding or cell wrapping. Heading and footer are also independent of each other — either, both, or neither may be present.
- **`target_width` source varies, algorithm does not**: Table derives `target_width` from fixed column widths computed before either call site runs; Tree, Expanded, Text, Yaml, Toml, Sql, and Html all derive it from the max line width of their own already-rendered body (§ Target-Width Computation Per Formatter). The four-step algorithm itself is identical either way.
- **Comment-wrapping is a wrapping step, not an algorithm variant**: Yaml/Toml/Sql/Html prepend a comment prefix (`"# "`, `"-- "`, or `"<!-- "`) — and, for Html only, append a comment suffix (`" -->"`) — before calling the unmodified `Heading::render_line()` (§ Comment-Wrapped Rendering) — the four-step algorithm and its historical bug fixes apply identically underneath the prefix/suffix.

### Complexity

- Time: O(n) where n is the total character count of title and heading fields — linear scan to build the content string.
- Space: O(1) beyond the output string — no intermediate collections.
