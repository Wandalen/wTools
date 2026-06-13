# Fix cell padding — apply `inner_padding` symmetrically to every cell, not just outer edges

## Execution State

- **Executor Type:** any
- **Actor:** dev
- **Claimed At:** 2026-06-13
- **Status:** ✅ (Complete)

## Goal

Fix the rendering path so that `inner_padding` spaces are emitted before AND after every cell in all border-using table styles (`bordered`, `grid`, `markdown`, `unicode_box`), eliminating the defect where `inner_padding = 1` in these presets produces spaces only at the outer table edges while inter-cell separators have no surrounding spaces.
(Motivated: `bordered` with `inner_padding=1` renders `| #|PID  |` — no space before the `│`/`|` separator between cells, despite the explicit `inner_padding = 1` preset config — making output visually cramped and non-standard; Observable: `TableFormatter::with_config(TableConfig::bordered()).format(&view)` for a two-column table produces `| col1 | col2 |` with 1 space on each side of every cell, and all separator lines widen symmetrically to match, e.g. `+------+-------+`; Scoped: changes limited to `src/formatters/table/rendering.rs` — specifically `format_single_line_row`, `format_multiline_row`, `format_ascii_horizontal_rule`, `format_unicode_horizontal_rule`, and the Dash/AsciiGrid inline branches of `format_header_separator` — plus exact-string updates in affected test files; no changes to `src/config.rs` or any other formatter; Testable: `w3 .test level::3` passes clean after changes and `assert!(output.contains("| col1 | col2 |"))` holds for bordered output)

## In Scope

All paths relative to the crate root (`module/core/data_fmt/`).

**Source — `src/formatters/table/rendering.rs`:**
- `format_single_line_row` — remove `idx == 0` guard on leading-padding block; remove `idx == cells.len() - 1` guard on trailing-padding block so `inner_padding` spaces fire before and after every cell
- `format_multiline_row` — same guard removals in the per-line rendering loop (lines ~167-195)
- `format_ascii_horizontal_rule` — remove `idx == 0` / `idx == widths.len() - 1` guards so fill chars are emitted before and after every column's dashes (required for geometric alignment with the fixed data rows)
- `format_unicode_horizontal_rule` — same guard removals as above
- `format_header_separator` Dash variant (inline loop, lines ~250-270) — same guard removals
- `format_header_separator` AsciiGrid variant (inline loop, lines ~273-298) — same guard removals (this branch is NOT delegated to `format_ascii_horizontal_rule`; it has its own guards)

Note: `format_top_border_if_needed`, `format_bottom_border_if_needed`, and `format_inter_row_sep_if_needed` delegate to the two horizontal-rule helpers above and require no direct changes.

**Tests:**
- `tests/table_styles_presets.rs` — update hardcoded expected strings that break; add behavioral assertion verifying `| col1 | col2 |` pattern
- `tests/table_rendering_borders.rs` — update hardcoded expected strings that break

## Out of Scope

- `src/config.rs` — `inner_padding` (default 1) and `outer_padding` (default true) already exist with correct values for the 4 affected styles; no new fields needed
- `plain`, `minimal`, `compact`, `csv`, `tsv` styles — `inner_padding = 0` or `outer_padding = false`; guard-removal has zero effect
- All other formatters (expanded, tree, JSON, HTML, SQL, etc.)
- Public API surface — no new methods, no removed methods, no type changes

## Desired Output

Using demo data: columns `#`(w=1), `PID`(w=5), `CPU%`(w=4), `RAM`(w=4), `Path`(w=27); `inner_padding=1`.

### unicode_box

**Before:**
```
┌──┬─────┬────┬────┬─────────────────────────────┐
│ #│PID  │CPU%│RAM │Path                         │
├──┼─────┼────┼────┼─────────────────────────────┤
│ 1│12345│0.5%│379M│$PRO/genai/pr_review         │
└──┴─────┴────┴────┴─────────────────────────────┘
```

**After:**
```
┌───┬───────┬──────┬──────┬─────────────────────────────┐
│ # │ PID   │ CPU% │ RAM  │ Path                        │
├───┼───────┼──────┼──────┼─────────────────────────────┤
│ 1 │ 12345 │ 0.5% │ 379M │ $PRO/genai/pr_review        │
└───┴───────┴──────┴──────┴─────────────────────────────┘
```

### bordered

**Before:**
```
| #|PID  |CPU%|RAM |Path                         |
+--+-----+----+----+-----------------------------+
| 1|12345|0.5%|379M|$PRO/genai/pr_review         |
```

**After:**
```
| # | PID   | CPU% | RAM  | Path                        |
+---+-------+------+------+-----------------------------+
| 1 | 12345 | 0.5% | 379M | $PRO/genai/pr_review        |
```

### grid

**Before:**
```
+--+-----+----+----+-----------------------------+
| #|PID  |CPU%|RAM |Path                         |
+--+-----+----+----+-----------------------------+
| 1|12345|0.5%|379M|$PRO/genai/pr_review         |
+--+-----+----+----+-----------------------------+
```

**After:**
```
+---+-------+------+------+-----------------------------+
| # | PID   | CPU% | RAM  | Path                        |
+---+-------+------+------+-----------------------------+
| 1 | 12345 | 0.5% | 379M | $PRO/genai/pr_review        |
+---+-------+------+------+-----------------------------+
```

### markdown

**Before:**
```
| #|PID  |CPU%|RAM |Path                         |
|--|-----|----|----|-----------------------------|
| 1|12345|0.5%|379M|$PRO/genai/pr_review         |
```

**After:**
```
| # | PID   | CPU% | RAM  | Path                        |
|---|-------|------|------|------------------------------|
| 1 | 12345 | 0.5% | 379M | $PRO/genai/pr_review        |
```

## Work Procedure

1. Open `src/formatters/table/rendering.rs`.
2. In `format_single_line_row` (lines ~44-92):
   - Change `if idx == 0 && self.config.has_outer_padding() && should_pad` → `if self.config.has_outer_padding() && should_pad` (leading padding before every cell)
   - Change `if idx == cells.len() - 1 && self.config.has_outer_padding() && should_pad` → `if self.config.has_outer_padding() && should_pad` (trailing padding after every cell)
   - Move the trailing-padding block to execute BEFORE `append_column_separator` so order is: content → trailing pad → separator → (next cell's leading pad)
3. In `format_multiline_row` (lines ~168-195): apply the same two guard removals to the per-line rendering loop.
4. In `format_ascii_horizontal_rule` (lines ~340-358): remove `idx == 0` guard on leading fill-chars; remove `idx == widths.len() - 1` guard on trailing fill-chars. This widens each column's dashes to `padding + width + padding`, matching the fixed data-row cell widths.
5. In `format_unicode_horizontal_rule` (lines ~373-391): same guard removals as step 4.
6. In `format_header_separator`, Dash branch (lines ~250-270): same guard removals as step 4.
7. In `format_header_separator`, AsciiGrid branch (lines ~273-298): same guard removals (this branch does NOT delegate to `format_ascii_horizontal_rule`).
8. Run `w3 .test level::3` to surface all exact-string assertion failures.
9. Update the failing hardcoded expected strings in `tests/table_styles_presets.rs` and `tests/table_rendering_borders.rs` to match the new wider output.
10. Add a new `test_cell_padding_all_separators` test in `tests/table_styles_presets.rs` asserting all 4 affected styles produce `| col1 | col2 |` pattern.
11. Run `w3 .test level::3` again to confirm clean pass.

## Test Matrix

| Style | Before (broken) | After (desired) | Assertion |
|-------|-----------------|-----------------|-----------|
| `bordered` | `\| #\|PID` | `\| # \| PID` | `output.contains("| col1 | col2 |")` |
| `grid` | `\| #\|PID` | `\| # \| PID` | same pattern |
| `markdown` | `\| #\|PID` | `\| # \| PID` | same pattern |
| `unicode_box` | `│ #│PID` | `│ # │ PID` | `output.contains("│ col1 │")` |
| `plain` | unchanged | unchanged | no assertion change needed |
| `csv` / `tsv` | unchanged | unchanged | no assertion change needed |

## Closes

null

## Verification Findings

Two dimensions failed the Verification Gate. Findings and resolution path follow.

**Finding 1 — MOST Goal Scoped (from MOST Goal Quality agent):**
"'5 functions in rendering.rs' is unverified and unnamed in the MOST goal sentence."

Resolution: The 5 specific functions are named in the In Scope section above. The MOST goal sentence references the file-level scope; the In Scope section provides function-level detail. This is an acceptable split — the MOST goal remains bounded and the function names are traceable. No rework required; this finding is resolved by the task structure.

**Finding 2 — Implementation Readiness agent:**
"Removing guards from `format_ascii_horizontal_rule` / `format_unicode_horizontal_rule` would break geometry."

Resolution: The agent's analysis assumed the horizontal-rule changes would be made in isolation. They must be made TOGETHER with the data-row cell-padding changes. After both changes, geometric consistency is preserved: data rows emit `padding + content + padding` between separators; horizontal rules emit `fill * padding + fill * width + fill * padding` between junctions — both use the same cell budget. Doing only one half would break alignment; doing both together maintains it. Work Procedure steps 2-7 cover all sites atomically. This finding is a misreading of the change scope, not a real defect. No rework required.

**Disposition:** Both findings are resolved by task content as written. Re-trigger Verification Gate to promote to 🎯 (Verified) if required by project policy; otherwise executor may proceed with Work Procedure as-is.

## Verification Record

- **Date:** 2026-06-13
- **Method:** MAAV — two independent Agent subagents (conformance + adversarial)
- **Test result:** 605/605 tests pass; 4/4 jobs clean (nextest, workspace nextest, doc tests, clippy)
- **Conformance:** `test_cell_padding_all_separators` exists in `tests/table_styles_presets.rs`; exercises `bordered`, `grid`, `markdown`, `unicode_box`; all pass
- **Adversarial:** no regression found; `| col1 | col2 |` pattern confirmed in all 4 affected styles; `plain`/`csv`/`tsv` unchanged
- **Verdict:** ✅ Complete
