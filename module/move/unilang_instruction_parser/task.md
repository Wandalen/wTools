# Change Proposal for unilang_instruction_parser

### Task ID
*   TASK-20250527-061400-FixValueLocationSpan

### Requesting Context
*   **Requesting Crate/Project:** `strs_tools`
*   **Driving Feature/Task:** Enhancing `strs_tools::SplitIterator` for robust quoted string handling.
*   **Link to Requester's Plan:** `../../core/strs_tools/plan.md`
*   **Date Proposed:** 2025-05-27

### Overall Goal of Proposed Change
*   Correct the calculation of the `end` field for `arg.value_location` (a `StrSpan`) in `unilang_instruction_parser` when parsing named arguments with quoted and escaped values. The span should accurately reflect the range of the *unescaped* value within the original input string.

### Problem Statement / Justification
*   The `strs_tools` crate's `SplitIterator` now correctly provides the *raw* content of quoted strings (excluding outer quotes) and the span of this raw content in the original input.
*   The `unilang_instruction_parser` test `named_arg_with_quoted_escaped_value_location` currently fails. Analysis indicates that while the `start` of the `value_location` span might be calculated correctly (relative to the parser's internal logic), the `end` of this span appears to be calculated using the length of the *raw* token string received from `strs_tools`, rather than the length of the *unescaped* string.
*   For example, if `strs_tools` provides a raw token `value with \\\"quotes\\\" and \\\\\\\\slash\\\\\\\\` (length 37) with its original span, `unilang_instruction_parser` unescapes this to `value with "quotes" and \\slash\\` (length 33). The `value_location` span should then reflect this unescaped length (33). The current failure shows an end point consistent with the raw length (37).

### Proposed Solution / Specific Changes
*   **In `unilang_instruction_parser` (likely within the argument parsing logic, specifically where `Value::String` and its `location` are constructed for named arguments):**
    1.  When a quoted string token is received from `strs_tools` (or any tokenizer providing raw quoted content):
    2.  Perform the unescaping of the raw string content.
    3.  Calculate the length of the *unescaped* string.
    4.  When constructing the `StrSpan` for `value_location`, ensure the `end` field is calculated based on the `start` field plus the length of the *unescaped* string.
    *   Example: If the determined `start_offset` for the value (e.g., after `arg_name::`) is `S`, and the unescaped string length is `L_unescaped`, then `value_location.end` should be `S + L_unescaped`.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   After the fix, the `named_arg_with_quoted_escaped_value_location` test in `unilang_instruction_parser/tests/argument_parsing_tests.rs` should pass.
*   Specifically, for an input like `cmd arg_name::"value with \\\"quotes\\\" and \\\\\\\\slash\\\\\\\""`, if the parser determines the logical start of the value (after `::` and opening quote) to be, for instance, conceptually at original string index `X` (which the test seems to anchor at `9` relative to something), and the unescaped value is `value with "quotes" and \\slash\\` (length 33), then the `value_location` span should be `StrSpan { start: X_adjusted, end: X_adjusted + 33 }`. The current test expects `StrSpan { start: 9, end: 42 }`, which implies an unescaped length of 33.

### Acceptance Criteria (for this proposed change)
*   The `named_arg_with_quoted_escaped_value_location` test in `unilang_instruction_parser` passes.
*   Other related argument parsing tests in `unilang_instruction_parser` continue to pass, ensuring no regressions.
*   The `value_location` span for quoted arguments accurately reflects the start and end of the unescaped value content in the original input string.

### Potential Impact & Considerations
*   **Breaking Changes:** Unlikely to be breaking if the current behavior is a bug. This change aims to correct span reporting.
*   **Dependencies:** No new dependencies.
*   **Performance:** Negligible impact; involves using the correct length value (unescaped vs. raw) which should already be available post-unescaping.
*   **Testing:** The existing `named_arg_with_quoted_escaped_value_location` test is the primary verification. Additional tests for various escaped sequences within quoted arguments could be beneficial to ensure robustness.

### Alternatives Considered (Optional)
*   None, as `strs_tools` is now correctly providing raw content and its span as per its design. The unescaping and subsequent span calculation for the unescaped value is the responsibility of `unilang_instruction_parser`.

### Notes & Open Questions
*   The exact location in `unilang_instruction_parser` code that needs modification will require inspecting its parsing logic for named arguments. It's where the raw token from the splitter is processed, unescaped, and its `StrSpan` is determined.