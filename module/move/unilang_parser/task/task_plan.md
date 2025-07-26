# Task Plan: Fix Path Parsing in `unilang_parser`

### Goal
*   To modify `unilang_parser` to correctly parse file paths containing dots as single argument values, preventing them from being incorrectly tokenized, and ensuring that CLI integration tests in the `unilang` crate pass.

### Ubiquitous Language (Vocabulary)
*   **Instruction:** A single command line input, e.g., `cat path::/path/to/file.txt`.
*   **Token:** A single unit recognized by the parser, like a command name, argument key, or value.
*   **Argument:** A key-value pair in an instruction, e.g., `path::/path/to/file.txt`.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/move/unilang_parser`
*   **Overall Progress:** 0/3 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Analyze Current Parsing Logic and Add Failing Test
    *   ⚫ Increment 2: Implement Path-Aware Parsing Logic
    *   ⚫ Increment 3: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/move/unilang` (Reason: To verify the fix with existing integration tests)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/move/unilang_parser/src/parser_engine.rs`
    *   `module/move/unilang_parser/src/instruction.rs`
    *   `module/move/unilang/tests/inc/phase2/cli_integration_test.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`

### Expected Behavior Rules / Specifications
*   Rule 1: A string like `cat path::/tmp/.tmp.file.txt` should be parsed into a single instruction.
*   Rule 2: The instruction should have the command `cat`.
*   Rule 3: The instruction should have one argument.
*   Rule 4: The argument key should be `path`.
*   Rule 5: The argument value should be `/tmp/.tmp.file.txt`.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| | | |

### Crate Conformance Check Procedure
*   Run `timeout 90 cargo test -p unilang_parser --all-targets`.
*   Run `timeout 90 cargo test -p unilang --test cli_integration_test`.
*   Run `timeout 90 cargo clippy -p unilang_parser -- -D warnings`.
*   Run `timeout 90 cargo clippy -p unilang -- -D warnings`.

### Increments
##### Increment 1: Analyze Current Parsing Logic and Add Failing Test
*   **Goal:** Understand the existing tokenization and parsing mechanism and add a new test case to `unilang_parser` that replicates the reported path parsing failure.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/move/unilang_parser/src/parser_engine.rs` to understand the parsing logic.
    *   Step 2: Create a new test file `module/move/unilang_parser/tests/path_parsing_test.rs`.
    *   Step 3: Add a test case to the new file that attempts to parse an instruction with a dot-containing path, like `cat path::/tmp/.test.file`, and asserts the correct structure. This test is expected to fail.
    *   Step 4: Run the tests for `unilang_parser` to confirm the new test fails as expected.
*   **Increment Verification:**
    *   Confirm that the new test `path_parsing_test` fails with a parsing error.
*   **Commit Message:** "feat(unilang_parser): Add failing test for path parsing"

##### Increment 2: Implement Path-Aware Parsing Logic
*   **Goal:** Modify the parser to correctly handle paths with dots as single argument values.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Modify the tokenization logic in `parser_engine.rs` to treat paths as a single token.
    *   Step 2: Run the tests for `unilang_parser` again, expecting the `path_parsing_test` to pass.
    *   Step 3: Run the integration tests for the `unilang` crate to ensure the fix works in the broader context.
*   **Increment Verification:**
    *   Confirm that all tests in `unilang_parser` pass.
    *   Confirm that `test_cli_cat_command_valid_file` and `test_cli_cat_command_non_existent_file` in `unilang` pass.
*   **Commit Message:** "fix(unilang_parser): Correctly parse paths with dots"

##### Increment 3: Finalization
*   **Goal:** Perform a final review, cleanup, and verification of the entire task's output.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Run a full Crate Conformance Check.
    *   Step 2: Review all changes for code style and documentation.
    *   Step 3: Remove any temporary files or test configurations.
*   **Increment Verification:**
    *   All checks in the Crate Conformance Check procedure must pass.
*   **Commit Message:** "chore(unilang_parser): Finalize path parsing fix"

### Task Requirements
*   The solution must not introduce any regressions in parsing other valid instruction formats.
*   The fix should be contained within the `unilang_parser` crate.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook.

### Assumptions
*   The core of the issue lies within the tokenization or argument splitting logic of `unilang_parser`.

### Out of Scope
*   Large-scale refactoring of the entire parser.
*   Fixing other unrelated parsing bugs.

### External System Dependencies
*   None

### Notes & Insights
*   The `strs_tools` crate is used for string splitting, so the issue might be related to its configuration or usage.

### Changelog
*   [Increment 0 | 2025-07-26 05:49:13 UTC] Initialized task plan.