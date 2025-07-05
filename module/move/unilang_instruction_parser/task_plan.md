# Task Plan: Fix Command Parsing in `unilang_instruction_parser`

### Goal
*   To fix a critical bug in `unilang_instruction_parser::Parser` where the command name is incorrectly parsed as a positional argument instead of being placed in `command_path_slices`. This will enable correct command identification in the `unilang` crate.

### Ubiquitous Language (Vocabulary)
*   **`GenericInstruction`**: The struct that represents a parsed command, containing fields for the command path, named arguments, and positional arguments.
*   **`command_path_slices`**: The field in `GenericInstruction` that should contain the components of the command name (e.g., `["test", "command"]` for `.test.command`).
*   **`Parser`**: The main entity in this crate responsible for parsing command strings into `GenericInstruction` instances.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/move/unilang_instruction_parser`
*   **Overall Progress:** 1/4 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Replicate the Bug with a Test
    *   ✅ Increment 2: Implement the Parser Fix
    *   ⚫ Increment 3: Verify the Fix and Clean Up
    *   ⚫ Increment 4: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** false
*   **Additional Editable Crates:**
    *   None

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./task.md` (The original change proposal)
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `src/parser_engine.rs`
    *   `src/instruction.rs`
    *   `tests/syntactic_analyzer_command_tests.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   None
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   None

### Expected Behavior Rules / Specifications
*   Rule 1: Given an input string like `.test.command arg1`, the parser must populate `GenericInstruction.command_path_slices` with `["test", "command"]`.
*   Rule 2: The first element of the input string, if it starts with a `.` or is a valid identifier, should be treated as the command, not a positional argument.
*   Rule 3: Positional arguments should only be populated with elements that follow the command.

### Crate Conformance Check Procedure
*   Step 1: Execute `timeout 90 cargo test -p unilang_instruction_parser --all-targets` via `execute_command`.
*   Step 2: Analyze `execute_command` output. If it fails, initiate Critical Log Analysis.
*   Step 3: If tests pass, execute `timeout 90 cargo clippy -p unilang_instruction_parser -- -D warnings` via `execute_command`.
*   Step 4: Analyze `execute_command` output. If it fails, initiate Linter Fix & Regression Check Procedure.

### Increments
##### Increment 1: Replicate the Bug with a Test
*   **Goal:** Create a new, failing test case that explicitly demonstrates the incorrect parsing of command paths.
*   **Specification Reference:** `task.md` section "Problem Statement / Justification".
*   **Steps:**
    *   Step 1: Create a new test file `tests/command_parsing_tests.rs`.
    *   Step 2: Add a test function `parses_command_path_correctly` to the new file.
    *   Step 3: In the test, use the `Parser` to parse the string `.test.command arg1`.
    *   Step 4: Assert that the resulting `GenericInstruction` has `command_path_slices` equal to `vec!["test", "command"]`.
    *   Step 5: Assert that the `positional_arguments` are `vec!["arg1"]` and do not contain the command.
    *   Step 6: Add the new test file to `tests/tests.rs`.
    *   Step 7: Perform Increment Verification.
    *   Step 8: Perform Crate Conformance Check (expecting failure on the new test).
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p unilang_instruction_parser --test command_parsing_tests` via `execute_command`.
    *   Step 2: Analyze the output to confirm that the `parses_command_path_correctly` test fails with an assertion error related to `command_path_slices` or `positional_arguments`.
*   **Commit Message:** "test(parser): Add failing test for incorrect command path parsing"

##### Increment 2: Implement the Parser Fix
*   **Goal:** Modify the parser logic to correctly distinguish command paths from arguments.
*   **Specification Reference:** `task.md` section "Proposed Solution / Specific Changes".
*   **Steps:**
    *   Step 1: Read `src/parser_engine.rs`.
    *   Step 2: Analyze the parsing logic to identify where the first element is being incorrectly handled.
    *   Step 3: Modify the logic to check if the first token is a command path. If so, populate `command_path_slices` and consume the token.
    *   Step 4: Ensure subsequent tokens are correctly parsed as arguments.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p unilang_instruction_parser --test command_parsing_tests` via `execute_command`.
    *   Step 2: Analyze the output to confirm the `parses_command_path_correctly` test now passes.
*   **Commit Message:** "fix(parser): Correctly parse command paths instead of treating them as arguments"

##### Increment 3: Verify the Fix and Clean Up
*   **Goal:** Ensure the fix works correctly and does not introduce any regressions. Clean up test code.
*   **Specification Reference:** `task.md` section "Acceptance Criteria".
*   **Steps:**
    *   Step 1: Run the full test suite for `unilang_instruction_parser`.
    *   Step 2: Review existing tests, especially in `tests/syntactic_analyzer_command_tests.rs`, to see if any were implicitly relying on the old, buggy behavior. Refactor them if necessary.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p unilang_instruction_parser --all-targets` via `execute_command`.
    *   Step 2: Analyze the output to confirm all tests pass.
*   **Commit Message:** "refactor(tests): Clean up tests after command parsing fix"

##### Increment 4: Finalization
*   **Goal:** Perform a final review and verification of the entire task's output.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Perform a self-critique of all changes against the plan's goal and requirements.
    *   Step 2: Run the Crate Conformance Check one last time.
    *   Step 3: Execute `git status` to ensure the working directory is clean.
*   **Increment Verification:**
    *   Step 1: Execute the full `Crate Conformance Check Procedure`.
    *   Step 2: Execute `git status` via `execute_command` and confirm the output shows no uncommitted changes.
*   **Commit Message:** "chore: Finalize command parsing fix"

### Task Requirements
*   The fix must correctly handle command paths with and without leading dots.
*   The fix must not introduce any performance regressions.
*   New tests must be added to cover the fixed behavior.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   Must use Rust 2021 edition.

### Assumptions
*   The `unilang` crate is not part of this task's scope, but its requirements drive this fix.
*   The core parsing logic is located within `src/parser_engine.rs`.

### Out of Scope
*   Making any changes to the `unilang` crate.
*   Changing the public API of the `Parser`.

### External System Dependencies
*   None

### Notes & Insights
*   This fix is critical for the architectural unification of `unilang`.

### Changelog
*   [Initial] Plan created to address command parsing bug.
*   [User Feedback] Updated `Permissions & Boundaries` to set `Add transient comments` to `false`.
*   [Increment 1 | 2025-07-05 10:33 UTC] Created `tests/command_parsing_tests.rs` and added it to `tests/tests.rs`. Confirmed the new tests fail as expected, replicating the bug.
*   [Increment 2 | 2025-07-05 10:57 UTC] Implemented the parser fix in `src/parser_engine.rs` and `src/config.rs`. Confirmed `command_parsing_tests` now pass.