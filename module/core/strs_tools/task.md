# Change Proposal for `strs_tools`

### Task ID
*   `TASK-20250525-UNILANG-SPLIT-QUOTING`

### Requesting Context
*   **Requesting Crate/Project:** `module/move/unilang_instruction_parser`
*   **Driving Feature/Task:** Correct parsing of quoted arguments with internal delimiters and escaped quotes.
*   **Link to Requester's Plan:** `module/move/unilang_instruction_parser/plan.md`
*   **Date Proposed:** 2025-05-25

### Overall Goal of Proposed Change
*   Modify `strs_tools::string::split::SplitIterator` to correctly tokenize strings containing quoted sections, ensuring that internal delimiters (e.g., spaces, `::`) within a quoted section are *not* treated as delimiters for the duration of that section. The entire content of a quoted section (excluding outer quotes, but including escaped inner quotes and delimiters) should be returned as a single `Delimeted` item.

### Problem Statement / Justification
*   The `unilang_instruction_parser` relies on `strs_tools::string::split::SplitIterator` for tokenization. When `SplitIterator` encounters a quoted section (e.g., `"value with spaces and :: delimiters"`), it currently treats the internal spaces and `::` as delimiters, breaking the quoted string into multiple `Split` items. This is incorrect behavior for a quoted string, which should be treated as a single literal value.
*   The current `handle_quoted_section` in `SplitIterator` attempts to consume the quoted content, but `SplitFastIterator` (its internal iterator) continues to find internal delimiters, leading to incorrect tokenization.
*   This prevents `unilang_instruction_parser` from correctly parsing commands with quoted arguments containing spaces or other delimiters, leading to parsing errors and hangs.

### Proposed Solution / Specific Changes
*   **Option 1 (Preferred): Modify `SplitIterator` to dynamically adjust `SplitFastIterator`'s delimiters.**
    *   Introduce a mechanism in `SplitIterator` to temporarily disable or change the set of active delimiters for its internal `SplitFastIterator` when inside a quoted section.
    *   When an opening quote is encountered, `SplitIterator` should switch `SplitFastIterator` to a mode where only the matching closing quote (and potentially escaped characters) are considered delimiters.
    *   Once the closing quote is found, switch back to the original set of delimiters.
*   **Option 2 (Alternative): Enhance `handle_quoted_section` to consume all internal tokens.**
    *   Modify `handle_quoted_section` to not just find the closing quote, but to also consume all intermediate `Split` items from `self.iterator` (the `SplitFastIterator`) until the closing quote is reached. These intermediate items should be discarded or concatenated into the main quoted string. This might be more complex to manage state.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   Given input: `cmd arg::"value with spaces and :: delimiters"`
*   `SplitIterator` should produce:
    *   `Split { string: "cmd", typ: Delimeted, ... }`
    *   `Split { string: " ", typ: Delimiter, ... }`
    *   `Split { string: "arg", typ: Delimeted, ... }`
    *   `Split { string: "::", typ: Delimiter, ... }`
    *   `Split { string: "value with spaces and :: delimiters", typ: Delimeted, ... }` (This should be a single item, with outer quotes stripped, and internal escapes handled by `unilang_instruction_parser` later).

### Acceptance Criteria (for this proposed change)
*   `strs_tools::string::split::SplitIterator` correctly tokenizes quoted strings as single delimited items, ignoring internal delimiters.
*   The `debug_hang_split_issue` test in `strs_tools` passes and produces the expected single `Split` item for the quoted string.
*   All tests in `module/move/unilang_instruction_parser` (especially those related to quoted arguments) pass after this change is implemented in `strs_tools`.

### Potential Impact & Considerations
*   **Breaking Changes:** This might introduce breaking changes if `SplitIterator`'s behavior for quoting is fundamentally altered. Careful consideration of existing uses of `SplitIterator` is needed.
*   **Performance:** The new logic should be efficient and not introduce performance regressions.
*   **Complexity:** The solution should aim for clarity and maintainability.

### Notes & Open Questions
*   The current `handle_quoted_section` logic for finding the unescaped postfix seems to be correct after the last fix. The problem is the interaction with `SplitFastIterator`'s continued tokenization.
*   The `SplitIterator` needs to effectively "take control" of the parsing when a quoted section begins, preventing `SplitFastIterator` from yielding internal delimiters.
