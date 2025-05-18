# Project Plan: `unilang_instruction_parser` (Revised V4)

### Goal
*   Implement a robust, non-panicking parser in `unilang_instruction_parser` for `unilang` CLI syntax, strictly adhering to `unilang/spec.md`.
*   Utilize `strs_tools::string::split` for lexical analysis/itemization.
*   Produce `Vec<GenericInstruction<'a>>` from `&str` or `&[&str]` input.
*   Provide precise, AST-node-level, location-aware error reporting using `SourceLocation`.

### Progress
*   Overall Task for unilang_instruction_parser: ⚙️ Parsing Logic - 40% Complete (Instruction grouping implemented)
*   Milestones Achieved:
    *   ✅ Increment 1: Core types adapted to `strs_tools::string::split` and `no_std` feature added.
    *   ✅ Increment 2: Parser entry points and `RichItem` stream generation implemented.
    *   ✅ Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries implemented.
*   Currently Working On:
    *   All steps for Increment 3 are complete.
*   Up Next:
    *   ⚫🚀 Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing (Needs plan revision)
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
    *   Target Component(s): `unilang_instruction_parser`
    *   Pre-Analysis: Increment 2 complete. `analyze_items_to_instructions` is a stub.
    *   Detailed Plan Step 1: In `parser_engine.rs`, begin actual implementation of `analyze_items_to_instructions(self, items: Vec<RichItem<'input>>)`.
    *   Detailed Plan Step 2: Iterate through the input `items` (which are `RichItem<'input>`).
    *   Detailed Plan Step 3: Identify groups of `RichItem`s that constitute a single potential instruction. These groups are separated by `RichItem`s where `kind == UnilangTokenKind::Delimiter(Cow::Borrowed(";;"))`.
        *   Collect `RichItem`s into a sub-vector for each potential instruction.
    *   Detailed Plan Step 4: For each sub-vector of `RichItem`s:
        *   If the sub-vector is empty (e.g., input like `cmd ;; ;; cmd2` or leading/trailing `;;` after filtering): Handle as per Expected Behavior E8 (e.g., return `ParseError::Syntax("Empty instruction segment".to_string())` or skip if spec allows). For now, assume error for empty segments.
        *   If non-empty, pass this sub-vector (e.g., `&[RichItem<'input>]`) to a new private helper method, e.g., `parse_single_instruction_from_rich_items(&self, instruction_rich_items: &[RichItem<'input>]) -> Result<GenericInstruction<'input>, ParseError>`. This new helper will be implemented in subsequent increments (4 & 5).
        *   For this increment (Increment 3), `parse_single_instruction_from_rich_items` can be a stub that returns a dummy `GenericInstruction` or `Err(ParseError)` to allow testing the grouping logic. For example, it could return `Ok(GenericInstruction { command_path_slices: vec![first_item_slice.to_string()], named_arguments: HashMap::new(), positional_arguments: vec![], help_requested: false, overall_location: /* derive from first/last item */ })` if `instruction_rich_items` is not empty.
    *   Detailed Plan Step 5: Collect the `Result<GenericInstruction, ParseError>` from each call to `parse_single_instruction_from_rich_items`. If any result is an `Err`, propagate it. Otherwise, collect `Ok` values into `Vec<GenericInstruction>`.
    *   Detailed Plan Step 6: Create `tests/syntactic_analyzer_command_tests.rs` (if not existing) and add tests for:
        *   Input with a single command (no `;;`). Expected: 1 instruction (dummy).
        *   Input with multiple commands separated by `;;`. Expected: N instructions (dummy).
        *   Edge cases: `cmd;;`, `;;cmd`, `;;`, `cmd1 ;;;; cmd2`. Verify correct number of (dummy) instructions or appropriate errors for empty segments based on E8.
    *   Verification Strategy: `cargo test --package unilang_instruction_parser --test syntactic_analyzer_command_tests`.
    *   Commit Message: `feat(unilang_parser): Implement instruction grouping by ';;' delimiter in analyze_items_to_instructions`
    *   **(This is a revised plan for Increment 3 based on the new itemizer.)**

*   ⚫ **Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing**
    *   (Plan to be revised: Will implement `parse_single_instruction_from_rich_items` focusing on path and help operator `?` from `UnilangTokenKind::Operator`.)
    *   **(Needs plan revision due to itemizer change)**
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
*   Increments 3-5 detailed plans need to be developed one by one as they become active.
