# Algorithm: Budget Allocation

### Scope

- **Purpose**: Drive test coverage for the flex budget allocation algorithm.
- **Responsibility**: Documents test cases for the flex budget allocation algorithm in `docs/algorithm/004_budget_allocation.md`.
- **In Scope**: All-flex natural width, single flex cap, all-Fixed graceful overflow, mixed fixed+flex allocation, CSV/TSV bypass, remainder distribution, budget floor clamping, 12-char threshold boundary.
- **Out of Scope**: Column fold detection (see `algorithm/005`); terminal width auto-detection (see `feature/005`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | all-flex columns fit within terminal — natural widths | ✅ |
| AC-2 | single flex column exceeds terminal — budget capped | ✅ |
| AC-3 | budget allocation not triggered when all columns fit within terminal | ✅ |
| AC-4 | all-Fixed columns sum exceeds terminal — graceful overflow | ✅ |
| AC-5 | mixed fixed + flex: flex column gets remaining budget | ✅ |
| AC-6 | CSV/TSV preset bypasses budget allocation | ✅ |
| AC-7 | remainder characters distributed to leftmost flex columns | ✅ |
| AC-8 | flex budget floored at minimum when terminal too narrow | ✅ |
| AC-9 | 12-character threshold boundary — column at exactly 12 chars uses Fixed | ✅ |
| AC-10 | overhead exceeds terminal width — all flex columns clamped to floor | ✅ |

---

### AC-1: all-flex columns fit within terminal — natural widths

- **Given:** A table with two flex columns (each cell < 12 chars, or explicit
  `ColumnFlex::Flex`); terminal width is larger than the total natural table width.
- **When:** Rendered with `auto_wrap=true`.
- **Then:** Budget allocation assigns each flex column its full natural content
  width; no cell is wrapped; one physical output line per data row.

---

### AC-2: single flex column exceeds terminal — budget capped

- **Given:** A table with one flex column whose cell content exceeds the terminal
  width; terminal set to 80 via `TableConfig::plain().terminal_width(Some(80))`.
- **When:** Rendered with `auto_wrap=true`.
- **Then:** The flex column's budget is capped at `terminal_width - overhead`; the
  cell content wraps to multiple sub-lines within that budget; the total visual
  width of each physical output line does not exceed the terminal width.

---

### AC-3: budget allocation not triggered when all columns fit within terminal

- **Given:** Three columns forced to `ColumnFlex::Fixed` with short cell content
  (each cell ≤ 12 visible chars); terminal width larger than total natural width.
- **When:** Rendered.
- **Then:** Budget allocation trigger condition (total ≤ terminal) is not met;
  no budget redistribution occurs; all columns retain their natural content
  widths exactly; no wrapping is applied; one physical line per data row.

---

### AC-4: all-Fixed columns sum exceeds terminal — graceful overflow

- **Given:** Three columns forced to `ColumnFlex::Fixed` via
  `TableConfig::plain().column_flex(vec![ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed])`;
  each cell contains 44+ characters so total width far exceeds `terminal_width(Some(40))`.
- **When:** Rendered.
- **Then:** No panic occurs; output is non-empty and contains all column content;
  the output width exceeds the configured terminal width (Fixed columns are never
  truncated by budget allocation); no wrapping is applied to Fixed columns.

---

### AC-5: mixed fixed + flex: flex column gets remaining budget

- **Given:** A table with two `ColumnFlex::Fixed` columns (short content) and one
  `ColumnFlex::Flex` column with long content; terminal width set explicitly.
- **When:** Rendered with `auto_wrap=true`.
- **Then:** The two fixed columns retain their natural widths; the flex column
  budget equals `terminal_width - sum(fixed_widths) - overhead`; the flex cell
  wraps within that budget.

---

### AC-6: CSV/TSV preset bypasses budget allocation regardless of content width

- **Given:** A table using `TableConfig::csv()` whose natural column widths exceed
  the terminal width; `auto_wrap=true`.
- **When:** Rendered.
- **Then:** No budget redistribution occurs; cell content is not wrapped; the
  output is well-formed CSV with natural content lengths preserved. (Source:
  `docs/algorithm/004_budget_allocation.md` — "Skipped entirely for CSV/TSV presets".)

---

### AC-7: remainder characters distributed to leftmost flex columns

- **Given:** A table with three `ColumnFlex::Flex` columns; terminal width is
  such that `budget % 3 == 1` (one remainder character after integer division).
- **When:** Rendered with `auto_wrap=true`.
- **Then:** The leftmost flex column receives one extra character compared to the
  middle and right flex columns; all three flex column budgets differ by at most 1;
  the column with the extra character is the leftmost among the three flex columns.

---

### AC-8: flex budget floored at minimum when terminal too narrow

- **Given:** A table with one `ColumnFlex::Fixed` column and one `ColumnFlex::Flex`
  column; terminal width smaller than the fixed column width plus separator overhead;
  the computed flex budget is zero or negative.
- **When:** Rendered with `auto_wrap=true`.
- **Then:** No panic occurs; the flex column receives a budget of at least 1
  character (Step 6 clamp: `max(base + extra, max(min_column_width, 1))`); output
  is non-empty.

---

### AC-9: 12-character threshold boundary — column at exactly 12 chars uses Fixed

- **Given:** A table where one column's maximum cell content is exactly 12 visible
  characters (at the threshold boundary for the auto-flex heuristic).
- **When:** Column flex classification is applied.
- **Then:** The column is treated as `ColumnFlex::Fixed` (not Flex); it retains
  its natural 12-character width and is excluded from budget redistribution.
  (Source: `docs/algorithm/004_budget_allocation.md` — "≤ 12 chars → Fixed".)

---

### AC-10: overhead exceeds terminal width — all flex columns clamped to floor

- **Given:** A table where the fixed column widths plus separator overhead already
  exceed the terminal width before any flex column budget is computed.
- **When:** Rendered with `auto_wrap=true`.
- **Then:** No panic occurs; each flex column receives the floor budget (minimum
  of 1 or `min_column_width`); the formatter does not produce negative budgets;
  output is non-empty even when total width necessarily exceeds terminal.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/algorithm/004_budget_allocation.md`](../../../docs/algorithm/004_budget_allocation.md) | Source algorithm spec — trigger condition, flex classification, budget steps |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/auto_wrap_test.rs`](../../auto_wrap_test.rs) | Algorithm test implementation (T01–T05, T14–T15) |
