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
*   **Overall Progress:** 5/7 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Setup and Analysis
    *   ✅ Increment 2: API Change - Use `Cow` for `Split.string`
    *   ✅ Increment 3: Fix Compilation Errors
    *   ✅ Increment 4: Implement Unescaping Logic
    *   ✅ Increment 5: Implement Quoted Segment Logic
    *   ⏳ Increment 6: Add New Tests for Unescaping and Quoting
    *   ⚫ Increment 7: Finalization

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
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Use `read_file` to load the content of:
        *   `module/core/strs_tools/src/string/split.rs`
        *   `module/core/strs_tools/src/lib.rs`
        *   `module/core/strs_tools/Cargo.toml`
        *   `module/core/strs_tools/tests/inc/split_test/quoting_options_tests.rs`
    *   Step 2: Analyze the read files to understand the current implementation of `Split`, `SplitIterator`, and how quoting is handled.
*   **Increment Verification:**
    *   Step 1: Confirm that all files were read successfully.
*   **Commit Message:** `chore(strs_tools): Begin refactoring of split iterator for unescaping`

##### Increment 2: API Change - Use `Cow` for `Split.string`
*   **Goal:** Modify the `Split` struct to use `Cow<'a, str>` for its `string` field to support returning owned, unescaped strings.
*   **Specification Reference:** "API Change Consideration" in the original proposal.
*   **Steps:**
    *   Step 1: In `module/core/strs_tools/src/string/split.rs`, change the type of the `string` field in the `Split` struct from `&'a str` to `Cow<'a, str>`.
    *   Step 2: Update the `Debug` and any other trait implementations for `Split` to handle the `Cow`.
    *   Step 3: Attempt to compile the crate using `timeout 90 cargo build -p strs_tools`. Expect failures.
    *   Step 4: Use the compiler output to identify all locations that need to be updated due to this breaking change.
*   **Increment Verification:**
    *   Step 1: The `Split` struct definition in `split.rs` must be updated to use `Cow<'a, str>`.
    *   Step 2: The `cargo build` command should fail, and the output should indicate errors related to the type change.
*   **Commit Message:** `feat(strs_tools): Change Split.string to Cow to support unescaping`

##### Increment 3: Fix Compilation Errors
*   **Goal:** Resolve all compilation errors caused by the change of `Split.string` to `Cow<'a, str>`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Based on the compiler errors from the previous increment, systematically update all code that creates or uses `Split` instances. This will likely involve wrapping existing `&str` values in `Cow::Borrowed(...)` and preparing for `Cow::Owned(...)`.
    *   Step 2: Run `timeout 90 cargo build -p strs_tools` repeatedly until all compilation errors are resolved.
*   **Increment Verification:**
    *   Step 1: The command `timeout 90 cargo build -p strs_tools` must pass successfully.
    *   Step 2: Run `timeout 90 cargo test -p strs_tools`. Some tests may fail due to logic changes, but it should compile.
*   **Commit Message:** `fix(strs_tools): Adapt codebase to Cow-based Split.string`

##### Increment 4: Implement Unescaping Logic
*   **Goal:** Implement the core logic to unescape characters within a string slice.
*   **Specification Reference:** "Perform unescaping of standard escape sequences" from the proposal.
*   **Steps:**
    *   Step 1: Use `read_file` to load `module/core/strs_tools/src/string/split.rs`.
    *   Step 2: In `module/core/strs_tools/src/string/split.rs`, add a new private helper function `fn unescape_str( input: &str ) -> Cow< '_, str >`.
    *   Step 3: Implement the logic for `unescape_str`:
        *   Search for the `\` character. If it's not found, return `Cow::Borrowed(input)`.
        *   If `\` is found, iterate through the input string's characters to build a new `String`.
        *   When a `\` is encountered, inspect the next character to handle valid escape sequences (`\"`, `\\`, `\n`, `\t`, `\r`) by appending their literal counterparts.
        *   If an escape sequence is not one of the recognized ones, append both the `\` and the character that follows it literally.
        *   Append all other characters as-is.
        *   Return `Cow::Owned(new_string)`.
    *   Step 4: In `module/core/strs_tools/src/string/split.rs`, add a new test module `#[cfg(test)] mod unescape_tests { ... }` at the end of the file.
    *   Step 5: Inside `unescape_tests`, add unit tests for the `unescape_str` function to cover various scenarios:
        *   A string with no escape sequences.
        *   Strings with each of the valid escape sequences (`\"`, `\\`, `\n`, `\t`, `\r`).
        *   A string with a mix of valid escape sequences.
        *   A string with an unrecognized escape sequence (e.g., `\z`) to ensure it's handled literally.
        *   An empty string.
        *   A string ending with a `\`.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p strs_tools --all-targets` via `execute_command`.
    *   Step 2: Analyze the output to confirm that all tests in the `unescape_tests` module pass successfully.
*   **Commit Message:** `feat(strs_tools): Implement unescaping logic for string splitting`

##### Increment 5: Implement Quoted Segment Logic
*   **Goal:** Modify the `SplitIterator` to correctly identify and consume an entire quoted string as a single token, and apply the new unescaping logic.
*   **Specification Reference:** "Ensure that when `quoting(true)` is enabled, the iterator consumes the entire quoted segment" from the proposal.
*   **Steps:**
    *   Step 1: Read the file `module/core/strs_tools/src/string/split.rs`.
    *   Step 2: In the `next()` method of `SplitIterator`, remove the `dbg!` macro calls that were used for debugging.
    *   Step 3: Run `timeout 90 cargo test -p strs_tools --all-targets` to confirm that all tests still pass after removing the debug macros.
*   **Increment Verification:**
    *   Step 1: Run `timeout 90 cargo test -p strs_tools --all-targets`. All tests must pass.
*   **Commit Message:** `feat(strs_tools): Make split iterator consume full quoted strings and unescape them`

##### Increment 6: Add New Tests for Unescaping and Quoting
*   **Goal:** Add new integration tests to verify the complete functionality and prevent future regressions.
*   **Specification Reference:** "Acceptance Criteria" from the proposal.
*   **Steps:**
    *   Step 1: Create a new test file: `module/core/strs_tools/tests/inc/split_test/quoting_and_unescaping_tests.rs`.
    *   Step 2: Use `read_file` to load `module/core/strs_tools/tests/inc/split_test/mod.rs`.
    *   Step 3: Use `insert_content` to add `pub mod quoting_and_unescaping_tests;` to `module/core/strs_tools/tests/inc/split_test/mod.rs`.
    *   Step 4: In the new test file, add a test case that is an exact copy of the MRE from the task description. Assert that the output for the quoted part is a single `Split` item with the correctly unescaped string.
    *   Step 5: Add more test cases covering:
        *   Strings with no quotes.
        *   Strings with empty quoted sections (`""`).
        *   Strings with multiple, different escape sequences.
        *   Quoted strings at the beginning, middle, and end of the input.
        *   Unterminated quoted strings (decide on expected behavior, e.g., treat as literal).
*   **Increment Verification:**
    *   Step 1: Run `timeout 90 cargo test -p strs_tools --test strs_tools_tests`. All new and existing tests must pass.
*   **Commit Message:** `test(strs_tools): Add comprehensive tests for quoting and unescaping`

##### Increment 7: Finalization
*   **Goal:** Perform a final review and verification of the entire task's output.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Perform a self-critique against all requirements in the plan.
    *   Step 2: Run the full `Crate Conformance Check Procedure`.
    *   Step 3: Ensure no regressions have been introduced.
    *   Step 4: Remove the original `module/core/strs_tools/task.md` if it still exists.
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
*   **Increment 4 (Implement Unescaping Logic):**
    *   **Issue:** Initial implementation of `unescape_str` caused lifetime errors (`E0597`) when its `Cow::Borrowed` return type was used in `SplitIterator::next` due to borrowing from a temporary `quoted_segment`.
    *   **Solution:** Forced `unescape_str` to always return `Cow::Owned` by calling `.into_owned()` on its result, breaking the invalid borrow. This required explicit type annotation and a two-step conversion to avoid compiler confusion.
    *   **Insight:** `Cow` can be tricky with lifetimes, especially when intermediate `Cow::Borrowed` values are created and then used in a context that outlives them. Explicitly converting to `Cow::Owned` can resolve such issues, but it's important to consider performance implications if many small strings are being unescaped.
*   **Increment 5 (Implement Quoted Segment Logic):**
    *   **Issue:** New tests for quoting and unescaping failed because `SplitIterator` was incorrectly preserving delimiter segments even when `preserving_delimeters(false)` was set. Additionally, an extra empty string segment was sometimes yielded when `preserving_empty` is true and a quoted segment is encountered.
    *   **Solution:** Modified the `SplitIterator::next` method to correctly apply the `skip` logic. The `skip` conditions for empty delimited segments and delimiter segments were combined with a logical OR (`||`) and placed at the beginning of the loop to ensure immediate skipping. This prevents unwanted segments from being yielded.
    *   **Insight:** The order and combination of `skip` conditions are crucial in iterators. A single `skip` flag that is conditionally overwritten can lead to subtle bugs. It's better to combine all skip conditions into a single boolean check at the start of the loop iteration.

### Changelog
*   [Increment 5 | 2025-07-12] Removed debug macros from `SplitIterator`.
*   [Increment 4 | 2025-07-12] Implemented `unescape_str` function with unit tests and fixed compilation issues.
*   [Increment 3 | 2025-07-10] Fixed compilation errors after changing `Split.string` to `Cow`.
*   [Increment 2 | 2025-07-10] Changed `Split.string` to `Cow<'a, str>` to support unescaping.
*   [Increment 1 | 2025-07-10] Read relevant files for analysis.
