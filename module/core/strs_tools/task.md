# Change Proposal for `strs_tools`

### Task ID
*   `TASK-20250524-UNILANG-ESCAPES`

### Requesting Context
*   **Requesting Crate/Project:** `module/move/unilang_instruction_parser`
*   **Driving Feature/Task:** Fixing all tests and warnings in `unilang_instruction_parser`, specifically tests related to escaped quotes.
*   **Link to Requester's Plan:** `module/move/unilang_instruction_parser/plan.md`
*   **Date Proposed:** 2025-05-24

### Overall Goal of Proposed Change
*   Improve the `strs_tools` crate's `SplitIterator` or related tokenization logic to correctly handle escaped quote characters within quoted strings, ensuring that the `Split` items produced accurately reflect the intended string content and do not prematurely terminate quoted values due to internal escape sequences.

### Problem Statement / Justification
*   The `unilang_instruction_parser` crate relies on `strs_tools` for initial string splitting and tokenization. Currently, tests in `unilang_instruction_parser` (e.g., `error_invalid_escape_sequence_location_str`, `error_invalid_escape_sequence_location_slice`, `unescaping_works_for_named_arg_value`, `unescaping_works_for_positional_arg_value`) are ignored because `strs_tools`'s `SplitIterator` appears to misinterpret escaped quote characters (e.g., `\"`) within quoted strings. This leads to incorrect `Split` items being generated, which then causes parsing errors in `unilang_instruction_parser` when attempting to unescape the string or determine its boundaries. The current behavior prevents `unilang_instruction_parser` from correctly parsing strings containing escaped quotes.

### Proposed Solution / Specific Changes
*   The core issue is that `strs_tools::string::split::SplitIterator` (or its underlying tokenizer) needs to correctly identify the boundaries of quoted strings, even when they contain escaped quote characters. The `SplitType::Delimeted` for quoted strings should encompass the entire quoted content, and the internal logic should not be confused by `\"` or `\'`.
*   **Internal Changes (high-level):** The `SplitIterator`'s logic for `preserving_quoting` and `quoting_pairs` needs to be robust against escaped quote characters. It should treat `\"` as part of the string content, not as a closing quote. This likely requires modifying the state machine or character-by-character processing within the tokenizer to correctly identify the *actual* closing quote.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   After the fix, `unilang_instruction_parser` should be able to parse inputs like:
    ```
    cmd "value with \"quotes\" and \\\\slash\\\\"
    cmd name::"value with \"quotes\""
    ```
*   And the `Split` items for the quoted parts should correctly span the entire quoted string, allowing `unescape_string_with_errors` in `unilang_instruction_parser` to correctly process the inner content.

### Acceptance Criteria (for this proposed change)
*   The `strs_tools` crate, when used by `unilang_instruction_parser`, correctly tokenizes strings containing escaped quotes.
*   Specifically, for an input like `"value with \"quotes\""`, the `Split` item for the quoted value should have `typ: SplitType::Delimeted` and `string: "\"value with \\\"quotes\\\""`.
*   The previously ignored tests in `unilang_instruction_parser` related to escaped quotes (e.g., `unescaping_works_for_named_arg_value`, `unescaping_works_for_positional_arg_value`, `error_invalid_escape_sequence_location_str`, `error_invalid_escape_sequence_location_slice`) should pass when un-ignored.

### Potential Impact & Considerations
*   **Breaking Changes:** Unlikely, as this is a bug fix. It should improve correctness without changing existing valid behavior.
*   **Dependencies:** No new dependencies.
*   **Performance:** Should be minimal.
*   **Testing:** New unit/integration tests should be added to `strs_tools` specifically for escaped quotes within quoted strings.

### Notes & Open Questions
*   None.
