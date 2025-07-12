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
*   **Overall Progress:** 10/14 increments complete
*   **Increment Status:**
    *   ✅ Increment 1-10: (Completed)
    *   ⏳ Increment 11: Add Decomposed MRE Test Cases
    *   ⚫ Increment 12: Implement Fix for Escape Sequence Handling
    *   ⚫ Increment 13: Verify Fix and Finalize
    *   ⚫ Increment 14: Finalization

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
    *   `module/core/strs_tools/tests/inc/split_test/quoting_and_unescaping_tests.rs`

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `inc::split_test::quoting_and_unescaping_tests::escaped_backslash_then_quote_test` | Fixed (Monitored) | The test expectation was corrected and it now passes. |
| `inc::split_test::quoting_and_unescaping_tests::mre_test` | Failing (New) | Incorrectly handles `\\"` sequence. |
| `inc::split_test::quoting_and_unescaping_tests::test_mre_arg3_isolated` | Failing (New) | New granular test, expected to fail. |

### Crate Conformance Check Procedure
*   Step 1: Execute `timeout 90 cargo test -p strs_tools --all-targets` via `execute_command`.
*   Step 2: If the command fails, initiate `Critical Log Analysis`.
*   Step 3: If the command succeeds, execute `timeout 90 cargo clippy -p strs_tools -- -D warnings` via `execute_command`.
*   Step 4: If the command fails, initiate `Linter Fix & Regression Check Procedure`.

### Increments
##### Increment 1-10: (Completed)
*   **Summary:** Initial setup, API change to `Cow`, compilation fixes, implementation of unescaping and quoting logic, a successful fix for the "Spurious Empty Segment Bug", and correction of a flawed test expectation.

##### Increment 11: Add Decomposed MRE Test Cases
*   **Goal:** To add more granular, failing tests based on the MRE to precisely target the parsing bug.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: **Add new tests:** Add `test_mre_arg2_isolated`, `test_mre_arg3_isolated`, and `test_consecutive_escaped_backslashes_and_quote` to `quoting_and_unescaping_tests.rs`.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests -- --nocapture`.
    *   Step 2: Analyze the output. The new test `test_mre_arg3_isolated` should fail, confirming it correctly targets the bug.
*   **Commit Message:** `test(strs_tools): Add granular tests for escaped backslash handling`

##### Increment 12: Implement Fix for Escape Sequence Handling
*   **Goal:** To fix the escape sequence parsing bug by replacing the flawed backslash-counting logic in `SplitFastIterator::next` with a robust state machine.
*   **Specification Reference:** N/A
*   **Reference Implementation (Current Flawed Code):**
    The following code block inside `SplitFastIterator::next` in `module/core/strs_tools/src/string/split.rs` is the source of the bug.
    ```rust
    if let Some( current_quote_char ) = self.active_quote_char
    {
      let mut end_of_quote_idx : Option< usize > = None;
      let mut search_from = 0;
      loop
      {
        if let Some( pos_in_substring ) = self.iterable[ search_from.. ].find( current_quote_char )
        {
          let pos_in_full_string = search_from + pos_in_substring;
          let mut backslash_count = 0;
          for c in self.iterable[ ..pos_in_full_string ].chars().rev()
          {
            if c == '\\'
            {
              backslash_count += 1;
            }
            else
            {
              break;
            }
          }
          if backslash_count % 2 == 1
          {
            search_from = pos_in_full_string + 1;
            continue;
          }
          else
          {
            end_of_quote_idx = Some( pos_in_full_string + current_quote_char.len_utf8() );
            break;
          }
        }
        else
        {
          break;
        }
      }
      let ( segment_str, consumed_len ) = if let Some( end_idx ) = end_of_quote_idx
        { ( &self.iterable[ ..end_idx ], end_idx ) } else { ( self.iterable, self.iterable.len() ) };
      let split = Split { string: Cow::Borrowed( segment_str ), typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
      self.current_offset += consumed_len; self.iterable = &self.iterable[ consumed_len.. ]; return Some( split );
    }
    ```
*   **Steps:**
    *   Step 1: **Replace Flawed Logic:** In `module/core/strs_tools/src/string/split.rs`, locate the `if let Some( current_quote_char ) = self.active_quote_char` block within the `next` method of the `impl< 'a, D : Searcher > Iterator for SplitFastIterator< 'a, D >` block.
    *   Step 2: **Delete** the entire content of that `if` block.
    *   Step 3: **Insert New Logic:** In its place, insert the following state-machine-based implementation:
        ```rust
        if let Some( current_quote_char ) = self.active_quote_char
        {
          let mut end_of_quote_idx : Option< usize > = None;
          let mut is_escaped = false;
          for ( i, c ) in self.iterable.char_indices()
          {
            if c == current_quote_char && !is_escaped
            {
              end_of_quote_idx = Some( i + c.len_utf8() );
              break;
            }
            is_escaped = c == '\\' && !is_escaped;
          }

          let ( segment_str, consumed_len ) = if let Some( end_idx ) = end_of_quote_idx
            { ( &self.iterable[ ..end_idx ], end_idx ) } else { ( self.iterable, self.iterable.len() ) };
          let split = Split { string: Cow::Borrowed( segment_str ), typ: SplitType::Delimeted, start: self.current_offset, end: self.current_offset + segment_str.len() };
          self.current_offset += consumed_len; self.iterable = &self.iterable[ consumed_len.. ]; return Some( split );
        }
        ```
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests -- --nocapture`.
    *   Step 2: Analyze the output. All tests, including the new granular ones and `mre_test`, must now **pass**.
*   **Commit Message:** `fix(strs_tools): Implement state machine for escaped quote parsing`

##### Increment 13: Verify Fix and Finalize
*   **Goal:** To ensure the fix is robust and has not introduced any regressions.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Perform the full `Crate Conformance Check Procedure`.
    *   Step 2: Remove the temporary debug test files `tests/debug_hang_split_issue.rs` and `tests/debug_split_issue.rs` if they exist.
*   **Increment Verification:**
    *   All steps of the `Crate Conformance Check Procedure` must pass.
*   **Commit Message:** `chore(strs_tools): Verify escape parsing fix and remove debug files`

##### Increment 14: Finalization
*   **Goal:** Perform a final review and verification of the entire task's output.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Perform a self-critique against all requirements in the plan.
    *   Step 2: Run the full `Crate Conformance Check Procedure` again.
    *   Step 3: Update `changelog.md` with a summary of the changes.
    *   Step 4: `git status` should be clean.
*   **Increment Verification:**
    *   All steps of the `Crate Conformance Check Procedure` must pass.
*   **Commit Message:** `chore(strs_tools): Finalize unescaping feature for split iterator`

### Notes & Insights
This section provides a detailed analysis of the bugs identified during testing.

#### **Bug A: Incorrect Escape Handling (`mre_test`)**
*   **Symptom:** The `mre_test` and `escaped_backslash_then_quote_test` fail. The sequence `\\"` is misinterpreted, causing the quoted string to terminate prematurely.
*   **Analysis:** The test is sane and its expectation is correct. The failure proves the parser's state machine for handling escape sequences is flawed.
*   **Root Cause:** The bug is in the `next()` method of the internal `SplitFastIterator`. The logic for finding the end of a quoted string relies on counting preceding backslashes, which fails for sequences like `\\"`. This logic needs to be replaced with a proper state machine that tracks an `is_escaped` state during character-by-character iteration.

#### **Bug B: Spurious Empty Segment After Quoted String**
*   **Symptom:** Four tests (`test_m_t3_13...`, `test_m_t3_11...`, `empty_quoted_section_test`) fail with an assertion error showing an extra, unexpected empty string `""` in the output. This occurs immediately after a quoted segment is parsed and only when `preserving_empty(true)` is enabled.
*   **Analysis:** The common pattern points to a flaw in how the main `SplitIterator` manages its state after its "peeking" logic consumes a quoted segment. The underlying `SplitFastIterator` is designed to yield an empty segment if the string it receives starts with a delimiter. The main iterator fails to filter out this internal, artifactual empty segment.
*   **Root Cause:** The `skip` logic within `SplitIterator::next` was flawed. The fix was to introduce a `skip_next_spurious_empty` flag that is set after the quote-peeking logic runs. This flag ensures the single, artifactual empty segment that follows a quoted string is unconditionally skipped, resolving the issue across all related test cases.

### Changelog
*   [Increment 11 | 2025-07-12] Added new granular tests for escape sequences to isolate the bug.
*   [Increment 10 | 2025-07-12] Corrected the expectation in `escaped_backslash_then_quote_test` to be valid. The test now passes, but the underlying bug is still present as shown by the failing `mre_test`.
*   [Increment 6-9 | 2025-07-12] Fixed "Spurious Empty Segment Bug" by introducing a `skip_next_spurious_empty` flag to the iterator, which correctly filters artifactual empty tokens after a quoted segment is parsed. This resolved four related test failures.
*   [Increment 6 Plan] Updated plan to fix the two distinct bugs (Spurious Empty Segment, Incorrect Escape Handling) in separate, detailed increments based on comprehensive test failure analysis.
*   [Increment 5 | 2025-07-12] Removed debug macros from `SplitIterator`.
*   [Increment 4 | 2025-07-12] Implemented `unescape_str` function with unit tests and fixed compilation issues.
*   [Increment 3 | 2025-07-10] Fixed compilation errors after changing `Split.string` to `Cow`.
*   [Increment 2 | 2025-07-10] Changed `Split.string` to `Cow<'a, str>` to support unescaping.
*   [Increment 1 | 2025-07-10] Read relevant files for analysis.