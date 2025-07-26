# Task: Implement New Parser Rules

### Goal
*   To implement the command path and argument parsing logic in the `unilang` crate according to the rules recently added to `spec_addendum.md`. This will involve refactoring the parser engine to correctly distinguish between command path segments and arguments based on the new dot-delimited and token-based rules.

### Ubiquitous Language (Vocabulary)
*   **Command Path**: The hierarchical name of a command (e.g., `cmd.subcmd`).
*   **Command Path Segment**: An individual part of the command path (e.g., `cmd`, `subcmd`).
*   **Argument**: A value passed to a command, either positional or named.
*   **Dot Delimiter**: A `.` character used to separate command path segments.

### Progress
*   **Roadmap Milestone:** M2: Core Parser Refinement
*   **Primary Editable Crate:** `module/move/unilang`
*   **Overall Progress:** Not Started
*   **Increment Status:** (To be planned)

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:** None

### Relevant Context
*   Control Files to Reference:
    *   `module/move/unilang/spec_addendum.md`
*   Files to Include (for planning):
    *   `module/move/unilang/src/lib.rs`
    *   `module/move/unilang/src/parser.rs` (if it exists)
    *   `module/move/unilang/tests/inc/phase1/full_pipeline_test.rs`
    *   `module/move/unilang/tests/inc/phase2/argument_types_test.rs`

### Expected Behavior Rules / Specifications
*   Refer to "Command Path and Argument Parsing Rules" in `spec_addendum.md`.

### Task Requirements
*   The implementation must correctly parse command paths and arguments according to all rules in `spec_addendum.md`.
*   Existing tests should be updated, and new tests should be added to cover the new rules, especially the edge cases defined in the spec.

### Out of Scope
*   Implementing command execution logic. This task is focused solely on parsing.