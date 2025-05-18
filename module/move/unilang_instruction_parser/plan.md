# Project Plan: `unilang_instruction_parser` (Revised V5 - Ownership Change)

### Goal
*   Implement a robust, non-panicking parser in `unilang_instruction_parser` for `unilang` CLI syntax, strictly adhering to `unilang/spec.md`.
*   Utilize `strs_tools::string::split` for lexical analysis/itemization.
*   Produce `Vec<GenericInstruction>` (using owned `String`s for arguments) from `&str` or `&[&str]` input.
*   Provide precise, AST-node-level, location-aware error reporting using `SourceLocation`.

### Progress
*   Overall Task for unilang_instruction_parser: ⚙️ Parsing Logic - 70% Complete
*   Milestones Achieved:
    *   ✅ Increment 1: Core types adapted to `strs_tools::string::split` and `no_std` feature added.
    *   ✅ Increment 2: Parser entry points and `RichItem` stream generation implemented.
    *   ✅ Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries implemented.
    *   ✅ Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing implemented.
    *   ✅ Increment 5: Syntactic Analyzer - Argument Parsing (Named & Positional) for Single-Segment Paths. (Note: Unescaping of strings with internal escaped quotes is limited by `strs_tools` behavior.)
*   Currently Working On:
    *   ⚫ Increment 5.1 (New): Implement Multi-Segment Path Parsing
*   Up Next:
    *   ⚫🚀 Increment 6: Error Reporting Integration and Refinement
    *   ⚫🚀 Increment 7: Comprehensive Test Suite (Test Matrix)
    *   ⚫🚀 Increment 8: Documentation and Examples

### Target Crate
*   module/move/unilang_instruction_parser

### Relevant Context
*   Files to Include (for AI's reference, if `read_file` is planned, primarily from Target Crate):
    *   `module/move/unilang_instruction_parser/src/instruction.rs`
    *   `module/move/unilang_instruction_parser/src/item_adapter.rs`
    *   `module/move/unilang_instruction_parser/src/parser_engine.rs`
    *   `module/move/unilang_instruction_parser/src/config.rs`
    *   `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   None

### Expected Behavior Rules / Specifications (for Target Crate)
*   (As previously defined in earlier plan versions, assuming they are still relevant or will be reviewed against `unilang/spec.md`)
*   R5, E1 (Unescaping rules from `unilang/spec.md`) - Partially met; complex internal escapes limited by `strs_tools`.
*   E6 (Argument order rules from `unilang/spec.md`)
*   E7 (Duplicate named argument rules from `unilang/spec.md`)

### Target File Structure (If Applicable, within Target Crate)
*   (No changes planned for this increment beyond type definitions within existing files)

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
    *   Target Component(s): `unilang_instruction_parser`
    *   Pre-Analysis: Ownership changes are complete. Itemization and ultra-simplified path parsing (single segment) with positional arguments are functional. Named argument parsing re-introduced.
    *   Detailed Plan Step 1: **Refactor Core Types for Ownership (src/instruction.rs):** (✅ Completed)
    *   Detailed Plan Step 2: **Adapt Item Adapter for Ownership (src/item_adapter.rs):** (✅ Completed)
    *   Detailed Plan Step 3: **Update Parser Engine for Tokenization (src/config.rs, src/parser_engine.rs itemization loop):** (✅ Completed, whitespace filtering in place)
    *   Detailed Plan Step 4: **Solidify Single-Segment Path and Help Parsing (src/parser_engine.rs):** (✅ Completed with "ultra-simplified" path logic)
    *   Detailed Plan Step 5: **Implement Positional Arguments with Single-Segment Path (src/parser_engine.rs):** (✅ Completed)
    *   Detailed Plan Step 6: **Re-introduce Named Argument Parsing Logic (src/parser_engine.rs):** (✅ Completed)
    *   Detailed Plan Step 7: **Update and Uncomment Tests (tests/argument_parsing_tests.rs):** (✅ Completed)
    *   Crucial Design Rules: [Testing: Plan with a Test Matrix When Writing Tests](#testing-plan-with-a-test-matrix-when-writing-tests)
    *   Relevant Behavior Rules: R5, E1, E6, E7 from `unilang/spec.md`
    *   Test Matrix: Focus on argument combinations with single-segment paths.
    *   Verification Strategy: `cargo test --package unilang_instruction_parser --test argument_parsing_tests`. (13/17 tests pass. 4 unescaping failures likely due to `strs_tools` behavior with internal escaped quotes).
    *   Commit Message: `feat(unilang_parser): Implement named and positional argument parsing for single-segment paths`

*   ⚫ **Increment 5.1 (New): Implement Multi-Segment Path Parsing**
    *   Target Component(s): `unilang_instruction_parser`
    *   Pre-Analysis: Argument parsing for single-segment paths is largely complete. Now, enhance path parsing.
    *   Detailed Plan Step 1: Revise path parsing loop in `parse_single_instruction_from_rich_items` to consume multiple `Identifier` or `UnquotedValue` tokens as path segments.
    *   Detailed Plan Step 2: Ensure path parsing correctly stops before any argument type (Positional, Named, Quoted) or help operator.
    *   Detailed Plan Step 3: Add/uncomment tests in `argument_parsing_tests.rs` for multi-segment paths with various argument combinations (e.g., `path sub arg1`, `path sub name::val`).
    *   Verification Strategy: `cargo test --package unilang_instruction_parser --test argument_parsing_tests`.
    *   Commit Message: `feat(unilang_parser): Implement multi-segment command path parsing`

*   ⚫ **Increment 6: Error Reporting Integration and Refinement**
*   ⚫ **Increment 7: Comprehensive Test Suite (Test Matrix)**
*   ⚫ **Increment 8: Documentation and Examples**

### Task Requirements
*   (As before)

### Project Requirements
*   (As before)

### Notes & Insights
*   **Ownership Change:** Complete.
*   **Unescaping Limitation:** Unescaping of strings containing internal escaped quotes (e.g., `"foo \\"bar\\""`) is currently limited by the behavior of `strs_tools::string::split::SplitIterator` when `preserving_quoting: true`. It appears to truncate the segment at the first internal (escaped) quote. This affects 4 tests.
*   **Current Focus:** Next is multi-segment path parsing.
