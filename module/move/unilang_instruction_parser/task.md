# Change Proposal for unilang_instruction_parser

### Task ID
*   TASK-20250629-050142-FixCommandParsing

### Requesting Context
*   **Requesting Crate/Project:** `module/move/unilang`
*   **Driving Feature/Task:** Refactoring `unilang` to use `unilang_instruction_parser` (Task Plan: `module/move/unilang/task_plan_architectural_unification.md`)
*   **Link to Requester's Plan:** `module/move/unilang/task_plan_architectural_unification.md`
*   **Date Proposed:** 2025-06-29

### Overall Goal of Proposed Change
*   To fix a critical bug in `unilang_instruction_parser::Parser` where the command name is incorrectly parsed as a positional argument instead of being placed in `command_path_slices`. This prevents `unilang` from correctly identifying commands.

### Problem Statement / Justification
*   When `unilang_instruction_parser::Parser::parse_single_str` or `parse_slice` is used with a command string like `.test.command arg1 arg2`, the parser incorrectly populates `GenericInstruction.positional_arguments` with `".test.command"` and `command_path_slices` remains empty.
*   This leads to `unilang::semantic::SemanticAnalyzer` failing to find the command, as it expects the command name to be in `command_path_slices`.
*   This bug fundamentally breaks the integration of `unilang_instruction_parser` with `unilang` and prevents the `unilang` architectural unification task from proceeding.

### Proposed Solution / Specific Changes
*   **Modify `unilang_instruction_parser::Parser`'s parsing logic:**
    *   The parser needs to correctly identify the first segment of the input as the command name (or command path slices if it contains dots) and populate `GenericInstruction.command_path_slices` accordingly.
    *   Subsequent segments should then be treated as arguments (named or positional).
*   **Expected API Changes:** No public API changes are expected for `Parser::parse_single_str` or `parse_slice`, but their internal behavior must be corrected.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   Given the input string `".test.command arg1 arg2"`, `parser.parse_single_str(".test.command arg1 arg2")` should produce a `GenericInstruction` similar to:
    ```rust
    GenericInstruction {
        command_path_slices: vec!["test", "command"], // Or ["test_command"] if it's a single segment
        named_arguments: HashMap::new(),
        positional_arguments: vec![
            Argument { value: "arg1", ... },
            Argument { value: "arg2", ... },
        ],
        // ... other fields
    }
    ```
*   The `unilang::semantic::SemanticAnalyzer` should then be able to successfully resolve the command.

### Acceptance Criteria (for this proposed change)
*   `unilang_instruction_parser`'s tests related to command parsing (if any exist) should pass after the fix.
*   After this fix is applied to `unilang_instruction_parser`, the `unilang` tests (specifically `test_path_argument_type` and others that currently fail with `COMMAND_NOT_FOUND`) should pass without requiring manual construction of `GenericInstruction` in `unilang`.

### Potential Impact & Considerations
*   **Breaking Changes:** No breaking changes to the public API are anticipated, only a correction of existing behavior.
*   **Dependencies:** No new dependencies.
*   **Performance:** The fix should not negatively impact parsing performance.
*   **Testing:** New unit tests should be added to `unilang_instruction_parser` to specifically cover the correct parsing of command names and arguments.

### Notes & Open Questions
*   The current `unilang` task will proceed by temporarily working around this parser bug by manually constructing `GenericInstruction` for its tests.