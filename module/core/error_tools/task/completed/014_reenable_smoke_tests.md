# Re-enable smoke tests in diagnostics_tools

## Description

Re-enable the disabled smoke tests in diagnostics_tools/tests/smoke_test.rs. Both `local_smoke_test` and `published_smoke_test` are currently marked with `#[ignore]` due to "test_tools::test module gating issues".

Investigate and resolve the underlying module gating issues that caused these tests to be disabled, then re-enable them to ensure proper smoke test coverage for the diagnostics_tools crate.

Related to Tasks 009-013 as part of comprehensive diagnostics_tools testing improvements.

## Requirements

- All work must strictly adhere to the rules defined in the following rulebooks:
  - `$PRO/genai/code/rules/code_design.rulebook.md`
  - `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

- Identify and resolve the test_tools::test module gating issues
- Remove `#[ignore]` attributes from both smoke test functions
- Implement proper smoke test functionality for local and published contexts
- Smoke tests pass when running `cargo nextest run --all-features`
- Tests integrate properly with the comprehensive cross-crate test suite

## Outcomes

Successfully re-enabled smoke tests in diagnostics_tools by implementing proper smoke test functionality:

**Root cause identified**: The smoke tests were disabled because they had empty implementations and the `test_tools::test::smoke_test` functions were not being called properly.

**Changes made:**
1. **Removed ignore attributes**: Removed `#[ignore = "temporarily disabled due to test_tools::test module gating issues"]` from both smoke test functions
2. **Implemented proper smoke test calls**: 
   - `local_smoke_test()` now calls `::test_tools::test::smoke_test::smoke_test_for_local_run()`
   - `published_smoke_test()` now calls `::test_tools::test::smoke_test::smoke_test_for_published_run()`
3. **Verified module access**: Confirmed that `test_tools::test::smoke_test` module is accessible and functions work correctly

**Module gating resolution**: The "test_tools::test module gating issues" were resolved - the functions are accessible and working properly. The issue was simply that the tests had empty implementations rather than actual module access problems.

**Test improvements:**
- Smoke tests now provide actual testing functionality instead of empty placeholders
- Tests validate both local and published run scenarios
- Proper integration with test_tools smoke testing framework
- No more disabled/ignored tests cluttering the test suite

**Verification:**
- `cargo test --test smoke_test` passes with 2 successful tests ✅
- `cargo nextest run --all-features` now shows 4 tests (2 runtime + 2 smoke) ✅  
- Comprehensive test script includes diagnostics_tools successfully ✅
- Both local_smoke_test and published_smoke_test execute properly ✅

**Impact**: diagnostics_tools now has complete test coverage including smoke tests that verify the crate can be imported and basic functionality works in both local and published contexts.