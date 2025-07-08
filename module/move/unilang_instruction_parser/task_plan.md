# Task Plan: Fix Command Parsing in `unilang_instruction_parser`

### Goal
*   To fix a critical bug in `unilang_instruction_parser::Parser` where the command name is incorrectly parsed as a positional argument instead of being placed in `command_path_slices`. This will enable correct command identification in the `unilang` crate **without introducing regressions**.

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
    *   ⚫ Increment 2: Revert Flawed Fix and Analyze Existing Tests
    *   ⚫ Increment 3: Implement Robust Parser Fix
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
    *   `src/config.rs`
    *   `src/instruction.rs`
    *   `tests/syntactic_analyzer_command_tests.rs`
    *   `tests/argument_parsing_tests.rs`
    *   `tests/command_parsing_tests.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   None
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   None

### Expected Behavior Rules / Specifications
*   Rule 1: Given an input string like `.test.command arg1`, the parser must populate `GenericInstruction.command_path_slices` with `["test", "command"]`.
*   Rule 2: The first element of the input string, if it starts with a `.` or is a valid identifier, should be treated as the command, not a positional argument.
*   Rule 3: Positional arguments should only be populated with elements that follow the command.
*   Rule 4: All existing tests in `argument_parsing_tests.rs` must continue to pass after the fix.

### Crate Conformance Check Procedure
*   Step 1: Execute `timeout 90 cargo test -p unilang_instruction_parser --all-targets` via `execute_command`.
*   Step 2: Analyze `execute_command` output. If it fails, initiate Critical Log Analysis.
*   Step 3: If tests pass, execute `timeout 90 cargo clippy -p unilang_instruction_parser -- -D warnings` via `execute_command`.
*   Step 4: Analyze `execute_command` output. If it fails, initiate Linter Fix & Regression Check Procedure.

### Increments
##### Increment 1: Replicate the Bug with a Test
*   **Goal:** Create a new, failing test case that explicitly demonstrates the incorrect parsing of command paths.
*   **Status:** ✅ **Completed**
*   **Commit Message:** "test(parser): Add failing test for incorrect command path parsing"

##### Increment 2: Revert Flawed Fix and Analyze Existing Tests
*   **Goal:** Revert the previous, regression-inducing fix and gain a full understanding of all existing test expectations before attempting a new fix.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Use `git restore` to revert the changes made to `src/parser_engine.rs` and `src/config.rs` in the previous attempt.
    *   Step 2: Read the contents of `tests/argument_parsing_tests.rs` and `tests/syntactic_analyzer_command_tests.rs` to fully understand the expected parsing behavior for all argument types.
    *   Step 3: Perform Increment Verification.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p unilang_instruction_parser --all-targets` via `execute_command`.
    *   Step 2: Analyze the output. Expect the new test `command_parsing_tests` to fail (as the bug is now re-introduced) and all other tests (like `argument_parsing_tests`) to pass. This confirms a successful revert.
*   **Commit Message:** "revert(parser): Revert flawed fix that introduced regressions"

##### Increment 3: Implement Robust Parser Fix
*   **Goal:** Modify the parser logic to correctly distinguish command paths from arguments, ensuring all existing tests continue to pass.
*   **Specification Reference:** `task.md` section "Proposed Solution / Specific Changes".
*   **Steps:**
    *   Step 1: Based on the analysis from Increment 2, design a modification to the parsing logic in `src/parser_engine.rs`.
    *   Step 2: The new logic must correctly identify the command token(s) at the start of the input and populate `command_path_slices`.
    *   Step 3: The logic must then correctly transition to parsing positional and named arguments without regression.
    *   Step 4: Implement the changes.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p unilang_instruction_parser --all-targets` via `execute_command`.
    *   Step 2: Analyze the output to confirm that **all** tests, including the new `command_parsing_tests` and the existing `argument_parsing_tests`, now pass.
*   **Commit Message:** "fix(parser): Correctly parse command paths without introducing argument parsing regressions"

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
*   [Rollback | 2025-07-05 11:26 UTC] Previous fix in `src/parser_engine.rs` and `src/config.rs` caused widespread test regressions. Reverting changes and re-planning the fix with a more robust approach.