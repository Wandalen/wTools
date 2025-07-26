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
*   **Primary Editable Crate:** `module/move/unilang`
*   **Overall Progress:** 1/1 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Define Command Path and Argument Parsing Rules

### Permissions & Boundaries
*   **Mode:** architect
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:** None

### Relevant Context
*   Control Files to Reference:
    *   `module/move/unilang/spec.md` (if it exists)
    *   `module/move/unilang/spec_addendum.md`

### Expected Behavior Rules / Specifications
*   (This task will define these rules in `spec_addendum.md`)

### Tests
| Test ID | Status | Notes |
|---|---|---|

### Crate Conformance Check Procedure
*   (N/A for this specification task)

### Increments

##### Increment 1: Define Command Path and Argument Parsing Rules
*   **Goal:** Refine `spec_addendum.md` to clearly define how command paths are parsed and how they transition into argument parsing.
*   **Specification Reference:** New specification to be created.
*   **Steps:**
    *   Step 1: Read `module/move/unilang/spec_addendum.md`.
    *   Step 2: Append the new parsing rules to `spec_addendum.md`.
    *   Step 3: Perform Increment Verification.
*   **Increment Verification:**
    *   1.  Read `module/move/unilang/spec_addendum.md` and verify the new section and rules are present and correctly formatted.
*   **Commit Message:** "docs(spec): Clarify command path and argument parsing rules"

### Task Requirements
*   The new specification must be clear and unambiguous.
*   It must resolve the current conflicts observed in `argument_parsing_tests.rs` and `syntactic_analyzer_command_tests.rs`.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.

### Assumptions
*   The user will approve the new specification.
*   The `Primary Editable Crate` is `module/move/unilang`.
*   `spec.md` does not exist, and only `spec_addendum.md` should be modified.

### Out of Scope
*   Implementing any parser changes based on the new specification. This task is purely for documentation.

### External System Dependencies
*   None

### Notes & Insights
*   This clarification is essential to unblock the parser bug fix.

### Changelog
*   [User Feedback | 2025-07-07 20:21 UTC] Task interrupted due to ambiguity in command path/argument parsing. Initiating Stuck Resolution Process.
*   [AI | 2025-07-26 05:41:54 UTC] Permissions approved. Proceeding with detailed planning.
*   [AI | 2025-07-26 05:42:26 UTC] Appended parsing rules to `spec_addendum.md`.
