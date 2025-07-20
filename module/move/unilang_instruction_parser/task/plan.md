# Task Plan: Stabilize `unilang_instruction_parser` via `strs_tools` Integration and Full Test Coverage

### Goal
*   To refactor the `unilang_instruction_parser` to be a simple, robust syntactic analyzer that correctly leverages the new tokenization and unescaping features of the `strs_tools` crate. The primary goal is to achieve 100% test coverage against the language specification (`spec.md`) and deliver a maintainable, production-ready parser.

### Ubiquitous Language (Vocabulary)
*   **`strs_tools`**: The dependency responsible for all low-level string tokenization, quoting, and unescaping.
*   **`spec.md`**: The primary specification document for the Unilang framework, especially Section 2 which defines the language syntax.
*   **`Test Matrix`**: A table mapping every rule in `spec.md` to a specific test case to ensure complete coverage.
*   **`Syntactic Analysis`**: The sole responsibility of this crate: to interpret the stream of tokens from `strs_tools` and structure it into a `GenericInstruction`.

### Progress
*   **Primary Editable Crate:** `module/move/unilang_instruction_parser`
*   **Overall Progress:** 0/6 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Deep Integration with `strs_tools`
    *   ⚫ Increment 2: Test Coverage Analysis and Planning
    *   ⚫ Increment 3: Implementation of Missing Tests and Bug Fixes
    *   ⚫ Increment 4: Parser Engine Simplification and Refactoring
    *   ⚫ Increment 5: Final Polish, Documentation, and Cleanup
    *   ⚫ Increment 6: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:** None

### Relevant Context
*   **Key Insight:** The `strs_tools` crate now performs unescaping directly when `quoting(true)` is used and `preserving_quoting(false)` is set (the default). The `Split` struct it yields contains a `Cow<'a, str>`, which will be `Owned` if unescaping occurred. This means `unilang_instruction_parser` **must not** implement its own unescaping logic and must instead consume the already-processed strings from `strs_tools`.
*   Control Files to Reference:
    *   `module/move/unilang/spec.md`
    *   `module/core/strs_tools/src/string/split.rs` (to understand the `Split` struct and options)

### Expected Behavior Rules / Specifications
*   The parser **must** correctly configure and use `strs_tools` for all tokenization.
*   The parser **must not** contain any of its own string unescaping logic.
*   The final test suite **must** cover every syntactic rule defined in Section 2 of `spec.md`.
*   The crate must have no `clippy` warnings and pass all tests.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| | | |

### Crate Conformance Check Procedure
*   Step 1: Execute `cargo test -p unilang_instruction_parser --all-targets`. Analyze output for failures. If any, initiate Critical Log Analysis.
*   Step 2: If tests pass, execute `cargo clippy -p unilang_instruction_parser -- -D warnings`. Analyze output for failures. If any, initiate Linter Fix & Regression Check Procedure.

### Increments

##### Increment 1: Deep Integration with `strs_tools`
*   **Goal:** To refactor the parser to correctly use `strs_tools` as the sole authority for tokenization and unescaping, removing all redundant local logic.
*   **Specification Reference:** `spec.md` Section 1.1 (Mandate to use `strs_tools`).
*   **Steps:**
    1.  **Configure `strs_tools`:** In `src/parser_engine.rs`, modify the `strs_tools::split()` call to be the single source of tokenization. The configuration should be: `.delimeter(vec![" ", "\n", "::", "?", "#", "."])`, `.preserving_delimeters(true)`, `.quoting(true)`, and `.preserving_quoting(false)`.
    2.  **Remove Redundant Logic:** Delete the `unescape_string` function from `src/item_adapter.rs` as it is now provided by `strs_tools`.
    3.  **Adapt to `Cow<str>`:** Modify the `parser_engine` and `item_adapter` to correctly handle the `Cow<'a, str>` from `strs_tools::Split`. Use `.into_owned()` where an owned `String` is required for the final `GenericInstruction`.
    4.  **Enable and Fix Tests:** Remove the `#[ignore]` attribute from all tests in `tests/argument_parsing_tests.rs` and `tests/error_reporting_tests.rs`.
    5.  **Initial Verification:** Run the entire test suite. Fix any breakages that arise from the new `strs_tools` integration. The goal is not to fix all logic bugs, but to ensure the crate compiles and runs with the new dependency structure.
*   **Increment Verification:**
    *   Execute `cargo check -p unilang_instruction_parser`. It must pass.
    *   Execute `cargo test -p unilang_instruction_parser --all-targets`. It is acceptable for some logic tests to fail, but crashes, panics, or compilation errors related to the integration must be resolved.
*   **Commit Message:** "refactor(parser): Integrate deeply with strs_tools for tokenization and unescaping"

##### Increment 2: Test Coverage Analysis and Planning
*   **Goal:** To perform a comprehensive analysis of test coverage against `spec.md` and to create a detailed plan (a Test Matrix) for all missing test cases.
*   **Specification Reference:** `spec.md` Section 2 (all rules).
*   **Steps:**
    1.  Systematically review every rule in `spec.md` Section 2 and Appendix B.
    2.  Map each rule to an existing test case in the project.
    3.  Identify all rules that are not explicitly and thoroughly tested. Key gaps identified so far include: trailing dots, various help operator usages, behavior of sequential `;;` delimiters, and edge cases with whitespace.
    4.  Create a comprehensive `Test Matrix` as a markdown table. Each row will represent a missing test case, detailing the input, the specific rule it covers, and the expected outcome (`Ok` or a specific `Err`).
    5.  Update this `task_plan.md` file to include the new `Test Matrix` in a dedicated section.
*   **Increment Verification:**
    *   The `task_plan.md` file is updated with a `### Test Matrix` section that is complete and covers all identified gaps.
*   **Commit Message:** "chore(planning): Analyze test coverage and create Test Matrix for spec adherence"

##### Increment 3: Implementation of Missing Tests and Bug Fixes
*   **Goal:** To write and pass all the new tests defined in the Test Matrix from Increment 2, fixing any bugs in the parser logic that are uncovered.
*   **Specification Reference:** `spec.md` Section 2.
*   **Steps:**
    1.  Create a new test file: `tests/spec_adherence_tests.rs`.
    2.  For each entry in the `Test Matrix`, implement the corresponding test case in `spec_adherence_tests.rs`. Initially, expect these tests to fail.
    3.  Run the new test suite.
    4.  Iteratively debug and fix the logic in `src/parser_engine.rs` and `src/item_adapter.rs` until all tests in `spec_adherence_tests.rs` and all other existing tests pass.
*   **Increment Verification:**
    *   Execute `cargo test -p unilang_instruction_parser --all-targets`. The command must pass with zero failures.
*   **Commit Message:** "test(parser): Implement full spec adherence test suite and fix uncovered bugs"

##### Increment 4: Parser Engine Simplification and Refactoring
*   **Goal:** With the safety of a complete test suite, refactor the `parser_engine.rs` for simplicity, clarity, and maintainability.
*   **Specification Reference:** N/A.
*   **Steps:**
    1.  Analyze the logic in `parse_single_instruction_from_rich_items` and `parse_multiple_instructions`.
    2.  Identify areas of unnecessary complexity, such as convoluted loops or state management.
    3.  Refactor the code into smaller, more focused helper functions where appropriate.
    4.  Improve variable names and add comments to clarify the logic of the parsing state machine.
    5.  After each significant refactoring step, run the full test suite to ensure no regressions have been introduced.
*   **Increment Verification:**
    *   Execute `cargo test -p unilang_instruction_parser --all-targets`. The command must pass.
    *   A manual code review confirms that the logic in `parser_engine.rs` is clearer and easier to understand.
*   **Commit Message:** "refactor(parser): Simplify and clarify parser engine logic"

##### Increment 5: Final Polish, Documentation, and Cleanup
*   **Goal:** To bring the crate to a production-quality standard by fixing all linter warnings, improving documentation, and cleaning up the codebase.
*   **Specification Reference:** N/A.
*   **Steps:**
    1.  Execute `cargo clippy -p unilang_instruction_parser -- -D warnings`.
    2.  Apply the `Linter Fix & Regression Check Procedure` to resolve every reported clippy warning.
    3.  Review all public-facing documentation (`lib.rs`, `README.md`, public structs and functions) to ensure it is accurate and reflects the final, stable implementation.
    4.  Update the examples in `examples/` to be simple and clear.
    5.  Remove any temporary or debug-related files/code.
*   **Increment Verification:**
    *   Execute `cargo clippy -p unilang_instruction_parser -- -D warnings`. The command must pass with no warnings.
    *   Execute `cargo test -p unilang_instruction_parser --all-targets`. The command must pass.
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
*   **Commit Message:** "chore(task): Complete stabilization of unilang_instruction_parser"

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
*   [2025-07-20 12:49 UTC] Created a new, comprehensive plan based on deep `strs_tools` analysis and test coverage review.
