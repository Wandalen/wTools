# Algorithm: Column Fold Detection

### Scope

- **Purpose**: Determine which columns exceed the terminal width budget and must fold to continuation lines below the row.
- **Responsibility**: Documents the fold point detection and continuation line rendering algorithm.
- **In Scope**: Cumulative width scan, fold point identification, primary vs overflow partitioning, continuation line format selection.
- **Out of Scope**: Budget allocation for primary columns (see `algorithm/004_budget_allocation.md`), cell wrapping (see `algorithm/002_word_wrapping.md`).

### Sources

| File | Relationship |
|------|--------------|
| `src/formatters/table/auto_fit.rs` | `determine_fold_point`, `render_fold_continuation`, `should_auto_fold` |
| `src/config/table_enums.rs` | `FoldStyle` enum |
| `src/config/table_config.rs` | `auto_fold`/`fold_style`/`fold_indent` fields |

### Tests

| File | Relationship |
|------|--------------|
| `tests/auto_fold_test.rs` | Column folding test scenarios T01–T25 (25 tests) |
| `tests/auto_fold_acceptance_test.rs` | Fold acceptance criteria CF AC-6–AC-11 plus 1 additional invariant test (7 tests) |

### Abstract

An O(C) algorithm that determines which columns must fold to continuation lines when total row width exceeds terminal width. Columns are scanned left-to-right accumulating widths; the first column that pushes the cumulative total past the terminal limit becomes the fold point. Primary columns (before the fold point) render in the table row; overflow columns render as labeled continuation lines below it.

### Trigger Condition

Fold detection runs when all four conditions hold:

1. `auto_fold` is `true` (default)
2. `auto_wrap` is also `true` (default) — fold detection is gated on wrapping being enabled, same as budget allocation (algorithm/004)
3. No explicit `column_widths` override is set
4. Style is not CSV/TSV (data formats never fold)

The fold point itself (Step 1) is computed unconditionally once the above hold; when no column's cumulative width exceeds the terminal budget, `determine_fold_point` returns `column_count` and nothing actually folds.

Header row is folded identically to data rows — it is sliced at `fold_point`, so only primary column headers render inline. Overflow column headers are not emitted as their own continuation row; instead their names are used as labels inside each data row's continuation lines (Labeled/Stacked styles).

### Algorithm

**Step 1 — Compute cumulative widths**

After budget allocation (algorithm/004), compute cumulative row width left-to-right:

```
content_so_far = 0
for each column i in 0..column_count:
  content_so_far += budget_width[i]
  sep_total = i × separator_width
  pad_total = if has_outer_padding: cell_inner_padding × 2 × (i + 1) else: 0
  border = if needs_border_pipes: 2 else: 0
  if content_so_far + sep_total + pad_total + border > terminal_width:
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
  Labeled (default):
    pairs = overflow columns with a non-empty value, each formatted "header_name: cell_value"
    emit: fold_indent + join( pairs, "  " )
  Bare:
    values = overflow columns with a non-empty value (no labels)
    emit: fold_indent + join( values, "  " )
  Stacked:
    for each overflow column with a non-empty value:
      emit: fold_indent + header_name + ": " + cell_value
```

Only Stacked produces one line per overflow column. Labeled and Bare each produce a single joined continuation line; if that line exceeds the terminal budget, word wrapping is applied (Step 5), which can still spread it across multiple physical output lines. Overflow columns with an empty value are filtered out before joining/emitting in all three styles.

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
| Mixed rows (some fit, some overflow) | Fold point is computed once for the whole table (not per-row); every row attempts continuation lines, but a row's continuation is empty when all its overflow-column values are empty |

### Complexity

- Time: O(C) per row where C = column count — single left-to-right scan
- Space: O(C) for the partition vectors
