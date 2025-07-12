# Task Plan: Fix Unescaping and Rename MRE Test

### Goal
*   To fix the unescaping logic in `strs_tools::string::split` and ensure all MRE tests have unique, descriptive names.

### Ubiquitous Language (Vocabulary)
*   **MRE:** Minimal Reproducible Example.
*   **Split:** A struct representing a segment of a split string.
*   **Unescaping:** The process of converting escape sequences (e.g., `\"`) into their literal representation (e.g., `"`).

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/strs_tools`
*   **Overall Progress:** 1/3 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Rename existing MRE test
    *   ⚫ Increment 2: Correct the failing MRE test
    *   ⚫ Increment 3: Fix the unescaping implementation

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   N/A

### Relevant Context
*   Files to Include:
    *   `module/core/strs_tools/task/task.md`
    *   `module/core/strs_tools/tests/inc/split_test/quoting_and_unescaping_tests.rs`
    *   `module/core/strs_tools/src/string/split.rs`

### Expected Behavior Rules / Specifications
*   The `split` iterator, when `quoting(true)`, should produce unescaped strings for quoted segments.
*   All tests should pass after the implementation fix.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `inc::split_test::quoting_and_unescaping_tests::mre_from_task_test` | Failing (New) | The test fails as expected, because the unescaping logic is not implemented yet. |

### Crate Conformance Check Procedure
*   Run `cargo test -p strs_tools`

### Increments
##### Increment 1: Rename existing MRE test
*   **Goal:** Rename the existing `mre_test` to avoid confusion.
*   **Specification Reference:** User feedback.
*   **Steps:**
    1.  Read `module/core/strs_tools/tests/inc/split_test/quoting_and_unescaping_tests.rs`.
    2.  Rename the test function `mre_test` to `mre_simple_unescape_test`.
    3.  Write the modified content back.
    4.  Run tests to confirm everything still works as before (except the known failure).
*   **Increment Verification:**
    *   The test `mre_test` should no longer exist.
    *   The test `mre_simple_unescape_test` should exist and pass.
*   **Commit Message:** `refactor(tests): Rename MRE test for clarity`

##### Increment 2: Correct the failing MRE test
*   **Goal:** Update the `mre_from_task_test` to reflect the correct expected unescaped output.
*   **Specification Reference:** `module/core/strs_tools/task/task.md`
*   **Steps:**
    1.  Read `module/core/strs_tools/tests/inc/split_test/quoting_and_unescaping_tests.rs`.
    2.  In `mre_from_task_test`, change the expected `string` for the quoted segment to its unescaped form: `"value with \"quotes\" and \\slash\\"`. This will require using `Cow::Owned`.
    3.  Adjust the expected `start` and `end` indices to match the `task.md` specification (`9` and `45`).
    4.  Write the modified content back.
    5.  Run tests. The test will still fail, but now the failure will be because the implementation doesn't produce an owned, unescaped string.
*   **Increment Verification:**
    *   The `mre_from_task_test` should be updated with the unescaped string and correct indices.
    *   The test should still fail, but with a different assertion failure (actual will be borrowed, expected will be owned).
*   **Commit Message:** `test(strs_tools): Correct MRE test to expect unescaped string`

##### Increment 3: Fix the unescaping implementation
*   **Goal:** Modify `SplitIterator` to correctly unescape quoted strings.
*   **Specification Reference:** `module/core/strs_tools/task/task.md`
*   **Steps:**
    1.  Read `module/core/strs_tools/src/string/split.rs`.
    2.  In the `SplitIterator::next` method, locate the logic that handles quoted segments.
    3.  When a quoted segment is processed and `preserving_quoting` is false, call the `unescape_str` function on the content of the quote.
    4.  The result of `unescape_str` will be a `Cow<str>`. If it's `Cow::Owned`, the `Split` struct should contain this owned string.
    5.  Adjust the `start` and `end` indices of the `Split` to match the original quoted string's boundaries, as per the `task.md` (`9` and `45` in the MRE).
    6.  Write the modified content back to `split.rs`.
    7.  Run all tests. All tests should now pass.
*   **Increment Verification:**
    *   Run `cargo test -p strs_tools`. All tests, including `mre_from_task_test`, should pass.
*   **Commit Message:** `fix(strs_tools): Implement unescaping for quoted strings in split iterator`

### Task Requirements
*   The new test must be based on the MRE in `module/core/strs_tools/task/task.md`.
*   All tests must pass upon completion.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.

### Assumptions
*   The `unescape_str` function is correct and can be used.

### Out of Scope
*   Large-scale refactoring of the split iterator beyond what's necessary for the fix.

### External System Dependencies
*   N/A

### Notes & Insights
*   The `Split` struct now derives `PartialEq` and `Eq`.
*   The initial MRE test was failing due to incorrect expectation in the test itself, and the lack of unescaping in the implementation.

### Changelog
*   [2025-07-12 23:54] Renamed MRE test.
*   [2025-07-12 23:52] Received feedback to make MRE names unique and fix the failing test.
*   [2025-07-12 23:51] Added failing MRE test case for quoting and unescaping.
*   [2025-07-12 23:47] Approved permissions.
