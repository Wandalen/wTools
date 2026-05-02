# Algorithm Spec: Budget Allocation

## Source
`docs/algorithm/004_budget_allocation.md`

## Test Implementation
`tests/auto_wrap_test.rs` (T01–T05, T14–T15)

## Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | all-flex columns fit within terminal — natural widths | ✅ |
| AC-2 | single flex column exceeds terminal — budget capped | ✅ |
| AC-3 | all-Fixed columns fit within terminal — exact specified widths | ✅ |
| AC-4 | all-Fixed columns sum exceeds terminal — graceful overflow | ✅ |
| AC-5 | mixed fixed + flex: flex column gets remaining budget | ✅ |

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

### AC-3: all-Fixed columns fit within terminal — exact specified widths

- **Given:** Three columns forced to `ColumnFlex::Fixed` with short cell content
  (each cell ≤ 12 visible chars); terminal width larger than total natural width.
- **When:** Rendered.
- **Then:** No budget redistribution occurs; each column retains its natural content
  width exactly; no wrapping is applied; one physical line per data row.

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
