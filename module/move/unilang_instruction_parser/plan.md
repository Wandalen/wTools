# Project Plan: `unilang_instruction_parser` (Revised V5 - Ownership Change)

    ### Goal
    *   Implement a robust, non-panicking parser in `unilang_instruction_parser` for `unilang` CLI syntax, strictly adhering to `unilang/spec.md`.
    *   Utilize `strs_tools::string::split` for lexical analysis/itemization.
    *   Produce `Vec<GenericInstruction>` (using owned `String`s for arguments) from `&str` or `&[&str]` input.
    *   Provide precise, AST-node-level, location-aware error reporting using `SourceLocation`.

    ### Progress
    *   Overall Task for unilang_instruction_parser: ⚙️ Parsing Logic - 85% Complete
    *   Milestones Achieved:
        *   ✅ Increment 1: Core types adapted to `strs_tools::string::split` and `no_std` feature added.
        *   ✅ Increment 2: Parser entry points and `RichItem` stream generation implemented.
        *   ✅ Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries implemented.
        *   ✅ Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing implemented.
        *   ✅ Increment 5: Syntactic Analyzer - Argument Parsing (Named & Positional) for Single-Segment Paths.
        *   ✅ Increment 5.1 (New - Stuck Resolution Strategy): Implement Multi-Segment Path Parsing (Isolated path logic now integrated and working with argument parsing).
    *   Currently Working On:
        *   ⚫🚀 Increment 6: Error Reporting Integration and Refinement (Up Next)
    *   Up Next:
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
        *   Commit Message: `feat(unilang_parser): Implement named and positional argument parsing for single-segment paths`

    *   ✅ **Increment 5.1 (New - Stuck Resolution Strategy): Implement Multi-Segment Path Parsing (Isolated)**
        *   Target Component(s): `unilang_instruction_parser`
        *   Pre-Analysis: Previous attempts to fix multi-segment path parsing failed. The parser incorrectly consumes arguments as path segments. This increment will now follow a stuck resolution strategy by isolating path parsing logic.
        *   **Sub-Increment 5.1.1: Isolate Path Parsing Logic in `parser_engine.rs`** (Completed)
        *   **Sub-Increment 5.1.2: Implement and Test Isolated Path Parsing** (Completed)
        *   **Sub-Increment 5.1.3: Reintegrate Help Operator Parsing and Test** (Completed)
        *   **Sub-Increment 5.1.4: Reintegrate Argument Parsing and Full Test Suite** (Completed - Path parsing logic now correctly handles multi-segment paths and integrates with argument parsing. Remaining test failures are due to external unescaping limitations.)
        *   Crucial Design Rules: [Implementation: Complete One Sub-Task Before Starting Another](#implementation-complete-one-sub-task-before-starting-another), [Stuck Resolution Process](#stuck-resolution-process)
        *   Relevant Behavior Rules: (General parsing rules)
        *   Verification Strategy: `cargo test --package unilang_instruction_parser --test argument_parsing_tests`. (14/18 pass, 4 known external failures)
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
    *   **Unescaping Limitation:** Unescaping of strings containing internal escaped quotes (e.g., `"foo \\"bar\\""`) is currently limited by the behavior of `strs_tools::string::split::SplitIterator` when `preserving_quoting: true`. It appears to truncate the segment at the first internal (escaped) quote. This affects 4 tests. These will not be addressed in Increment 5.1.
    *   **Current Focus:** Increment 5.1 successfully completed. Path parsing now correctly handles multi-segment paths like "path sub" and distinguishes them from arguments, based on the greedy consumption rule (path is all leading Identifiers/UnquotedValues until a non-path-like token or `::` is encountered).
