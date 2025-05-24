# Project Plan: Fix Clippy Warnings and Unescaping in `strs_tools`

### Goal
*   Address all clippy warnings in `module/core/strs_tools` to ensure clean compilation with `-D warnings` enabled.
*   Fix the `SplitType::Delimeter` typo in `src/string/split.rs`.
*   Investigate and resolve string unescaping issues in `strs_tools` that cause failures in `unilang_instruction_parser` tests.

### Progress
*   ✅ Increment 1: Fix Clippy Warnings and Typo
*   ⚫ Increment 2: Investigate and Fix String Unescaping Issues

### Target Crate
*   `module/core/strs_tools`

### Relevant Context
*   Files to Include (for AI's reference, primarily from Target Crate):
    *   `module/core/strs_tools/src/string/split.rs`
    *   `module/core/strs_tools/src/string/isolate.rs`
    *   `module/core/strs_tools/src/string/mod.rs`
    *   `module/core/strs_tools/Cargo.toml`
    *   `module/move/unilang_instruction_parser/plan.md` (for context on the requesting crate)
    *   `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs` (for failing test context)
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`
    *   `unilang_instruction_parser`

### Expected Behavior Rules / Specifications (for Target Crate)
*   `cargo clippy -p strs_tools -- -D warnings` should exit with code 0 and report no warnings.
*   The functionality of `strs_tools` (especially string splitting and isolation) should remain unchanged, except for the typo fix.
*   String unescaping in `strs_tools` should correctly handle escape sequences, allowing `unilang_instruction_parser`'s tests related to unescaping to pass.

### Target File Structure (If Applicable, within Target Crate)
*   No major file structure changes are planned, only modifications to existing files.

### Increments

*   ✅ Increment 1: Fix Clippy Warnings and Typo
    *   Detailed Plan Step 1: Read `module/core/strs_tools/src/string/split.rs`.
    *   Detailed Plan Step 2: Identify and refactor `if/else` structures to remove redundant `else` blocks.
    *   Detailed Plan Step 3: Identify and collapse nested `if` statements into single `if` conditions.
    *   Detailed Plan Step 4: Identify and remove explicit `return` keywords where the expression is implicitly returned.
    *   Detailed Plan Step 5: Add `#[panics]` sections to documentation for functions that may panic (e.g., `SplitOptions::form` due to `unwrap()`).
    *   Detailed Plan Step 6: Change `SplitType::Delimeter` to `SplitType::Delimeted` in `src/string/split.rs`.
    *   Pre-Analysis: The `task.md` provides clear guidance on the types of clippy warnings and the typo.
    *   Crucial Design Rules: [Code Style: Do Not Reformat Arbitrarily], [Comments and Documentation], [Handling Panics vs Recoverable Errors]
    *   Relevant Behavior Rules: `cargo clippy -p strs_tools -- -D warnings` should exit with code 0.
    *   Verification Strategy:
        *   Execute `cargo clippy -p module/core/strs_tools -- -D warnings` via `execute_command` and analyze output.
        *   Execute `cargo test -p module/core/strs_tools` via `execute_command` and analyze output.
    *   Commit Message: `fix(strs_tools): Address clippy warnings and typo in split.rs`

*   ⚫ Increment 2: Investigate and Fix String Unescaping Issues
    *   Detailed Plan Step 1: Read `module/core/strs_tools/src/string/isolate.rs` and `module/core/strs_tools/src/string/split.rs` to understand string splitting, quoting, and unescaping logic.
    *   Detailed Plan Step 2: Read `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs` to understand the context of failing unescaping tests.
    *   Detailed Plan Step 3: Identify the specific functions in `strs_tools` responsible for handling escape sequences and determine if they correctly preserve or pass through escape sequences for subsequent unescaping.
    *   Detailed Plan Step 4: Implement necessary changes in `strs_tools` to ensure correct handling of escape sequences during tokenization/splitting.
    *   Pre-Analysis: This increment requires deeper investigation into the interaction between `strs_tools` and `unilang_instruction_parser`'s unescaping logic.
    *   Crucial Design Rules: [Visibility: Keep Implementation Details Private], [Error Handling: Use a Centralized Approach]
    *   Relevant Behavior Rules: `unilang_instruction_parser`'s unescaping tests should pass.
    *   Verification Strategy:
        *   Execute `cargo test -p module/core/strs_tools` via `execute_command` and analyze output.
        *   Execute `cargo test -p module/move/unilang_instruction_parser` via `execute_command` and analyze output, specifically looking for the unescaping tests to pass.
    *   Commit Message: `fix(strs_tools): Resolve string unescaping issues for unilang_instruction_parser`

### Task Requirements
*   All changes must be within `module/core/strs_tools`.
*   Changes to `module/move/unilang_instruction_parser` are not permitted in this task.
*   All clippy warnings must be resolved.
*   The typo `Delimeter` -> `Delimeted` must be fixed.
*   String unescaping must work correctly.

### Project Requirements
*   Must use Rust 2021 edition.
*   All new APIs must be async (not applicable for this task).
*   All dependencies must be centralized in workspace `Cargo.toml`.
*   Lints must be defined in workspace `Cargo.toml` and inherited by crates.

### Notes & Insights
*   The `task.md` explicitly mentions `SplitType::Delimeter` typo at line 162 in `strs_tools/src/string/split.rs`.
*   The unescaping issue is described as "raw string provided to `unescape_string_with_errors` in `unilang_instruction_parser` is not as expected (e.g., backslashes are already consumed or misinterpreted)". This suggests the problem might be in how `strs_tools` processes the input string *before* `unilang_instruction_parser` attempts to unescape it.