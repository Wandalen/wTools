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
    *   ⏳ Increment 5: Syntactic Analyzer - Argument Parsing (Named, Positional)
*   Up Next:
    *   ⚫🚀 Increment 6: Error Reporting Integration and Refinement
    *   ⚫🚀 Increment 7: Comprehensive Test Suite (Test Matrix)
    *   ⚫🚀 Increment 8: Documentation and Examples

### Relevant Context
*   **Primary Target Component:** `unilang_instruction_parser`
*   **Primary Language(s):** Rust
*   **Dependencies:** `strs_tools` (specifically `strs_tools::string::split`), `error_tools`, `iter_tools`.
*   **Itemizer:** `strs_tools::string::split` module.
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
*   (As previously defined)

### Expected Behavior Rules (Unilang Specific - to be confirmed against `unilang/spec.md`)
*   (As previously defined)

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

*   ⏳ **Increment 5: Syntactic Analyzer - Argument Parsing (Named, Positional)**
    *   Target Component(s): `unilang_instruction_parser`
    *   Pre-Analysis: Increment 4 complete. `parse_single_instruction_from_rich_items` now parses path and help. Remaining `RichItem`s need to be parsed as arguments. Unescaping logic (R5, E1) needs to be considered/implemented. Argument order (E6) and duplicate named args (E7) rules from `unilang/spec.md` are critical.
    *   Detailed Plan Step 1: In `parser_engine.rs`, continue implementing `parse_single_instruction_from_rich_items`. Use the `RichItem`s remaining after path and help operator parsing (available via `remaining_items_idx` from Increment 4 logic).
    *   Detailed Plan Step 2: Implement Positional Argument Parsing:
        *   Iterate through the remaining `RichItem`s.
        *   If a `RichItem`'s `kind` is `UnilangTokenKind::Identifier(...)`, `UnilangTokenKind::UnquotedValue(...)`, or `UnilangTokenKind::QuotedValue(...)`, and it's not part of a named argument sequence (see next step), treat it as a positional argument.
        *   **Unescaping (R5, E1):** For `QuotedValue` and potentially `UnquotedValue` (if spec requires unescaping for them), implement or call unescaping logic. The result should be `Cow<'input, str>`. For now, assume `s.as_ref()` is sufficient if no escapes are handled yet, or use `s.to_string()` if ownership is simpler initially. A `TODO` for full unescaping.
        *   Create `Argument<'input>` with `name_slice: None`, the (potentially unescaped) `value: Cow<'input, str>`, and `value_location`. Add to `GenericInstruction.positional_arguments`.
        *   Adhere to argument order rules (E6 from `unilang/spec.md`). For example, if positional arguments must come before named ones, stop positional parsing if a named argument indicator (`::`) is seen.
    *   Detailed Plan Step 3: Implement Named Argument Parsing:
        *   Look for the pattern: `RichItem(Identifier | UnquotedValue)` (name) `RichItem(Delimiter("::"))` `RichItem(Identifier | UnquotedValue | QuotedValue)` (value).
        *   Extract `name_slice` (raw `String` from `Identifier`/`UnquotedValue`'s payload).
        *   Extract and potentially unescape the value `Cow<'input, str>`.
        *   Create `Argument<'input>` with `name_slice: Some(name_string_owned_by_map_key)`, `value`, `name_location`, `value_location`.
        *   Store in `GenericInstruction.named_arguments` (key is `String`, value is `Argument<'input>`).
        *   Handle duplicate named arguments as per E7 from `unilang/spec.md` (e.g., error or last one wins).
        *   Report `ParseError` for malformations (e.g., `name::` then EOF, `::value`, name/value wrong `UnilangTokenKind`).
    *   Detailed Plan Step 4: After iterating through all remaining items, if any `RichItem` was not consumed as part of a valid argument, it's a syntax error (e.g., an unexpected `Operator` or `Delimiter` not `::`).
    *   Detailed Plan Step 5: Implement basic unescaping logic (placeholder if full spec is complex).
        *   Create a helper function e.g., `fn unescape_string(s: &str) -> Cow<str>`. For now, it can just return `Cow::Borrowed(s)` or handle very simple sequences like `\\` -> `\`. Add `TODO` for full spec compliance. This function could be in `item_adapter.rs` or a new `utils.rs`.
    *   Detailed Plan Step 6: Update tests in `tests/argument_parsing_tests.rs` (create if not existing):
        *   Positional arguments only.
        *   Named arguments only.
        *   Mixed arguments (respecting order E6).
        *   Values requiring unescaping (once basic unescaping is in).
        *   Error conditions: malformed named args, duplicate named args (per E7), order violations (per E6).
        *   Verify `Argument.name_location`, `Argument.value_location`, `Argument.name_slice` (for named), and `Argument.value`.
    *   Verification Strategy: `cargo test --package unilang_instruction_parser --test argument_parsing_tests`.
    *   Commit Message: `feat(unilang_parser): Implement named and positional argument parsing`

*   ⚫ **Increment 6: Error Reporting Integration and Refinement**
*   ⚫ **Increment 7: Comprehensive Test Suite (Test Matrix)**
*   ⚫ **Increment 8: Documentation and Examples**

### Requirements (Task-Specific for Primary Target Component)
*   **TSR1:** The API of `strs_tools::string::split` is now known. The parser must adapt.
*   **TSR2:** `unilang/spec.md` must be consulted to finalize Expected Behavior rules E6, E7, E8 and to guide the new classification logic and unescaping.

### Notes & Insights
*   **Itemizer Change Impact:** Switching to `strs_tools::string::split` is a major change. The parser now has more responsibilities.
*   The `UnilangTokenKind` and `classify_split` function are central.
*   Argument parsing (Inc 5) will introduce more complexity, especially around unescaping and adhering to `unilang/spec.md` for argument structure.
