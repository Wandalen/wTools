# Project Plan: Fix and Improve `module/move/unilang_instruction_parser`

### Goal
*   Fix all tests and warnings of crate `module/move/unilang_instruction_parser`.
*   Ensure all tests are enabled and according to specification.
*   Make `Readme.md` concise and clearly communicate the purpose of the crate.
*   Organize examples consistently with other crates and ensure they are useful for developers.

### Progress
*   ✅ All Increments Complete

### Target Crate
*   `module/move/unilang_instruction_parser`

### Relevant Context
*   Files to Include:
    *   `module/move/unilang_instruction_parser/Cargo.toml`
    *   `module/move/unilang_instruction_parser/Readme.md`
    *   `module/move/unilang_instruction_parser/src/config.rs`
    *   `module/move/unilang_instruction_parser/src/error.rs`
    *   `module/move/unilang_instruction_parser/src/instruction.rs`
    *   `module/move/unilang_instruction_parser/src/item_adapter.rs`
    *   `module/move/unilang_instruction_parser/src/lib.rs`
    *   `module/move/unilang_instruction_parser/src/parser_engine.rs`
    *   `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/comprehensive_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/error_reporting_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/parser_config_entry_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/syntactic_analyzer_command_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/tests.rs`
    *   `module/move/unilang_instruction_parser/tests/inc/mod.rs`
    *   `module/move/unilang_instruction_parser/examples/basic_usage.rs`
*   External Crates Requiring `task.md` Proposals:
    *   `module/core/strs_tools` (Reason: Clippy warnings prevent clean compilation with `-D warnings`, and tokenization issues affect unescaping tests in `unilang_instruction_parser`.)
    *   `module/core/former_meta` (Reason: Compilation error `E0554` and clippy warnings block workspace build.)
    *   `module/move/willbe` / `module/alias/cargo_will` (Reason: Output filename collisions block clean workspace build.)

### Expected Behavior Rules / Specifications (for Target Crate)
*   (To be defined as issues are identified)

### Target File Structure (If Applicable)
*   (No major structural changes planned initially, only content modifications)

### Increments

*   ✅ Increment 1: Initial Build and Test Run
    *   Detailed Plan Step 1: Execute `cargo test -p unilang_instruction_parser` to identify failing tests.
    *   Detailed Plan Step 2: Execute `cargo clippy -p unilang_instruction_parser -- -D warnings` to identify warnings.
    *   Pre-Analysis: Assess current state of tests and warnings.
    *   Crucial Design Rules: N/A
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: Analyze `execute_command` output for test failures and clippy warnings.
    *   Commit Message: "chore(unilang_instruction_parser): Initial build and test run to identify issues"

*   ✅ Increment 2: Fix Warnings and Basic Compilation Errors
    *   Detailed Plan Step 1: Analyze `cargo clippy` output and fix identified warnings.
    *   Detailed Plan Step 2: Analyze `cargo test` output for compilation errors and fix them.
    *   Pre-Analysis: Based on Increment 1's output.
    *   Crucial Design Rules: [Code Style: Do Not Reformat Arbitrarily], [Lints and warnings]
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: Execute `cargo clippy -p unilang_instruction_parser -- -D warnings` and `cargo build -p unilang_instruction_parser`. Analyze `execute_command` output for success (no warnings, no compilation errors).
    *   Commit Message: "fix(unilang_instruction_parser): Address clippy warnings and compilation errors"

*   ✅ Increment 3: Enable and Fix Tests
    *   Detailed Plan Step 1: Modify `src/parser_engine.rs` to correctly handle quoted values as positional arguments, not command path segments, and correctly terminate command path on `::` delimiter.
    *   Detailed Plan Step 2: Read all test files (`tests/*.rs`, `tests/inc/mod.rs`) to identify disabled tests (e.g., `#[ignore]`, `#[cfg(test)]` blocks that might be commented out).
    *   Detailed Plan Step 3: Enable any disabled tests.
    *   Detailed Plan Step 4: Analyze failing tests and fix their logic.
    *   Pre-Analysis: Based on Increment 1's output and test file content.
    *   Crucial Design Rules: [Testing: Standard Directory for All Tests], [Testing: Plan with a Test Matrix When Writing Tests]
    *   Relevant Behavior Rules: Quoted values after the initial command should be treated as positional arguments. `::` delimiter should terminate command path. `.` and `/` in unquoted tokens should be treated as path separators. Positional arguments after named arguments should be allowed in the doctest.
    *   Verification Strategy: Execute `cargo test -p unilang_instruction_parser`. Analyze `execute_command` output for all tests passing.
    *   Commit Message: "fix(unilang_instruction_parser): Enable and fix failing tests"

*   ✅ Increment 4: Review and Refine Test Specifications
    *   Detailed Plan Step 1: Review `src/instruction.rs` to understand the `GenericInstruction` and `Argument` structures.
    *   Detailed Plan Step 2: Review `src/parser_engine.rs` and `src/item_adapter.rs` to ensure the parsing logic is fully covered by tests.
    *   Detailed Plan Step 3: Identify any edge cases or complex interactions that might not be explicitly tested.
    *   Detailed Plan Step 4: Add a new comprehensive test `ct6_1_command_path_with_dots_and_slashes` to `tests/comprehensive_tests.rs`.
    *   Pre-Analysis: All existing tests pass. Focus on completeness and clarity.
    *   Crucial Design Rules: [Testing: Plan with a Test Matrix When Writing Tests], [Comments and Documentation]
    *   Relevant Behavior Rules: Command paths can contain `.` and `/` as separators within a single token.
    *   Verification Strategy: Execute `cargo test -p unilang_instruction_parser`. Analyze `execute_command` output for all tests passing.
    *   Test Matrix:
        *   #### Test Matrix for Command Path with Dots and Slashes
            | ID    | Input                                     | Expected Command Path Slices | Expected Positional Args | Expected Named Args | Expected Help | Notes                                     |
            |-------|-------------------------------------------|------------------------------|--------------------------|---------------------|---------------|-------------------------------------------|
            | CT6.1 | `cmd.sub/path arg1 name::val`             | `["cmd", "sub", "path", "arg1"]` | `[]`                     | `{"name": "val"}`   | `false`       | Command path with `.` and `/` separators. |
    *   Commit Message: "refactor(unilang_instruction_parser): Refine test specifications and coverage"

*   ✅ Increment 5: Update `Readme.md`
    *   Detailed Plan Step 1: Read `module/move/unilang_instruction_parser/Readme.md`.
    *   Detailed Plan Step 2: Rewrite the `Readme.md` to be concise and clearly communicate the crate's purpose.
    *   Pre-Analysis: Current `Readme.md` content.
    *   Crucial Design Rules: [Comments and Documentation]
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: Confirm `write_to_file` success.
    *   Commit Message: "docs(unilang_instruction_parser): Update Readme.md for clarity and conciseness"

*   ✅ Increment 6: Organize and Improve Examples
    *   Detailed Plan Step 1: Read existing examples in `examples/`.
    *   Detailed Plan Step 2: Review examples for usefulness and clarity.
    *   Detailed Plan Step 3: Rename/restructure examples to match common patterns in other crates (e.g., `_trivial_sample.rs`, `_more.rs`).
    *   Detailed Plan Step 4: Improve example code and add new examples if necessary to demonstrate key features.
    *   Pre-Analysis: Current examples content and structure.
    *   Crucial Design Rules: [Comments and Documentation]
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: Execute `cargo build --examples -p unilang_instruction_parser`. Analyze `execute_command` output for successful compilation of examples.
    *   Commit Message: "feat(unilang_instruction_parser): Organize and improve examples"

### Task Requirements
*   Fix all tests and warnings.
*   Ensure all tests are enabled.
*   Ensure all tests are according to specification.
*   `Readme.md` is concise and clearly communicates purpose.
*   Examples are organized like other crates.
*   Examples are useful for developers.

### Project Requirements
*   (No specific project requirements identified yet, will add if discovered)

### Notes & Insights
*   Initial assessment suggests a focus on test stability and documentation.
*   Clippy warnings in `strs_tools` are blocking clean compilation with `-D warnings`. A `task.md` has been proposed for this.
*   Unescaping tests in `unilang_instruction_parser` are currently ignored due to dependency on `strs_tools`'s tokenization issues.
*   Compilation errors and output filename collisions in `former_meta`, `willbe`, and `cargo_will` are blocking clean workspace builds. `task.md` proposals have been created for these.
