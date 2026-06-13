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
| [`tests/table_caption_test.rs`](../../tests/table_caption_test.rs) | Test implementation — FC-1 through FC-6 |

### Features

| File | Relationship |
|------|-------------|
| [001_table_formatting.md](001_table_formatting.md) | Feature extended by table caption |

### Algorithms

| File | Relationship |
|------|-------------|
| [007_caption_rendering.md](../algorithm/007_caption_rendering.md) | Algorithm implementing caption line assembly |

### Invariants

| File | Relationship |
|------|-------------|
| [005_caption.md](../invariant/005_caption.md) | Behavioral guarantees for caption rendering |

### APIs

| File | Relationship |
|------|-------------|
| [003_config_types.md](../api/003_config_types.md) | `TableCaption` and `TableConfig::caption()` API surface |

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

#### Construction

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

Terminal width is resolved via the same four-tier chain used by auto-fit: `TableConfig::terminal_width` override (Tier 0) → `$COLUMNS` env var (Tier 1) → `terminal_size` crate, feature-gated (Tier 2) → fallback 120 (Tier 3).

#### Interaction with Other Features

- **auto_wrap / auto_fold**: Caption rendering is independent. The caption line is not subject to column folding or cell wrapping.
- **ANSI coloring**: Caption text is emitted as plain text. ANSI decoration is not in scope for this feature.
- **All 9 table styles**: `caption` is style-agnostic — it renders the same titled rule regardless of `BorderVariant`.
- **`auto_wrap(false)`**: Terminal width is still used for the trailing rule calculation; caption always fills to the resolved terminal width.

See `invariant/005_caption.md` for no-caption passthrough, width ceiling, and single-line output guarantees.
