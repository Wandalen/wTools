# Project Plan: `unilang_instruction_parser` (Revised V5 - Ownership Change)

### Goal
*   Implement a robust, non-panicking parser in `unilang_instruction_parser` for `unilang` CLI syntax, strictly adhering to `unilang/spec.md`.
*   Utilize `strs_tools::string::split` for lexical analysis/itemization.
*   Produce `Vec<GenericInstruction>` (using owned `String`s for arguments) from `&str` or `&[&str]` input.
*   Provide precise, AST-node-level, location-aware error reporting using `SourceLocation`.

### Progress
*   Overall Task for unilang_instruction_parser: ⚙️ Parsing Logic - 95% Complete
*   Milestones Achieved:
    *   ✅ Increment 1: Core types adapted to `strs_tools::string::split` and `no_std` feature added.
    *   ✅ Increment 2: Parser entry points and `RichItem` stream generation implemented.
    *   ✅ Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries implemented.
    *   ✅ Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing implemented.
    *   ✅ Increment 5: Syntactic Analyzer - Argument Parsing (Named & Positional) for Single-Segment Paths.
    *   ✅ Increment 5.1 (New - Stuck Resolution Strategy): Implement Multi-Segment Path Parsing.
    *   ✅ Increment 6: Error Reporting Integration and Refinement.
    *   ✅ Increment 7: Comprehensive Test Suite (Test Matrix) implemented with initial set of tests.
*   Currently Working On:
    *   ⚫🚀 Increment 8: Documentation and Examples (Up Next)

### Target Crate
*   module/move/unilang_instruction_parser

### Relevant Context
*   Files to Include (for AI's reference, if `read_file` is planned, primarily from Target Crate):
    *   `module/move/unilang_instruction_parser/src/instruction.rs`
    *   `module/move/unilang_instruction_parser/src/item_adapter.rs`
    *   `module/move/unilang_instruction_parser/src/parser_engine.rs`
    *   `module/move/unilang_instruction_parser/src/config.rs`
    *   `module/move/unilang_instruction_parser/src/error.rs`
    *   `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/error_reporting_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/comprehensive_tests.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   None

### Expected Behavior Rules / Specifications (for Target Crate)
*   (As previously defined, referencing `unilang/spec.md`)
*   Path parsing: Greedy consumption of `Identifier` and `UnquotedValue` tokens until a non-path-like token or a named argument (`name::value`) is encountered. Handles empty path for initial "name::val" and respects slice segment boundaries.
*   Argument parsing: Handles positional, named (`name::value`), and quoted arguments. Supports options for duplicate named args and positional args after named.
*   Help operator `?`: Parsed if it's the last token after the command path.
*   Instruction separator `;;`: Splits input into multiple `GenericInstruction`s.
*   Error reporting: Provides `ErrorKind` and `SourceLocation` for syntax violations.
*   Unescaping: Standard escapes (`\\`, `\"`, `\'`, `\n`, `\t`) are handled within quoted values. Invalid escapes (e.g., `\x`) result in a `ParseError`.

### Target File Structure (If Applicable, within Target Crate)
*   New test file: `module/move/unilang_instruction_parser/tests/comprehensive_tests.rs` (Created)

### Increments

#### Phase 1: Setup and Core Structures
*   ✅ **Increment 1: ...**
*   ...

#### Phase 2: Parsing Engine Implementation
*   ✅ **Increment 2: ...**
*   ...
*   ✅ **Increment 5.1: ...**
*   ✅ **Increment 6: ...**

*   ✅ **Increment 7: Comprehensive Test Suite (Test Matrix)**
    *   Target Component(s): `unilang_instruction_parser` (new test file `tests/comprehensive_tests.rs`).
    *   Pre-Analysis: Existing tests cover many specific cases. This increment aims to create a more systematic test suite.
    *   Detailed Plan Step 1: Defined initial Test Matrix factors. (Completed)
    *   Detailed Plan Step 2: Implemented initial set of test cases in `tests/comprehensive_tests.rs` covering CT1.1-CT1.6, CT2.1, CT3.1, CT4.1-CT4.2, CT5.1. (Completed)
    *   Detailed Plan Step 3: Test Matrix in plan file updated with initial rows. (Completed)
    *   **Test Matrix (Accumulated - more rows can be added in future tasks):**

        | ID    | Input Type | Path Complexity | Help Op | Arguments                                  | Quoting        | Escapes      | Separator | Options                               | Expected Outcome (Simplified)                               |
        |-------|------------|-----------------|---------|--------------------------------------------|----------------|--------------|-----------|---------------------------------------|-------------------------------------------------------------|
        | CT1.1 | single_str | single          | absent  | val (unquoted)                             | none           | none         | none      | default                               | Path: `cmd val` (greedy)                                    |
        | CT1.2 | single_str | multi           | absent  | name1::val1 (unquoted)                     | none           | none         | none      | default                               | Path: `p1 p2`, Named: `n1:v1`                               |
        | CT1.3 | single_str | single          | present | none                                       | none           | none         | none      | default                               | Path: `cmd`, Help: true                                     |
        | CT1.4 | single_str | single          | absent  | pos1 ("quoted val")                        | double         | none         | none      | default                               | Path: `cmd`, Pos: `quoted val`                              |
        | CT1.5 | single_str | single          | absent  | name1::"esc\\nval"                         | double         | std          | none      | default                               | Path: `cmd`, Named: `n1:esc\nval`                           |
        | CT1.6 | single_str | single          | absent  | name1::"bad\\xval"                         | double         | invalid      | none      | default                               | Error: Invalid escape                                       |
        | CT2.1 | slice      | multi           | absent  | pos1, name1::val1                          | mixed          | none         | none      | allow_pos_after_named=false           | Path: `p1 p2`, Pos: `pos1`, Named: `n1:v1`                  |
        | CT3.1 | single_str | single          | absent  | arg1 (path); name::val (arg)               | none           | none         | `;;`      | default                               | Instr1: Path `cmd1 arg1`; Instr2: Path `cmd2`, Named `name:val`|
        | CT4.1 | single_str | single          | absent  | name::val1, name::val2                     | none           | none         | none      | error_on_duplicate=true               | Error: Duplicate named                                      |
        | CT4.2 | single_str | single          | absent  | name::val1, name::val2                     | none           | none         | none      | error_on_duplicate=false              | Path: `cmd`, Named: `name:val2` (last wins)                 |
        | CT5.1 | single_str | no path         | absent  | name::val                                  | none           | none         | none      | default                               | Path: `[]`, Named: `name:val`                               |

    *   Crucial Design Rules: [Testing: Plan with a Test Matrix When Writing Tests](#testing-plan-with-a-test-matrix-when-writing-tests)
    *   Relevant Behavior Rules: All parser behavior rules from `unilang/spec.md`.
    *   Verification Strategy: `cargo test --package unilang_instruction_parser --test comprehensive_tests` (All 11 current tests pass). `cargo test --package unilang_instruction_parser --test error_reporting_tests` (All 13 tests pass). `cargo test --package unilang_instruction_parser --test argument_parsing_tests` (14/18 pass, 4 known external unescaping failures).
    *   Commit Message: `test(unilang_parser): Add initial comprehensive test suite based on Test Matrix`

*   ⚫ **Increment 8: Documentation and Examples**

### Task Requirements
*   (As before)

### Project Requirements
*   (As before)

### Notes & Insights
*   (As before, plus any new insights from Increment 7 planning)
*   The Test Matrix is initiated and can be expanded in future work or if more specific edge cases are identified. The current comprehensive tests cover the primary planned scenarios.
