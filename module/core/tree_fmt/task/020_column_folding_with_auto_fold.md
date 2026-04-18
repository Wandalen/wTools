# Column folding with auto-fold

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** 🎯 (Available)
- **Validated By:** null
- **Validation Date:** null

## Goal

Add column folding to `TableFormatter` so that when total row width exceeds terminal after wrapping (Task 019), overflow columns are moved to labeled continuation lines below the row — enabled by default via `auto_fold: true`, composing with Strategy 2 wrapping, verified by `cargo nextest run auto_fold` passing 20+ tests. (Motivated: tables with many columns (6-8+) still overflow after wrapping when fixed-width columns consume most of the budget; Observable: `TableConfig` gains `auto_fold`, `fold_style`, `fold_indent` fields; `FoldStyle` enum is re-exported; `format_internal` renders continuation lines for overflow columns; Scoped: Strategy 1 folding only — wrapping is Task 019; Testable: `RUSTFLAGS="-D warnings" cargo nextest run --all-features`)

## In Scope

- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/config.rs` — add `FoldStyle` enum (`Bare`, `Labeled`, `Stacked`); add `auto_fold: bool`, `fold_style: FoldStyle`, `fold_indent: String` fields to `TableConfig`; add getters and builder methods; defaults: `auto_fold = true`, `fold_style = Labeled`, `fold_indent = "    "`
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs` — implement fold detection and rendering in `format_internal`: after budget allocation (Task 019), if total still exceeds terminal width, determine fold point, render primary columns as table row, emit overflow columns as continuation line(s) below the row
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/lib.rs` — re-export `FoldStyle`
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/tests/auto_fold_test.rs` — comprehensive test file covering all Test Matrix scenarios

## Out of Scope

- Terminal width detection and budget allocation — implemented by Task 019
- `ColumnFlex` enum and `auto_wrap` field — implemented by Task 019
- Documentation updates — already completed by doc_tsk
- Changes to other formatters (`ExpandedFormatter`, `TreeFormatter`, etc.)
- Custom fold renderers or pluggable fold formatting

## Description

After Task 019 (cell auto-wrapping), tables with many columns may still overflow. For example, a table with 8 columns where 5 are Fixed (ID, Lines, Rules, Valid, Source) consumes ~40 chars of fixed width plus separators; the remaining 3 Flex columns (File, Path, Purpose) each get only ~25 chars of budget in a 120-column terminal — still tight.

Column folding (Strategy 1 from `docs/feature/auto_fit.md`) solves this by moving overflow columns to continuation lines below the row. The fold point is the first column index where the cumulative width exceeds the terminal budget. All columns from that index onward are rendered as continuation lines.

**Fold detection algorithm:**
```
cumulative = 0
for each column in columns:
  cumulative += column_width + separator_width
  if cumulative > terminal_width:
    fold_point = column_index
    break
primary_columns = columns[..fold_point]
overflow_columns = columns[fold_point..]
```

**Fold rendering (Labeled style, default):**
```
ID  File                    Lines  Rules
--  ----------------------  -----  -----
b1  governance.rulebook.md  120    23
    Path: /home/user1/pro/genai/governance/governance.rulebook.md
    Purpose: Governance principles and anti-duplication rules.
```

**Fold rendering (Bare style):**
```
b1  governance.rulebook.md  120    23
    /home/user1/pro/genai/governance/governance.rulebook.md
    Governance principles and anti-duplication rules.
```

**Fold rendering (Stacked style):**
Same as Labeled but each overflow column on its own continuation line (Labeled also stacks by default — difference is Bare puts all values on one line when they fit).

**Combination with Strategy 2:** Folded values can themselves wrap if they exceed terminal width minus fold indent.

Auto-folding is skipped when: `auto_fold` is false, all columns fit after wrapping, or the style is CSV/TSV.

**Dependency:** This task requires Task 019 (terminal_width field, budget allocation, ColumnFlex classification). If Task 019 is not complete, this task is blocked.

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- `auto_fold` must default to `true` — zero configuration required
- When `auto_fold` is `false`, behavior must be identical to Task 019 output (wrapping only, no folding)
- Continuation lines must use `fold_indent` prefix and `fold_style` format
- CSV and TSV presets must auto-disable folding
- Header row must NOT fold — only data rows fold
- All existing tests (including Task 019 tests) must pass without modification

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note code style (2-space indent, no `cargo fmt`), builder conventions.

2. **Read documentation** — Read `docs/feature/auto_fit.md` § Strategy 1 and § Combination as source of truth; read `docs/api/config_types.md § FoldStyle` for type spec.

3. **Read source code** — Read `src/config.rs` (current TableConfig including Task 019 additions), `src/formatters/table.rs` (`format_internal` including Task 019 budget allocation).

4. **Write Test Matrix** — populate the full matrix before opening any test file.

5. **Write failing tests** — create `tests/auto_fold_test.rs`. Import `RowBuilder`, `TableFormatter`, `TableConfig`, `FoldStyle`, `ColumnFlex`, `Format`. Write all Test Matrix cases. Confirm all fail: `cargo nextest run auto_fold`.

6. **Implement `src/config.rs`** — add `FoldStyle` enum with `Debug, Clone, Copy, PartialEq, Eq, Default` (default = `Labeled`); add three fields to `TableConfig`; add `Default` entries (`auto_fold: true`, `fold_style: FoldStyle::Labeled`, `fold_indent: "    ".to_string()`); add getters and builder setters; ensure CSV/TSV presets set `auto_fold: false`.

7. **Implement `src/formatters/table.rs`** — add `determine_fold_point` private method; add `render_fold_continuation` private method; wire into `format_internal` after budget allocation: if `auto_fold` is true and cumulative width exceeds terminal, compute fold point, render primary columns normally, render overflow columns as continuation lines after each row.

8. **Implement `src/lib.rs`** — add `pub use config::FoldStyle;` re-export.

9. **Green state** — all existing + new tests pass: `RUSTFLAGS="-D warnings" cargo nextest run --all-features`.

10. **Doc tests** — `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features`.

11. **Clippy** — `cargo clippy --all-targets --all-features -- -D warnings`.

12. **Walk Validation Checklist** — every item YES.

13. **Update task status** — ✅ in `task/readme.md`, Priority=0, Advisability=0, move to `task/completed/`.

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---|---|---|
| T01 | Table fits within terminal after wrapping | `auto_fold: true`, enough width | No folding — all columns rendered in table row |
| T02 | 6-column table overflows 80-col terminal | `terminal_width(Some(80))`, 6 cols | Last 2 columns fold to continuation lines |
| T03 | Labeled fold style (default) | `fold_style: Labeled` | Continuation shows `"ColName: value"` format |
| T04 | Bare fold style | `.fold_style(FoldStyle::Bare)` | Continuation shows values only, no labels |
| T05 | Stacked fold style | `.fold_style(FoldStyle::Stacked)` | Each overflow column on its own continuation line with label |
| T06 | Custom fold indent `">>> "` | `.fold_indent(">>> ".into())` | Continuation lines start with `">>> "` |
| T07 | `auto_fold: false` disables folding | `.auto_fold(false)`, wide table | No continuation lines; same as Task 019 output |
| T08 | CSV preset auto-disables folding | `TableConfig::csv()`, many columns | No folding; all columns in CSV row |
| T09 | TSV preset auto-disables folding | `TableConfig::tsv()`, many columns | No folding; all columns in TSV row |
| T10 | Fold + wrap combination | fold point at col 4, folded value long | Folded value wraps within terminal width minus indent |
| T11 | Multiple rows all fold at same point | 3 rows, same fold point | Consistent fold point; each row has continuation |
| T12 | Mixed rows: some fit, some fold | row 1 fits, row 2 overflows | Only row 2 has continuation lines |
| T13 | Single overflow column | fold at last column only | One continuation line with "ColName: value" |
| T14 | All columns overflow except first | very narrow terminal (40) | Only first column in table; rest fold |
| T15 | Fold + sub-row detail | data with both fold and detail | Fold continuation appears; then detail line after fold |
| T16 | Fold + alternating row colors | alternating colors enabled + fold | Continuation lines respect or ignore row colors (no ANSI bleed) |
| T17 | Fold + bordered style | `TableConfig::bordered()` + fold | Primary columns have borders; continuation is plain indented text |
| T18 | Fold + unicode_box style | `TableConfig::unicode_box()` + fold | Unicode borders for primary; continuation outside box |
| T19 | Header row does NOT fold | any fold scenario | Header row always renders all columns; only data rows fold |
| T20 | Empty table (headers only) with narrow terminal | headers-only, narrow | Headers render (possibly truncated); no fold needed |
| T21 | `Format::format` trait path | `Format::format(&formatter, &view)` | Folding works via Format trait |
| T22 | Fold point with explicit `column_flex` | `.column_flex(vec![Fixed, Fixed, Flex, Flex, Flex])` | Fold starts at first Flex column that overflows |

## Acceptance Criteria

- `FoldStyle` enum is publicly exported from `tree_fmt` crate root
- `TableConfig::auto_fold(false)` produces identical output to Task 019 behavior (wrapping only)
- Tables with `auto_fold: true` and total width exceeding terminal after wrapping produce continuation lines for overflow columns
- `FoldStyle::Labeled` renders `"ColName: value"` continuation format
- `FoldStyle::Bare` renders values without labels
- CSV and TSV presets do not fold regardless of terminal width
- Header row never folds — only data rows produce continuation lines
- Folded values compose with wrapping (Strategy 2) when they exceed remaining width
- All 22 test cases in `tests/auto_fold_test.rs` pass
- All pre-existing tests (including Task 019) pass without modification

## Validation

### Checklist

Desired answer for every question is YES.

**Config — types and fields**
- [ ] Does `FoldStyle` enum exist with `Bare`, `Labeled`, `Stacked` variants?
- [ ] Does `FoldStyle` derive `Default` with `Labeled` as default?
- [ ] Does `TableConfig` have `auto_fold: bool` field defaulting to `true`?
- [ ] Does `TableConfig` have `fold_style: FoldStyle` field defaulting to `Labeled`?
- [ ] Does `TableConfig` have `fold_indent: String` field defaulting to `"    "`?
- [ ] Do getter methods exist for all three new fields?
- [ ] Do builder setter methods exist for all three new fields, all `#[must_use]`?
- [ ] Do CSV and TSV presets set `auto_fold: false`?

**Renderer — fold detection and rendering**
- [ ] Does `format_internal` detect fold point when total width exceeds terminal?
- [ ] Does `format_internal` skip folding when `auto_fold` is false?
- [ ] Does `format_internal` skip folding when all columns fit?
- [ ] Does `determine_fold_point` return the correct column index?
- [ ] Does `render_fold_continuation` emit labeled format by default?
- [ ] Does `render_fold_continuation` support all three FoldStyle variants?
- [ ] Does the header row render ALL columns (never folds)?

**Combination with Strategy 2**
- [ ] Do folded values wrap when they exceed remaining terminal width minus fold indent?
- [ ] Does the rendering pipeline apply wrapping first, then folding?

**Re-exports**
- [ ] Is `FoldStyle` re-exported from `src/lib.rs`?

**Backward compatibility**
- [ ] Do all existing tests (including Task 019 auto_wrap tests) pass without modification?
- [ ] Does `auto_fold(false)` produce identical output to Task 019?

**Out of Scope confirmation**
- [ ] Are `ColumnFlex`, `auto_wrap`, `terminal_width` unchanged by this task?
- [ ] Are other formatters (Expanded, Tree, Text, etc.) unchanged?

**Build and tests**
- [ ] Does `RUSTFLAGS="-D warnings" cargo nextest run --all-features` pass?
- [ ] Does `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` pass?
- [ ] Does `cargo clippy --all-targets --all-features -- -D warnings` pass?

### Measurements

- [ ] M1 — `FoldStyle` type present: `grep -c "FoldStyle" src/config.rs` → expected ≥8 (enum + 3 variants + field + default + getter + builder); was: 0
- [ ] M2 — `auto_fold` field present: `grep -c "auto_fold" src/config.rs` → expected ≥4 (field + default + getter + builder); was: 0
- [ ] M3 — fold rendering in renderer: `grep -c "fold\|continuation" src/formatters/table.rs` → expected ≥6; was: 0
- [ ] M4 — new test file exists: `wc -l tests/auto_fold_test.rs` → expected ≥250 lines; was: file does not exist
- [ ] M5 — full suite green: `RUSTFLAGS="-D warnings" cargo nextest run --all-features 2>&1 | tail -1` → expected "passed"

### Invariants

- [ ] I1 — test suite passes: `RUSTFLAGS="-D warnings" cargo nextest run --all-features` → all tests pass
- [ ] I2 — compiler clean: `cargo clippy --all-targets --all-features -- -D warnings` → 0 warnings
- [ ] I3 — doc tests pass: `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` → all pass

### Anti-faking checks

- [ ] AF1 — fold actually produces continuation lines: `cargo test labeled_fold_produces_continuation 2>&1 | grep "ok"` → expected 1; fold logic exists but untested = violation
- [ ] AF2 — header never folds: `cargo test header_row_never_folds 2>&1 | grep "ok"` → expected 1; folded header = broken table
- [ ] AF3 — CSV does not fold: `cargo test csv_preset_auto_disables_folding 2>&1 | grep "ok"` → expected 1; CSV folding = data corruption
- [ ] AF4 — combination works: `cargo test fold_plus_wrap_combination 2>&1 | grep "ok"` → expected 1; strategies must compose
- [ ] AF5 — no assert!(true): `grep -c "assert!( true )" tests/auto_fold_test.rs` → expected 0; trivially-passing tests = violation

## Outcomes

*(Empty — populated upon task completion)*
