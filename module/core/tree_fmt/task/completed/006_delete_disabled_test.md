# Delete Disabled Test in Unicode Display Width Alignment

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)

## Goal

The `#[ignore]` test `correct_behavior_display_width_padding` is deleted from
`tests/unicode_display_width_alignment.rs`, leaving zero disabled tests in the codebase
and zero `#[ignore]` attributes in any test file.

MOST breakdown:
- **Motivated** — "Never disable, ignore, or skip tests. Fix them or remove them."
  (Testing Principles, CLAUDE.md). The ignored test demonstrates an algorithm manually
  without calling any `tree_fmt` API; it carries no testing value.
- **Observable** — `grep -rn "#\[ignore" tests/` returns zero matches.
- **Scoped** — delete one `#[test]` function (27 lines) from one file; no other changes.
- **Testable** — `w3 .test l::3` passes with zero failures; no `#[ignore]` attributes remain.

## In Scope

- Delete `correct_behavior_display_width_padding` (lines 182–228 of
  `tests/unicode_display_width_alignment.rs`) including its doc comment

## Out of Scope

- Implementing `pad_to_display_width()` — deferred to a future task
- Modifying any other test in the file
- Changing any source code

## Description

`correct_behavior_display_width_padding` was written as a forward-looking specification
test showing what correct display-width-aware padding would look like after a future API
(`pad_to_display_width`) is implemented. It was disabled with `#[ignore]` because that
API does not exist. The test body manually computes padding using `unicode-width` directly
— it does not call any `tree_fmt` function. The knowledge it encapsulates (algorithm for
display-width padding) is already preserved in the file-level 5-section doc comment and
in `src/ansi_str.rs`. Deletion loses no information.

## Requirements

- All work must strictly adhere to all applicable rulebooks (`kbase .rulebooks`)
- No test may be disabled, ignored, or skipped — remove or fix

## Acceptance Criteria

- `grep -rn "#\[ignore" tests/` returns zero matches
- `w3 .test l::3` passes with zero failures and zero warnings

## Work Procedure

1. Open `tests/unicode_display_width_alignment.rs`
2. Delete the `correct_behavior_display_width_padding` function and its leading doc comment
3. Run `w3 .test l::3` — confirm green
4. Verify: `grep -rn "#\[ignore" tests/` → zero matches
5. Update task status in `task/readme.md`

## Validation List

- [x] `grep -rn "#\[ignore" tests/` returns zero matches?
- [x] `w3 .test l::3` passes with zero failures, zero warnings? *(323 nextest + 73 doc tests, 0 clippy warnings)*
- [x] Is the file-level 5-section doc comment still present (knowledge preserved)?

## Validation Procedure

**VP1 — No disabled tests**
`grep -rn "#\[ignore" tests/` — expect zero matches.

**VP2 — Full test suite**
`w3 .test l::3` — expect 0 failures, 0 warnings.

## Outcomes

*(Completed. Task delivered and verified per acceptance criteria.)*
