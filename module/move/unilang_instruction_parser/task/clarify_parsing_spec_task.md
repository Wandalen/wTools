# Task: Clarify Command Path and Argument Parsing Specification

### Goal
*   To explicitly define the rules for parsing command paths and arguments in `spec_addendum.md`, resolving ambiguities regarding the role of spaces and identifiers in distinguishing between command path segments and arguments. This clarification is crucial for consistent and correct parser implementation.

### Ubiquitous Language (Vocabulary)
*   **Command Path**: The hierarchical name of a command (e.g., `cmd subcmd`).
*   **Command Path Segment**: An individual part of the command path (e.g., `cmd`, `subcmd`).
*   **Argument**: A value passed to a command, either positional or named.
*   **Space Delimiter**: A whitespace character used to separate tokens.
*   **Dot Delimiter**: A `.` character used to separate command path segments.

### Progress
*   **Roadmap Milestone:** M2: Core Parser Refinement
*   **Primary Editable Crate:** `module/move/unilang_instruction_parser`
*   **Overall Progress:** 0/1 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Define Command Path and Argument Parsing Rules

### Permissions & Boundaries
*   **Mode:** architect
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:** None

### Relevant Context
*   Control Files to Reference:
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include:
    *   `module/move/unilang_instruction_parser/src/parser_engine.rs` (for current implementation context)
    *   `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs` (for current test expectations)
    *   `module/move/unilang_instruction_parser/tests/syntactic_analyzer_command_tests.rs` (for current test expectations)

### Expected Behavior Rules / Specifications
*   (This task will define these rules in `spec_addendum.md`)

### Crate Conformance Check Procedure
*   (N/A for this specification task)

### Increments

##### Increment 1: Define Command Path and Argument Parsing Rules
*   **Goal:** Refine `sped.md` and `spec_addendum.md` that clearly defines how command paths are parsed and how they transition into argument parsing.
*   **Specification Reference:** New specification to be created.
*   **Steps:**
    *   Step 1: Read `spec_addendum.md` and `spec.md`.
    *   Step 2: Add the following rules:
        *   **Rule 0: Space are ignored:** Spaces are ignored and number of spaces is ignored.
        *   **Rule 1: Command Path Delimitation:** The command path consists of one or more segments. Segments are always separated by single dot (`.`). Spaces (single or many) might be injected before/after `.`, spaces are ignored.
            *   Example: `.cmd.subcmd` -> `["cmd", "subcmd"]`
            *   Example: `.cmd. subcmd` -> `["cmd", "subcmd"]`
            *   Example: `.cmd   .  subcmd` -> `["cmd", "subcmd"]`
            *   Example: `.cmd.subcmd.` -> `["cmd", "subcmd", "."]`
            *   Example: `.cmd.subcmd?` -> `["cmd", "subcmd", "?"]`
            *   Example: `.cmd.subcmd ?` -> `["cmd", "subcmd", "?"]`
        *   **Rule 2: Transition to Arguments:** The command path ends and argument parsing begins when:
            *   A token is encountered that is *not* an identifier, a space, or a dot (e.g., an operator like `::` or `?`, or a quoted string).
            *   An identifier is followed by a token that is *not* a dot, and is also not `::`. In this case, the identifier is the last command path segment, and the subsequent token is the first argument.
            *   The end of the input is reached after an identifier or a dot.
        *   **Rule 3: Leading/Trailing Dots:** Leading dots (`.cmd`) are ignored. Trailing dots (`cmd.`) are considered part of the last command path segment if no arguments follow. If arguments follow, a trailing dot on the command path is an error.
        *   **Rule 4: Help Operator (`?`):** The `?` operator is valid not only immediately after the command path (i.e., as the first argument or the first token after the command path), but also `?` might be preceded by by other arguments, but `?` is always the last. If command has other arguments before `?` then semantic meaning of `?` should be expaining not only the command but those specific arguments.
        *   **Rule 5: Positional Arguments:** Positional arguments are any non-named arguments that follow the command path.
        *   **Rule 6: Named Arguments:** Named arguments are identified by the `name::value` syntax.
    *   Step 3: Perform Increment Verification.
*   **Increment Verification:**
    *   1.  Read `spec_addendum.md` and verify the new section and rules are present and correctly formatted.
*   **Commit Message:** "docs(spec): Clarify command path and argument parsing rules"

### Task Requirements
*   The new specification must be clear and unambiguous.
*   It must resolve the current conflicts observed in `argument_parsing_tests.rs` and `syntactic_analyzer_command_tests.rs`.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.

### Assumptions
*   The user will approve the new specification.

### Out of Scope
*   Implementing any parser changes based on the new specification. This task is purely for documentation.

### External System Dependencies
*   None

### Notes & Insights
*   This clarification is essential to unblock the parser bug fix.

### Changelog
*   [User Feedback | 2025-07-07 20:21 UTC] Task interrupted due to ambiguity in command path/argument parsing. Initiating Stuck Resolution Process.