# Task 008 — Fix Caption Width: Fill to Rendered Table Width

## Execution State

- **State**: ✅ (Completed)
- **ID**: 008
- **Slug**: fix_caption_width
- **Executor**: dev

## MOST Goal

Fix `render_caption_if_present` to fill the caption trailing rule to the actual rendered table display width (`compute_total_row_width(primary_widths)`) instead of the terminal width (`resolve_terminal_width()`), so the caption rule right-edge aligns with the table right-edge for all table configurations.

- **Motivated**: Captions on narrow tables currently overflow the table width by filling to terminal width (120 fallback), creating severe visual misalignment. The rendered table width is the correct fill target — it reflects actual column layout after auto-fit.
- **Observable**: Level 3 (`w3 .test level::3`) passes after FC-3, FT-4, and FT-8 test assertions are updated to verify fill-to-table-width semantics; no regressions in other caption tests.
- **Scoped**: Limited to `src/formatters/table/row_rendering.rs`, `src/formatters/table/mod.rs`, and `tests/table_caption_test.rs` — no other files require change.
- **Testable**: Concrete pass/fail determined by Level 3 output count and specific assertion messages.

## Null Hypothesis

Without this fix, `render_caption_if_present` continues to use `resolve_terminal_width()`, producing caption lines that fill to 120 chars on narrow tables — a persistent visual bug. The fix is a committed need with an observable, testable outcome and zero speculative scope.

## In Scope

- `src/formatters/table/row_rendering.rs` — change `render_caption_if_present` signature to accept `table_width: usize`; replace `self.resolve_terminal_width()` with the parameter
- `src/formatters/table/mod.rs` — update call site in `format_internal()` to pass `self.compute_total_row_width(primary_widths)` as `table_width`
- `tests/table_caption_test.rs` — rewrite FC-3 (verify `chars().count() == actual_table_width`), FT-4 (set up a table whose width exactly equals content length), FT-8 (verify `chars().count() == actual_table_width`)

## Out of Scope

- Renaming `TableCaption` → `Heading` or adding `with_` prefix (see Task 009)
- Adding new examples (see Task 010)
- Changes to `resolve_terminal_width()` or auto-fit column budget behavior

## Work Procedure

1. Run `w3 .test level::3` to establish passing baseline (618 nextest + 74 doc + 0 clippy).
2. Open `src/formatters/table/row_rendering.rs`; change `render_caption_if_present` to accept `table_width: usize`; replace `let tw = self.resolve_terminal_width();` with `let tw = table_width;`.
3. Open `src/formatters/table/mod.rs`; locate the call `self.render_caption_if_present(&mut output)` (just before `format_top_border_if_needed`); change it to `self.render_caption_if_present(&mut output, self.compute_total_row_width(primary_widths))`.
4. Confirm compilation: `RUSTFLAGS="-D warnings" cargo check --all-features`.
5. Rewrite `caption_fills_to_terminal_width_fc3` in `tests/table_caption_test.rs`:
   - Remove the `terminal_width(Some(60))` call (caption no longer fills to terminal width)
   - Compute `expected_width` by calling `compute_total_row_width` logic for the two-column table (Name/Alice, Age/30): `plain` style has no borders, single space separator → width = max(4,5) + 1 + max(3,3) = 5+1+3 = 9
   - Assert `caption_line.chars().count() == 9`
6. Rewrite `caption_content_equals_terminal_width_no_trailing_rule_ft4`:
   - Reuse `two_col_view()` (Name/Alice, Age/30): plain style → `table_width = 9`
   - Choose title with exactly 4 chars so `3 + 1 + 4 + 1 == 9`; use `"Abcd"` — change `TableCaption::new("Caption Exactly")` to `TableCaption::new("Abcd")`
   - Remove the `terminal_width(Some(20))` call; the table width (9) now governs caption width
   - Assert `!caption_line.ends_with('─')` and `caption_line.chars().count() == 9`
7. Rewrite `caption_empty_title_lead_only_no_separator_ft8`:
   - Remove `terminal_width(Some(20))`
   - Assert `chars().count() == 9` (same two-column plain table = 9 chars wide)
8. Run `w3 .test level::3`; fix any failures; iterate until Level 3 passes.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|---|---|---|
| Short title + 1 field; plain 2-col table (width=9) | `TableConfig::plain().caption(TableCaption::new("T").field("F"))` | `caption_line.chars().count() == 9` |
| Content exactly fills table_width | caption content length = `table_width` | no trailing `─`; `chars().count() == table_width` |
| Content exceeds table_width | long title on narrow table | no trailing `─`; title verbatim; no panic |
| Empty title; plain 2-col table (width=9) | `TableCaption::new("")` | `chars().count() == 9`; no `·` separator |
| No caption | `TableConfig::plain()` without `.caption()` | byte-identical to baseline; no regression |
| Grid style with caption | `TableConfig::grid().caption(...)` | caption before `+---+` border; fills to grid table_width |

## Validation

Run `w3 .test level::3` and confirm:
- 618 nextest pass (or same count as baseline)
- 74 doc tests pass
- 0 clippy warnings

## Related Documentation

- [`docs/feature/007_table_caption.md`](../../docs/feature/007_table_caption.md) — feature spec: table-width fill behavior (updated)
- [`docs/invariant/005_caption.md`](../../docs/invariant/005_caption.md) — invariant 2: width ceiling against rendered table width (updated)
- [`docs/algorithm/007_caption_rendering.md`](../../docs/algorithm/007_caption_rendering.md) — step 3: table_width computation (updated)
- [`tests/docs/feature/007_table_caption.md`](../../tests/docs/feature/007_table_caption.md) — FT-3/FT-4/FT-8 cases marked ⏳ (pending this task)
- [`tests/docs/algorithm/007_caption_rendering.md`](../../tests/docs/algorithm/007_caption_rendering.md) — AC-4/AC-5/AC-6 updated to table_width

**Closes:** null

## Affected Entities

- `src/formatters/table/row_rendering.rs` — mutated: `render_caption_if_present` signature change
- `src/formatters/table/mod.rs` — mutated: call site update

## History

- **[2026-06-15]** `CREATED` — Fix render_caption_if_present to fill caption trailing rule to rendered table width instead of terminal width.

## Verification Record

- **Date**: 2026-06-15
- **Method**: MAAV — 4 independent parallel subagents (no self-verification)
- **Scope Coherence**: PASS — In Scope non-empty (3 specific files); Out of Scope non-empty (3 exclusions); observable end-state; no scope creep; no vague scope
- **MOST Goal Quality**: PASS — Motivated (visual bug with fallback 120 chars); Observable (Level 3 + named test functions); Scoped (3 files, 3 exclusions); Testable (618/74/0 counts)
- **Value / YAGNI**: PASS — Concrete committed need (bug exists in source, ⏳ spec markers); no speculative scope; null hypothesis answered; proportionate (minimal 3-file fix)
- **Implementation Readiness**: Initial finding: Step 6 vague ("e.g., with min_column_width or specific column data"). Fixed: Step 6 now specifies exact title "Abcd" (4 chars), expected table_width=9, concrete assertions. Re-verified: PASS
- **Result**: ✅ COMPLETED — implemented via `001_heading_implementation.plan.md` Phase 1; MAAV gate passed
