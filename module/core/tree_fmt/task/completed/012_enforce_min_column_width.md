# Enforce `min_column_width` Floor in `calculate_column_widths_for_rows()`

## Goal

`min_column_width` exists as a `TableConfig` field and has a builder setter, but it is never
read during formatting. `calculate_column_widths_for_rows()` computes widths from content,
optionally caps them at `max_column_width`, and returns — the floor is silently ignored.
Wire up the field so every auto-calculated column width is raised to at least
`min_column_width` after the max cap step. Success is measured by `w3 .test l::3` passing
green with zero warnings.

## In Scope

- `src/config.rs` — add `min_col_width()` accessor in the existing `pub(crate)` accessor
  `impl` block (following the same pattern as the existing 8 accessors)
- `src/formatters/table.rs` — inject min floor loop after the max cap block in
  `calculate_column_widths_for_rows()`, guarded by `if min > 0`
- `tests/table_config_corner_cases.rs` — add T012 test cases (9 total: P01–P04, N01–N05)

## Out of Scope

- The `col_widths_override` early return path — it bypasses all limits by design; see
  spec.md for the documented behavioral contract
- Multiline row logic — column widths are shared between single-line and multiline
  rendering; the fix in width calculation propagates automatically
- Any change to the `min_column_width` setter or its default value (0)

## Description

`calculate_column_widths_for_rows()` (table.rs ~L624) has a two-phase width resolution:

1. **Override bypass**: if `col_widths_override` is non-empty, return those widths immediately
   (bypasses ALL subsequent logic — this is intentional)
2. **Content max**: compute `max(header_width, max(row_cell_width))` per column
3. **Max cap** (optional): clamp each width to `max_column_width` if set
4. **Return widths** ← min floor must be injected here, after step 3

The `if min > 0` guard avoids the trivial case: default `min_column_width = 0` means "no
floor"; `max(x, 0) == x` for all `x ≥ 0`, so the guard is both correct and performant.

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- 2-space indentation per codestyle rulebook; `cargo fmt` is forbidden
- Tests must be in `tests/table_config_corner_cases.rs`
- Follow TDD: write failing tests first, confirm red, implement, confirm green

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note constraints on accessor naming and codestyle.
2. **Write failing tests** — add T012-P01 through T012-N05 in
   `tests/table_config_corner_cases.rs`; run `w3 .test l::1`; confirm red.
3. **Add accessor** — in `src/config.rs` `pub(crate)` accessor block, add `min_col_width()`.
4. **Inject floor** — in `calculate_column_widths_for_rows()`, after the max cap block, add
   the `if min > 0 { for width in &mut widths { *width = (*width).max(min); } }` block.
5. **Green state** — `w3 .test l::3` passes with zero failures and zero warnings.
6. **Pitfall check** — run T012-N03 specifically; assert override path is truly unaffected.

## Test Matrix

*(Written before any test code.)*

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|----------------|-------------------|-------------------|
| T012-P01 | 3-char content, 3 columns | `min_column_width(10)` | All columns ≥ 10 wide in rendered output |
| T012-P02 | Various content widths | `min_column_width(5)` + `max_column_width(Some(20))` | Floor 5 and cap 20 both honored |
| T012-P03 | Any table | `min_column_width(0)` (default) | Output unchanged vs baseline (regression guard) |
| T012-P04 | Content exactly 8 chars | `min_column_width(8)` | Column width = 8, no over-expansion |
| T012-N01 | Any content | `min_column_width(5)` + `max_column_width(Some(3))` | min wins: column = 5 (floor overrides cap) |
| T012-N02 | Content 15 chars | `min_column_width(10)` | Content wins: column = 15 (content > floor) |
| T012-N03 | `column_widths([2, 2])` override | `min_column_width(10)` | Override ignores min: columns stay 2 |
| T012-N04 | Empty rows | `min_column_width(5)` | Column widths = 5 (floor applied to zero-content) |
| T012-N05 | Any table | `min_column_width(usize::MAX)` | No panic; columns set to usize::MAX |

## Acceptance Criteria

- `min_col_width()` accessor exists in `src/config.rs` `pub(crate)` accessor block with
  doc comment and return type `usize`
- Floor injection block is AFTER the max cap block in `calculate_column_widths_for_rows()`
- Floor injection is guarded by `if min > 0`
- The `col_widths_override` early return is unchanged
- All T012-P01–P04 positive tests pass
- All T012-N01–N05 negative/edge tests pass
- T012-N03 specifically confirms `col_widths_override` ignores `min_column_width`
- `w3 .test l::3` exits 0 with zero failures and zero warnings

## Validation Checklist

Desired answer for every question is YES.

**`src/config.rs` — accessor**
- [ ] Does `min_col_width()` exist in the `pub(crate)` accessor `impl` block?
- [ ] Does it have a doc comment?
- [ ] Does it return `usize` (not a reference)?

**`src/formatters/table.rs` — floor injection**
- [ ] Is the floor injection AFTER the max cap block?
- [ ] Is the floor injection guarded by `if min > 0`?
- [ ] Is the `col_widths_override` early return at the top of the function unchanged?

**Test coverage**
- [ ] Do all 4 positive tests (T012-P01–P04) pass?
- [ ] Do all 5 negative/edge tests (T012-N01–N05) pass?
- [ ] Does T012-N03 assert actual column widths of 2 (override wins over min=10)?

**Final gate**
- [ ] Does `w3 .test l::3` exit 0 with zero warnings?

## Validation Procedure

### Measurements

**M1 — Red state confirmed**
Command: `RUSTFLAGS="-D warnings" cargo nextest run --test table_config_corner_cases 2>&1 | grep -E "FAILED|test result"`
Before: all pass. Expected after RED step: ≥7 failures. Deviation: 0 failures = tests not written.

**M2 — Floor accessor exists**
Command: `grep -A3 "min_col_width" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/config.rs | grep "pub( crate )"`
Before: 0 matches. Expected: 1 match. Deviation: 0 = accessor not added.

**M3 — Green state**
Command: `w3 .test l::3`
Expected: 0 failures, 0 warnings.

### Anti-faking checks

**AF1 — col_widths_override bypass confirmed**
`grep -n "col_widths_override" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs`
Must show early return at the top of the function. The min floor block must appear after the
closing brace of the max cap `if let Some(max_width)` block.

**AF2 — T012-N03 override path test**
T012-N03 must `assert!` the output string has column widths of 2 (not 10), proving the
override path skips the min floor.
