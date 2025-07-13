# Change Proposal for strs_tools

### Task ID
*   TASK-20250713-UNESCAPING-BUG

### Requesting Context
*   **Requesting Crate/Project:** `unilang_instruction_parser`
*   **Driving Feature/Task:** Correct parsing of quoted strings in Unilang instructions.
*   **Link to Requester's Plan:** `module/move/unilang_instruction_parser/task/task_plan.md`
*   **Date Proposed:** 2025-07-13

### Overall Goal of Proposed Change
*   To fix a bug in `strs_tools::string::split` where quoted strings containing escaped quotes (`\"`) or escaped backslashes (`\\`) are not correctly unescaped when `quoting(true)` is enabled. The goal is for the `Split` struct's `string` field to contain the fully unescaped value.

### Problem Statement / Justification
*   The `unilang_instruction_parser` relies on `strs_tools` for tokenization, including handling of quoted strings. Currently, `strs_tools::string::split` with `quoting(true)` correctly identifies quoted segments but fails to unescape characters like `\"` and `\\` within those segments. This leads to incorrect parsing of instruction arguments that contain such escaped characters, causing functional errors in `unilang_instruction_parser`. The bug was identified and confirmed by a Minimal Reproducible Example (MRE) test case.

### Proposed Solution / Specific Changes
*   **File:** `src/string/split.rs`
*   **Function:** `SplitFastIterator::next`
*   **Change:** Modify the loop that searches for the end of a quoted segment to correctly handle escaped characters. The current `is_escaped` boolean toggle is insufficient. A more robust iteration is needed to skip characters immediately following a backslash.
*   **Example of current buggy behavior (from MRE):**
    Input: `".command "hello \" world""`
    `strs_tools::split` output for quoted part: `"hello \\\" world"` (incorrectly retains backslashes)
    Expected `strs_tools::split` output for quoted part: `hello " world` (correctly unescaped)

*   **Proposed Code Change (conceptual, based on previous attempt):**
    Replace the `is_escaped` logic in `SplitFastIterator::next` with a loop that explicitly skips characters after a backslash.

    ```rust
    // Inside SplitFastIterator::next, within the `if let Some( current_quote_char ) = self.active_quote_char` block:
    let mut end_of_quote_idx : Option< usize > = None;
    let mut char_indices = self.iterable.char_indices();
    'outer: while let Some( ( i, c ) ) = char_indices.next()
    {
      if c == '\\'
      {
        // Skip the escaped character
        char_indices.next();
      }
      else if c == current_quote_char
      {
        end_of_quote_idx = Some( i + c.len_utf8() );
        break 'outer;
      }
    }
    ```
    This change ensures that the `SplitFastIterator` correctly identifies the end of the quoted segment, allowing the `unescape_str` function (which already exists and handles unescaping) to receive the correct input.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   When `strs_tools::split` is called with `quoting(true)` on an input string like `".command "hello \\" world""`, the resulting `Split` item for the quoted part should have its `string` field contain `Cow::Owned("hello \" world")` (i.e., the backslash before the quote is removed, and the quote is unescaped).
*   Similarly, for `".command "path\\\\to\\\\file""`, the `Split` item should contain `Cow::Owned("path\\to\\file")`.

### Acceptance Criteria (for this proposed change)
*   The `strs_tools::string::split` function, when used with `quoting(true)`, must correctly unescape `\"` and `\\` sequences within quoted segments.
*   A new test case (similar to the MRE created previously) should be added to `strs_tools` to verify this specific unescaping behavior.
*   All existing `strs_tools` tests must continue to pass.

### Potential Impact & Considerations
*   **Breaking Changes:** No breaking changes are anticipated to the public API of `strs_tools`. This is a bug fix.
*   **Dependencies:** No new dependencies are required.
*   **Performance:** The change involves a slightly more complex loop for parsing quoted strings, but the performance impact is expected to be negligible for typical string lengths.
*   **Testing:** New unit/integration tests should be added to `strs_tools` to cover the unescaping of `\"` and `\\` within quoted strings.

### Notes & Open Questions
*   The `unescape_str` function already exists in `strs_tools::string::split` and appears to handle the actual unescaping correctly. The issue is that `SplitFastIterator` is not providing the correct string slice to `unescape_str` due to its flawed quote-end detection logic.