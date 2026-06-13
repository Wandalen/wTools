# Feature: Table Caption

### Scope

- **Purpose**: Allow callers to attach a captioned title line above a table — a titled rule that spans the terminal width — carrying a title and optional caption fields.
- **Responsibility**: Document the `TableCaption` type, its API, rendering algorithm, and integration with `TableConfig`.
- **In Scope**: `TableCaption` struct, caption fields, field separator, rule character, lead width, `TableConfig::caption()` builder, rendering position and terminal-width behavior.
- **Out of Scope**: Table body rendering (see `001_table_formatting.md`); auto-fit algorithm (see `005_auto_fit.md`); terminal width detection (see `005_auto_fit.md § Terminal Width Detection`).

### Sources

| File | Relationship |
|------|-------------|
| [`src/config.rs`](../../src/config.rs) | `TableCaption` struct definition and `TableConfig::caption()` builder |
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | Caption rendering in `format_internal()` |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/docs/feature/007_table_caption.md`](../../tests/docs/feature/007_table_caption.md) | Test spec — FC-N cases |

### Vocabulary

| Term | Definition |
|------|-----------|
| **caption** | The full titled-rule line printed above the table header |
| **title** | The primary text in the caption — e.g., `"Needs Review"` |
| **caption field** | An additional metadata item appended to the title with the field separator — e.g., `"28 PRs"` |
| **field separator** | The character placed between caption fields — default `·` (U+00B7 MIDDLE DOT) |
| **rule character** | The horizontal fill character used for the rule sections — default `─` (U+2500 BOX DRAWINGS LIGHT HORIZONTAL) |
| **lead width** | The number of rule characters emitted before the title text — default 3 |
| **titled rule** | The visual format: `─── Title · Field1 · Field2 ──────────────...` filling the terminal width |

### Design

#### Motivation

CLI tools that display filtered or summarised tables benefit from a concise header line that names the data set and shows top-level aggregates alongside:

```
─── Needs Review · 28 PRs · 15 repos ─────────────────────────────────────────────
Repository                       PR#  State  Title
-------------------------------  ---  -----  ----------------------------------
Wandalen/cgtools                 129  open   NEED REVIEW : 3d line dash
```

The line carries the title `"Needs Review"` and two caption fields `"28 PRs"` and `"15 repos"`. The rule chars fill the remainder of the terminal width, creating a visually prominent section separator.

#### API

```rust
// Type definition (src/config.rs)
pub struct TableCaption {
    title  : String,
    fields : Vec<String>,
    // Formatting is fixed: field_sep = '·' (U+00B7), rule_char = '─' (U+2500), lead_width = 3
}

impl TableCaption {
    pub fn new(title: impl Into<String>) -> Self
    pub fn field(mut self, f: impl Into<String>) -> Self
}

// Integration on TableConfig (src/config.rs)
impl TableConfig {
    pub fn caption(mut self, caption: TableCaption) -> Self
}
```

**Minimal usage:**
```rust
let config = TableConfig::plain()
    .caption(TableCaption::new("Needs Review"));
// ─── Needs Review ──────────────────────────────────────────────────────────────
// Name   Age
// -----  ---
// Alice  30
```

**With caption fields:**
```rust
let config = TableConfig::plain()
    .caption(
        TableCaption::new("Needs Review")
            .field("28 PRs")
            .field("15 repos")
    );
// ─── Needs Review · 28 PRs · 15 repos ─────────────────────────────────────────
```

#### Rendering Algorithm

The caption is rendered immediately before the table top border (or header row when no top border exists). The rendering steps are:

1. Build the content string: `title` followed by `" {field_sep} {field}"` for each caption field.
2. Build the lead: `rule_char` × `lead_width` + ` ` (e.g., `"─── "`).
3. Build the trailing rule: compute `trail_width = terminal_width - lead_width - 1 - content.chars().count() - 1` (subtracting lead chars, the space after lead, content char-count, and one trailing space). Use `.chars().count()`, not `.len()` — `·` (U+00B7) and `─` (U+2500) are multi-byte in UTF-8. Clamp `trail_width` to 0 if negative.
4. Emit: `lead + content + " " + rule_char × trail_width + "\n"`.

Terminal width is resolved via the same three-tier chain used by auto-fit: `TableConfig::terminal_width` override → `terminal_size` crate (feature-gated) → fallback 120.

#### Interaction with Other Features

- **auto_wrap / auto_fold**: Caption rendering is independent. The caption line is not subject to column folding or cell wrapping.
- **ANSI coloring**: Caption text is emitted as plain text. ANSI decoration is not in scope for this feature.
- **All 9 table styles**: `caption` is style-agnostic — it renders the same titled rule regardless of `BorderVariant`.
- **`auto_wrap(false)`**: Terminal width is still used for the trailing rule calculation; caption always fills to the resolved terminal width.

#### Invariants

- A `None` caption (default) emits no additional output — zero impact on existing tables.
- The caption line never exceeds the resolved terminal width (trailing rule is clamped to 0 if content already exceeds terminal width).
- The caption appears on exactly one output line ending with `\n`.

### Acceptance Criteria

| ID | Criterion |
|----|-----------|
| AC-1 | `TableConfig::plain().caption(TableCaption::new("T"))` renders a line starting with `"─── T "` before the first table line |
| AC-2 | The caption line total length equals the resolved terminal width (or content length + lead + 1 trailing space when content exceeds terminal width) |
| AC-3 | Each `.field("F")` call appends ` · F` to the content string |
| AC-4 | A table with no caption (`TableConfig::plain()`) produces output identical to current behavior — no regression |
| AC-5 | Caption renders above the top border for `grid` and `unicode_box`, and above the header row for `plain`, `bordered`, and `markdown` |
