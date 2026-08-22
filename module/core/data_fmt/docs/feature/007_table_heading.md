# Feature: Heading and Footer Titled Rules

### Scope

- **Purpose**: Allow callers to attach a titled rule line — above the formatted output (heading) and/or below it (footer) — that matches the formatter's rendered width, carrying a title and optional heading fields.
- **Responsibility**: Document the `Heading` type, its API, rendering algorithm, and integration with `TableConfig`, `TreeConfig`, `ExpandedConfig`, and `TextFormatter` at both the heading and footer positions.
- **In Scope**: `Heading` struct, heading fields, field separator, rule character, lead width, `TableConfig::with_heading()`/`with_footer()`, `TreeConfig::with_heading()`/`with_footer()`, `ExpandedConfig::with_heading()`/`with_footer()`, and `TextFormatter::with_heading()`/`with_footer()` builders, rendering position and target-width behavior.
- **Out of Scope**: Table body rendering (see `001_table_formatting.md`); auto-fit algorithm (see `005_auto_fit.md`); terminal width detection (see `005_auto_fit.md § Terminal Width Detection`); Yaml/Toml/Sql heading/footer coverage (planned, not yet implemented — see `task/018_heading_footer_multi_formatter.md`).

### Sources

| File | Relationship |
|------|-------------|
| [`src/config/table_heading.rs`](../../src/config/table_heading.rs) | `Heading` struct definition; `Heading::render_line()` and `render_rule_if_present()` — shared rendering path reused by every adopting formatter |
| [`src/config/table_config.rs`](../../src/config/table_config.rs) | `TableConfig::with_heading()` / `with_footer()` builder setters |
| [`src/config/tree_config.rs`](../../src/config/tree_config.rs) | `TreeConfig::with_heading()` / `with_footer()` builder setters |
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | Table call sites in `format_internal()` — pass `compute_total_row_width(primary_widths)` as the target width |
| [`src/formatters/tree/mod.rs`](../../src/formatters/tree/mod.rs) | Tree call sites in `wrap_with_heading_footer()` — pass the widest rendered body line's display width as the target width |
| [`src/config/expanded_config.rs`](../../src/config/expanded_config.rs) | `ExpandedConfig::with_heading()` / `with_footer()` builder setters (plain `pub` fields, no accessor methods — see `api/003_config_types.md`) |
| [`src/formatters/expanded.rs`](../../src/formatters/expanded.rs) | Expanded call sites in `wrap_with_heading_footer()` — pass the widest rendered body line's display width as the target width, same technique as Tree |
| [`src/formatters/text.rs`](../../src/formatters/text.rs) | `TextFormatter::with_heading()` / `with_footer()` builder setters (no separate config type — plain `pub` fields directly on `TextFormatter`) and call sites in `wrap_with_heading_footer()`, invoked once from the shared `Format::format()` funnel point — pass the widest rendered body line's display width as the target width, same technique as Tree/Expanded |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_heading_test.rs`](../../tests/table_heading_test.rs) | Table heading test implementation — FC-1 through FC-6 |
| [`tests/table_footer_test.rs`](../../tests/table_footer_test.rs) | Table footer test implementation — FT-10..FT-18 |
| [`tests/tree_heading_test.rs`](../../tests/tree_heading_test.rs) | Tree heading/footer test implementation — FT-19..FT-24 |
| [`tests/expanded_heading_test.rs`](../../tests/expanded_heading_test.rs) | Expanded heading/footer test implementation — FT-25..FT-30 |
| [`tests/text_heading_test.rs`](../../tests/text_heading_test.rs) | Text heading/footer test implementation — FT-31..FT-36 |

### Features

| File | Relationship |
|------|-------------|
| [001_table_formatting.md](001_table_formatting.md) | Feature extended by table heading |

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

#### Rendering Algorithm

The heading is rendered immediately before the formatted output begins (Table: before the top border, or the header row when no top border exists for the selected style; Tree: before the first rendered line; Expanded: before the first record line; Text: before the first rendered line of whichever variant produced the body); a footer is rendered immediately after the formatted output ends (Table: after the bottom border, or after the last row when no bottom border exists; Tree: after the last rendered line; Expanded: after the last record line; Text: after the last rendered line). Both positions, across every adopting formatter, share one position-agnostic and formatter-agnostic rendering function (`Heading::render_line()`, invoked via the `render_rule_if_present()` wrapper) — it receives whichever `Heading` value the caller passes (a `heading_ref()`/`footer_ref()` accessor on Table/Tree, direct `self.config.heading.as_ref()`/`self.config.footer.as_ref()` field access on Expanded, or direct `self.heading.as_ref()`/`self.footer.as_ref()` field access on Text — no `self.config` prefix, since Text has no separate config type) plus a `target_width`, and has no awareness of "above" vs "below" placement or of which formatter is calling it. The rendering steps are identical everywhere:

1. Build the content string: sanitize line breaks in title and each field (replace `\r\n`, `\r`, `\n` with space), then concatenate `title` followed by `" {field_sep} {field}"` for each heading field.
2. Build the lead: `rule_char` × `lead_width` + ` ` (e.g., `"─── "`).
3. Build the trailing rule: compute `trail_width = target_width - lead_width - 1 - unicode_visual_len(&content) - 1`. `target_width` is supplied by the calling formatter and derived differently per formatter — Table computes it via `compute_total_row_width(primary_widths)` (column widths, separators, per-column padding, and border pipes, fixed before either call site runs); Tree, Expanded, and Text all compute it as the maximum display width across their own already-rendered body's lines (see `algorithm/007_heading_rendering.md § Target-Width Computation Per Formatter`). Use `unicode_visual_len` (display column count), not `.len()` or `.chars().count()` — CJK characters are 1 char but 2 display columns; `·` (U+00B7) and `─` (U+2500) are multi-byte in UTF-8. Clamp `trail_width` to 0 if negative.
4. Emit: `lead + content + " " + rule_char × trail_width + "\n"`.

The trailing rule fills to the calling formatter's target width — never the terminal width. This ensures the heading rule aligns with the right edge of the formatted output regardless of how wide the terminal is.

#### Interaction with Other Features

- **auto_wrap / auto_fold**: Heading and footer rendering are independent of Table's auto-wrap/auto-fold. Neither line is subject to column folding or cell wrapping. (Tree, Expanded, and Text have no equivalent wrap/fold mechanism to interact with.)
- **ANSI coloring**: Heading and footer text are emitted as plain text. ANSI decoration is not in scope for this feature.
- **All 9 table styles**: on Table, both the heading and footer are style-agnostic — they render the same titled rule regardless of `BorderVariant`. On Tree, the same rule is agnostic to `show_branches`/`indent_size`/symbol configuration; on Expanded, it is agnostic to `padding_side`/`key_value_separator`/`indent_prefix` configuration; on Text, it is agnostic to `variant`/`indent`/`separator` configuration — all 6 `TextVariant` styles are wrapped identically.
- **`terminal_width` setting**: Continues to control Table's auto-fit column budget allocation; does not affect heading or footer line width on any formatter. Width always tracks the formatter's own rendered output, never the terminal.
- **Header and footer independence**: `.with_heading()` and `.with_footer()` are independent optional fields on `TableConfig`, `TreeConfig`, `ExpandedConfig`, and `TextFormatter` — either, both, or neither may be set. Setting one has no effect on the other's presence or content.
- **Expanded's `record_separator`**: heading/footer are entirely independent of `record_separator` (the per-record `"-[ RECORD N ]"` divider rendered once per row, inside the body). Heading/footer bracket the *entire* formatted output exactly once each; `record_separator` repeats *per record*. Setting one has no effect on the other.

See `invariant/005_heading.md` for no-heading/no-footer passthrough, width ceiling, and single-line output guarantees.
