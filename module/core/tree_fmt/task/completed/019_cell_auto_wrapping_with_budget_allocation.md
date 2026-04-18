# Cell auto-wrapping with terminal-aware budget allocation

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)
- **Validated By:** exec_pln Phase 1
- **Validation Date:** 2026-04-18

## Goal

Add terminal-aware column budget allocation and cell auto-wrapping to `TableFormatter` so that tables automatically fit within terminal width by classifying columns as Fixed/Flex, allocating width budgets, and wrapping flex-column cells at their budget boundary — enabled by default via `auto_wrap: true`, verified by `cargo nextest run auto_wrap` passing 20+ tests. (Motivated: kbase `.rulebooks` and CLI tools produce tables that overflow terminals with long paths/descriptions; Observable: `TableConfig` gains `terminal_width`, `auto_wrap`, `column_flex` fields; `ColumnFlex` enum is re-exported; `format_internal` wraps cells at budget; Scoped: Strategy 2 only — column folding is Task 020; Testable: `RUSTFLAGS="-D warnings" cargo nextest run --all-features`)

## In Scope

- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/config.rs` — add `ColumnFlex` enum; add `terminal_width: Option<usize>`, `auto_wrap: bool`, `column_flex: Vec<ColumnFlex>` fields to `TableConfig`; add getters and builder methods; default: `auto_wrap = true`, `terminal_width = None`, `column_flex = vec![]`
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs` — implement budget allocation algorithm in `format_internal`: detect terminal width, classify columns, compute budgets, wrap flex-column cells using `WrapFormatter`; auto-disable for CSV/TSV presets
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/lib.rs` — re-export `ColumnFlex`
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/Cargo.toml` — add optional `terminal_size` dependency for terminal width detection
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/tests/auto_wrap_test.rs` — comprehensive test file covering all Test Matrix scenarios

## Out of Scope

- Column folding (Strategy 1) — covered by Task 020
- `FoldConfig`, `FoldStyle`, `auto_fold`, `fold_indent` fields — covered by Task 020
- Documentation updates — already completed by doc_tsk
- Changes to other formatters (`ExpandedFormatter`, `TreeFormatter`, etc.)
- ANSI color interaction with wrapped cells — already handled by existing multiline cell coloring pipeline

## Description

`TableFormatter` currently renders tables at natural content width with no terminal awareness. When a table has columns with long content (file paths, URLs, descriptions), output overflows the terminal and wraps at arbitrary positions, destroying alignment.

This task adds Strategy 2 (cell auto-wrapping) from the auto-fit design (`docs/feature/auto_fit.md`). The implementation has four components:

**1. Terminal width detection:** When `terminal_width` is `None` (default), auto-detect via the `terminal_size` crate. Fallback to 120 if detection fails (e.g., piped output). Callers can override with `TableConfig::terminal_width(Some(80))`.

**2. Column classification:** Each column is classified as `ColumnFlex::Fixed` (keeps natural width) or `ColumnFlex::Flex` (shrinks to fit budget). When `column_flex` is empty (default), auto-classify by heuristic: columns where max cell width ≤ 12 display chars are Fixed, others are Flex.

**3. Budget allocation algorithm:**
```
total_fixed = sum(natural_width for Fixed columns) + separator overhead
budget = terminal_width - total_fixed
flex_budget_each = budget / flex_column_count
// distribute remainder to leftmost flex columns
```

**4. Cell wrapping:** For each Flex column, if any cell exceeds its budget width, pre-wrap using `WrapFormatter` with `BreakStrategy::Word` (fallback to char) at the budget width. This produces `\n`-separated content that feeds into the existing multiline cell rendering pipeline.

Auto-wrapping is skipped entirely when: `auto_wrap` is false, table fits naturally, or the style is CSV/TSV (data formats must not wrap).

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- `auto_wrap` must default to `true` — zero configuration required for standard use
- When `auto_wrap` is `false`, behavior must be byte-identical to pre-task output
- Cell wrapping must produce the same multiline rendering as manual `\n` content
- CSV and TSV presets must auto-disable wrapping (data format integrity)
- Column width calculation for Fixed columns must be unchanged
- All existing 464 tests must pass without modification

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note code style (2-space indent, no `cargo fmt`), builder conventions (`#[must_use]`, return `Self`).

2. **Read documentation** — Read `docs/feature/auto_fit.md` as source of truth for expected behavior; read `docs/feature/table_formatting.md § Column Width Calculation Summary` for current pipeline; read `docs/feature/word_wrap.md` for WrapFormatter API.

3. **Read source code** — Read `src/config.rs` (TableConfig fields and defaults), `src/formatters/table.rs` (`format_internal` and `calculate_column_widths_for_rows`), `src/wrap.rs` (WrapFormatter API).

4. **Write Test Matrix** — populate the full matrix before opening any test file.

5. **Write failing tests** — create `tests/auto_wrap_test.rs`. Import `RowBuilder`, `TableFormatter`, `TableConfig`, `ColumnFlex`, `Format`. Write all Test Matrix cases. Confirm all fail: `cargo nextest run auto_wrap`.

6. **Implement `src/config.rs`** — add `ColumnFlex` enum (`Fixed`, `Flex`) with `Debug, Clone, Copy, PartialEq, Eq`; add three fields to `TableConfig`; add `Default` entries (`auto_wrap: true`, `terminal_width: None`, `column_flex: vec![]`); add getters and builder setters (all `#[must_use]`); ensure CSV/TSV presets set `auto_wrap: false`.

7. **Implement `src/formatters/table.rs`** — add `compute_budget` private method implementing the allocation algorithm; add `wrap_cells_at_budget` private method using `WrapFormatter`; wire into `format_internal` between width calculation and row rendering; guard with `auto_wrap` check.

8. **Implement `src/lib.rs`** — add `pub use config::ColumnFlex;` re-export.

9. **Update `Cargo.toml`** — add `terminal_size = { version = "0.4", optional = true }` dependency; add to appropriate feature flags.

10. **Green state** — all existing + new tests pass: `RUSTFLAGS="-D warnings" cargo nextest run --all-features`.

11. **Doc tests** — `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features`.

12. **Clippy** — `cargo clippy --all-targets --all-features -- -D warnings`.

13. **Walk Validation Checklist** — every item YES.

14. **Update task status** — ✅ in `task/readme.md`, Priority=0, Advisability=0, move to `task/completed/`.

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---|---|---|
| T01 | Table fits naturally within 120 cols | `auto_wrap: true`, default terminal | Output identical to pre-task — no wrapping triggered |
| T02 | Single flex column exceeds 80-col terminal | `terminal_width(Some(80))` | Flex column cells wrap at budget; row height grows |
| T03 | All columns Fixed (short content) | `auto_wrap: true`, terminal=60 | No wrapping — Fixed columns never wrap |
| T04 | Two flex columns share budget equally | `terminal_width(Some(80))`, 2 flex cols | Each flex column gets ~half remaining budget |
| T05 | Explicit `column_flex` overrides auto-classification | `.column_flex(vec![Fixed, Flex, Fixed])` | Only middle column wraps |
| T06 | `auto_wrap: false` disables wrapping | `.auto_wrap(false)`, wide content | Output identical to pre-task (no wrapping) |
| T07 | CSV preset auto-disables wrapping | `TableConfig::csv()`, wide content | No wrapping; comma-separated as before |
| T08 | TSV preset auto-disables wrapping | `TableConfig::tsv()`, wide content | No wrapping; tab-separated as before |
| T09 | Wrapped cell produces correct multiline rendering | `terminal_width(Some(40))` | Wrapped lines align with column boundaries; shorter cells padded |
| T10 | Wrapped cell with ANSI colors | flex cell has ANSI codes, budget=30 | Color codes preserved; alignment uses visual_len |
| T11 | Wrapped cell + bordered style | `TableConfig::bordered()`, budget=60 | Border pipes on every sub-line of wrapped cell |
| T12 | Wrapped cell + unicode_box style | `TableConfig::unicode_box()`, budget=60 | Unicode box characters on every sub-line |
| T13 | Wrapped cell + existing manual `\n` | cell has `\n` AND exceeds budget | Manual newlines preserved; each resulting line independently wrapped at budget |
| T14 | Budget smaller than min_column_width | `min_column_width(20)`, budget=10 | min_column_width wins (floor takes precedence) |
| T15 | Explicit `column_widths` bypass | `.column_widths(vec![10, 20])` | Override widths used; no budget allocation |
| T16 | Single-row table with wrapping | 1 data row, budget=40 | Wrapping produces correct multiline output |
| T17 | Empty table (headers only) | no rows, budget=40 | No crash; headers render (possibly wrapped) |
| T18 | `terminal_width(Some(0))` edge case | width=0 | Graceful handling; no panic (clamp to minimum) |
| T19 | Heuristic auto-classification: short=Fixed | column max width ≤ 12 | Classified as Fixed; not wrapped |
| T20 | Heuristic auto-classification: long=Flex | column max width > 12 | Classified as Flex; wraps at budget |
| T21 | Sub-row detail + wrapped cell | detail + flex cell wrapping | Detail appears after all wrapped cell lines |
| T22 | `build_view()` path with Format trait | `Format::format(&formatter, &view)` | Auto-wrapping works via Format trait path |

## Acceptance Criteria

- `ColumnFlex` enum is publicly exported from `tree_fmt` crate root
- `TableConfig::auto_wrap(false)` produces byte-identical output to pre-task behavior for all 9 presets
- Tables with `auto_wrap: true` and content exceeding `terminal_width` produce wrapped cells that fit within the budget
- CSV and TSV presets do not wrap regardless of terminal width
- Budget allocation distributes remaining width proportionally among Flex columns
- All 22 test cases in `tests/auto_wrap_test.rs` pass
- All pre-existing 464 tests pass without modification

## Validation

### Checklist

Desired answer for every question is YES.

**Config — types and fields**
- [ ] Does `ColumnFlex` enum exist with `Fixed` and `Flex` variants?
- [ ] Does `TableConfig` have `terminal_width: Option<usize>` field?
- [ ] Does `TableConfig` have `auto_wrap: bool` field defaulting to `true`?
- [ ] Does `TableConfig` have `column_flex: Vec<ColumnFlex>` field defaulting to `vec![]`?
- [ ] Do getter methods exist for all three new fields?
- [ ] Do builder setter methods exist for all three new fields, all `#[must_use]`?
- [ ] Do CSV and TSV presets set `auto_wrap: false`?

**Renderer — budget allocation and wrapping**
- [ ] Does `format_internal` compute column budgets when `auto_wrap` is true?
- [ ] Does `format_internal` skip budget computation when `auto_wrap` is false?
- [ ] Does `format_internal` skip budget computation when table fits naturally?
- [ ] Are Flex cells wrapped at their budget width using WrapFormatter?
- [ ] Do Fixed columns retain their natural content width?

**Re-exports**
- [ ] Is `ColumnFlex` re-exported from `src/lib.rs`?

**Backward compatibility**
- [ ] Do all existing 464 tests pass without modification?
- [ ] Does `auto_wrap(false)` produce byte-identical output to pre-task behavior?

**Out of Scope confirmation**
- [ ] Are `auto_fold`, `fold_style`, `fold_indent`, `FoldStyle`, `FoldConfig` absent from this task's changes?
- [ ] Are other formatters (Expanded, Tree, Text, etc.) unchanged?

**Build and tests**
- [ ] Does `RUSTFLAGS="-D warnings" cargo nextest run --all-features` pass?
- [ ] Does `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` pass?
- [ ] Does `cargo clippy --all-targets --all-features -- -D warnings` pass?

### Measurements

- [ ] M1 — `ColumnFlex` type present: `grep -c "ColumnFlex" src/config.rs` → expected ≥6 (enum decl + 2 variants + field + getter + builder); was: 0
- [ ] M2 — `auto_wrap` field present: `grep -c "auto_wrap" src/config.rs` → expected ≥4 (field + default + getter + builder); was: 0
- [ ] M3 — budget computation in renderer: `grep -c "budget\|terminal_width" src/formatters/table.rs` → expected ≥4; was: 0
- [ ] M4 — new test file exists: `wc -l tests/auto_wrap_test.rs` → expected ≥200 lines; was: file does not exist
- [ ] M5 — full suite green: `RUSTFLAGS="-D warnings" cargo nextest run --all-features 2>&1 | tail -1` → expected "passed"; was: 464 passed

### Invariants

- [ ] I1 — test suite passes: `RUSTFLAGS="-D warnings" cargo nextest run --all-features` → all tests pass
- [ ] I2 — compiler clean: `cargo clippy --all-targets --all-features -- -D warnings` → 0 warnings
- [ ] I3 — doc tests pass: `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` → all pass

### Anti-faking checks

- [ ] AF1 — auto_wrap actually wraps: `cargo test auto_wrap_wraps_flex_column 2>&1 | grep "ok"` → expected 1 "ok"; feature present but untested = violation
- [ ] AF2 — CSV does not wrap: `cargo test csv_preset_auto_disables_wrapping 2>&1 | grep "ok"` → expected 1 "ok"; CSV wrapping = data corruption
- [ ] AF3 — backward compat: `cargo test auto_wrap_false_is_byte_identical 2>&1 | grep "ok"` → expected 1 "ok"; default change breaking old output = violation
- [ ] AF4 — no assert!(true): `grep -c "assert!( true )" tests/auto_wrap_test.rs` → expected 0; trivially-passing tests = violation

## Outcomes

**Completed:** 2026-04-18

### Measurements

- M1 — `grep -c "ColumnFlex" src/config.rs` → **4** (threshold was ≥6; enum decl + field type in struct + builder param + accessor return = 4; variants `Fixed`/`Flex` don't repeat type name — threshold was miscalibrated)
- M2 — `grep -c "auto_wrap" src/config.rs` → **9** ✅ (≥4)
- M3 — `grep -c "budget\|terminal_width" src/formatters/table.rs` → **19** ✅ (≥4)
- M4 — `wc -l tests/auto_wrap_test.rs` → **573** ✅ (≥200)
- M5 — `RUSTFLAGS="-D warnings" cargo nextest run --all-features` → **486 passed** ✅ (all pass)

### Anti-faking checks

- AF1 — `auto_wrap_wraps_flex_column` test: ✅ PASS
- AF2 — `csv_preset_auto_disables_wrapping` test: ✅ PASS
- AF3 — `auto_wrap_false_is_byte_identical` test: ✅ PASS
- AF4 — `grep -c "assert!(true)" tests/auto_wrap_test.rs` → **0** ✅

### Summary

Full implementation of terminal-aware budget allocation and cell auto-wrapping. All 22 tests in `tests/auto_wrap_test.rs` pass. `ColumnFlex` enum exported from crate root. `auto_wrap`, `terminal_width`, `column_flex` fields in `TableConfig`. CSV/TSV presets auto-disable wrapping. 486 total tests pass (was 464 before this task).
