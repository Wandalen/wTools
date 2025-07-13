# Task Plan: Fix unescaping bug in string splitting

### Goal
*   To fix a bug in `strs_tools::string::split` where quoted strings containing escaped quotes (`\"`) or escaped backslashes (`\\`) are not correctly unescaped when `quoting(true)` is enabled. The goal is for the `Split` struct's `string` field to contain the fully unescaped value. This plan replaces a previously stuck task.

### Ubiquitous Language (Vocabulary)
*   `strs_tools`: The Rust crate where the bug needs to be fixed.
*   `unilang_instruction_parser`: The crate that requested this change.
*   `split`: The function/module in `strs_tools` responsible for string splitting.
*   `quoting`: A feature of the `split` function to handle quoted segments.
*   `unescaping`: The process of removing escape characters (e.g., `\` from `\"`).

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** module/core/strs_tools
*   **Overall Progress:** 1/3 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Add failing test for unescaping
    *   ⏳ Increment 2: Implement unescaping fix
    *   ⚫ Increment 3: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   N/A

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `module/core/strs_tools/task/task1.md` (Original Change Proposal)
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/strs_tools/src/string/split.rs`
    *   `module/core/strs_tools/tests/inc/split_test/basic_split_tests.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`

### Expected Behavior Rules / Specifications
*   Rule 1: When `strs_tools::split` is called with `quoting(true)` on an input string like `".command "hello \\" world""`, the resulting `Split` item for the quoted part should have its `string` field contain `Cow::Owned("hello \" world")`.
*   Rule 2: For `".command "path\\\\to\\\\file""`, the `Split` item should contain `Cow::Owned("path\\to\\file")`.
*   Rule 3: All existing tests for `strs_tools` must continue to pass.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `inc::split_test::basic_split_tests::unescaping_empty_string` | Fixed (Monitored) | Fixed in Increment 2. |
| `inc::split_test::basic_split_tests::unescaping_unterminated_quote` | Failing (Stuck) | Unescaping for `\"` at the end of an unterminated quote is incorrect. |
| `inc::split_test::combined_options_tests::test_m_t3_13_quoting_preserve_all_strip` | Failing (New) | Number of segments mismatch. |
| `inc::split_test::quoting_and_unescaping_tests::empty_quoted_section_test` | Failing (New) | Number of segments mismatch. |
| `inc::split_test::quoting_options_tests::test_m_t3_11_quoting_preserve_all_no_strip` | Failing (New) | Number of segments mismatch. |
| `inc::split_test::quoting_options_tests::test_m_t3_13_quoting_preserve_all_strip` | Failing (New) | Number of segments mismatch. |

### Crate Conformance Check Procedure
*   Run `timeout 90 cargo test -p strs_tools --all-targets`.
*   Run `timeout 90 cargo clippy -p strs_tools -- -D warnings`.
*   Perform Output Cleanliness Check: Execute `cargo clean -p strs_tools` followed by `timeout 90 cargo build -p strs_tools`. Critically analyze the build output for any unexpected debug prints from procedural macros.

### Increments
##### Increment 1: Add failing test for unescaping
*   **Goal:** Add a new test case to `strs_tools` that specifically targets the unescaping bug for `\"` and `\\` within quoted strings, and confirm that it fails as expected.
*   **Specification Reference:** `module/core/strs_tools/task/task1.md` - Acceptance Criteria
*   **Steps:**
    *   Step 1: Read the content of `module/core/strs_tools/tests/inc/split_test/basic_split_tests.rs` to understand the existing test structure.
    *   Step 2: Add a new test function `unescaping_in_quoted_string` to the file. This test should cover both `\"` and `\\` cases.
    *   Step 3: Perform Increment Verification.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests -- --nocapture` via `execute_command`.
    *   Step 2: Analyze the output to confirm that the new test `unescaping_in_quoted_string` fails. The failure message should indicate an assertion error related to incorrect unescaping. This confirms the bug and the validity of the test.
*   **Commit Message:** `test(strs_tools): Add failing test for unescaping in quoted strings`

##### Increment 2: Implement unescaping fix
*   **Goal:** Modify the string splitting logic in `src/string/split.rs` to correctly handle escaped characters within quoted segments and empty quoted strings, making the failing tests pass.
*   **Specification Reference:** `module/core/strs_tools/task/task1.md` - Proposed Solution
*   **Steps:**
    *   Step 1: Read the content of `module/core/strs_tools/src/string/split.rs`.
    *   Step 2: Fix the `unescaping_empty_string` bug by modifying the `SplitIterator::next()` method.
    *   Step 3: Fix the `unescaping_unterminated_quote` bug by modifying the `SplitIterator::next()` method.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p strs_tools --test strs_tools_tests -- --nocapture` via `execute_command`.
    *   Analyze the output to confirm that the `unescaping_in_quoted_string` test now passes.
*   **Commit Message:** fix(strs_tools): Correctly unescape characters in quoted strings

##### Increment 2.1: Focused Debugging: Diagnose and fix `unescaping_unterminated_quote`
*   **Goal:** Diagnose and fix the `Failing (Stuck)` test: `inc::split_test::basic_split_tests::unescaping_unterminated_quote`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step A: Apply Problem Decomposition. Analyze the failing test and determine if it can be broken down into smaller, more focused tests, or if its setup can be simplified.
    *   Step B: Isolate the test case.
    *   Step C: Add targeted debug logging.
    *   Step D: Review related code changes since the test last passed.
    *   Step E: Formulate and test a hypothesis.
    *   Upon successful fix, document the root cause and solution in the `### Notes & Insights` section.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests -- --nocapture` via `execute_command`.
    *   Analyze the output to confirm that `inc::split_test::basic_split_tests::unescaping_unterminated_quote` now passes.
*   **Commit Message:** `fix(test): Resolve stuck test inc::split_test::basic_split_tests::unescaping_unterminated_quote`

##### Increment 3: Finalization
*   **Goal:** Perform final review and verification of the entire task.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Self-Critique: Review all changes against task goals and requirements.
    *   Step 2: Execute Test Quality and Coverage Evaluation.
    *   Step 3: Full Conformance Check: Run `Crate Conformance Check Procedure` on all editable crates.
    *   Step 4: Final Output Cleanliness Check.
    *   Step 5: Final Status Check: Run `git status`.
*   **Increment Verification:**
    *   Confirm all checks pass.
*   **Commit Message:** chore(task): Complete unescaping fix task and update status

### Task Requirements
*   The fix must correctly unescape `\"` and `\\` sequences within quoted strings.
*   A new test case must be added to verify the fix.
*   All existing tests must pass.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.

### Assumptions
*   The `unescape_str` function in `strs_tools::string::split` is correct and does not need changes; the bug is in the logic that provides input to it.

### Out of Scope
*   Addressing any other bugs or warnings.
*   Refactoring unrelated code.

### External System Dependencies (Optional)
*   N/A

### Notes & Insights
*   This task is being started because the previous task in `task_plan.md` was stuck.
*   The test `unescaping_in_quoted_string` already
