# Algorithm: Column Fold Detection

### Scope

- **Purpose**: Determine which columns exceed the terminal width budget and must fold to continuation lines below the row.
- **Responsibility**: Documents the fold point detection and continuation line rendering algorithm.
- **In Scope**: Cumulative width scan, fold point identification, primary vs overflow partitioning, continuation line format selection.
- **Out of Scope**: Budget allocation for primary columns (see `algorithm/004_budget_allocation.md`), cell wrapping (see `algorithm/002_word_wrapping.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/table/mod.rs` | `determine_fold_point`, `render_fold_continuation`, `should_auto_fold` |
| source | `src/config.rs` | `FoldStyle` enum, `auto_fold`/`fold_style`/`fold_indent` fields |
| test | `tests/auto_fold_test.rs` | Column folding test scenarios (24 tests) |
| doc | `../feature/005_auto_fit.md` | Auto-fit feature overview — Strategy 1 |
| doc | `../invariant/004_column_fold_invariants.md` | Fold behavioral guarantees |
| task | `../../task/completed/020_column_folding_with_auto_fold.md` | Implementation task (completed) |

### Abstract

An O(C) algorithm that determines which columns must fold to continuation lines when total row width exceeds terminal width. Columns are scanned left-to-right accumulating widths; the first column that pushes the cumulative total past the terminal limit becomes the fold point. Primary columns (before the fold point) render in the table row; overflow columns render as labeled continuation lines below it.

### Trigger Condition

Fold detection runs when all three conditions hold:

1. `auto_fold` is `true` (default)
2. Total row width exceeds terminal width after budget allocation and wrapping (Strategy 2)
3. Style is not CSV/TSV (data formats never fold)

Header row is exempt — headers always render all columns inline, never fold.

### Algorithm

**Step 1 — Compute cumulative widths**

After budget allocation (algorithm/004), compute cumulative row width left-to-right:

```
cumulative = 0
for each column i in 0..column_count:
  cumulative += budget_width[i] + separator_width
  if cumulative > terminal_width:
    fold_point = max(i, 1)   // clamp: first column always stays primary
    break
```

If no column exceeds terminal, fold_point = column_count (no folding needed).

**Step 2 — Partition columns**

```
primary_columns  = columns[ 0 .. fold_point ]
overflow_columns = columns[ fold_point .. column_count ]
```

Primary columns render as a normal table row. Overflow columns render as continuation lines.

**Step 3 — Render primary columns**

Primary columns use the standard table rendering pipeline (borders, separators, alignment, multiline cells). Column widths are their budget widths from Step 1.

**Step 4 — Render continuation lines**

For each overflow column, emit a continuation line using the configured `FoldStyle`:

```
match fold_style:
  Labeled:
    for each overflow column:
      emit: fold_indent + header_name + ": " + cell_value
  Bare:
    emit: fold_indent + join( cell_values, "  " )
  Stacked:
    for each overflow column:
      emit: fold_indent + header_name + ": " + cell_value
```

Labeled and Stacked produce one line per overflow column. Bare joins all values on a single line; if that line exceeds the terminal budget, word wrapping is applied (same as Step 5 below).

**Step 5 — Wrap folded values**

If a continuation line exceeds `terminal_width - visual_len(fold_indent)`, apply word wrapping to the value portion. This composes Strategy 1 (folding) with Strategy 2 (wrapping).

### Rendering Order per Row

```
1. Primary column lines (may span multiple lines if cells wrap)
2. Continuation lines for overflow columns
3. Sub-row detail lines (if any)
```

### Edge Cases

| Case | Behavior |
|------|----------|
| Single overflow column | One continuation line |
| All columns overflow except first | Only first column in table; rest fold |
| Very narrow terminal (< first column width) | First column renders at natural width; all others fold |
| Mixed rows (some fit, some overflow) | Fold point computed per-row; only overflowing rows have continuation lines |

### Complexity

- Time: O(C) per row where C = column count — single left-to-right scan
- Space: O(C) for the partition vectors
