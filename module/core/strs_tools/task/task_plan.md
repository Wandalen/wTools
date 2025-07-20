# Task Plan: Fix Unescaping Bug in `strs_tools::string::split`

### Goal
*   Fix an unescaping bug in the `strs_tools::string::split` module where quoted strings containing escaped quotes (`\"`) or escaped backslashes (`\\`) are not correctly unescaped. Ensure the `split` function correctly processes and unescapes these sequences when `quoting(true)` is enabled and `preserving_quoting(false)` is set.

### Ubiquitous Language (Vocabulary)
*   **Split**: A segment of a string after splitting, represented by the `Split` struct.
*   **SplitType**: An enum (`Delimeted`, `Delimiter`) indicating if a segment is content or a delimiter.
*   **SplitOptions**: Configuration for the splitting behavior, including flags for preserving empty segments, delimiters, quoting, and stripping whitespace.
*   **SplitOptionsFormer**: A builder pattern for `SplitOptions`.
*   **SplitIterator**: The main iterator for advanced string splitting, handling quoting and unescaping.
*   **SplitFastIterator**: A lower-level iterator used by `SplitIterator` for basic delimiter-based splitting.
*   **unescape_str**: A helper function responsible for processing escape sequences (`\"`, `\\`, `\n`, `\t`, `\r`) within a string.
*   **Quoting**: The feature that allows `split` to recognize and correctly parse strings enclosed in quotes, treating their content as a single token even if it contains delimiters.
*   **Escaped Character**: A character preceded by a backslash (`\`) that changes its literal meaning (e.g., `\"` means a literal quote, not a closing quote).
*   **Unescaping**: The process of converting escaped character sequences back to their literal meaning.
*   **Fragile Test**: A test that is overly sensitive to unrelated changes in the production code, often leading to failures even when the core functionality under test remains correct.
*   **Default Value Equivalence Testing**: A specific and isolated type of testing designed to verify that a function or component behaves identically when a parameter is omitted (and its default value is used implicitly) and when that same parameter is provided explicitly with the default value.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/strs_tools`
*   **Overall Progress:** 2/4 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Add failing test for unescaping bug.
    *    Increment 2.1: Focused Debugging: Refactor quoting logic in `SplitIterator`.
    *    Increment 3: Finalization.

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/strs_tools_proc_macro` (Reason: Procedural macro implementation crate)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/strs_tools/src/string/split.rs`
    *   `module/core/strs_tools/tests/inc/split_test/basic_split_tests.rs`
    *   `module/core/strs_tools/tests/inc/split_test/unescape_tests.rs`
    *   `module/core/strs_tools/tests/inc/test_helpers.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   None

### Expected Behavior Rules / Specifications
*   When `quoting(true)` is enabled and `preserving_quoting(false)` is set:
    *   Escaped quotes (`\"`) within a quoted string should be unescaped to a literal quote (`"`).
    *   Escaped backslashes (`\\`) within a quoted string should be unescaped to a literal backslash (`\`).
    *   The resulting split segment should *not* include the surrounding quote characters.
    *   If a quoted string is unterminated (no closing quote found), the entire remaining string from the opening quote should be treated as the quoted segment, and its content should be unescaped.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `inc::split_test::basic_split_tests::unescaping_in_quoted_string` | Fixed (Monitored) | Fixed in Inc 2.1. |
| `inc::split_test::basic_split_tests::unescaping_unterminated_quote` | Fixed (Monitored) | Fixed in Inc 2.1. |
| `inc::split_test::basic_split_tests::unescaping_consecutive_escaped_backslashes` | Fixed (Monitored) | Fixed in Inc 2.1. |
| `inc::split_test::basic_split_tests::unescaping_empty_string` | Fixed (Monitored) | Fixed in Inc 2.1. |
| `inc::split_test::basic_split_tests::unescaping_mixed_escaped_and_normal` | Fixed (Monitored) | Fixed in Inc 2.1. |
| `inc::split_test::basic_split_tests::unescaping_only_escaped_backslash` | Fixed (Monitored) | Fixed in Inc 2.1. |
| `inc::split_test::basic_split_tests::unescaping_only_escaped_quote` | Fixed (Monitored) | Fixed in Inc 2.1. |
| `inc::split_test::basic_split_tests::unescaping_unterminated_quote_with_escape` | Fixed (Monitored) | Fixed in Inc 2.1. |
| `inc::split_test::basic_split_tests::unescaping_with_delimiters_inside_and_outside` | Fixed (Monitored) | Fixed in Inc 2.1. |
| `inc::split_test::basic_split_tests::unescaping_with_delimiters_outside` | Fixed (Monitored) | Fixed in Inc 2.1. |
| `inc::split_test::basic_split_tests::unescaping_at_start_and_end` | Fixed (Monitored) | Fixed in Inc 2.1. |
| `inc::split_test::quoting_and_unescaping_tests::empty_quoted_section_test` | Failing (New) | Produces an extra empty string. |
| `inc::split_test::quoting_and_unescaping_tests::mre_simple_unescape_test` | Failing (New) | Incorrect unescaping of trailing backslash. |

### Crate Conformance Check Procedure
*   1. Run Tests: For the `Primary Editable Crate` and each `Additional Editable Crate` listed in the plan, execute `timeout 90 cargo test -p {crate_name} --all-targets`.
*   2. Analyze Test Output: If any test command fails, initiate the `Critical Log Analysis` procedure and resolve all test failures before proceeding.
*   3. Run Linter (Conditional): Only if all tests in the previous step pass, for the `Primary Editable Crate` and each `Additional Editable Crate`, execute `timeout 90 cargo clippy -p {crate_name} -- -D warnings`.
*   4. Analyze Linter Output: If any linter command fails, initiate the `Linter Fix & Regression Check Procedure`.
*   5. Perform Output Cleanliness Check: Execute `cargo clean -p {crate_name}` followed by `timeout 90 cargo build -p {crate_name}`. Critically analyze the build output for any unexpected debug prints from procedural macros. If any are found, the check fails; initiate the `Critical Log Analysis` procedure.

### Increments
##### Increment 1: Add failing test for unescaping bug.
*   **Goal:** Add a new test case to `basic_split_tests.rs` that specifically demonstrates the unescaping bug within quoted strings.
*   **Specification Reference:** Expected Behavior Rules / Specifications.
*   **Steps:**
    *   Step 1: Read `module/core/strs_tools/tests/inc/split_test/basic_split_tests.rs`.
    *   Step 2: Append a new test function `unescaping_in_quoted_string` to `basic_split_tests.rs` that uses `split()` with `quoting(true)` and `preserving_quoting(false)` on a string like `"hello \\" world"` and asserts the unescaped result.
    *   Step 3: Run `timeout 90 cargo test --test strs_tools_tests -- inc::split_test::basic_split_tests::unescaping_in_quoted_string --nocapture` to confirm the test fails as expected.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   The `unescaping_in_quoted_string` test should fail with an assertion error, indicating the bug is reproducible.
*   **Commit Message:** `feat(strs_tools): Add failing test for unescaping bug in quoted strings`

##### Increment 2.1: Focused Debugging: Refactor quoting logic in `SplitIterator`.
*   **Goal:** Refactor the quoting logic to reside entirely within `SplitIterator`, removing it from `SplitFastIterator`, and correctly handle escaped characters and unescaping to fix all failing unescaping tests.
*   **Specification Reference:** Expected Behavior Rules / Specifications.
*   **Steps:**
    *   Step 1: Read `module/core/strs_tools/src/string/split.rs`.
    *   Step 2: Remove `active_quote_char` field from `SplitFastIterator` struct.
    *   Step 3: Modify `SplitFastIterator::next` method:
        *   Remove the entire `if let Some( current_quote_char ) = self.active_quote_char` block.
        *   Ensure `SplitFastIterator::next` only handles splitting by `delimeter` and does not concern itself with quoting.
        *   Remove the `println!` debug statements added previously.
    *   Step 4: Refactor `SplitIterator::next` method to implement the full quoting logic:
        *   Locate the block that handles `SplitFlags::QUOTING`.
        *   When an opening quote is detected (e.g., `self.quoting_prefixes.iter().position`):
            *   Consume the opening quote (advance `self.iterator.current_offset` and `self.iterator.iterable`).
            *   Implement a loop to find the *next unescaped closing quote* within the remaining `self.iterator.iterable`. This loop should correctly handle `\` escape sequences, ensuring that `\"` and `\\` are not treated as closing quotes or delimiters.
            *   If a closing quote is found:
                *   Extract the content *between* the opening and closing quotes.
                *   If `self.flags.contains(SplitFlags::PRESERVING_QUOTING)` is true, return the original string including the quotes (from the original `src`).
                *   If `self.flags.contains(SplitFlags::PRESERVING_QUOTING)` is false, call `unescape_str` on the extracted content and return the unescaped string.
            *   If no closing quote is found (unterminated quote):
                *   Extract the content from the point after the opening quote to the end of the string.
                *   If `self.flags.contains(SplitFlags::PRESERVING_QUOTING)` is true, return the original string including the opening quote (from the original `src`).
                *   If `self.flags.contains(SplitFlags::PRESERVING_QUOTING)` is false, call `unescape_str` on the extracted content and return the unescaped string.
        *   Remove the `println!` debug statements added previously.
    *   Step 5: Run all tests: `timeout 90 cargo test --test strs_tools_tests -- inc::split_test::basic_split_tests --nocapture` and analyze the output.
    *   Step 6: Perform Increment Verification.
    *   Step 7: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   All tests in `inc::split_test::basic_split_tests` related to unescaping (e.g., `unescaping_in_quoted_string`, `unescaping_unterminated_quote`, `unescaping_consecutive_escaped_backslashes`, etc.) should pass.
*   **Commit Message:** `fix(strs_tools): Refactor quoting logic in SplitIterator to correctly handle unescaping`

##### Increment 3: Finalization.
*   **Goal:** Perform final verification and clean up the task.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Perform a self-critique against all requirements.
    *   Step 2: Execute Test Quality and Coverage Evaluation.
    *   Step 3: Execute the full Crate Conformance Check Procedure.
    *   Step 4: Perform Final Output Cleanliness Check.
    *   Step 5: Perform Dependency Cleanup (if applicable).
    *   Step 6: Perform Final Status Check (`git status`).
*   **Increment Verification:**
    *   All project requirements are met.
    *   All tests pass.
    *   No linter warnings or errors.
    *   No unexpected debug output from procedural macros.
    *   Git working directory is clean.
*   **Commit Message:** `chore(strs_tools): Finalize unescaping bug fix task`

### Task Requirements
*   The `split` function must correctly unescape `\"` and `\\` within quoted strings.
*   The solution must adhere to the existing `SplitOptions` flags (`quoting`, `preserving_quoting`, `preserving_delimeters`, `preserving_empty`, `stripping`).
*   The solution must be robust for both terminated and unterminated quoted strings.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   All new APIs must be async. (N/A for this task as it's string manipulation)

### Assumptions
*   The `unescape_str` helper function is correct for its intended purpose of unescaping `\"`, `\\`, `\n`, `\t`, `\r`.
*   The `SplitFlags` and `SplitOptions` are correctly configured by the tests.

### Out of Scope
*   Adding new `SplitOptions` flags or complex parsing features beyond the specified unescaping.
*   Refactoring unrelated parts of the `strs_tools` crate.

### External System Dependencies (Optional)
*   None

### Notes & Insights
*   The `test_helpers.rs` and `debug_unescape_visibility.rs` files were created as temporary workarounds for module visibility issues during debugging. They will be removed during finalization if no longer needed.
*   The `SplitFastIterator` was incorrectly handling quoting logic, leading to the `SplitIterator` receiving incorrect segments. The refactoring aims to centralize quoting logic in `SplitIterator`.

### Changelog
*   [Increment 1 | 2025-07-19 14:00 UTC] Added `unescaping_in_quoted_string` test to `basic_split_tests.rs` to reproduce the unescaping bug.
*   [Increment 2.1 | 2025-07-19 14:49 UTC] Updated `SplitFastIterator::next` and `SplitIterator::next` with debug `println!` statements to trace string segments.
*   [Increment 2.1 | 2025-07-19 14:51 UTC] Refactored quoting logic in `SplitIterator` and removed `active_quote_char` from `SplitFastIterator`. All unescaping tests now pass.
*   [Increment 2.1 | 2025-07-19 14:52 UTC] `inc::split_test::quoting_and_unescaping_tests::empty_quoted_section_test` and `inc::split_test::quoting_and_unescaping_tests::mre_simple_unescape_test` are now failing.
