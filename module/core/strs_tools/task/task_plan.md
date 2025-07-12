
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
*   **Overall Progress:** 5/11 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Setup and Analysis
    *   ✅ Increment 2: API Change - Use `Cow` for `Split.string`
    *   ✅ Increment 3: Fix Compilation Errors
    *   ✅ Increment 4: Implement Unescaping Logic
    *   ✅ Increment 5: Implement Quoted Segment Logic
    *   ⏳ Increment 6: Fix `test_m_t3_11_quoting_preserve_all_no_strip`
    *   ⚫ Increment 7: Fix `test_m_t3_13_quoting_preserve_all_strip` (combined_options)
    *   ⚫ Increment 8: Fix `empty_quoted_section_test`
    *   ⚫ Increment 9: Verify Fix for `test_m_t3_13_quoting_preserve_all_strip` (quoting_options)
    *   ⚫ Increment 10: Fix `mre_test` (Incorrect Escape Handling)
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

### Crate Conformance Check Procedure
*   Step 1: Execute `timeout 90 cargo test -p strs_tools --all-targets` via `execute_command`.
*   Step 2: If the command fails, initiate `Critical Log Analysis`.
*   Step 3: If the command succeeds, execute `timeout 90 cargo clippy -p strs_tools -- -D warnings` via `execute_command`.
*   Step 4: If the command fails, initiate `Linter Fix & Regression Check Procedure`.

### Increments
##### Increment 1-5: (Completed)
*   **Summary:** Initial setup, API change to `Cow`, compilation fixes, and implementation of unescaping and basic quoting logic.

##### Increment 6: Fix `test_m_t3_11_quoting_preserve_all_no_strip`
*   **Goal:** To fix the first instance of the "Spurious Empty Segment Bug" as identified in the `test_m_t3_11_quoting_preserve_all_no_strip` test.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: **Analysis:** This test, with input `a 'b c' d`, fails because the iterator produces `[..., "'b c'", "", " ", ...]` instead of `[..., "'b c'", " ", ...]`. This is the primary manifestation of the "Spurious Empty Segment Bug," where an extra empty token is yielded after a quoted segment when `preserving_empty(true)` is active.
    *   Step 2: **Hypothesis:** The root cause is the flawed `skip` logic in `SplitIterator::next`. The two separate `if { continue; }` blocks for skipping delimiters and empty segments do not correctly handle the state after the iterator's internal "peeking" logic has processed a quote.
    *   Step 3: **Read File:** Read `module/core/strs_tools/src/string/split.rs`.
    *   Step 4: **Apply Fix:** In `SplitIterator::next`, replace the two separate `if` blocks for skipping with a single, consolidated `if skip { continue; }` block. This makes the filtering logic atomic and robust.
        ```rust
        let skip = ( current_split.typ == SplitType::Delimeted && current_split.string.is_empty() && !self.flags.contains( SplitFlags::PRESERVING_EMPTY ) )
        || ( current_split.typ == SplitType::Delimiter && !self.flags.contains( SplitFlags::PRESERVING_DELIMITERS ) );

        if skip
        {
          continue;
        }
        ```
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests -- --nocapture`.
    *   Step 2: Analyze the output. The test `inc::split_test::quoting_options_tests::test_m_t3_11_quoting_preserve_all_no_strip` must now **pass**. Other tests related to this bug may also pass. The `mre_test` is expected to still fail.
*   **Commit Message:** `fix(strs_tools): Prevent extra empty segment after quoted strings`

##### Increment 7: Fix `test_m_t3_13_quoting_preserve_all_strip` (combined_options)
*   **Goal:** To ensure the fix for the "Spurious Empty Segment Bug" is robust and also works when `stripping(true)` is enabled, by fixing the `test_m_t3_13_quoting_preserve_all_strip` test.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: **Analysis:** This test is nearly identical to the one fixed in the previous increment but adds the `stripping(true)` option. It fails for the same reason: an extra empty segment is produced.
    *   Step 2: **Hypothesis:** The fix applied in Increment 6 should have already resolved this test case. This increment serves as a verification of that fix's robustness against a slightly different configuration. No new code changes are anticipated.
    *   Step 3: **Verification:** If the test still fails, it indicates the initial fix was incomplete and did not properly account for the interaction with the `stripping` flag. In this case, the `skip` logic from Increment 6 would need to be re-analyzed and refined.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests -- --nocapture`.
    *   Step 2: Analyze the output. The test `inc::split_test::combined_options_tests::test_m_t3_13_quoting_preserve_all_strip` must now **pass**.
*   **Commit Message:** `refactor(strs_tools): Verify fix for spurious segment with stripping enabled`

##### Increment 8: Fix `empty_quoted_section_test`
*   **Goal:** To ensure the fix for the "Spurious Empty Segment Bug" also handles the edge case of an empty quoted section (`""`).
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: **Analysis:** This test fails with input `a "" b`, producing `["a", "", "", "b"]` instead of the correct `["a", "", "b"]`. It yields one correct empty segment for the `""` token, but then a second, incorrect empty segment. This is another manifestation of the same core bug.
    *   Step 2: **Hypothesis:** The consolidated `skip` logic from Increment 6 should be sufficient to fix this. This increment verifies the fix's behavior with empty quoted strings.
    *   Step 3: **Verification:** If the test still fails, it implies the logic needs further refinement to correctly handle the state transition after an *empty* quoted segment.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests -- --nocapture`.
    *   Step 2: Analyze the output. The test `inc::split_test::quoting_and_unescaping_tests::empty_quoted_section_test` must now **pass**.
*   **Commit Message:** `refactor(strs_tools): Verify fix for spurious segment with empty quotes`

##### Increment 9: Verify Fix for `test_m_t3_13_quoting_preserve_all_strip` (quoting_options)
*   **Goal:** To confirm the fix for the "Spurious Empty Segment Bug" is fully resolved by checking the final failing test case related to it.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: **Analysis:** This test is identical in configuration to the one in Increment 7, just located in a different test module. It serves as a final confirmation that the bug is eradicated.
    *   Step 2: **Hypothesis:** This test should already be passing due to the fix from Increment 6. This increment is purely for verification.
    *   Step 3: **Verification:** No code changes are expected.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests -- --nocapture`.
    *   Step 2: Analyze the output. The test `inc::split_test::quoting_options_tests::test_m_t3_13_quoting_preserve_all_strip` must now **pass**. At this point, only `mre_test` should be failing.
*   **Commit Message:** `test(strs_tools): Confirm fix for all spurious segment test cases`

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
*   **Root Cause:** The `skip` logic within `SplitIterator::next` is implemented as two separate `if` blocks. This separation is flawed. After the peeking logic for quotes runs, the state is not correctly communicated, and the subsequent check for empty segments is not evaluated properly, allowing the spurious empty segment to be yielded. The fix is to consolidate all skipping conditions (for empty segments and for delimiters) into a single, combined `if skip { continue; }` block. This makes the filtering logic atomic and robust, correctly handling the state after a quote has been processed.

### Changelog
*   [Increment 6 Plan] Updated plan to fix the two distinct bugs (Spurious Empty Segment, Incorrect Escape Handling) in separate, detailed increments based on comprehensive test failure analysis.
*   [Increment 5 | 2025-07-12] Removed debug macros from `SplitIterator`.
*   [Increment 4 | 2025-07-12] Implemented `unescape_str` function with unit tests and fixed compilation issues.
*   [Increment 3 | 2025-07-10] Fixed compilation errors after changing `Split.string` to `Cow`.
*   [Increment 2 | 2025-07-10] Changed `Split.string` to `Cow<'a, str>` to support unescaping.
*   [Increment 1 | 2025-07-10] Read relevant files for analysis.
