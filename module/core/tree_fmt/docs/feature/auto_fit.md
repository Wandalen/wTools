# Auto-Fit Table Rendering

### Purpose

Automatically fit table output within terminal width using two cooperating strategies: cell auto-wrapping (Strategy 2) and column folding (Strategy 1). Both are ON by default. Zero configuration required for standard use.

### Motivation

CLI tables with long path or description columns overflow standard 80/120-column terminals. Before auto-fit, callers had three poor options: truncate (loses information), add a wide column (overflows), or manually pre-process strings. Auto-fit solves this at the formatter level.

### Strategies

#### Strategy 2 — Cell Auto-Wrapping

When a column's content exceeds its allocated width budget, the cell wraps to multiple lines instead of truncating. Cell height grows; column width stays within budget.

```
ID  File                    Lines  Path
--  ----------------------  -----  ---------------------------
b1  governance.rulebook.md  120    /home/user1/pro/genai/
                                   governance/governance.
                                   rulebook.md
```

Cell wrapping reuses the existing multiline cell rendering pipeline. The `WrapFormatter` handles word-boundary splitting.

#### Strategy 1 — Column Folding

When total row width still exceeds terminal after wrapping, overflow columns fold to continuation lines below the row. Continuation lines use labeled format by default.

```
ID  File                    Lines  Rules
--  ----------------------  -----  -----
b1  governance.rulebook.md  120    23
    Path: /home/user1/pro/genai/governance/governance.rulebook.md
```

Column folding reuses the sub-row detail line infrastructure (see `table_formatting.md § Sub-Row Detail Lines`).

#### Combination

Both strategies compose: primary columns have budgeted widths with wrapping; overflow columns fold to continuation lines where folded values can themselves wrap.

```
ID  File                    Lines  Rules
--  ----------------------  -----  -----
b1  governance.rulebook.md  120    23
    Path: /home/user1/pro/genai/governance/
          governance.rulebook.md
```

### Default Rendering Pipeline

When `auto_wrap` and `auto_fold` are both `true` (default):

1. Measure terminal width (auto-detect or use `terminal_width` override; fallback: 120)
2. Compute natural width of each column from content
3. If `sum(natural_widths) + separators ≤ terminal_width` — render normally (no intervention)
4. Classify columns via `column_flex` — `Fixed` columns keep natural width, `Flex` columns share remaining budget
5. Strategy 2: wrap flex cells that exceed their budget (cell height grows)
6. If total still exceeds terminal — Strategy 1: fold overflow columns to continuation lines
7. Render combined result

### Column Classification

Each column is classified as `Fixed` or `Flex`:

| Classification | Behavior | Heuristic (when auto-classified) |
|---------------|----------|----------------------------------|
| `Fixed` | Keeps natural width; never wrapped or folded | Max cell width ≤ 12 display chars |
| `Flex` | Shrinks to budget; content wraps if needed | Max cell width > 12 display chars |

Auto-classification applies when `column_flex` is empty (default). Callers can override with explicit `ColumnFlex` assignments per column.

### Configuration

All fields have sensible defaults — auto-fit works without any configuration.

| Field | Type | Default | Behavior |
|-------|------|---------|----------|
| `terminal_width` | `Option<usize>` | `None` (auto-detect) | Target width for budget allocation |
| `auto_wrap` | `bool` | `true` | Enable Strategy 2 (cell wrapping at budget) |
| `auto_fold` | `bool` | `true` | Enable Strategy 1 (column folding) |
| `column_flex` | `Vec<ColumnFlex>` | `vec![]` (auto-classify) | Per-column flex classification |
| `fold_style` | `FoldStyle` | `Labeled` | Continuation line format |
| `fold_indent` | `String` | `"    "` (4 spaces) | Indent prefix for continuation lines |

#### Disabling Auto-Fit

```rust
// Disable both (pre-auto-fit behavior: unlimited width)
let config = TableConfig::plain()
  .auto_wrap( false )
  .auto_fold( false );

// Disable only folding (wraps cells but never folds columns)
let config = TableConfig::plain()
  .auto_fold( false );

// Disable only wrapping (folds columns but truncates, does not wrap)
let config = TableConfig::plain()
  .auto_wrap( false );
```

### Progressive Degradation

| Condition | Strategy 2 | Strategy 1 | Result |
|-----------|-----------|-----------|--------|
| Fits naturally | not needed | not needed | Normal render |
| Tight but flex columns absorb | wraps flex cells | not needed | Taller rows |
| Still overflows after wrapping | wraps remaining | folds overflow cols | Continuation lines with wrapped values |
| Both disabled | — | — | Unlimited width (pre-auto-fit behavior) |

### Interaction with Existing Features

- **Column truncation** (`max_column_width`): When `auto_wrap` is true and `ColumnOverflow::Wrap` applies, wrapping takes precedence over truncation for flex columns. Fixed columns and explicit `ColumnOverflow::Truncate` still truncate.
- **Multiline cells**: Auto-wrapped cells produce multiline output via the same pipeline as manual `\n` cells.
- **Sub-row detail lines**: Detail lines are emitted after all row content lines (including wrapped lines) and after any folded continuation lines.
- **ANSI coloring**: Wrapped and folded lines respect the per-line color/reset algorithm (no ANSI bleed).
- **CSV/TSV**: Auto-fit is automatically disabled for `csv()` and `tsv()` presets (data formats must not wrap or fold).

### See Also

- `table_formatting.md` — base table features (multiline cells, truncation, coloring, sub-rows)
- `../api/config_types.md § TableConfig` — field reference and builder API
- `word_wrap.md` — underlying WrapFormatter used by Strategy 2
