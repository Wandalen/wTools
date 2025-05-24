# Project Plan: Fix and Improve `module/move/unilang_instruction_parser`

### Goal
*   Fix all tests and warnings of crate `module/move/unilang_instruction_parser`.
*   Ensure all tests are enabled and according to specification.
*   Make `Readme.md` concise and clearly communicate the purpose of the crate.
*   Organize examples in the same way as examples of other crates and ensure they are useful for developers.

### Progress
*   ✅ Initial Plan Created
*   ✅ Increment 1: Initial Build and Test Check
*   ✅ Increment 3: Fix Warnings and Test Failures (Trailing Delimiter Bug Fixed)
*   ❌ Increment 2: Enable All Tests (Needs Revisit - `strs_tools` bug isolated)
*   ✅ Increment 4: Review and Refine Readme
*   ✅ Increment 5: Organize and Improve Examples
*   ⏳ Increment 6: Debug and Fix `strs_tools` Escaped Quotes Bug

### Target Crate
*   `module/move/unilang_instruction_parser`

### Relevant Context
*   Files to Include:
    *   `module/move/unilang_instruction_parser/Cargo.toml`
    *   `module/move/unilang_instruction_parser/Readme.md`
    *   `module/move/unilang_instruction_parser/examples/unilang_instruction_parser_basic.rs`
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
    *   `module/core/strs_tools/src/string/split.rs` (for direct modification in Increment 6)
*   Crates for Documentation:
    *   `module/move/unilang_instruction_parser`
    *   `module/core/former` (for example organization reference)

### Expected Behavior Rules / Specifications (for Target Crate)
*   All `cargo test` commands for the target crate must pass.
*   `cargo clippy` for the target crate must report no warnings.
*   `Readme.md` should be concise, clear, and explain the crate's purpose and basic usage.
*   Examples should be well-structured, useful, and follow the pattern of `module/core/former/examples`.

### Target File Structure (If Applicable, within Target Crate)
*   `module/move/unilang_instruction_parser/examples/unilang_instruction_parser_trivial.rs` (rename if needed)
*   `module/move/unilang_instruction_parser/Readme.md` (modified)

### Increments

*   ✅ Increment 1: Initial Build and Test Check
    *   Detailed Plan Step 1: Run `cargo test -p unilang_instruction_parser` to identify failing tests.
    *   Detailed Plan Step 2: Run `cargo clippy -p unilang_instruction_parser -- -D warnings` to identify warnings.
    *   Pre-Analysis: Assessed current test and warning status. Encountered persistent failure in `empty_instruction_segment_trailing_semicolon` test.
    *   Crucial Design Rules: None specific.
    *   Relevant Behavior Rules: All `cargo test` commands for the target crate must pass; `cargo clippy` for the target crate must report no warnings.
    *   Verification Strategy: Analyze `execute_command` output for test failures and warnings.
    *   Commit Message: "chore(unilang_instruction_parser): Initial build and test check"

*   ✅ Increment 3: Fix Warnings and Test Failures (Trailing Delimiter Bug Fixed)
    *   Detailed Plan Step 1: Temporarily simplify `analyze_items_to_instructions` in `src/parser_engine.rs` to *only* check for the trailing `;;` condition and return `ErrorKind::TrailingDelimiter` if met, otherwise `Ok(Vec::new())`.
    *   Detailed Plan Step 2: Run `cargo test -p unilang_instruction_parser --test tests -- empty_instruction_segment_trailing_semicolon_debug -- --nocapture` to verify the simplified logic.
    *   Pre-Analysis: Previous attempts to fix the trailing delimiter bug have failed. This step aimed to isolate the problem by removing all other parsing logic.
    *   Crucial Design Rules: None specific.
    *   Relevant Behavior Rules: The `empty_instruction_segment_trailing_semicolon_debug` test should pass.
    *   Verification Strategy: Analyze `execute_command` output.
    *   Commit Message: "fix(unilang_instruction_parser): Debugging trailing semicolon error with simplified parser"

*   ❌ Increment 2: Enable All Tests (Needs Revisit - `strs_tools` bug isolated)
    *   Detailed Plan Step 1: Read `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs`, `module/move/unilang_instruction_parser/tests/comprehensive_tests.rs`, `module/move/unilang_instruction_parser/tests/error_reporting_tests.rs` to identify any disabled tests.
    *   Detailed Plan Step 2: For tests ignored due to external dependencies (e.g., `strs_tools`), create/update a `task.md` proposal in the external crate's root directory. (This step was previously done, but now the strategy is to fix directly).
    *   Detailed Plan Step 3: For tests ignored for other reasons, un-ignore them and fix any resulting failures.
    *   Pre-Analysis: Identified ignored tests in `argument_parsing_tests.rs` and `error_reporting_tests.rs` due to `strs_tools` bug. User feedback requires direct fix.
    *   Crucial Design Rules: Testing: Avoid Writing Automated Tests Unless Asked (ensuring existing tests are enabled, not adding new ones unless specified).
    *   Relevant Behavior Rules: All tests are enabled and passing.
    *   Verification Strategy: Run `cargo test -p unilang_instruction_parser --all-targets` and analyze output.
    *   Commit Message: "fix(unilang_instruction_parser): Propose strs_tools fix to enable all tests" (This commit message will be updated for the new Increment 6)

*   ✅ Increment 4: Review and Refine Readme
    *   Detailed Plan Step 1: Read `module/move/unilang_instruction_parser/Readme.md`.
    *   Detailed Plan Step 2: Draft a concise and clear Readme content that communicates the crate's purpose.
    *   Detailed Plan Step 3: Use `write_to_file` to update `Readme.md`.
    *   Pre-Analysis: Assess current Readme content for clarity and conciseness.
    *   Crucial Design Rules: Comments and Documentation (focus on rationale, conciseness).
    *   Relevant Behavior Rules: `Readme.md` should be concise, clear, and explain the crate's purpose and basic usage.
    *   Verification Strategy: Confirm `write_to_file` success.
    *   Commit Message: "docs(unilang_instruction_parser): Refine Readme for clarity and conciseness"

*   ✅ Increment 5: Organize and Improve Examples
    *   Detailed Plan Step 1: Read `module/move/unilang_instruction_parser/examples/unilang_instruction_parser_trivial_sample.rs`.
    *   Detailed Plan Step 2: Review `module/core/former/examples/` for organization patterns.
    *   Detailed Plan Step 3: Rename `unilang_instruction_parser_trivial_sample.rs` to `unilang_instruction_parser_basic.rs` and simplify its content.
    *   Detailed Plan Step 4: Ensure examples are useful and well-documented.
    *   Pre-Analysis: Assessed current example quality and organization.
    *   Crucial Design Rules: Comments and Documentation, Enhancements: Only Implement What’s Requested (focus on improving existing examples, not adding new features).
    *   Relevant Behavior Rules: Examples should be well-structured, useful, and follow the pattern of `module/core/former/examples`.
    *   Verification Strategy: Run `cargo build -p module/move/unilang_instruction_parser --examples` and analyze output. Confirm file structure changes.
    *   Commit Message: "docs(unilang_instruction_parser): Organize and improve examples"

*   ⏳ Increment 6: Debug and Fix `strs_tools` Escaped Quotes Bug
    *   Detailed Plan Step 1: Revert `strs_tools` changes in `module/core/strs_tools/src/string/split.rs` to re-introduce the `break` statement.
    *   Detailed Plan Step 2: Re-add `#[ignore]` attributes to the 4 tests in `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs` and `module/move/unilang_instruction_parser/tests/error_reporting_tests.rs`.
    *   Detailed Plan Step 3: Run `cargo test -p unilang_instruction_parser --all-targets` to confirm no hangs and all *other* tests pass.
    *   Detailed Plan Step 4: Debug `strs_tools::string::split::SplitIterator::handle_quoted_section` to correctly handle escaped quotes without hanging. This may involve adding debug prints or simplifying test cases.
    *   Detailed Plan Step 5: Apply the fix to `module/core/strs_tools/src/string/split.rs`.
    *   Detailed Plan Step 6: Remove `#[ignore]` attributes from the 4 tests in `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs` and `module/move/unilang_instruction_parser/tests/error_reporting_tests.rs`.
    *   Detailed Plan Step 7: Run `cargo test -p unilang_instruction_parser --all-targets` to verify all tests pass.
    *   Pre-Analysis: The previous attempt to fix `strs_tools` resulted in a hang. This increment focuses on isolating and correctly fixing that bug.
    *   Crucial Design Rules: Proc Macro: Development Workflow (applying debugging principles), Testing: Plan with a Test Matrix When Writing Tests (if new tests are needed for `strs_tools`).
    *   Relevant Behavior Rules: All tests are enabled and passing.
    *   Verification Strategy: Analyze `execute_command` output for test results and hangs.
    *   Commit Message: "fix(strs_tools): Debug and fix escaped quotes tokenization bug"

### Task Requirements
*   Fix all tests and warnings.
*   All tests must be enabled.
*   All tests must be according to specification.
*   Readme must be concise and clearly communicate purpose.
*   Examples must be organized like other crates' examples.
*   Examples must be useful for developers.

### Project Requirements
*   (No project-wide requirements identified yet)
*   **New Global Constraint:** Never use `#[allow(clippy::missing_errors_doc)]`.

### Notes & Insights
*   The `task.md` file exists in the target crate, which might contain additional context or previous tasks. I will ignore it for now as the current task is clearly defined.
