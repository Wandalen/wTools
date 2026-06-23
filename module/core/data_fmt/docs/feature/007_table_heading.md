# Feature: Table Heading

### Scope

- **Purpose**: Allow callers to attach a titled heading line above a table — a titled rule that matches the rendered table width — carrying a title and optional heading fields.
- **Responsibility**: Document the `Heading` type, its API, rendering algorithm, and integration with `TableConfig`.
- **In Scope**: `Heading` struct, heading fields, field separator, rule character, lead width, `TableConfig::with_heading()` builder, rendering position and table-width behavior.
- **Out of Scope**: Table body rendering (see `001_table_formatting.md`); auto-fit algorithm (see `005_auto_fit.md`); terminal width detection (see `005_auto_fit.md § Terminal Width Detection`).

### Sources

| File | Relationship |
|------|-------------|
| [`src/config/table_heading.rs`](../../src/config/table_heading.rs) | `Heading` struct definition |
| [`src/config/table_config.rs`](../../src/config/table_config.rs) | `TableConfig::with_heading()` builder setter |
| [`src/formatters/table/row_rendering.rs`](../../src/formatters/table/row_rendering.rs) | `render_heading_if_present()` — heading line assembly |
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | Call site in `format_internal()` where `primary_widths` is passed to heading renderer |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_heading_test.rs`](../../tests/table_heading_test.rs) | Test implementation — FC-1 through FC-6 |

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

#### Rendering Algorithm

The heading is rendered immediately before the table top border (or header row when no top border exists). The rendering steps are:

1. Build the content string: sanitize line breaks in title and each field (replace `\r\n`, `\r`, `\n` with space), then concatenate `title` followed by `" {field_sep} {field}"` for each heading field.
2. Build the lead: `rule_char` × `lead_width` + ` ` (e.g., `"─── "`).
3. Build the trailing rule: compute `trail_width = table_width - lead_width - 1 - unicode_visual_len(&content) - 1` where `table_width` is the actual rendered display width of the table (computed by `compute_total_row_width(primary_widths)`, accounting for column widths, separators, per-column padding, and border pipes). Use `unicode_visual_len` (display column count), not `.len()` or `.chars().count()` — CJK characters are 1 char but 2 display columns; `·` (U+00B7) and `─` (U+2500) are multi-byte in UTF-8. Clamp `trail_width` to 0 if negative.
4. Emit: `lead + content + " " + rule_char × trail_width + "\n"`.

The trailing rule fills to the rendered table width — not the terminal width. This ensures the heading rule aligns with the right edge of the table regardless of how wide the terminal is.

#### Interaction with Other Features

- **auto_wrap / auto_fold**: Heading rendering is independent. The heading line is not subject to column folding or cell wrapping.
- **ANSI coloring**: Heading text is emitted as plain text. ANSI decoration is not in scope for this feature.
- **All 9 table styles**: the heading is style-agnostic — it renders the same titled rule regardless of `BorderVariant`.
- **`terminal_width` setting**: Continues to control auto-fit column budget allocation; does not affect heading line width. Heading width is determined by actual rendered table width.

See `invariant/005_heading.md` for no-heading passthrough, width ceiling, and single-line output guarantees.
