# Invariant: Heading Rendering

### Scope

- **Purpose**: Define the behavioral guarantees that heading and footer rendering must maintain in all configurations, table styles, and adopting formatters.
- **Responsibility**: Documents three invariants — no-heading/no-footer passthrough, width ceiling, and single-line output — applying identically to both the heading (above output) and footer (below output) positions, across every formatter that adopts `Heading`.
- **In Scope**: Absent heading/footer behavior, target-width ceiling, output line count guarantee, at both positions, across formatters.
- **Out of Scope**: Heading content format (see `feature/007_table_heading.md`), rendering algorithm (see `algorithm/007_heading_rendering.md`).

### Features

| File | Relationship |
|------|-------------|
| [007_table_heading.md](../feature/007_table_heading.md) | Feature whose invariants are defined here |

### Algorithms

| File | Relationship |
|------|-------------|
| [007_heading_rendering.md](../algorithm/007_heading_rendering.md) | Algorithm whose outputs these invariants constrain |

### Sources

| File | Relationship |
|------|-------------|
| [`src/config/table_heading.rs`](../../src/config/table_heading.rs) | `Heading::render_line()` / `render_rule_if_present()` — shared rendering path enforcing all three invariants for every caller |
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | Table call sites in `format_internal()` — pass `compute_total_row_width(primary_widths)` as the target width |
| [`src/formatters/tree/mod.rs`](../../src/formatters/tree/mod.rs) | Tree call sites in `wrap_with_heading_footer()` — pass the max rendered line width as the target width |
| [`src/formatters/expanded.rs`](../../src/formatters/expanded.rs) | Expanded call sites in `wrap_with_heading_footer()` — pass the max rendered line width as the target width |
| [`src/formatters/text.rs`](../../src/formatters/text.rs) | Text call sites in `wrap_with_heading_footer()`, invoked once from `Format::format()` — pass the max rendered line width as the target width |
| [`src/formatters/yaml.rs`](../../src/formatters/yaml.rs) | Yaml call sites in `wrap_with_heading_footer()` — pass the max rendered line width to `render_commented_rule_if_present()` (`"# "` prefix) |
| [`src/formatters/toml_fmt.rs`](../../src/formatters/toml_fmt.rs) | Toml call sites — identical to Yaml, same `"# "` prefix |
| [`src/formatters/sql.rs`](../../src/formatters/sql.rs) | Sql call sites, invoked from both of `Format::format()`'s return points — `"-- "` prefix |
| [`src/formatters/html.rs`](../../src/formatters/html.rs) | Html call site in `wrap_with_heading_footer()`, invoked once from the single `Format::format()` return point — pass the max rendered line width to `render_commented_rule_if_present()` (`"<!-- "` prefix, `" -->"` suffix) |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_heading_test.rs`](../../tests/table_heading_test.rs) | Table heading invariant tests |
| [`tests/table_footer_test.rs`](../../tests/table_footer_test.rs) | Table footer invariant tests |
| [`tests/tree_heading_test.rs`](../../tests/tree_heading_test.rs) | Tree heading/footer invariant tests |
| [`tests/expanded_heading_test.rs`](../../tests/expanded_heading_test.rs) | Expanded heading/footer invariant tests |
| [`tests/text_heading_test.rs`](../../tests/text_heading_test.rs) | Text heading/footer invariant tests |
| [`tests/yaml_heading_test.rs`](../../tests/yaml_heading_test.rs) | Yaml comment-wrapped heading/footer invariant tests |
| [`tests/toml_heading_test.rs`](../../tests/toml_heading_test.rs) | Toml comment-wrapped heading/footer invariant tests |
| [`tests/sql_heading_test.rs`](../../tests/sql_heading_test.rs) | Sql comment-wrapped heading/footer invariant tests, including the BUG-020 empty-rows branch |
| [`tests/html_heading_test.rs`](../../tests/html_heading_test.rs) | Html comment-delimited heading/footer invariant tests, including the `include_wrapper` interaction |

### Invariant Statement

#### Invariant 1 — No-Heading/No-Footer Passthrough

A table with neither heading nor footer attached produces output byte-identical to the output produced by the same table before the heading/footer feature existed. Each rule path is a strict additive extension, independent of the other — no code runs on the heading path when no heading is set, and no code runs on the footer path when no footer is set. Setting one does not activate or alter the other.

#### Invariant 2 — Width Ceiling

Neither the heading line nor the footer line ever exceeds the calling formatter's target width. When a rule's content (lead prefix + content + trailing space) equals or exceeds the target width, the trailing rule width is clamped to zero. Each line may be shorter than the target width but never longer. When content alone exceeds the target width, that line may exceed it — the invariant guarantee is that the trailing rule is clamped to zero and content is never truncated. This guarantee holds independently at each position — a wide heading does not affect footer width, and vice versa. What varies per formatter is *how the target width itself is derived* (Table: `compute_total_row_width(primary_widths)`, fixed before either call site runs; Tree/Expanded/Text/Yaml/Toml/Sql/Html: the max display width across the already-rendered body's lines — see `algorithm/007_heading_rendering.md § Target-Width Computation Per Formatter`) — the clamp-to-zero guarantee this invariant states is identical regardless of that source. Yaml/Toml/Sql/Html apply one or two additional `saturating_sub` calls before the same clamp: the comment prefix's own display width is always subtracted from `target_width`, and — for Html only, since Yaml/Toml/Sql pass an empty `""` suffix — the comment suffix's own display width is subtracted too, together producing `inner_width` (§ Comment-Wrapped Rendering), so the *commented* line — prefix plus rule plus suffix — is what stays within the ceiling, not the rule alone.

#### Invariant 3 — Single Output Line

Each of the heading and footer, when present, always occupies exactly one output line ending with a newline character. No combination of title length, heading field count, terminal width, or embedded line breaks can cause either line to span multiple lines.

### Enforcement Mechanism

Invariant 1 is enforced by an early-exit guard in `render_rule_if_present()` (and, identically, in `render_commented_rule_if_present()` for the comment-wrapped formatters) that skips the render path entirely when the `rule` parameter passed by the call site is `None` — every formatter's heading call site passes its own heading value (a `heading_ref()` accessor on Table/Tree; direct `self.config.heading.as_ref()` field access on Expanded, whose `ExpandedConfig` fields are all `pub` — see `api/003_config_types.md`; direct `self.heading.as_ref()` field access on Text/Yaml/Toml/Sql/Html, none of which have a separate config type — the fields live on the formatter struct itself), every footer call site passes its own footer value likewise, each independently `None` by default. (Tree, Expanded, Text, Yaml, Toml, Sql, and Html additionally short-circuit one level up, in their own `wrap_with_heading_footer()`, returning the body unchanged when both are `None` — an optimization, not a second enforcement point; the `render_rule_if_present()`/`render_commented_rule_if_present()` guard would produce the same byte-identical result on its own.) Invariant 2 is enforced by the `saturating_sub` clamp applied to the trailing rule width computation in `Heading::render_line()`, using whatever target width the caller passed in — never the terminal width — so the same clamp logic runs for every call site across every formatter since they all share one function; Yaml/Toml/Sql/Html's additional prefix-width (and, for Html, suffix-width) `saturating_sub` calls in `render_commented_rule_if_present()` run before that shared clamp, never replacing it. Invariant 3 is enforced by `sanitize_line_breaks()` in `content_str()` which replaces all line-break sequences (`\r\n`, `\r`, `\n`) with spaces before assembly, and by the single-string assembly in the rendering algorithm which appends exactly one newline at the end; for the comment-wrapped formatters, `render_commented_rule_if_present()` strips that trailing newline before pushing `comment_suffix`, then appends exactly one fresh newline after prefix + line + suffix, so the invariant is re-established at the new end of line rather than relying on `render_line()`'s own newline surviving unmodified. Both the comment prefix (`"# "`, `"-- "`, or `"<!-- "`) and the comment suffix (`""` for Yaml/Toml/Sql, `" -->"` for Html) are plain literals with no embedded newline of their own, so neither can introduce a second line.

### Violation Consequences

| Invariant | Consequence of Violation |
|-----------|------------------------|
| No-heading/no-footer passthrough | Silent regression for all existing callers — every formatter's output changes without any user action |
| Width ceiling | Heading or footer line overflows the calling formatter's target width; output wraps or truncates unexpectedly in terminal display |
| Single output line | Formatter output layout corrupts; adjacent rows, borders, or tree lines shift by an unexpected number of lines |
