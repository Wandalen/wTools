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
| `inc::split_test::basic_split_tests::unescaping_empty_string` | Failing (New) | `split` returns `[]` instead of `[""]` for empty quoted string `""`. |
| `inc::split_test::basic_split_tests::unescaping_unterminated_quote` | Failing (New) | Unescaping for `\"` at the end of an unterminated quote is incorrect. |

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
*   **Goal:** Modify the string splitting logic in `src/string/split.rs` to correctly handle escaped characters within quoted segments, making the new test pass.
*   **Specification Reference:** `module/core/strs_tools/task/task1.md` - Proposed Solution
*   **Steps:**
    *   Step 1: Read the content of `module/core/strs_tools/src/string/split.rs`.
    *   Step 2: In `SplitIterator::next`, comment out `self.skip_next_spurious_empty = true;` to fix the empty quoted string bug.
    *   Step 3: Analyze the remaining failure in `unescaping_unterminated_quote` and apply a fix.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p strs_tools --test strs_tools_tests -- --nocapture` via `execute_command`.
    *   Analyze the output to confirm that the `unescaping_in_quoted_string` test now passes.
*   **Commit Message:** fix(strs_tools): Correctly unescape characters in quoted strings

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
*   The test `unescaping_in_quoted_string` already exists and passes. The failing tests are `unescaping_empty_string` and `unescaping_unterminated_quote`.
*   The fix for `unescaping_unterminated_quote` described in the plan seems to be already applied in `SplitFastIterator::next`, yet the test fails. This implies the bug is more subtle or located elsewhere.

You are absolutely right to question them, and I apologize for jumping ahead to a fix. Analyzing the sanity of the tests is the correct next step.

After a detailed review of both failing tests and the project's specification, I can confirm that **both tests are sane and are correctly identifying legitimate bugs** in the current implementation.

Here is the breakdown of why each test is correct:

### 1. `unescaping_empty_string`

*   **Test Code:**
    ```rust
    let src = r#""""#; // Input is an empty quoted string
    let iter = split()
      .src( src )
      .quoting( true )
      .preserving_empty( true ) // Key option
      .perform();
    let splits: Vec<_> = iter.map(|e| String::from(e.string)).collect();
    assert_eq!( splits, vec![ "" ] ); // Expects one empty string
    ```
*   **Failure:** It actually produces `[]` (an empty vector) instead of `[""]`.
*   **Sanity Rationale:**
    1.  The input `""` is a valid quoted section.
    2.  With `quoting(true)`, the iterator should recognize this as a single token.
    3.  By default, `preserving_quoting` is `false`, so the outer `"` characters are stripped, leaving an empty string (`""`) as the token's content.
    4.  The option `preserving_empty(true)` explicitly tells the iterator **not to discard** empty tokens.
    5.  Therefore, the iterator is expected to yield exactly one token: the empty string.

*   **Conclusion:** The test is **sane**. It correctly asserts that an empty but quoted field should be preserved when `preserving_empty(true)` is active. The bug is in the implementation, which is incorrectly dropping this token.

### 2. `unescaping_unterminated_quote`

*   **Test Code:**
    ```rust
    let src = r#""abc\"""#; // Input is an unclosed quote ending with an escaped quote
    let iter = split()
      .src( src )
      .quoting( true )
      .perform();
    let splits: Vec<_> = iter.map(|e| String::from(e.string)).collect();
    assert_eq!( splits, vec![ r#"abc""# ] ); // Expects the unescaped result
    ```
*   **Failure:** It actually produces `["abc\\"]` instead of `["abc\""]`.
*   **Sanity Rationale:**
    1.  The core purpose of the active task (`task_plan.md`) is to fix unescaping.
    2.  The sequence `\"` should be unescaped into a literal `"` character.
    3.  The input string is `abc\"`. After unescaping, this should become `abc"`.
    4.  The test correctly asserts that the expected output is `vec!["abc\""]`.
    5.  The actual output `vec!["abc\\"]` shows that the implementation is incorrectly transforming the `\"` sequence into a literal backslash `\`, which is a bug.

*   **Conclusion:** The test is **sane**. It correctly defines the expected behavior for unescaping a quote character, even in the edge case of an unterminated string. The bug is in the implementation's unescaping logic.

Both tests are valid and are fulfilling their purpose as defined in the task plan: to fail in a predictable way that demonstrates the existing bugs, thereby guiding the development of a correct fix.

Of course. Here is a detailed guideline on how to fix the two failing tests. The issues stem from two distinct logical errors in the iterators.

### Root Cause Analysis

1.  **`unescaping_empty_string` Failure**: This test fails because when the iterator encounters an empty quoted string (`""`), it correctly parses it but then incorrectly flags the resulting empty token as "spurious" and skips it. The logic does not differentiate between a legitimate empty token from a quoted source and a truly spurious empty token that might appear between delimiters.

2.  **`unescaping_unterminated_quote` Failure**: This test fails because the internal `SplitFastIterator` has a flawed mechanism for detecting escaped characters. It uses a simple boolean toggle (`is_escaped`) which cannot correctly parse sequences like `\\"` (an escaped backslash followed by a quote). It misinterprets the final quote as being escaped. The fix is to replace this logic with a more robust character-skipping approach.

---

### Detailed Step-by-Step Fix Instructions

You will need to make changes in one file: `module/core/strs_tools/src/string/split.rs`.

#### Step 1: Fix the Empty Quoted String Bug

This fix involves preventing the iterator from incorrectly discarding a valid empty token that comes from a quoted source like `""`.

*   **File to Edit**: `module/core/strs_tools/src/string/split.rs`
*   **Method to Modify**: `SplitIterator::next()`

**Guideline:**
Locate the following block of code inside the `SplitIterator::next()` method. The problem is the line `self.skip_next_spurious_empty = true;`, which is too aggressive. You need to remove it to allow the main loop's `preserving_empty` logic to handle the token correctly.

**BEFORE:**
```rust
// in module/core/strs_tools/src/string/split.rs, inside SplitIterator::next()

        let mut current_split = effective_split_opt?;
        if quote_handled_by_peek
        {
          self.skip_next_spurious_empty = true;
        }
        if self.skip_next_spurious_empty && current_split.typ == SplitType::Delimeted && current_split.string.is_empty()
        {
//...
```

**AFTER (Remove the highlighted line):**
```rust
// in module/core/strs_tools/src/string/split.rs, inside SplitIterator::next()

        let mut current_split = effective_split_opt?;
        if quote_handled_by_peek
        {
          // self.skip_next_spurious_empty = true; // This line is removed
        }
        if self.skip_next_spurious_empty && current_split.typ == SplitType::Delimeted && current_split.string.is_empty()
        {
//...
```
By removing that line, you allow the empty string token generated from `""` to proceed to the next check, which correctly preserves it because the test sets `preserving_empty(true)`.

#### Step 2: Fix the Unescaping Logic for Unterminated Quotes

This is the main fix for the unescaping bug. It involves replacing the faulty escape-handling logic in the *inner* iterator, `SplitFastIterator`.

*   **File to Edit**: `module/core/strs_tools/src/string/split.rs`
*   **Method to Modify**: `SplitFastIterator::next()`

**Guideline:**
The current implementation uses a simple boolean toggle to track escaped characters, which is insufficient. You need to replace the entire loop that finds the end of a quoted section with a more robust one that properly skips the character immediately following a backslash (`\`).

**BEFORE:**
```rust
// in module/core/strs_tools/src/string/split.rs, inside SplitFastIterator::next()

      if let Some( current_quote_char ) = self.active_quote_char
        {
          let mut end_of_quote_idx : Option< usize > = None;
          let mut is_escaped = false;
          for ( i, c ) in self.iterable.chars().enumerate()
          {
            if c == '\\'
            {
              is_escaped = !is_escaped;
              continue;
            }
            if c == current_quote_char && !is_escaped
            {
              end_of_quote_idx = Some( i + 1 );
              break;
            }
            is_escaped = false;
          }
//...
```

**AFTER (Replace the entire `for` loop block):**
```rust
// in module/core/strs_tools/src/string/split.rs, inside SplitFastIterator::next()

      if let Some( current_quote_char ) = self.active_quote_char
        {
          let mut end_of_quote_idx : Option< usize > = None;
          let mut char_indices = self.iterable.char_indices();
          'outer: while let Some( ( i, c ) ) = char_indices.next()
          {
            if c == '\\'
            {
              // Skip the escaped character, effectively ignoring it for quote-termination purposes
              char_indices.next();
            }
            else if c == current_quote_char
            {
              end_of_quote_idx = Some( i + c.len_utf8() );
              break 'outer;
            }
          }
//...
```
This new loop correctly iterates using `char_indices` to handle multi-byte characters properly and simply skips the next character after a `\`, which is the correct and robust way to handle escapes in this context.

### Verification

After applying both changes:

1.  Run the tests again:
    ```sh
    cargo test --test strs_tools_tests
    ```
2.  Both `unescaping_empty_string` and `unescaping_unterminated_quote` should now pass, along with all other tests.

### Changelog
* [Increment 1 | 2025-07-13 02:19:05 UTC] Verified existing failing tests for unescaping.
