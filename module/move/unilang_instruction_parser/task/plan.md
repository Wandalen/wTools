# Task Plan: Refactor and Stabilize `unilang_instruction_parser`

### Goal
*   To refactor the `unilang_instruction_parser` crate to resolve internal inconsistencies, implement all required functionality according to `spec.md`, and ensure the codebase is robust, fully tested, and maintainable. The final crate will serve as a reliable, specification-compliant parser for the Unilang ecosystem.

### Ubiquitous Language (Vocabulary)
*   **`spec.md`**: The primary specification document for the Unilang framework.
*   **`GenericInstruction`**: The structured representation of a single parsed command, as defined in `src/instruction.rs`.
*   **`Argument`**: The structured representation of a command argument, as defined in `src/instruction.rs`.
*   **`Parser Engine`**: The core logic within `src/parser_engine.rs` responsible for syntactic analysis.
*   **`Item Adapter`**: The component in `src/item_adapter.rs` that classifies raw string tokens from `strs_tools`.
*   **`Unescaping`**: The process of converting escape sequences (e.g., `\"`) into their literal character equivalents (e.g., `"`).

### Progress
*   **Primary Editable Crate:** `module/move/unilang_instruction_parser`
*   **Overall Progress:** 1/6 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Unify Data Structures and Refactor Parser Engine
    *   ⏳ Increment 2: Implement String Unescaping Logic
    *   ⚫ Increment 3: Stabilize Core Parsing and Fix Logic Bugs
    *   ⚫ Increment 4: Comprehensive Specification Adherence Testing
    *   ⚫ Increment 5: Final Polish, Documentation, and Cleanup
    *   ⚫ Increment 6: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:** None

### Relevant Context
*   Control Files to Reference:
    *   `module/move/unilang/spec.md`
    *   `module/move/unilang_instruction_parser/spec_addendum.md`
*   Files to Include (for AI's reference):
    *   All files within `module/move/unilang_instruction_parser/src/`
    *   All files within `module/move/unilang_instruction_parser/tests/`
    *   `module/move/unilang_instruction_parser/Cargo.toml`

### Expected Behavior Rules / Specifications
*   The parser **must** produce `instruction::GenericInstruction` and `instruction::Argument` structs as its output.
*   The parser **must** correctly unescape string values for arguments and subjects according to the specification.
*   The parser **must** pass all existing and newly created tests.
*   The parser **must** have no `clippy` warnings with the workspace-defined lints.
*   The parser's behavior **must** be fully compliant with `module/move/unilang/spec.md`, Section 2.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| | | |

### Crate Conformance Check Procedure
*   Step 1: Execute `cargo test -p unilang_instruction_parser --all-targets`. Analyze output for failures. If any, initiate Critical Log Analysis.
*   Step 2: If tests pass, execute `cargo clippy -p unilang_instruction_parser -- -D warnings`. Analyze output for failures. If any, initiate Linter Fix & Regression Check Procedure.

### Increments

##### Increment 1: Unify Data Structures and Refactor Parser Engine
*   **Goal:** To refactor the parser engine to eliminate the outdated internal `GenericInstruction` struct and use the official, public-facing data structures from the `instruction` module. This establishes a consistent and correct foundation for future work.
*   **Specification Reference:** `spec.md` Section 10.2 (Parser Responsibilities), `src/instruction.rs` (as de-facto API contract).
*   **Steps:**
    1.  Read the content of `src/parser_engine.rs`.
    2.  Use `search_and_replace` to remove the local `GenericInstruction` struct definition from `src/parser_engine.rs`.
    3.  Use `insert_content` to add `use crate::instruction::{ Argument, GenericInstruction };` to `src/parser_engine.rs`.
    4.  Refactor `parse_single_instruction_from_rich_items` and `parse_multiple_instructions` to construct and return the official `instruction::GenericInstruction` and `instruction::Argument` types. This will involve multiple `search_and_replace` calls to update field names and construction logic (e.g., `command_path` to `command_path_slices`, `arguments` to `positional_arguments`, etc.).
    5.  Perform Increment Verification.
*   **Increment Verification:**
    *   Execute `cargo check -p unilang_instruction_parser`. The command must succeed, confirming the data structure unification and refactoring is syntactically correct.
*   **Commit Message:** "refactor(parser): Unify data structures to use official instruction types"

##### Increment 2: Implement String Unescaping Logic
*   **Goal:** To implement and integrate the missing string unescaping functionality, ensuring that all quoted values are correctly processed as per the specification.
*   **Specification Reference:** `src/instruction.rs` documentation for `Argument::value` (states it is unescaped).
*   **Steps:**
    1.  Read `src/item_adapter.rs`.
    2.  Add a new public function `unescape_string` to `src/item_adapter.rs`. This function will take a `&str` and a `SourceLocation` and return a `Result<String, ParseError>`. It will handle `\\`, `\"`, `\'`, `\n`, and `\t` escape sequences. If an invalid escape sequence is found, it should return an `Err` with `ErrorKind::InvalidEscapeSequence` and a `SourceLocation` pointing to the invalid sequence.
    3.  Read `src/parser_engine.rs`.
    4.  In `parse_single_instruction_from_rich_items`, when creating `Argument` structs for both positional and named arguments, call `item_adapter::unescape_string` on the raw string value before assigning it to the `value` field.
    5.  Read `tests/argument_parsing_tests.rs` and `tests/error_reporting_tests.rs`.
    6.  Remove the `#[ignore]` attribute from the six tests related to unescaping.
    7.  Perform Increment Verification.
*   **Increment Verification:**
    *   Execute `cargo test -p unilang_instruction_parser --test argument_parsing_tests`.
    *   Execute `cargo test -p unilang_instruction_parser --test error_reporting_tests`.
    *   All tests in these files, especially the newly enabled ones, must pass.
*   **Commit Message:** "feat(parser): Implement and integrate string unescaping logic"

##### Increment 3: Stabilize Core Parsing and Fix Logic Bugs
*   **Goal:** To achieve a fully passing test suite by fixing the logical bugs and inconsistencies in the parser engine, including the known trailing delimiter and span location issues.
*   **Specification Reference:** `spec.md` Section 2.
*   **Steps:**
    1.  Execute the full test suite via `cargo test -p unilang_instruction_parser --all-targets`.
    2.  For each failing test, apply the `Critical Log Analysis` procedure.
    3.  **Focus Area 1 (Trailing Delimiter):** Refactor the logic in `parse_multiple_instructions` to be more robust. Simplify the loop and the end-of-input checks to correctly identify and report `ErrorKind::TrailingDelimiter` and `ErrorKind::EmptyInstructionSegment` without panicking or producing incorrect results.
    4.  **Focus Area 2 (Span Location):** Investigate the `qqq:` comment in `tests/argument_parsing_tests.rs` regarding the incorrect span for `named_arg_with_quoted_escaped_value_location`. Trace the `SourceLocation` from `strs_tools::Split` through `item_adapter::classify_split` to the final `Argument` struct. Correct the logic that calculates or propagates the `end` offset.
    5.  Iteratively fix bugs until the entire test suite passes.
*   **Increment Verification:**
    *   Execute `cargo test -p unilang_instruction_parser --all-targets`. The command must pass with no failing tests.
*   **Commit Message:** "fix(parser): Stabilize engine, fix trailing delimiter and span location bugs"

##### Increment 4: Comprehensive Specification Adherence Testing
*   **Goal:** To guarantee the parser is fully compliant with the language specification by systematically creating and passing tests for every rule defined in `spec.md`.
*   **Specification Reference:** `spec.md` Section 2.
*   **Steps:**
    1.  Create a new test file: `tests/spec_adherence_tests.rs`.
    2.  Create a checklist of every explicit and implicit rule in `spec.md`, Section 2 (e.g., "Rule 2.3: Trailing dot is a syntax error", "Rule 2.1: Whitespace around dots is ignored", etc.).
    3.  For each rule on the checklist, review existing tests to see if it's already covered.
    4.  For any rule not adequately covered, write a new, focused test case in `spec_adherence_tests.rs`.
    5.  Ensure each new test case asserts the correct behavior (either a specific `Ok(GenericInstruction)` structure or a specific `Err(ParseError)` with the right `ErrorKind`).
    6.  Run the new test suite and fix any failures in the parser engine until all spec adherence tests pass.
*   **Increment Verification:**
    *   Execute `cargo test -p unilang_instruction_parser --test spec_adherence_tests`. The command must pass.
*   **Commit Message:** "test(parser): Add comprehensive test suite for spec adherence"

##### Increment 5: Final Polish, Documentation, and Cleanup
*   **Goal:** To bring the crate to a production-quality standard by fixing all linter warnings, improving documentation, and cleaning up the codebase.
*   **Specification Reference:** N/A.
*   **Steps:**
    1.  Execute `cargo clippy -p unilang_instruction_parser -- -D warnings`.
    2.  Apply the `Linter Fix & Regression Check Procedure` to resolve every reported clippy warning.
    3.  Review all public-facing documentation (`lib.rs`, `README.md`, public structs and functions) to ensure it is accurate, clear, and reflects the now-stable implementation.
    4.  Update the examples in `examples/` to be simple, clear, and demonstrate the correct and final API usage.
    5.  Remove any temporary or debug-related files/code that are no longer needed (e.g., `tests/temp_unescape_test.rs` if it exists).
*   **Increment Verification:**
    *   Execute `cargo clippy -p unilang_instruction_parser -- -D warnings`. The command must pass with no warnings.
    *   Execute `cargo test -p unilang_instruction_parser --all-targets`. The command must pass.
    *   Manually review documentation for clarity and correctness.
*   **Commit Message:** "chore(parser): Final polish, fix all clippy warnings and update docs"

##### Increment 6: Finalization
*   **Goal:** To perform a final, holistic review and verification of the entire task's output.
*   **Specification Reference:** All project requirements.
*   **Steps:**
    1.  Perform a self-critique of all changes against the plan's `Goal` and `Expected Behavior Rules`.
    2.  Execute the full `Crate Conformance Check Procedure` one last time.
    3.  Execute `git status` to ensure the working directory is clean.
*   **Increment Verification:**
    *   All steps of the `Crate Conformance Check Procedure` must pass.
    *   `git status` must show a clean working tree.
*   **Commit Message:** "chore(task): Complete refactoring and stabilization of unilang_instruction_parser"

### Task Requirements
*   Fix all tests and warnings.
*   All tests must be enabled.
*   All tests must be according to specification `module/move/unilang/spec.md`.
*   Readme must be concise and clearly communicate purpose.
*   Examples must be organized like other crates' examples.
*   Examples must be useful for developers.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   Never use `#[allow(clippy::missing_errors_doc)]`.

### Assumptions
*   The `strs_tools` crate functions correctly as per its own specification for tokenizing quoted strings.

### Out of Scope
*   Modifying any crate other than `unilang_instruction_parser`.
*   Implementing features not described in `spec.md`.

### External System Dependencies
*   None.

### Notes & Insights
*   The previous plan was abandoned due to significant architectural drift between the implementation and the crate's public API. This new plan prioritizes fixing the foundation before addressing feature-level bugs.

### Changelog
*   [Increment 1] Refactored `parser_engine.rs` to construct the official `Argument` and `GenericInstruction` structs.
*   [Increment 1] Updated function signatures in `parser_engine.rs` to use the official `GenericInstruction` type.
*   [Increment 1] Removed outdated local `GenericInstruction` and imported the official one from the `instruction` module.
*   [2025-07-20 12:33 UTC] Initial plan created to refactor and stabilize the crate.
