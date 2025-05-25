# Project Plan: Enhance SplitIterator for Quoted Sections in `strs_tools`

### Goal
*   Modify `strs_tools::string::split::SplitIterator` to correctly tokenize strings containing quoted sections, ensuring that internal delimiters within a quoted section are *not* treated as delimiters. The entire content of a quoted section (excluding outer quotes, but including escaped inner quotes and delimiters) should be returned as a single `Delimeted` item.

### Progress
*   ✅ Increment 1: Stabilize current quoting logic & address warnings (Stuck Resolution)
*   ✅ Increment 1.5: Fix empty segment generation with `preserving_empty` and quoting
*   ⏳ Increment 2: Verify integration with `unilang_instruction_parser` (In Progress)

### Target Crate
*   `module/core/strs_tools`

### Relevant Context
*   Files to Include (for AI's reference, primarily from Target Crate):
    *   `module/core/strs_tools/src/string/split.rs`
    *   `module/core/strs_tools/tests/debug_hang_split_issue.rs`
    *   `module/core/strs_tools/tests/inc/split_test/quoting_options_tests.rs`
    *   `module/core/strs_tools/tests/inc/split_test/combined_options_tests.rs`
    *   `module/move/unilang_instruction_parser/plan.md` (for context on the requesting crate)
    *   `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs` (for failing test context)
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`

### Expected Behavior Rules / Specifications (for Target Crate)
*   Rule 1: Given input `cmd arg::"value with spaces and :: delimiters"`, `SplitIterator` should produce:
    *   `Split { string: "cmd", typ: Delimeted, ... }`
    *   `Split { string: " ", typ: Delimiter, ... }`
    *   `Split { string: "arg", typ: Delimeted, ... }`
    *   `Split { string: "::", typ: Delimiter, ... }`
    *   `Split { string: "value with spaces and :: delimiters", typ: Delimeted, ... }` (single item, outer quotes stripped).
*   Rule 2: When an opening quote is encountered, `SplitIterator` should switch its internal `SplitFastIterator` to a mode where only the matching closing quote (and potentially escaped characters) are considered delimiters.
*   Rule 3: Once the closing quote is found, `SplitIterator` should switch `SplitFastIterator` back to the original set of delimiters.

### Target File Structure (If Applicable, within Target Crate)
*   No major file structure changes are planned.

### Increments

*   ✅ Increment 1: Stabilize current quoting logic & address warnings (Stuck Resolution)
    *   Detailed Plan Step 1: (Done) Implemented dynamic delimiter adjustment logic in `SplitIterator` and `SplitFastIterator` in `module/core/strs_tools/src/string/split.rs`.
    *   Detailed Plan Step 2: (Done) Added new unit tests to `module/core/strs_tools/tests/inc/split_test/quoting_options_tests.rs`.
    *   Detailed Plan Step 3: (Done) Temporarily commented out the 3 failing tests.
    *   Detailed Plan Step 4: (Done) Fix compiler warnings in `module/core/strs_tools/src/string/split.rs`.
    *   Pre-Analysis: The core quoting logic for many cases might be correct. Isolating the problematic tests will help confirm this.
    *   Crucial Design Rules: [Comments and Documentation]
    *   Relevant Behavior Rules: Rule 1, Rule 2, Rule 3 (for non-failing cases).
    *   Verification Strategy:
        *   Execute `cargo test -p strs_tools` via `execute_command`. Analyze output (expecting all *uncommented* tests to pass).
        *   Execute `cargo clippy -p strs_tools -- -D warnings` via `execute_command`. Analyze output (expecting no warnings from `split.rs`).
    *   Test Matrix: (Already developed and partially implemented)
    *   Commit Message: `refactor(strs_tools): Stabilize quote handling, address warnings, temp. ignore 3 tests`

*   ✅ Increment 1.5: Fix empty segment generation with `preserving_empty` and quoting
    *   Detailed Plan Step 1: (Done) Analyzed `SplitIterator::next()` and `SplitFastIterator::next()` interaction.
    *   Detailed Plan Step 2: (Done) Refined `SplitIterator::next()` with `last_yielded_token_was_delimiter` state and preemptive empty segment logic.
    *   Detailed Plan Step 3: (Done) Uncommented `inc::split_test::combined_options_tests::test_m_t3_13_quoting_preserve_all_strip`.
    *   Detailed Plan Step 4: (Done) Added and removed temporary `println!` statements.
    *   Detailed Plan Step 5: (Done) Tested `test_m_t3_13_quoting_preserve_all_strip` - PASSED.
    *   Detailed Plan Step 6: (Done) Logic refined.
    *   Detailed Plan Step 7: (Done) Uncommented `inc::split_test::quoting_options_tests::test_m_t3_11_quoting_preserve_all_no_strip`. Tested - PASSED.
    *   Detailed Plan Step 8: (Done) Uncommented `inc::split_test::quoting_options_tests::test_m_t3_13_quoting_preserve_all_strip`. Tested - PASSED.
    *   Detailed Plan Step 9: (Done) Removed all temporary `println!` statements from `split.rs`.
    *   Pre-Analysis: The critical part is the order of operations in `SplitIterator::next()`: let SFI yield, then SI analyzes that yield and the *remaining* SFI iterable for quotes.
    *   Crucial Design Rules: [Testing: Plan with a Test Matrix When Writing Tests]
    *   Relevant Behavior Rules: Correct production of empty segments when `preserving_empty(true)` even with adjacent quotes.
    *   Verification Strategy:
        *   Execute `cargo test -p strs_tools` via `execute_command`. All tests (including the 3 re-enabled ones) should pass.
        *   Execute `cargo clippy -p strs_tools -- -D warnings` via `execute_command`.
    *   Commit Message: `fix(strs_tools): Correct empty segment handling with quoting and preserving_empty`

*   ⏳ Increment 2: Verify integration with `unilang_instruction_parser`
    *   Detailed Plan Step 1: Execute `cargo test -p unilang_instruction_parser --all-targets` via `execute_command`.
    *   Detailed Plan Step 2: Analyze the output of the `execute_command`. If all tests pass, the integration is successful. If `unilang_instruction_parser` tests fail due to `strs_tools` changes, revise plan to fix `strs_tools`.
    *   Pre-Analysis: This increment assumes Increment 1.5 was successful and all `strs_tools` tests pass.
    *   Crucial Design Rules: N/A (Verification only).
    *   Relevant Behavior Rules: Acceptance criteria from `module/core/strs_tools/-task.md` (i.e., `unilang_instruction_parser` tests related to argument parsing should pass).
    *   Verification Strategy: The `execute_command` in Step 1 and analysis in Step 2 is the verification.
    *   Commit Message: `chore(strs_tools): Verify quoted split integration with unilang_instruction_parser`

### Task Requirements
*   All changes must be within `module/core/strs_tools`.
*   The solution should follow "Option 1 (Preferred): Modify `SplitIterator` to dynamically adjust `SplitFastIterator`'s delimiters." from the task description.
*   The `debug_hang_split_issue` test in `strs_tools` must pass.
*   All tests in `module/move/unilang_instruction_parser` (especially those related to quoted arguments) must pass after this change is implemented in `strs_tools`.

### Project Requirements
*   Must use Rust 2021 edition.
*   All new APIs must be async (not applicable for this task).
*   All dependencies must be centralized in workspace `Cargo.toml`.
*   Lints must be defined in workspace `Cargo.toml` and inherited by crates.

### Notes & Insights
*   The `last_yielded_token_was_delimiter` state in `SplitIterator` was key to correctly inserting empty segments before a quote that followed a delimiter when `preserving_empty` is true.