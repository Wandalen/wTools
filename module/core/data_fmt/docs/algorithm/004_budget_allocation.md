# Algorithm: Budget Allocation

### Scope

- **Purpose**: Distribute terminal width among table columns so flex-column cells can auto-wrap at their budgeted boundary.
- **Responsibility**: Documents the column budget allocation algorithm used by the auto-fit pipeline.
- **In Scope**: Column classification, fixed/flex partitioning, budget distribution with remainder, minimum width clamping.
- **Out of Scope**: Cell wrapping within budgets (see `algorithm/002_word_wrapping.md`), fold detection (see `algorithm/005_column_fold_detection.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/table/mod.rs` | `classify_columns`, `compute_column_budgets` |
| source | `src/config.rs` | `ColumnFlex` enum, `terminal_width` field |
| test | `tests/auto_wrap_test.rs` | Budget allocation test scenarios (T02–T05, T14–T15) |
| doc | `../feature/005_auto_fit.md` | Auto-fit feature overview and pipeline |
| doc | `../api/003_config_types.md` | `ColumnFlex`, `TableConfig` field reference |

### Abstract

An O(C) algorithm that distributes available terminal width among table columns so flex columns receive a wrapping budget. Fixed columns (narrow content, max cell width ≤ 12) retain their natural width. Flex columns (wide content) share the remaining budget equally, with remainder characters distributed left-to-right. No flex column is expanded beyond its natural content width.

### Related Tasks

- [`task/019`](../../task/019_cell_auto_wrapping_with_budget_allocation.md) — Implementation task

### Trigger Condition

Budget allocation runs when all three conditions hold:

1. `auto_wrap` is `true` (default)
2. No explicit `column_widths` override is set
3. Total natural row width exceeds the resolved terminal width

Skipped entirely for CSV/TSV presets (data formats must not wrap).

### Algorithm

**Step 1 — Resolve terminal width**

Three-tier fallback (see `feature/005_auto_fit.md § Terminal Width Detection`):
1. Explicit `terminal_width` config → use directly (clamp 0 to 1)
2. `terminal_size` crate query (when feature enabled) → use detected width
3. Hardcoded fallback → 120

**Step 2 — Classify columns**

When `column_flex` is empty (default), auto-classify by heuristic:

```
for each column:
  if max_cell_visual_width ≤ 12:
    classify as Fixed
  else:
    classify as Flex
```

When `column_flex` has explicit entries, use them directly. If shorter than column count, pad with `Flex`.

**Step 3 — Compute overhead**

```
separator_total = separator_visual_width × (column_count - 1)
outer = if has_outer_padding: cell_inner_padding × 2 else: 0
border = if needs_border_pipes: 2 else: 0
overhead = separator_total + outer + border
```

**Step 4 — Sum fixed column widths**

```
fixed_total = sum( natural_width for columns classified as Fixed )
```

**Step 5 — Distribute budget among flex columns**

```
budget = terminal_width - fixed_total - overhead
flex_count = count of Flex columns
base = budget / flex_count        (integer division)
remainder = budget % flex_count   (distribute to leftmost flex columns)
```

**Step 6 — Assign per-column budgets**

```
flex_idx = 0
for each column i:
  if Fixed:
    budget[i] = natural_width[i]
  else:  // Flex
    extra = 1 if flex_idx < remainder else 0
    b = max( base + extra, max( min_column_width, 1 ) )
    budget[i] = min( b, natural_width[i] )  // never expand beyond content
    flex_idx += 1
```

Key behaviors:
- Flex columns are capped at their natural width (never expanded beyond content)
- `min_column_width` floor takes precedence over budget (Step 6 clamp)
- When flex_count is 0, all columns keep natural widths (no budget allocation)
- Remainder columns (leftmost flex) get 1 extra character each

### Complexity

- Time: O(C) where C = column count — single pass after classification
- Space: O(C) for the budget vector
