# Change Proposal for `strs_tools` (Temporary)
### Task ID
*   `TASK-20250720-192600-StrsToolsSplitEnhancement`

### Requesting Context
*   **Requesting Crate/Project:** `unilang_instruction_parser`
*   **Driving Feature/Task:** Implementing `spec.md` Rule 2: "End of Command Path & Transition to Arguments" which states that a quoted string should trigger the end of the command path and the beginning of argument parsing.
*   **Link to Requester's Plan:** `module/move/unilang_instruction_parser/task/plan.md`
*   **Date Proposed:** 2025-07-20

### Overall Goal of Proposed Change
*   Enhance the `strs_tools::string::split::Split` struct to include a `was_quoted: bool` field. This field will indicate whether the `Split` item originated from a quoted string in the original input.

### Problem Statement / Justification
*   The `unilang_instruction_parser` needs to distinguish between a quoted string (e.g., `"val with spaces"`) and an invalid identifier (e.g., `!arg`, `123`) when parsing the command path. According to `spec.md` Rule 2, encountering a quoted string should end the command path and transition to argument parsing, while an invalid identifier should result in a syntax error.
*   Currently, `strs_tools::string::split` is configured with `preserving_quoting(false)`, meaning it removes quotes and unescapes the content. The `Split` struct itself does not carry information about whether the original segment was quoted.
*   This lack of information prevents `unilang_instruction_parser` from correctly implementing `spec.md` Rule 2, as it cannot differentiate between a valid quoted string (which should be treated as a positional argument) and an invalid identifier (which should be an error in the command path). Both are currently classified as `Unrecognized` by `item_adapter`, leading to incorrect parsing behavior.

### Proposed Solution / Specific Changes
*   **API Changes:**
    *   Modify the `strs_tools::string::split::Split` struct to add a new public field:
        ```rust
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Split< 'a >
        {
          /// The string content of the segment.
          pub string : std::borrow::Cow< 'a, str >,
          /// The type of the segment (delimited or delimiter).
          pub typ : SplitType,
          /// The starting byte index of the segment in the original string.
          pub start : usize,
          /// The ending byte index of the segment in the original string.
          pub end : usize,
          /// Indicates if the segment originated from a quoted string.
          pub was_quoted : bool,
        }
        ```
*   **Behavioral Changes:**
    *   The `was_quoted` field in `Split` must be correctly populated by the `strs_tools::string::split::SplitIterator`.
    *   When `SplitIterator` processes a quoted section (i.e., when `self.flags.contains(SplitFlags::QUOTING)` is true and it consumes a quoting prefix/postfix), the resulting `Split` item's `was_quoted` field should be set to `true`. Otherwise, it should be `false`.
*   **Internal Changes (high-level, if necessary to explain public API):**
    *   Adjust the logic within `SplitIterator::next` to set the `was_quoted` flag based on whether the segment was enclosed in quotes. This will involve modifying the `if self.flags.contains(SplitFlags::QUOTING)` block where `effective_split_opt` is determined.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   The `unilang_instruction_parser` will be able to:
    *   Correctly identify when a command path ends due to a quoted string.
    *   Pass the quoted string (as a positional argument) to the `GenericInstruction`.
    *   Distinguish between quoted strings and invalid identifiers in the command path.

### Acceptance Criteria (for this proposed change)
*   The `Split` struct in `strs_tools` includes a `pub was_quoted: bool` field.
*   The `was_quoted` field is correctly set to `true` for segments that originated from quoted strings (e.g., `"hello world"`, `'foo'`) and `false` otherwise.
*   All existing `strs_tools` tests continue to pass after the change.

### Potential Impact & Considerations
*   **Breaking Changes:** Adding a field to a public struct is a minor breaking change if consumers are doing pattern matching on the struct directly without `..`. However, given `strs_tools` is a low-level utility, this is generally acceptable for a minor version bump.
*   **Dependencies:** No new external dependencies.
*   **Performance:** Minimal impact, a single boolean flag.
*   **Testing:** New unit tests should be added to `strs_tools` to specifically verify the `was_quoted` flag's behavior for various quoting scenarios.

### Alternatives Considered (Optional)
*   **Parsing quotes in `unilang_instruction_parser` directly:** This was rejected as it violates the `strs_tools` mandate (Section 1.1) to handle low-level tokenization, including quoting.
*   **Using `preserving_quoting(true)` in `strs_tools`:** This would make `strs_tools` return quotes as part of the string, allowing `unilang_instruction_parser` to detect them. However, `unilang_instruction_parser` would then have to manually strip quotes and unescape, which is `strs_tools`'s responsibility when `preserving_quoting(false)` is used. This would lead to duplicated logic and a less clean separation of concerns.

### Notes & Open Questions
*   None