# Algorithm: CLI Help Alignment

### Scope

- **Purpose**: Detect section headers and align descriptions to the longest key for CLI help text output.
- **Responsibility**: Documents the two-pass scan/render algorithm used by the CliHelp text variant.
- **In Scope**: Section header detection, key-width measurement, description alignment, indentation, blank line insertion.
- **Out of Scope**: Other text variants (see `../variant/028_text_bullets.md` and siblings), table column alignment (see `003_tree_column_alignment.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/text.rs` | TextFormatter CliHelp variant implementation |
| test | `tests/text.rs` | Text output tests including CliHelp |
| doc | `../variant/033_text_cli_help.md` | Variant attribute descriptor for CliHelp |

### Abstract

A two-pass algorithm for rendering CLI help text with automatic section detection and description alignment. Pass 1 scans all rows to identify section headers and determine the maximum key width. Pass 2 formats output with correct indentation, aligns all descriptions to a uniform column, and inserts blank lines between sections.

### Input Data Conventions

The CliHelp variant expects tabular input with at most two columns following these row conventions:

- **Section header**: First column is all-uppercase text (optionally with whitespace and underscores); second column is empty. Rendered with a colon suffix and no indentation.
- **Key-description pair**: Both columns populated — first is the key or term, second is the description. Description is aligned to the column determined in Pass 1.
- **Simple line**: Only first column populated. Rendered as an indented line with no alignment applied.

### Algorithm

Two-pass approach: measure all key widths first, then render with alignment.

#### Pass 1 — Scan

Identify section headers and measure the maximum key width across all non-header rows.

1. Iterate all rows; classify each as section header, key-description pair, or simple line.
2. For each key-description pair, record the display width of the key column.
3. Compute `max_key_width` as the maximum key width observed.

#### Pass 2 — Render

Format each row using the classification and `max_key_width` from Pass 1.

1. **Section header**: Emit header text with colon suffix and no indentation; insert a blank line before the header (except at the start of output).
2. **Key-description pair**: Emit indentation + key + padding to `max_key_width + gap` + description.
3. **Simple line**: Emit indentation + first column only.

#### Key Properties

- **Description column** is at position `indent + max_key_width + gap` where gap defaults to 2 spaces.
- **Blank lines** are automatically inserted before each section header to visually separate sections.
- **Indentation depth** is configurable; defaults to 2 spaces.
- **Mixed content** — section headers, aligned pairs, and simple lines — may appear in any order.

### Complexity

- Time: O(rows) — two linear passes over the row list.
- Space: O(1) extra — only `max_key_width` is retained between passes.

### Interaction with Other Features

| Feature | Interaction |
|---------|-------------|
| Configurable indentation | Indentation depth applied uniformly to all non-header rows |
| ANSI color support | Color codes applied per row after alignment; visual width excludes escape sequences |
| Section detection | Based on column content pattern, not explicit metadata |
