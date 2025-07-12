
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
*   **Overall Progress:** 5/8 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Setup and Analysis
    *   ✅ Increment 2: API Change - Use `Cow` for `Split.string`
    *   ✅ Increment 3: Fix Compilation Errors
    *   ✅ Increment 4: Implement Unescaping Logic
    *   ✅ Increment 5: Implement Quoted Segment Logic
    *   ⏳ Increment 6: Fix Spurious Empty Segment Bug
    *   ⚫ Increment 7: Fix Incorrect Escape Handling Bug
    *   ⚫ Increment 8: Finalization

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
    *   `module/core/strs_tools/src/lib.rs`
    *   `module/core/strs_tools/Cargo.toml`
    *   `module/core/strs_tools/tests/inc/split_test/quoting_options_tests.rs`
    *   `module/core/strs_tools/tests/inc/split_test/quoting_and_unescaping_tests.rs`
    *   `module/core/strs_tools/tests/inc/split_test/combined_options_tests.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`

### Expected Behavior Rules / Specifications
*   Rule 1: When `quoting(true)` is enabled, a string like `"a b"` with an internal space delimiter should be returned as a single `Delimited` token with the content `a b`.
*   Rule 2: When `quoting(true)` is enabled, escape sequences like `\"` and `\\` inside a quoted string must be unescaped in the final `Split.string` value.
*   Rule 3: The `Split.string` field should be changed to `Cow<'a, str>` to accommodate both borrowed slices (for non-quoted/non-unescaped content) and owned strings (for unescaped content).

### Crate Conformance Check Procedure
*   Step 1: Execute `timeout 90 cargo test -p strs_tools --all-targets` via `execute_command`.
*   Step 2: If the command fails, initiate `Critical Log Analysis`.
*   Step 3: If the command succeeds, execute `timeout 90 cargo clippy -p strs_tools -- -D warnings` via `execute_command`.
*   Step 4: If the command fails, initiate `Linter Fix & Regression Check Procedure`.
*   Step 5: If the command succeeds, perform `Output Cleanliness Check` by running `cargo clean -p strs_tools` then `timeout 90 cargo build -p strs_tools` and analyzing the output for debug prints.

### Increments
##### Increment 1: Setup and Analysis
*   **Goal:** Read all relevant files to build a complete understanding of the current implementation of the `split` iterator and its tests.
*   **Commit Message:** `chore(strs_tools): Begin refactoring of split iterator for unescaping`

##### Increment 2: API Change - Use `Cow` for `Split.string`
*   **Goal:** Modify the `Split` struct to use `Cow<'a, str>` for its `string` field to support returning owned, unescaped strings.
*   **Commit Message:** `feat(strs_tools): Change Split.string to Cow to support unescaping`

##### Increment 3: Fix Compilation Errors
*   **Goal:** Resolve all compilation errors caused by the change of `Split.string` to `Cow<'a, str>`.
*   **Commit Message:** `fix(strs_tools): Adapt codebase to Cow-based Split.string`

##### Increment 4: Implement Unescaping Logic
*   **Goal:** Implement the core logic to unescape characters within a string slice.
*   **Commit Message:** `feat(strs_tools): Implement unescaping logic for string splitting`

##### Increment 5: Implement Quoted Segment Logic
*   **Goal:** Modify the `SplitIterator` to correctly identify and consume an entire quoted string as a single token, and apply the new unescaping logic.
*   **Commit Message:** `feat(strs_tools): Make split iterator consume full quoted strings and unescape them`

##### Increment 6: Fix Spurious Empty Segment Bug
*   **Goal:** To fix the bug where an extra empty segment is incorrectly yielded after a quoted segment when `preserving_empty(true)` is enabled.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: **Analysis:** The tests `test_m_t3_13_quoting_preserve_all_strip`, `empty_quoted_section_test`, and `test_m_t3_11_quoting_preserve_all_no_strip` all fail because an extra empty `""` is produced after a quoted segment. This happens because the main `SplitIterator`'s `next` method has special "peeking" logic for quotes. After it consumes a quoted segment, the underlying `SplitFastIterator` is given the rest of the string (e.g., ` d`). Because this starts with a delimiter, the fast iterator correctly yields an empty segment first. The main iterator's `skip` logic is flawed and fails to filter this artifact.
    *   Step 2: **Read File:** Read `module/core/strs_tools/src/string/split.rs`.
    *   Step 3: **Locate Flawed Logic:** In `SplitIterator::next`, find the two separate `if` blocks that use `continue` to skip segments.
    *   Step 4: **Apply Fix:** Replace the two separate `if` blocks with a single, consolidated `if skip { ... }` block. The `skip` variable will combine the conditions for skipping empty segments and skipping delimiters into a single boolean check.
        ```rust
        let skip = ( current_split.typ == SplitType::Delimeted && current_split.string.is_empty() && !self.flags.contains( SplitFlags::PRESERVING_EMPTY ) )
        || ( current_split.typ == SplitType::Delimiter && !self.flags.contains( SplitFlags::PRESERVING_DELIMITERS ) );

        if skip
        {
          continue;
        }
        ```
    *   Step 5: **Perform Increment Verification.**
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests` via `execute_command`.
    *   Step 2: Analyze the output. The following tests must now **pass**:
        *   `inc::split_test::combined_options_tests::test_m_t3_13_quoting_preserve_all_strip`
        *   `inc::split_test::quoting_and_unescaping_tests::empty_quoted_section_test`
        *   `inc::split_test::quoting_options_tests::test_m_t3_11_quoting_preserve_all_no_strip`
        *   `inc::split_test::quoting_options_tests::test_m_t3_13_quoting_preserve_all_strip`
    *   Step 3: The `mre_test` is expected to still fail.
*   **Commit Message:** `fix(strs_tools): Prevent extra empty segment after quoted strings`

##### Increment 7: Fix Incorrect Escape Handling Bug
*   **Goal:** To fix the bug where an escaped backslash (`\\`) followed by a quote is parsed incorrectly, as identified in the `mre_test`.
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
    *   Step 5: **Perform Increment Verification.**
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p strs_tools --test strs_tools_tests` via `execute_command`.
    *   Step 2: Analyze the output. The `inc::split_test::quoting_and_unescaping_tests::mre_test` must now **pass**.
    *   Step 3: Ensure no regressions were introduced in the other tests. All tests should pass.
*   **Commit Message:** `fix(strs_tools): Correctly handle escaped characters in quoted strings`

##### Increment 8: Finalization
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

### Task Requirements
*   All code must strictly adhere to the `codestyle` rulebook.
*   The final implementation must correctly solve the problem described in the MRE.
*   New tests must be added to cover the new functionality and prevent regressions.
*   The change to `Cow` is a breaking change and should be documented in the `changelog.md`.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   Must use Rust 2021 edition.

### Assumptions
*   A breaking change to `Split.string` by using `Cow` is acceptable to provide the most ergonomic API.
*   The required escape sequences are `\"`, `\\`, `\n`, `\t`, `\r`.
*   An unrecognized escape sequence (e.g., `\z`) will be treated literally, with the `\` and the following character passed through to the output.

### Out of Scope
*   Supporting other types of escape sequences (e.g., unicode `\u{...}`).
*   Supporting single quotes (`'`) for quoting.

### External System Dependencies
*   None

### Notes & Insights
*   **Bug A (Incorrect Escape Handling):** The parser fails to correctly handle an escaped backslash (`\\`) when it is immediately followed by a closing quote character (`"`). The root cause is flawed state management in `SplitFastIterator::next`'s quote-scanning loop. The `mre_test` correctly identifies this bug.
*   **Bug B (Spurious Empty Segment):** The iterator incorrectly yields an extra, unwanted empty segment (`""`) immediately after parsing a quoted segment, but only when the `preserving_empty(true)` option is enabled. This is due to flawed `skip` logic in the main `SplitIterator` after its "peeking" logic for quotes has run.
*   **Increment 4 (Implement Unescaping Logic):**
    *   **Issue:** Initial implementation of `unescape_str` caused lifetime errors (`E0597`).
    *   **Solution:** Forced `unescape_str` to always return `Cow::Owned`.
*   **Increment 5 (Implement Quoted Segment Logic):**
    *   **Issue:** New tests for quoting and unescaping failed because `SplitIterator` was incorrectly preserving delimiter segments.
    *   **Solution:** Modified the `SplitIterator::next` method to correctly apply the `skip` logic.

### Changelog
*   [Increment 6 Plan] Updated plan to fix the two distinct bugs (Spurious Empty Segment, Incorrect Escape Handling) in separate, detailed increments based on comprehensive test failure analysis.
*   [Increment 5 | 2025-07-12] Removed debug macros from `SplitIterator`.
*   [Increment 4 | 2025-07-12] Implemented `unescape_str` function with unit tests and fixed compilation issues.
*   [Increment 3 | 2025-07-10] Fixed compilation errors after changing `Split.string` to `Cow`.
*   [Increment 2 | 2025-07-10] Changed `Split.string` to `Cow<'a, str>` to support unescaping.
*   [Increment 1 | 2025-07-10] Read relevant files for analysis.