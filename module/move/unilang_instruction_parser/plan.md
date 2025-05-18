# Project Plan: `unilang_instruction_parser` (Revised V4)

### Goal
*   Implement a robust, non-panicking parser in `unilang_instruction_parser` for `unilang` CLI syntax, strictly adhering to `unilang/spec.md`.
*   Utilize `strs_tools::string::split` for lexical analysis/itemization.
*   Produce `Vec<GenericInstruction<'a>>` from `&str` or `&[&str]` input.
*   Provide precise, AST-node-level, location-aware error reporting using `SourceLocation`.

### Progress
*   Overall Task for unilang_instruction_parser: ⚙️ Parsing Logic - 50% Complete (Path and help operator parsing implemented)
*   Milestones Achieved:
    *   ✅ Increment 1: Core types adapted to `strs_tools::string::split` and `no_std` feature added.
    *   ✅ Increment 2: Parser entry points and `RichItem` stream generation implemented.
    *   ✅ Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries implemented.
    *   ✅ Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing implemented.
*   Currently Working On:
    *   All steps for Increment 4 are complete.
*   Up Next:
    *   ⚫🚀 Increment 5: Syntactic Analyzer - Argument Parsing (Named, Positional) (Needs plan revision)
    *   ⚫🚀 Increment 6: Error Reporting Integration and Refinement
    *   ⚫🚀 Increment 7: Comprehensive Test Suite (Test Matrix)
    *   ⚫🚀 Increment 8: Documentation and Examples

### Relevant Context
*   **Primary Target Component:** `unilang_instruction_parser`
*   **Primary Language(s):** Rust
*   **Dependencies:** `strs_tools` (specifically `strs_tools::string::split`), `error_tools`, `iter_tools`.
*   **Itemizer:** `strs_tools::string::split` module.
    *   Key types: `strs_tools::string::split::Split<'a>`, `strs_tools::string::split::SplitType`, `strs_tools::string::split::SplitOptionsFormer<'a>`, `strs_tools::string::split::SplitIterator<'a>`.
*   `unilang/spec.md`: The authoritative source for `unilang` lexical and syntactic grammar.
*   **Workspace:** Yes
*   **Internal `RichItem` (defined in `src/item_adapter.rs`):**
    ```rust
    #[derive(Debug, Clone)]
    pub struct RichItem<'a> { /* ... */ }
    ```
*   **Internal `UnilangTokenKind` (defined in `src/item_adapter.rs`):**
    ```rust
    pub enum UnilangTokenKind<'a> { /* ... */ }
    ```
*   **Module Structure:**
    *   `src/lib.rs`, `src/instruction.rs`, `src/error.rs`, `src/config.rs`, `src/parser_engine.rs`, `src/item_adapter.rs`

### Project Requirements (for Primary Target Component and interactions)
*   (As previously defined, with R5 and R12 revised for new itemizer)

### Expected Behavior Rules (Unilang Specific - to be confirmed against `unilang/spec.md`)
*   (As previously defined, with E1, E2, E4, E5 revised for new itemizer)

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
    *   Target Component(s): `unilang_instruction_parser`
    *   Pre-Analysis: Increment 3 complete. `parse_single_instruction_from_rich_items` is a stub.
    *   Detailed Plan Step 1: In `parser_engine.rs`, begin actual implementation of `parse_single_instruction_from_rich_items(&self, instruction_rich_items: &[RichItem<'input>])`.
    *   Detailed Plan Step 2: Initialize a `GenericInstruction<'input>`. Determine its `overall_location` from the span of the first to the last `RichItem` in `instruction_rich_items`.
    *   Detailed Plan Step 3: Parse Command Path:
        *   Iterate from the start of `instruction_rich_items`.
        *   Consume `RichItem`s if their `kind` is `UnilangTokenKind::Identifier(...)` or `UnilangTokenKind::UnquotedValue(...)`. Add the `String` payload from these kinds to `GenericInstruction.command_path_slices`.
        *   Stop path parsing when a `RichItem` is encountered whose `kind` is not suitable for a path segment (e.g., `Operator`, `Delimiter` like `::`, or if argument parsing rules dictate).
        *   If no path segments are found but other items exist (e.g., only a `?`), this is valid for a help request on the "current context" (empty path).
    *   Detailed Plan Step 4: Parse Help Operator (`?`):
        *   After path parsing (or if no path was parsed), check if the *next significant `RichItem`* (or the last item if it's the only one remaining in `instruction_rich_items` after path items are conceptually consumed) is `kind == UnilangTokenKind::Operator(Cow::Borrowed("?"))`.
        *   If so, set `GenericInstruction.help_requested = true`. This `RichItem` is then consumed.
        *   A `?` appearing elsewhere (e.g., within arguments, or not as the effective last element of the command/path part) should result in a `ParseError::Syntax` as per E2, likely when argument parsing begins and finds an unexpected operator.
    *   Detailed Plan Step 5: Store any remaining `RichItem`s from `instruction_rich_items` (those not part of the command path or the help operator) to be processed by argument parsing logic in Increment 5. For this increment, these remaining items can be ignored by the stub logic within `parse_single_instruction_from_rich_items` after path/help is determined.
    *   Detailed Plan Step 6: Update tests in `tests/syntactic_analyzer_command_tests.rs`:
        *   Re-enable and adapt tests for simple paths (e.g., "cmd", "cmd subcmd").
        *   Re-enable and adapt tests for help operator (e.g., "cmd ?", "?", "cmd sub ?").
        *   Ensure `command_path_slices` (now `Vec<String>`) and `help_requested` are correctly populated.
        *   Verify `overall_location`.
    *   Verification Strategy: `cargo test --package unilang_instruction_parser --test syntactic_analyzer_command_tests`.
    *   Commit Message: `feat(unilang_parser): Implement command path and help operator parsing`

*   ⚫ **Increment 5: Syntactic Analyzer - Argument Parsing (Named, Positional)**
    *   (Plan to be revised: Will complete `parse_single_instruction_from_rich_items` focusing on arguments. Unescaping logic will be needed here or called from here.)
    *   **(Needs plan revision due to itemizer change)**

#### Phase 3: Refinements and Testing
*   ⚫ **Increment 6: Error Reporting Integration and Refinement**
*   ⚫ **Increment 7: Comprehensive Test Suite (Test Matrix)**
*   ⚫ **Increment 8: Documentation and Examples**

### Requirements (Task-Specific for Primary Target Component)
*   **TSR1:** The API of `strs_tools::string::split` is now known. The parser must adapt.
*   **TSR2:** `unilang/spec.md` must be consulted to finalize Expected Behavior rules E6, E7, E8 and to guide the new classification logic and unescaping.

### Notes & Insights
*   **Itemizer Change Impact:** Switching to `strs_tools::string::split` is a major change. The parser now has more responsibilities.
*   The `UnilangTokenKind` and `classify_split` function are central.
*   Increments 4-5 detailed plans need to be developed one by one as they become active.
