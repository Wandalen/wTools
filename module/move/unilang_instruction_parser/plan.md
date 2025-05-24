# Project Plan: `unilang_instruction_parser` (Revised V5 - Ownership Change)

### Goal
*   Implement a robust, non-panicking parser in `unilang_instruction_parser` for `unilang` CLI syntax, strictly adhering to `unilang/spec.md`.
*   Utilize `strs_tools::string::split` for lexical analysis/itemization.
*   Produce `Vec<GenericInstruction>` (using owned `String`s for arguments) from `&str` or `&[&str]` input.
*   Provide precise, AST-node-level, location-aware error reporting using `SourceLocation`.
*   Ensure all tests pass and are not ignored, where feasible within `unilang_instruction_parser`.

### Progress
*   Overall Task for unilang_instruction_parser: ❌ **CRITICAL ISSUE: Segmentation Fault during Clippy Analysis**
*   Milestones Achieved:
    *   ✅ Increment 1: Core types adapted to `strs_tools::string::split` and `no_std` feature added.
    *   ✅ Increment 2: Parser entry points and `RichItem` stream generation implemented.
    *   ✅ Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries implemented.
    *   ✅ Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing implemented.
    *   ✅ Increment 5: Syntactic Analyzer - Argument Parsing (Named & Positional) for Single-Segment Paths.
    *   ✅ Increment 5.1 (New - Stuck Resolution Strategy): Implement Multi-Segment Path Parsing.
    *   ✅ Increment 6: Error Reporting Integration and Refinement.
    *   ✅ Increment 7: Comprehensive Test Suite (Test Matrix) implemented with initial set of tests.
    *   ✅ Increment 8: Documentation and Examples
    *   ✅ Increment 9: Address Test Failures (Workaround, Parser Fix, and External Bug Report)
    *   ✅ Increment 10: Refine Parser Behavior for Comments and Align Config Entry Tests
    *   ✅ Increment 11: Investigate and Resolve Segmentation Fault in `argument_parsing_tests.rs` (Segfault no longer occurring with current test run; ignored tests confirmed)
    *   ✅ Increment 12: Align and Verify Test Matrix CT2.1
*   Next Increments:
    *   ❌ **Increment 13: Investigate and Resolve Segmentation Fault during Clippy Analysis**
    *   ⚫ Increment 13.1: (Follow-up) Address Clippy Lints in `unilang_instruction_parser` Source Code (after segfault resolved)
    *   ⚫ Increment 13.2: (Follow-up) Investigate `unreachable_pattern` warnings in `error_reporting_tests.rs` (after lints resolved)
    *   ⚫ Increment 14: Final Verification and Comprehensive Test Run

### Target Crate
*   module/move/unilang_instruction_parser

### Relevant Context
*   Files to Include (for AI's reference, if `read_file` is planned, primarily from Target Crate):
    *   `module/move/unilang_instruction_parser/src/lib.rs`
    *   `module/move/unilang_instruction_parser/src/instruction.rs`
    *   `module/move/unilang_instruction_parser/src/item_adapter.rs`
    *   `module/move/unilang_instruction_parser/src/parser_engine.rs`
    *   `module/move/unilang_instruction_parser/src/config.rs`
    *   `module/move/unilang_instruction_parser/src/error.rs`
    *   `module/move/unilang_instruction_parser/Readme.md`
    *   `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/parser_config_entry_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/syntactic_analyzer_command_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/comprehensive_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/error_reporting_tests.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   `module/core/strs_tools` (Reason: Clippy lint violations, Unescaping/tokenization bug)

### Expected Behavior Rules / Specifications (for Target Crate)
*   (As previously defined, referencing `unilang/spec.md`)
*   Path parsing: Greedy consumption of `Identifier` and `UnquotedValue` tokens until a non-path-like token or a named argument (`name::value`) is encountered. Handles empty path for initial "name::val" and respects slice segment boundaries.
*   Argument parsing: Handles positional, named (`name::value`), and quoted arguments. Supports options for duplicate named args and positional args after named.
*   Help operator `?`: Parsed if it's the last token after the command path.
*   Instruction separator `;;`: Splits input into multiple `GenericInstruction`s. Each string in a slice input `&[&str]` also forms a new instruction context unless joined by `;;`.
*   Error reporting: Provides `ErrorKind` and `SourceLocation` for syntax violations.
*   Unescaping: Standard escapes (`\\`, `\"`, `\'`, `\n`, `\t`) are handled within quoted values. Invalid escapes (e.g., `\x`) result in a `ParseError`.
*   Comments: Lines/segments starting with `#` should be ignored and produce no instructions.

### Target File Structure (If Applicable, within Target Crate)
*   `module/move/unilang_instruction_parser/examples/basic_usage.rs` (Created)
*   `module/move/unilang_instruction_parser/Readme.md` (Created)

### Increments

#### Phase 1: Setup and Core Structures
*   ✅ **Increment 1: Adapt to `strs_tools::string::split` & Define Core Structures**
    *   Commit Message: `refactor(unilang_parser): Adapt core types to strs_tools::string::split API and add RichItem`

#### Phase 2: Parsing Engine Implementation
*   ✅ **Increment 2: Implement Parser Entry Points and `RichItem` Stream Generation**
    *   Commit Message: `feat(unilang_parser): Implement parser entry points and RichItem stream generation using string::split`
*   ✅ **Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries**
    *   Commit Message: `feat(unilang_parser): Implement instruction grouping by ';;' delimiter in analyze_items_to_instructions`
*   ✅ **Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing**
    *   Commit Message: `feat(unilang_parser): Implement command path and help operator parsing`
*   ✅ **Increment 5: Syntactic Analyzer - Argument Parsing (Named & Positional) for Single-Segment Paths**
    *   Commit Message: `feat(unilang_parser): Implement named and positional argument parsing for single-segment paths`
*   ✅ **Increment 5.1 (New - Stuck Resolution Strategy): Implement Multi-Segment Path Parsing**
    *   Commit Message: `feat(unilang_parser): Implement multi-segment command path parsing`
*   ✅ **Increment 6: Error Reporting Integration and Refinement**
    *   Commit Message: `feat(unilang_parser): Enhance error reporting with precise locations and new test cases`
*   ✅ **Increment 7: Comprehensive Test Suite (Test Matrix)**
    *   Commit Message: `test(unilang_parser): Add initial comprehensive test suite based on Test Matrix`
*   ✅ **Increment 8: Documentation and Examples**
    *   Commit Message: `docs(unilang_parser): Add crate and API documentation, Readme, and basic usage example`
*   ✅ **Increment 9: Address Test Failures (Workaround, Parser Fix, and External Bug Report)**
    *   Commit Message: `fix(unilang_parser): Correct path parsing logic and test assertions, ignore remaining known failures`
*   ✅ **Increment 10: Refine Parser Behavior for Comments and Align Config Entry Tests**
    *   Commit Message: `fix(unilang_parser): Improve comment handling, align config entry tests`
    *   **Test Matrix (Accumulated - more rows can be added in future tasks):**

        | ID    | Input Type | Path Complexity | Help Op | Arguments                                  | Quoting        | Escapes      | Separator | Options                               | Expected Outcome (Simplified)                               |
        |-------|------------|-----------------|---------|--------------------------------------------|----------------|--------------|-----------|---------------------------------------|-------------------------------------------------------------|
        | CT1.1 | single_str | single          | absent  | val (unquoted)                             | none           | none         | none      | default                               | Path: `cmd val` (greedy)                                    |
        | CT1.2 | single_str | multi           | absent  | name1::val1 (unquoted)                     | none           | none         | none      | default                               | Path: `p1 p2`, Named: `n1:v1`                               |
        | CT1.3 | single_str | single          | present | none                                       | none           | none         | none      | default                               | Path: `cmd`, Help: true                                     |
        | CT1.4 | single_str | single          | absent  | pos1 ("quoted val")                        | double         | none         | none      | default                               | Path: `cmd`, Pos: `quoted val`                              |
        | CT1.5 | single_str | single          | absent  | name1::"esc\\nval"                         | double         | std          | none      | default                               | Path: `cmd`, Named: `n1:esc\nval`                           |
        | CT1.6 | single_str | single          | absent  | name1::"bad\\xval"                         | double         | invalid      | none      | default                               | Error: Invalid escape                                       |
        | CT2.1 | slice      | multi           | absent  | pos1, name1::val1                          | mixed          | none         | none      | allow_pos_after_named=false           | 3 Instr: 1(Path: `p1 p2`), 2(Path: `pos1`), 3(Named: `n1:v1`)|
        | CT3.1 | single_str | single          | absent  | arg1 (path); name::val (arg)               | none           | none         | `;;`      | default                               | Instr1: Path `cmd1 arg1`; Instr2: Path `cmd2`, Named `name:val`|
        | CT4.1 | single_str | single          | absent  | name::val1, name::val2                     | none           | none         | none      | error_on_duplicate=true               | Error: Duplicate named                                      |
        | CT4.2 | single_str | single          | absent  | name::val1, name::val2                     | none           | none         | none      | error_on_duplicate=false              | Path: `cmd`, Named: `name:val2` (last wins)                 |
        | CT5.1 | single_str | no path         | absent  | name::val                                  | none           | none         | none      | default                               | Path: `[]`, Named: `name:val`                               |

#### Phase 3: Finalization and Verification
*   ✅ **Increment 11: Investigate and Resolve Segmentation Fault in `argument_parsing_tests.rs`**
    *   Detailed Plan Step 1: Read `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs` to get a list of all test function names. (Done)
    *   Detailed Plan Step 2: For each test function in `argument_parsing_tests.rs` (starting from the top, ensuring ignored tests are temporarily un-ignored for this step): Execute `cargo test -p unilang_instruction_parser --test argument_parsing_tests -- <test_name_exact_match> -- --nocapture` via `execute_command`. Analyze `execute_command` output. If a segfault occurs, this test is the trigger (or one of them). Note the test name. If no segfault, re-ignore the test if it was one of the 4 known unescaping-related tests. (Done - no segfault with individual runs, ignored tests handled)
    *   Detailed Plan Step 3: If a specific test `[CRASHING_TEST_NAME]` is identified: (Skipped - no single test caused segfault)
    *   Detailed Plan Step 4: If no single test triggers it, plan to test in batches. (Revised - ran full suite with --nocapture, no segfault)
    *   Pre-Analysis: A segmentation fault occurred when running the full `argument_parsing_tests.rs` suite. The 4 unescaping tests were re-ignored prior to this.
    *   Crucial Design Rules: N/A (focus on critical bug fixing)
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: Execute `cargo test -p unilang_instruction_parser --test argument_parsing_tests -- --show-output --nocapture` via `execute_command`. Analyze output. (Done - passed, 4 ignored, no segfault)
    *   Commit Message: `test(unilang_parser): Verify argument_parsing_tests stability, confirm ignored tests`

*   ✅ **Increment 12: Align and Verify Test Matrix CT2.1** (Depends on Increment 11)
    *   Detailed Plan Step 1: Review Test Matrix row CT2.1: `Input: slice | Path: multi | Help: absent | Args: pos1, name1::val1 | Quoting: mixed | Escapes: none | Separator: none | Options: allow_pos_after_named=false | Expected: 3 Instr: 1(Path: p1 p2), 2(Path: pos1), 3(Named: n1:v1)`. (Done)
    *   Detailed Plan Step 2: Locate the test function covering CT2.1 (likely in `comprehensive_tests.rs`, e.g., `ct2_1_slice_multi_path_mixed_args`). If it doesn't exist, create it. (Done, test `ct2_1_slice_multi_path_mixed_args` exists)
    *   Detailed Plan Step 3: Ensure the test implementation accurately reflects the CT2.1 specification, especially the input slice structure and expected separate instructions. (Done, implementation matches)
    *   Detailed Plan Step 4: Execute `cargo test -p unilang_instruction_parser --test comprehensive_tests -- ct2_1_slice_multi_path_mixed_args --show-output` (or the correct test name) via `execute_command`. (Done, test passed)
    *   Detailed Plan Step 5: If the test fails, apply Critical Log Analysis to the `execute_command` output. Implement necessary fixes in the parser logic (e.g., `parser_engine.rs`) or the test itself to ensure alignment with CT2.1. (Skipped, test passed)
    *   Pre-Analysis: The plan mentioned "Aligning Test Matrix CT2.1" as a current focus. This increment ensures it's explicitly handled. The `parser_engine.rs` was previously updated to treat `segment_idx` changes as instruction boundaries, which fixed `ct2_1_slice_multi_path_mixed_args`. This increment will re-verify this.
    *   Crucial Design Rules: [Testing: Plan with a Test Matrix When Writing Tests]
    *   Relevant Behavior Rules: [Instruction separator], [Argument parsing]
    *   Verification Strategy: `cargo test -p unilang_instruction_parser --test comprehensive_tests -- ct2_1_slice_multi_path_mixed_args --show-output` (or the correct test name) passes, based on `execute_command` output. (Done, passed)
    *   Commit Message: `test(unilang_parser): Align and verify Test Matrix CT2.1 (slice input behavior)`

*   ⏳ **Increment 13: Investigate and Resolve Segmentation Fault during Clippy Analysis**
    *   Pre-Analysis: A segmentation fault occurred during `cargo clippy` analysis of `unilang_instruction_parser`. This increment will investigate and resolve it.
    *   Detailed Plan Step 1: Revert the last change made to `module/move/unilang_instruction_parser/src/item_adapter.rs` (collapsing `if` statements).
    *   Detailed Plan Step 2: Re-run `cargo clippy --package unilang_instruction_parser --tests --no-deps -- -A clippy::uninlined_format_args -D warnings` via `execute_command` to check if the segfault persists.
    *   Detailed Plan Step 3: If segfault persists, proceed with isolating the problematic code (minimal reproducible example, binary search within files).
    *   Crucial Design Rules: N/A (focus on critical bug fixing)
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: `cargo clippy` runs without segfault.
    *   Commit Message: Will depend on the fix. E.g., `fix(unilang_parser): Resolve segfault during clippy analysis`

*   ⚫ **Increment 13.1: (Follow-up) Address Clippy Lints in `unilang_instruction_parser` Source Code (after segfault resolved)**
    *   Pre-Analysis: After segfault is resolved, address all remaining clippy lints in `unilang_instruction_parser` source files.
    *   Detailed Plan Step 1: Execute `cargo clippy --package unilang_instruction_parser --tests --no-deps -- -A clippy::uninlined_format_args -D warnings` via `execute_command` to get a fresh list of lints.
    *   Detailed Plan Step 2: Systematically go through each reported clippy lint in `unilang_instruction_parser/src/` and apply fixes.
    *   Detailed Plan Step 3: Use `write_to_file` for each file modification.
    *   Detailed Plan Step 4: Re-run `cargo clippy` after each logical group of fixes.
    *   Crucial Design Rules: Adhere to Codestyle Rules when fixing lints.
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: `cargo clippy` (as above) runs with no warnings/errors for `unilang_instruction_parser`. `cargo test -p unilang_instruction_parser --all-targets -- --show-output --skip ...` passes.
    *   Commit Message: `style(unilang_parser): Address clippy lints in library source code`

*   ⚫ **Increment 13.2: (Follow-up) Investigate `unreachable_pattern` warnings in `error_reporting_tests.rs` (after lints resolved)**
    *   Pre-Analysis: After library lints are fixed, check if `unreachable_pattern` warnings persist in `error_reporting_tests.rs`.
    *   Detailed Plan Step 1: Execute `cargo clippy --package unilang_instruction_parser --tests --no-deps -- -A clippy::uninlined_format_args -D warnings` via `execute_command`.
    *   Detailed Plan Step 2: If `unreachable_pattern` warnings are still present in `tests/error_reporting_tests.rs`:
        *   Read `tests/error_reporting_tests.rs`.
        *   Analyze and refactor the specific match statements or test logic to eliminate the warnings.
    *   Verification Strategy: `cargo clippy` (as above) shows no `unreachable_pattern` warnings in `error_reporting_tests.rs`. `cargo test --test error_reporting_tests` passes.
    *   Commit Message: `fix(unilang_parser): Address unreachable_pattern warnings in error_reporting_tests`

*   ⚫ **Increment 14: Final Verification and Comprehensive Test Run** (Depends on Increment 13, 13.1, 13.2)
    *   Detailed Plan Step 1: Execute `cargo test -p unilang_instruction_parser --all-targets -- --show-output --skip test_unescape_internal_quotes_truncated_segment --skip test_unescape_internal_quotes_multiple_escapes --skip test_unescape_internal_quotes_mixed_escaped_and_normal --skip test_unescape_internal_quotes_at_boundaries` (or similar, to skip tests that were confirmed to be re-ignored due to the external `strs_tools` bug in Increment 11) via `execute_command`.
    *   Detailed Plan Step 2: Analyze the `execute_command` output from Step 1. Ensure all other tests pass.
    *   Detailed Plan Step 3: Execute `cargo clippy --package unilang_instruction_parser --all-targets --all-features --no-deps -- -A clippy::uninlined_format_args -D warnings` via `execute_command`.
    *   Detailed Plan Step 4: Analyze the `execute_command` output from Step 3. Ensure no new clippy warnings or errors are present.
    *   Detailed Plan Step 5: Execute `git status` via `execute_command`.
    *   Detailed Plan Step 6: Analyze the `execute_command` output from Step 5. Ensure the working directory is clean (no uncommitted changes).
    *   Pre-Analysis: This is the final check before task completion.
    *   Crucial Design Rules: N/A
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: All `execute_command` calls complete successfully, and their outputs indicate all tests (excluding explicitly re-ignored ones) pass, no new clippy issues, and a clean git status.
    *   Commit Message: `chore(unilang_parser): Complete final verification and test suite execution`

### Task Requirements
*   (As before)

### Project Requirements
*   (As before)

### Notes & Insights
*   **Ownership Change:** Complete.
*   **Unescaping Limitation:** The 4 failing tests in `argument_parsing_tests.rs` are due to `strs_tools::string::split` truncating segments with internal escaped quotes. These are confirmed `#[ignore]` with `// aaa:` comments. A `task.md` in `strs_tools` addresses this.
*   **`parser_config_entry_tests.rs` Issues:** All tests in this suite now pass after parser enhancements for comment handling and test expectation alignment for simple commands and unterminated quotes.
*   **Error Location for `StrSpan` Escapes:** (No change to this note)
*   **Clippy Lints in `strs_tools`:** A `task.md` in `strs_tools` addresses clippy lints.
*   **Test Warnings in `unilang_instruction_parser`:**
    *   `missing_docs` for `tests/tests.rs` was fixed.
    *   `unused_imports` in `tests/comprehensive_tests.rs` were fixed.
    *   Multiple `unreachable_pattern` warnings in `tests/error_reporting_tests.rs` persist. Increment 13.2 aims to address these after library lints.
*   **Parser Bug with `parse_slice` State:** The `analyze_items_to_instructions` function was updated to treat `segment_idx` changes as instruction boundaries. This fixed `parse_slice_simple_command_placeholder` and `ct2_1_slice_multi_path_mixed_args`. The original note about `error_on_positional_after_named` state carrying over might still be relevant if more complex slice interactions are tested, but the primary boundary issue is resolved.
*   **Segmentation Fault:** A previous attempt to run `cargo clippy` on `unilang_instruction_parser` resulted in a segfault. This is now the focus of Increment 13.
