# Task Plan: Enhance `strs_tools::split` to Support Unescaping in Quoted Strings

### Goal
*   To enhance `strs_tools::string::split` functionality to correctly parse and unescape quoted strings, treating them as single tokens and removing escape sequences when `quoting(true)` is enabled. This will resolve the issue reported by the `unilang_instruction_parser` crate.

### Ubiquitous Language (Vocabulary)
*   **Split:** A struct representing a segment of a string, which can be either a `Delimiter` or `Delimited` content.
*   **Quoted String:** A string enclosed in double quotes (`"`) that should be treated as a single token.
*   **Unescaping:** The process of converting escape sequences (e.g., `\"`, `\\`) into their literal character representations.
*   **MRE:** Minimal Reproducible Example.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/strs_tools`
*   **Overall Progress:** 9/11 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Setup and Analysis
    *   ✅ Increment 2: API Change - Use `Cow` for `Split.string`
    *   ✅ Increment 3: Fix Compilation Errors
    *   ✅ Increment 4: Implement Unescaping Logic
    *   ✅ Increment 5: Implement Quoted Segment Logic
    *   ✅ Increment 6: Fix `test_m_t3_11_quoting_preserve_all_no_strip`
    *   ✅ Increment 7: Fix `test_m_t3_13_quoting_preserve_all_strip` (combined_options)
    *   ✅ Increment 8: Fix `empty_quoted_section_test`
    *   ✅ Increment 9: Verify Fix for `test_m_t3_13_quoting_preserve_all_strip` (quoting_options)
    *   ⏳ Increment 10: Fix `mre_test` (Incorrect Escape Handling)
    *   ⚫ Increment 11: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   None

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./spec.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/strs_tools/src/string/split.rs`
    *   `module/core/strs_tools/tests/inc/split_test/quoting_options_tests.rs`
    *   `module/core/strs_tools/tests/inc/split_test/quoting_and_unescaping_tests.rs`
    *   `module/core/strs_tools/tests/inc/split_test/combined_options_tests.rs`

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `inc::split_test::quoting_and_unescaping_tests::mre_test` | Failing (New) | Incorrectly handles `\\"` sequence. |

### Crate Conformance Check Procedure
*   Step 1: Execute `timeout 90 cargo test -p strs_tools --all-targets` via `execute_command`.
*   Step 2: If the command fails, initiate `Critical Log Analysis`.
*   Step 3: If the command succeeds, execute `timeout 90 cargo clippy -p strs_tools -- -D warnings` via `execute_command`.
*   Step 4: If the command fails, initiate `Linter Fix & Regression Check Procedure`.

### Increments
##### Increment 1-9: (Completed)
*   **Summary:** Initial setup, API change to `Cow`, compilation fixes, implementation of unescaping and quoting logic, and a successful fix for the "Spurious Empty Segment Bug".

##### Increment 10: Fix `mre_test` (Incorrect Escape Handling)
*   **Goal:** To fix the "Incorrect Escape Handling Bug" where an escaped backslash (`\\`) followed by a quote is parsed incorrectly.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: **Analysis:** The `mre_test` fails because the input `r#""arg3 \\" "#` is split into `arg3` and `\\\\\"` instead of a single token `arg3 \`. The root cause is in `SplitFastIterator::next`. The `for` loop that scans for the closing quote does not correctly manage the state of the `prev_char_is_escape` flag, causing it to misinterpret the sequence `\\"`.
    *   Step 2: **Read File:** Read `module/core/strs_tools/src/string/split.rs`.
    *   Step 3: **Locate Flawed Logic:** In `SplitFastIterator::next`, find the `for` loop inside the `if let Some( current_quote_char ) = self.active_quote_char` block.
    *   Step 4: **Apply Fix:** Replace the buggy `for` loop with the corrected version that properly handles the escape state.
        ```rust
        let mut end_of_quote_idx : Option< usize > = None;
        let mut prev_char_is_escape = false;
        for ( char_idx, ch ) in self.iterable.char_indices()
        {
          if prev_char_is_escape
          {
            prev_char_is_escape = false;
          }
          else if ch == '\\'
          {
            prev_char_is_escape = true;
          }
          else if ch == current_quote_char
          {
            end_of_quote_idx = Some( char_idx + ch.len_utf8() );
            break;
          }
        }
        ```
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests -- --nocapture`.
    *   Step 2: Analyze the output. The `inc::split_test::quoting_and_unescaping_tests::mre_test` must now **pass**. All other tests must also pass.
*   **Commit Message:** `fix(strs_tools): Correctly handle escaped characters in quoted strings`

##### Increment 11: Finalization
*   **Goal:** Perform a final review and verification of the entire task's output.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Perform a self-critique against all requirements in the plan.
    *   Step 2: Run the full `Crate Conformance Check Procedure`.
    *   Step 3: Remove any temporary debug files (e.g., `debug_split_issue.rs`, `debug_hang_split_issue.rs`).
    *   Step 4: Update `changelog.md` with a summary of the changes.
*   **Increment Verification:**
    *   Step 1: All steps of the `Crate Conformance Check Procedure` must pass.
    *   Step 2: `git status` should be clean.
*   **Commit Message:** `chore(strs_tools): Finalize unescaping feature for split iterator`

### Notes & Insights
This section provides a detailed analysis of the bugs identified during testing.

#### **Bug A: Incorrect Escape Handling (`mre_test`)**
*   **Symptom:** The `mre_test` fails. The input `r#""arg3 \\" "#` is incorrectly split into two tokens (`arg3` and `\\\\\"`) instead of one (`arg3 \`).
*   **Analysis:** The test is sane and its expectation is correct. It simulates a real-world scenario where a quoted argument contains an escaped backslash. The failure proves the parser's state machine for handling escape sequences is flawed.
*   **Root Cause:** The bug is in the `next()` method of the internal `SplitFastIterator`. The `for` loop that scans for the end of a quoted string uses a simple boolean flag, `prev_char_is_escape`, to track escape sequences. This logic is insufficient for handling the sequence `\\"`. When the parser sees the first `\`, it sets the flag. When it sees the second `\`, it consumes the flag and continues. When it then sees the `"`, the flag is `false`, and it incorrectly terminates the token. The logic needs to be more robust to correctly model the state transitions. The fix involves rewriting this loop to correctly handle the escape state, ensuring that only a quote character that is *not* preceded by an active escape flag terminates the string.

#### **Bug B: Spurious Empty Segment After Quoted String**
*   **Symptom:** Four tests (`test_m_t3_13...`, `test_m_t3_11...`, `empty_quoted_section_test`) fail with an assertion error showing an extra, unexpected empty string `""` in the output. This occurs immediately after a quoted segment is parsed and only when `preserving_empty(true)` is enabled.
*   **Analysis:** The common pattern points to a flaw in how the main `SplitIterator` manages its state after its "peeking" logic consumes a quoted segment. The underlying `SplitFastIterator` is designed to yield an empty segment if the string it receives starts with a delimiter. The main iterator fails to filter out this internal, artifactual empty segment.
*   **Root Cause:** The `skip` logic within `SplitIterator::next` was flawed. The fix was to introduce a `skip_next_spurious_empty` flag that is set after the quote-peeking logic runs. This flag ensures the single, artifactual empty segment that follows a quoted string is unconditionally skipped, resolving the issue across all related test cases.

### Changelog
*   [Increment 6-9 | 2025-07-12] Fixed "Spurious Empty Segment Bug" by introducing a `skip_next_spurious_empty` flag to the iterator, which correctly filters artifactual empty tokens after a quoted segment is parsed. This resolved four related test failures.
*   [Increment 6 Plan] Updated plan to fix the two distinct bugs (Spurious Empty Segment, Incorrect Escape Handling) in separate, detailed increments based on comprehensive test failure analysis.
*   [Increment 5 | 2025-07-12] Removed debug macros from `SplitIterator`.
*   [Increment 4 | 2025-07-12] Implemented `unescape_str` function with unit tests and fixed compilation issues.
*   [Increment 3 | 2025-07-10] Fixed compilation errors after changing `Split.string` to `Cow`.
*   [Increment 2 | 2025-07-10] Changed `Split.string` to `Cow<'a, str>` to support unescaping.
*   [Increment 1 | 2025-07-10] Read relevant files for analysis.
