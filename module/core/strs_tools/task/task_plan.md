# Task Plan: Implement MRE Test for Unescaping

### Goal
*   To add a new test case to `strs_tools` that serves as a minimal reproducible example (MRE) for the unescaping and quoting issue described in `module/core/strs_tools/task/task.md`.

### Ubiquitous Language (Vocabulary)
*   **MRE:** Minimal Reproducible Example.
*   **Split:** A struct representing a segment of a split string.
*   **Unescaping:** The process of converting escape sequences (e.g., `\"`) into their literal representation (e.g., `"`).

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/strs_tools`
*   **Overall Progress:** 1/1 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Add MRE test case

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
*   The new test should replicate the MRE from `task.md`.
*   The test should assert that the output matches the expected `Split` structures, particularly for the quoted and escaped parts.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `inc::split_test::quoting_and_unescaping_tests::mre_from_task_test` | Failing (New) | The test fails as expected, because the unescaping logic is not implemented yet. |

### Crate Conformance Check Procedure
*   Run `cargo test -p strs_tools`

### Increments
##### Increment 1: Add MRE test case
*   **Goal:** Add the MRE from `task.md` as a new test in `quoting_and_unescaping_tests.rs`.
*   **Specification Reference:** `module/core/strs_tools/task/task.md`
*   **Steps:**
    1.  Read `module/core/strs_tools/tests/inc/split_test/quoting_and_unescaping_tests.rs`.
    2.  Add the new test function `mre_from_task_test` to the file. This test will be based on the MRE from `task.md`. It will check the full `Split` struct, not just the string output.
    3.  The test will construct the expected `Vec<Split<'_>>`. The `string` field for the quoted part should be the *unescaped* version as per the task description.
    4.  Use `assert_eq!` to compare the collected splits with the expected vector.
    5.  Write the updated content to `module/core/strs_tools/tests/inc/split_test/quoting_and_unescaping_tests.rs`.
    6.  Perform Increment Verification.
*   **Increment Verification:**
    1.  Execute `cargo test -p strs_tools --test strs_tools_tests -- --nocapture`.
    2.  Analyze the output. The test `mre_from_task_test` is expected to fail because the unescaping logic is not yet implemented. The other tests should pass.
*   **Commit Message:** `test(strs_tools): Add MRE for quoting and unescaping`

### Task Requirements
*   The new test must be based on the MRE in `module/core/strs_tools/task/task.md`.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.

### Assumptions
*   The bug described in `task.md` exists, so the new test is expected to fail initially.

### Out of Scope
*   Fixing the bug itself. This task is only about adding the test case.

### External System Dependencies
*   N/A

### Notes & Insights
*   The existing `mre_test` is not the same as the one in the task description.
*   The `Split` struct now derives `PartialEq` and `Eq`.

### Changelog
*   [2025-07-12 23:51] Added failing MRE test case for quoting and unescaping.
*   [2025-07-12 23:47] Approved permissions.
