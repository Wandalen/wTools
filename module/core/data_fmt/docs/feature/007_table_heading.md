# Feature: Heading and Footer Titled Rules

### Scope

- **Purpose**: Allow callers to attach a titled rule line — above the formatted output (heading) and/or below it (footer) — that matches the formatter's rendered width, carrying a title and optional heading fields.
- **Responsibility**: Document the `Heading` type, its API, rendering algorithm, and integration with `TableConfig`, `TreeConfig`, `ExpandedConfig`, `TextFormatter`, `YamlFormatter`, `TomlFormatter`, `SqlFormatter`, and `HtmlFormatter` at both the heading and footer positions.
- **In Scope**: `Heading` struct, heading fields, field separator, rule character, lead width, `TableConfig::with_heading()`/`with_footer()`, `TreeConfig::with_heading()`/`with_footer()`, `ExpandedConfig::with_heading()`/`with_footer()`, `TextFormatter::with_heading()`/`with_footer()`, `YamlFormatter::with_heading()`/`with_footer()`, `TomlFormatter::with_heading()`/`with_footer()`, `SqlFormatter::with_heading()`/`with_footer()`, and `HtmlFormatter::with_heading()`/`with_footer()` builders, rendering position and target-width behavior, comment-prefixed rendering for Yaml/Toml/Sql, comment-delimited (prefix + suffix) rendering for Html.
- **Out of Scope**: Table body rendering (see `001_table_formatting.md`); auto-fit algorithm (see `005_auto_fit.md`); terminal width detection (see `005_auto_fit.md § Terminal Width Detection`); Json/Logfmt heading/footer coverage (no comment syntax / no comment convention — excluded, see `task/018_heading_footer_multi_formatter.md`); a visible `<caption>`-element banner for Html (rejected alternative — singular by spec, cannot represent independent heading + footer; see `task/completed/019_html_heading_footer.md`).

### Sources

| File | Relationship |
|------|-------------|
| [`src/config/table_heading.rs`](../../src/config/table_heading.rs) | `Heading` struct definition; `Heading::render_line()`, `render_rule_if_present()`, and `render_commented_rule_if_present()` (prefix + suffix wrapping) — shared rendering paths reused by every adopting formatter |
| [`src/config/table_config.rs`](../../src/config/table_config.rs) | `TableConfig::with_heading()` / `with_footer()` builder setters |
| [`src/config/tree_config.rs`](../../src/config/tree_config.rs) | `TreeConfig::with_heading()` / `with_footer()` builder setters |
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | Table call sites in `format_internal()` — pass `compute_total_row_width(primary_widths)` as the target width |
| [`src/formatters/tree/mod.rs`](../../src/formatters/tree/mod.rs) | Tree call sites in `wrap_with_heading_footer()` — pass the widest rendered body line's display width as the target width |
| [`src/config/expanded_config.rs`](../../src/config/expanded_config.rs) | `ExpandedConfig::with_heading()` / `with_footer()` builder setters (plain `pub` fields, no accessor methods — see `api/003_config_types.md`) |
| [`src/formatters/expanded.rs`](../../src/formatters/expanded.rs) | Expanded call sites in `wrap_with_heading_footer()` — pass the widest rendered body line's display width as the target width, same technique as Tree |
| [`src/formatters/text.rs`](../../src/formatters/text.rs) | `TextFormatter::with_heading()` / `with_footer()` builder setters (no separate config type — plain `pub` fields directly on `TextFormatter`) and call sites in `wrap_with_heading_footer()`, invoked once from the shared `Format::format()` funnel point — pass the widest rendered body line's display width as the target width, same technique as Tree/Expanded |
| [`src/formatters/yaml.rs`](../../src/formatters/yaml.rs) | `YamlFormatter::with_heading()` / `with_footer()` builder setters (no separate config type) and call sites in `wrap_with_heading_footer()`, invoked from `Format::format()` — pass the widest rendered body line's display width to `render_commented_rule_if_present()` with a `"# "` comment prefix |
| [`src/formatters/toml_fmt.rs`](../../src/formatters/toml_fmt.rs) | `TomlFormatter::with_heading()` / `with_footer()` builder setters and call sites — identical technique to Yaml, same `"# "` prefix |
| [`src/formatters/sql.rs`](../../src/formatters/sql.rs) | `SqlFormatter::with_heading()` / `with_footer()` builder setters and call sites, invoked from both of `Format::format()`'s return points (the BUG-020 empty-rows early return and the final populated-rows return) — `"-- "` comment prefix |
| [`src/formatters/html.rs`](../../src/formatters/html.rs) | `HtmlFormatter::with_heading()` / `with_footer()` builder setters (plain `pub` fields, no accessor methods — same convention as Text/Yaml/Toml/Sql) and call site in `wrap_with_heading_footer()`, invoked once from the single `Format::format()` return point — pass the widest rendered line's display width to `render_commented_rule_if_present()` with `"<!-- "` prefix and `" -->"` suffix, the only adopting formatter to use a non-empty suffix |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_heading_test.rs`](../../tests/table_heading_test.rs) | Table heading test implementation — FC-1 through FC-6 |
| [`tests/table_footer_test.rs`](../../tests/table_footer_test.rs) | Table footer test implementation — FT-10..FT-18 |
| [`tests/tree_heading_test.rs`](../../tests/tree_heading_test.rs) | Tree heading/footer test implementation — FT-19..FT-24 |
| [`tests/expanded_heading_test.rs`](../../tests/expanded_heading_test.rs) | Expanded heading/footer test implementation — FT-25..FT-30 |
| [`tests/text_heading_test.rs`](../../tests/text_heading_test.rs) | Text heading/footer test implementation — FT-31..FT-36 |
| [`tests/yaml_heading_test.rs`](../../tests/yaml_heading_test.rs) | Yaml comment-wrapped heading/footer test implementation — FT-37..FT-41 |
| [`tests/toml_heading_test.rs`](../../tests/toml_heading_test.rs) | Toml comment-wrapped heading/footer test implementation — FT-42..FT-46 |
| [`tests/sql_heading_test.rs`](../../tests/sql_heading_test.rs) | Sql comment-wrapped heading/footer test implementation — FT-47..FT-52, including the BUG-020 empty-rows branch |
| [`tests/html_heading_test.rs`](../../tests/html_heading_test.rs) | Html comment-delimited heading/footer test implementation — FT-53..FT-58, including the `include_wrapper` interaction |

### Features

| File | Relationship |
|------|-------------|
| [001_table_formatting.md](001_table_formatting.md) | Feature extended by heading and footer titled rules |

### Algorithms

| File | Relationship |
|------|-------------|
| [007_heading_rendering.md](../algorithm/007_heading_rendering.md) | Algorithm implementing heading line assembly |

### Invariants

| File | Relationship |
|------|-------------|
| [005_heading.md](../invariant/005_heading.md) | Behavioral guarantees for heading rendering |

### APIs

| File | Relationship |
|------|-------------|
| [003_config_types.md](../api/003_config_types.md) | `Heading` and `TableConfig::with_heading()` API surface |

### Design

#### Motivation

CLI tools that display filtered or summarised tables benefit from a concise header line that names the data set and shows top-level aggregates alongside:

```
─── Needs Review · 28 PRs · 15 repos ─────────────────────────────────────────────
Repository                       PR#  State  Title
-------------------------------  ---  -----  ----------------------------------
Wandalen/cgtools                 129  open   NEED REVIEW : 3d line dash
```

The line carries the title `"Needs Review"` and two heading fields `"28 PRs"` and `"15 repos"`. The rule chars fill the remainder of the rendered table width, aligning the heading rule with the right edge of the table below it.

#### Construction

```rust
// Type definition (src/config/table_heading.rs)
pub struct Heading {
    title  : String,
    fields : Vec<String>,
    // Formatting is fixed: field_sep = '·' (U+00B7), rule_char = '─' (U+2500), lead_width = 3
}

impl Heading {
    pub fn new(title: impl Into<String>) -> Self
    pub fn with_field(mut self, f: impl Into<String>) -> Self
}

// Integration on TableConfig (src/config/table_config.rs)
impl TableConfig {
    pub fn with_heading(mut self, heading: Heading) -> Self
    pub fn with_footer(mut self, footer: Heading) -> Self
}
```

**Minimal usage:**
```rust
let config = TableConfig::plain()
    .with_heading(Heading::new("Needs Review"));
// ─── Needs Review ──────────────────────────────────────────────────────────────
// Name   Age
// -----  ---
// Alice  30
```

**With heading fields:**
```rust
let config = TableConfig::plain()
    .with_heading(
        Heading::new("Needs Review")
            .with_field("28 PRs")
            .with_field("15 repos")
    );
// ─── Needs Review · 28 PRs · 15 repos ─────────────────────────────────────────
```

**Footer usage:**
```rust
let config = TableConfig::plain()
    .with_footer(Heading::new("52 rows"));
// Name   Age
// -----  ---
// Alice  30
// ─── 52 rows ────────────────────────────────────────────────────────────────────
```

**Header and footer together:**
```rust
let config = TableConfig::plain()
    .with_heading(Heading::new("Needs Review"))
    .with_footer(Heading::new("28 PRs · 15 repos"));
// ─── Needs Review ──────────────────────────────────────────────────────────────
// Name   Age
// -----  ---
// Alice  30
// ─── 28 PRs · 15 repos ─────────────────────────────────────────────────────────
```

**Tree usage:**
```rust
let config = TreeConfig::new()
    .with_heading(Heading::new("Project Files"))
    .with_footer(Heading::new("12 files"));
let output = TreeFormatter::with_config(config).format(&tree, render_item);
// ─── Project Files ─────────────
// root
// ├── src
// │   └── main.rs
// ─── 12 files ───────────────────
```
Tree has no fixed column width, so the rule fills to the widest rendered line instead of a table-style width — see § Rendering Algorithm below.

**Expanded usage:**
```rust
let config = ExpandedConfig::new()
    .with_heading(Heading::new("Users"))
    .with_footer(Heading::new("2 records"));
let output = ExpandedFormatter::with_config(config).format(&view).unwrap_or_default();
// ─── Users ──────────
// -[ RECORD 1 ]
// Name | Alice
// Age  | 30
// -[ RECORD 2 ]
// Name | Bob
// Age  | 25
// ─── 2 records ──────
```
Like Tree, Expanded has no fixed column width — the rule fills to the widest rendered line, computed after the full vertical-record body is built. This is entirely independent of `record_separator` (the `"-[ RECORD N ]"` line repeated once per record, inside the body): heading/footer bracket the whole output exactly once each, `record_separator` repeats once per record.

**Text usage:**
```rust
let formatter = TextFormatter::bullets()
    .with_heading(Heading::new("Users"))
    .with_footer(Heading::new("2 records"));
let output = formatter.format(&view).unwrap_or_default();
// ─── Users ──────
// • Alice 30
// • Bob 25
// ─── 2 records ──
```
`TextFormatter` has no separate config type — `heading`/`footer` are plain `pub` fields directly on the formatter (matching its own pre-existing all-`pub`-fields, no-accessor convention), set via `with_heading()`/`with_footer()`. Like Tree and Expanded, Text has no fixed column width — the rule fills to the widest line across whichever of the 6 `TextVariant` styles (`Bullets`, `Numbered`, `Sections`, `KeyValue`, `Compact`, `CliHelp`) produced the body. Both fields are read at a single funnel point — `TextFormatter`'s `Format::format()` implementation calls `wrap_with_heading_footer()` once on its `match`-produced body before returning — so every variant, including `CliHelp`'s own internal empty-rows early return, is wrapped uniformly with no per-variant wiring.

**Yaml usage:**
```rust
let formatter = YamlFormatter::new()
    .with_heading(Heading::new("Users"))
    .with_footer(Heading::new("2 records"));
let output = formatter.format(&view).unwrap_or_default();
// # ─── Users ──────
// - Age: '30'
//   Name: Alice
// - Age: '25'
//   Name: Bob
// # ─── 2 records ──
```
YAML has no fixed column width, so — like Tree/Expanded/Text — the rule fills to the widest rendered body line. Because YAML uses `#` for comments, the rendered rule is prefixed with `"# "` before being emitted, so the heading/footer stay valid YAML rather than corrupting the document. `YamlFormatter` has no separate config type — `heading`/`footer` are plain `pub` fields directly on the formatter, same convention as `TextFormatter`.

**Toml usage:**
```rust
let formatter = TomlFormatter::new()
    .with_heading(Heading::new("Users"))
    .with_footer(Heading::new("2 records"));
let output = formatter.format(&view).unwrap_or_default();
// # ─── Users ────────
// [[row]]
// Age = "30"
// Name = "Alice"
//
// [[row]]
// Age = "25"
// Name = "Bob"
// # ─── 2 records ────
```
TOML also uses `#` for comments, so `TomlFormatter` reuses the identical `"# "`-prefixed rendering as Yaml — same target-width derivation, same builder shape.

**Sql usage:**
```rust
let formatter = SqlFormatter::new("users")
    .with_heading(Heading::new("Users"))
    .with_footer(Heading::new("2 records"));
let output = formatter.format(&view).unwrap_or_default();
// -- ─── Users ────────────────────────────────
// INSERT INTO "users" ("Name", "Age") VALUES
//   ('Alice', 30),
//   ('Bob', 25);
// -- ─── 2 records ────────────────────────────
```
SQL uses `--` for line comments, so `SqlFormatter` prefixes the rendered rule with `"-- "` instead of `"# "`. `SqlFormatter::format()` has two return points — the BUG-020 early return for zero rows (which would otherwise emit invalid `INSERT INTO ... VALUES;` SQL) and the normal populated-rows return — both call the same `wrap_with_heading_footer()`, so heading/footer apply regardless of which branch produced the body. Unlike every other adopting formatter's body, the populated-rows SQL body ends in a bare `;` with no trailing newline; `wrap_with_heading_footer()` inserts a separating `\n` before the footer when one is present, so the footer still lands on its own line rather than concatenating onto the closing `;`.

**Html usage:**
```rust
let formatter = HtmlFormatter::new()
    .with_heading(Heading::new("Users"))
    .with_footer(Heading::new("2 records"));
let output = formatter.format(&view).unwrap_or_default();
// <!-- ─── Users ──────────────────────── -->
// <table>
//   <tbody>
//     <tr><td>Alice</td><td>30</td></tr>
//     <tr><td>Bob</td><td>25</td></tr>
//   </tbody>
// </table>
// <!-- ─── 2 records ──────────────────── -->
```
HTML has no line-comment syntax — `<!-- -->` is a *delimited* comment, so an unclosed `<!--` would silently swallow everything up to the next `-->` (including the `<table>` markup itself). `HtmlFormatter` is therefore the only adopting formatter that passes a non-empty `comment_suffix` to `render_commented_rule_if_present()`: `"<!-- "` prefix and `" -->"` suffix, both subtracted from the target width so the fully-delimited line — not just the rule portion — fills to the widest rendered line. `HtmlFormatter` has no separate config type — `heading`/`footer` are plain `pub` fields directly on the formatter, same convention as Text/Yaml/Toml/Sql. When `include_wrapper` is enabled, `wrap_with_heading_footer()` still wraps the *entire* rendered output — the heading lands before `<!DOCTYPE html>`, not between the wrapper and `<table>` — matching every other formatter's "wrap the whole rendered body" pattern rather than reaching inside the optional prelude. Like `SqlFormatter`'s body, HTML output ends with a bare closing tag (`</table>` or `</html>`) and no trailing newline, so the same separating-`\n`-before-footer guard applies.

#### Rendering Algorithm

The heading is rendered immediately before the formatted output begins (Table: before the top border, or the header row when no top border exists for the selected style; Tree: before the first rendered line; Expanded: before the first record line; Text/Yaml/Toml/Sql/Html: before the first rendered line of the body); a footer is rendered immediately after the formatted output ends (Table: after the bottom border, or after the last row when no bottom border exists; Tree: after the last rendered line; Expanded: after the last record line; Text/Yaml/Toml/Sql/Html: after the last rendered line). Both positions, across every adopting formatter, share one position-agnostic and formatter-agnostic rendering function (`Heading::render_line()`, invoked via the `render_rule_if_present()` wrapper for Table/Tree/Expanded/Text, or the `render_commented_rule_if_present()` wrapper for Yaml/Toml/Sql/Html) — it receives whichever `Heading` value the caller passes (a `heading_ref()`/`footer_ref()` accessor on Table/Tree, direct `self.config.heading.as_ref()`/`self.config.footer.as_ref()` field access on Expanded, or direct `self.heading.as_ref()`/`self.footer.as_ref()` field access on Text/Yaml/Toml/Sql/Html — no `self.config` prefix, since none of them have a separate config type) plus a `target_width`, and has no awareness of "above" vs "below" placement or of which formatter is calling it. The rendering steps are identical everywhere:

1. Build the content string: sanitize line breaks in title and each field (replace `\r\n`, `\r`, `\n` with space), then concatenate `title` followed by `" {field_sep} {field}"` for each heading field.
2. Build the lead: `rule_char` × `lead_width` + ` ` (e.g., `"─── "`).
3. Build the trailing rule: compute `trail_width = target_width - lead_width - 1 - unicode_visual_len(&content) - 1`. `target_width` is supplied by the calling formatter and derived differently per formatter — Table computes it via `compute_total_row_width(primary_widths)` (column widths, separators, per-column padding, and border pipes, fixed before either call site runs); Tree, Expanded, Text, Yaml, Toml, Sql, and Html all compute it as the maximum display width across their own already-rendered body's lines (see `algorithm/007_heading_rendering.md § Target-Width Computation Per Formatter`). Use `unicode_visual_len` (display column count), not `.len()` or `.chars().count()` — CJK characters are 1 char but 2 display columns; `·` (U+00B7) and `─` (U+2500) are multi-byte in UTF-8. Clamp `trail_width` to 0 if negative.
4. Emit: `lead + content + " " + rule_char × trail_width + "\n"`.

The trailing rule fills to the calling formatter's target width — never the terminal width. This ensures the heading rule aligns with the right edge of the formatted output regardless of how wide the terminal is.

For Yaml/Toml/Sql/Html, `render_commented_rule_if_present()` runs extra steps around `Heading::render_line()`: it subtracts the comment prefix's own display width (`"# "` → 2, `"-- "` → 3, `"<!-- "` → 5) — and, for Html only, the comment suffix's own display width (`" -->"` → 4) — from `target_width` to get `inner_width`, so the *commented* line's total width — prefix plus rule plus suffix — equals `target_width`, not the rule alone. Yaml/Toml/Sql pass `comment_suffix: ""` (line comments have no closing delimiter, so the suffix step is a no-op for them); Html passes `comment_suffix: " -->"`, the only non-empty suffix among adopting formatters. See `algorithm/007_heading_rendering.md § Comment-Wrapped Rendering` for the full mechanics.

#### Interaction with Other Features

- **auto_wrap / auto_fold**: Heading and footer rendering are independent of Table's auto-wrap/auto-fold. Neither line is subject to column folding or cell wrapping. (Tree, Expanded, Text, Yaml, Toml, Sql, and Html have no equivalent wrap/fold mechanism to interact with.)
- **ANSI coloring**: Heading and footer text are emitted as plain text. ANSI decoration is not in scope for this feature.
- **All 9 table styles**: on Table, both the heading and footer are style-agnostic — they render the same titled rule regardless of `BorderVariant`. On Tree, the same rule is agnostic to `show_branches`/`indent_size`/symbol configuration; on Expanded, it is agnostic to `padding_side`/`key_value_separator`/`indent_prefix` configuration; on Text, it is agnostic to `variant`/`indent`/`separator` configuration — all 6 `TextVariant` styles are wrapped identically; on Sql, it is agnostic to `SqlVariant`/`empty_as_null` configuration — all 4 dialects (`Ansi`, `PostgreSQL`, `MySQL`, `SQLite`) are wrapped identically; on Html, it is agnostic to `variant`/`table_id`/`include_wrapper` configuration — all 4 `HtmlVariant` themes (`Minimal`, `Bootstrap`, `Tailwind`, `Custom`) are wrapped identically.
- **`terminal_width` setting**: Continues to control Table's auto-fit column budget allocation; does not affect heading or footer line width on any formatter. Width always tracks the formatter's own rendered output, never the terminal.
- **Header and footer independence**: `.with_heading()` and `.with_footer()` are independent optional fields on `TableConfig`, `TreeConfig`, `ExpandedConfig`, `TextFormatter`, `YamlFormatter`, `TomlFormatter`, `SqlFormatter`, and `HtmlFormatter` — either, both, or neither may be set. Setting one has no effect on the other's presence or content.
- **Expanded's `record_separator`**: heading/footer are entirely independent of `record_separator` (the per-record `"-[ RECORD N ]"` divider rendered once per row, inside the body). Heading/footer bracket the *entire* formatted output exactly once each; `record_separator` repeats *per record*. Setting one has no effect on the other.
- **Yaml/Toml/Sql/Html's comment syntax**: heading/footer are the only construct in this feature that must stay valid within the target format's own syntax — Yaml/Toml comments (`#`) and Sql line comments (`--`) need only a prefix; Html's `<!-- -->` is a delimited comment and additionally needs the `" -->"` suffix, or everything up to the next `-->` in the document would be swallowed. `render_commented_rule_if_present()` wraps every rendered rule line accordingly (§ Rendering Algorithm above). This has no equivalent on Table/Tree/Expanded/Text, whose output has no comment syntax to preserve.
- **Sql's BUG-020 empty-rows guard**: heading/footer are independent of the empty-rows early return that avoids emitting invalid `INSERT INTO ... VALUES;` SQL — both of `SqlFormatter::format()`'s return points call the same `wrap_with_heading_footer()`, so heading/footer apply whether or not any rows were present.
- **Html's `include_wrapper` prelude**: heading/footer bracket the *entire* rendered output, including the optional `<!DOCTYPE>`/`<html>`/`<body>` wrapper when `include_wrapper` is `true` — the heading is the first line of output and the footer the last, regardless of whether the wrapper is present. There is no variant where heading/footer are inserted between the wrapper and the `<table>` tag.

See `invariant/005_heading.md` for no-heading/no-footer passthrough, width ceiling, and single-line output guarantees.
