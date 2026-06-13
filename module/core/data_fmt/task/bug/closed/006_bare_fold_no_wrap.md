# BUG-006: Bare Fold Style Did Not Wrap on Terminal Overflow

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/formatters/table/auto_fit.rs` — `FoldStyle::Bare` branch

## Root Cause

The `Bare` fold style branch emitted joined overflow values unconditionally without checking
whether `unicode_visual_len(joined_line) > terminal`, unlike the `Labeled` and `Stacked`
branches which included wrapping guards. This caused `Bare` fold output to exceed terminal
width with no continuation lines.

## Fix Location

`src/formatters/table/auto_fit.rs` — `FoldStyle::Bare` match arm.
`Fix(BUG-006)`: added wrap guard mirroring the other two styles.

## Pitfall

The `Bare` branch has no label prefix, so wrapped continuation lines carry only value
fragments. Tests must verify that wrapping produces lines of `≤ terminal_width`.

## Test Reference

`tests/auto_fold_test.rs` — `bug_reproducer(BUG-006)`:
`bug_reproducer_bare_fold_no_wrap`
