# Change Proposal for `unilang_parser`

### Task ID
*   `TASK-20250723-215601-FixPathParsing`

### Requesting Context
*   **Requesting Crate/Project:** `unilang`
*   **Driving Feature/Task:** Architectural Unification in `unilang` (specifically, CLI integration tests failing due to path parsing).
*   **Link to Requester's Plan:** `../unilang/task/task_plan.md`
*   **Date Proposed:** 2025-07-23

### Overall Goal of Proposed Change
*   To modify `unilang_parser` to correctly parse file paths containing dots as single argument values, preventing them from being incorrectly tokenized.

### Problem Statement / Justification
*   The `unilang_parser` currently misinterprets file paths that contain dots (e.g., `/tmp/.tmpQ0DwU0/temp_file.txt`). It incorrectly tokenizes the path at the dot, leading to a `Syntax("Unexpected token '.' in arguments")` error. This prevents the `unilang` CLI from correctly processing commands that take file paths as arguments, as demonstrated by failing integration tests.

### Proposed Solution / Specific Changes
*   **Behavioral Changes:** The parser's tokenization logic (likely within `classify_split` or related argument parsing functions) needs to be adjusted to recognize and treat a full file path (even with multiple dot-separated components) as a single, cohesive argument value. This may involve:
    *   Prioritizing path-like string recognition over general delimiter splitting.
    *   Modifying the tokenizer to handle sequences like `::` or `.` within what is clearly intended to be a single argument value (e.g., a file path).
*   **Internal Changes (high-level):** The core parsing algorithm for arguments needs to be reviewed to ensure that it correctly identifies and consumes complete argument values, especially when they contain characters that might otherwise be interpreted as delimiters (like `.`).

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   When `unilang_parser::Parser::parse_single_instruction` is invoked with a string like `cat path::/tmp/.tmpQ0DwU0/temp_file.txt`, the `path` argument should be parsed as a single `Argument` with `value: "/tmp/.tmpQ0DwU0/temp_file.txt"`.
*   The `unilang` CLI's `cat` command should successfully read the content of a file specified with a path containing dots.

### Acceptance Criteria (for this proposed change)
*   `unilang_parser` successfully parses file paths containing dots without `Unexpected token '.'` errors.
*   The `unilang` crate's integration tests, specifically `test_cli_cat_command_valid_file` and `test_cli_cat_command_non_existent_file`, pass without parsing errors related to file paths.

### Potential Impact & Considerations
*   **Breaking Changes:** No breaking changes to the public API are anticipated, as this is a fix to internal parsing logic.
*   **Dependencies:** No new dependencies are expected.
*   **Performance:** The change should not negatively impact parsing performance.
*   **Testing:** New unit tests should be added to `unilang_parser` to specifically cover parsing of file paths with dots and other special characters.

### Notes & Open Questions
*   The current `unilang` task is blocked by this parsing issue. A local patch will be used to proceed with `unilang`'s development, but a permanent fix in `unilang_parser` is required.