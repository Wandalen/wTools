# Project Plan: `unilang_instruction_parser` (Revised V4)

### Goal
*   Implement a robust, non-panicking parser in `unilang_instruction_parser` for `unilang` CLI syntax, strictly adhering to `unilang/spec.md`.
*   Utilize a general-purpose itemizer (placeholder: `strs_tools::string::tokenizer_core`) for lexical analysis.
*   Produce `Vec<GenericInstruction<'a>>` from `&str` or `&[&str]` input.
*   Provide precise, AST-node-level, location-aware error reporting using `SourceLocation`.

### Progress
*   Overall Task for unilang_instruction_parser: 🏗️ Foundational Setup - 10% Complete (Core local structures defined; `strs_tools` integration points need path correction & confirmation)
*   Milestones Achieved:
    *   ✅ Basic Crate Structure and Local Types Defined (parts of Increment 1)
*   Currently Working On:
    *   ❗ **Action Required:** Confirm/Resolve `strs_tools` itemizer dependency and its API.
    *   ⏳ Increment 1: Finalize Core Structures & Initial Configuration (pending itemizer path correction & API confirmation)
*   Up Next:
    *   ⚫🚀 Increment 2: Implement Parser Entry Points and `RichItem` Stream Generation
    *   ⚫🚀 Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries
    *   ⚫🚀 Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing
    *   ⚫🚀 Increment 5: Syntactic Analyzer - Argument Parsing (Named, Positional)
    *   ⚫🚀 Increment 6: Error Reporting Integration and Refinement
    *   ⚫🚀 Increment 7: Comprehensive Test Suite (Test Matrix)
    *   ⚫🚀 Increment 8: Documentation and Examples

### Relevant Context
*   **Primary Target Component:** `unilang_instruction_parser`
*   **Primary Language(s):** Rust
*   **Dependencies:** `strs_tools` (for itemization), `error_tools`, `iter_tools`.
*   **CRITICAL: `strs_tools` Itemizer Dependency & API:**
    *   `unilang_instruction_parser/Cargo.toml` uses `features = ["string_parse_request"]` for `strs_tools`. This feature's module (`strs_tools::string::parse_request`) is for higher-level parsing, **not** general-purpose itemization.
    *   This plan assumes a **placeholder module** `strs_tools::string::tokenizer_core` provides types like `Itemizer`, `Item`, `ItemKind` (enum: `Identifier`, `QuotedValue`, `UnquotedValue`, `Delimiter`, `Operator`, `Whitespace`, `Comment`, `Unknown`), `ItemizerOptions`, and itemization-specific `ErrorKind`/`ParseError`.
    *   **Resolution Path:**
        1.  **Action for User/`strs_tools` maintainer:** Confirm if `strs_tools` has an existing feature/module for generic, configurable itemization.
        2.  If yes: Update `unilang_instruction_parser/Cargo.toml` and all code/plan paths to use the correct `strs_tools` feature and types.
        3.  If no: A `task.md` must be generated for `strs_tools` to implement this generic itemizer, or an alternative itemizer crate must be chosen. This plan is contingent on such an itemizer being available.
*   `unilang/spec.md`: The authoritative source for `unilang` lexical and syntactic grammar.
*   **Workspace:** Yes
*   **Key `strs_tools` types (ASSUMED from `tokenizer_core` placeholder):** `tokenizer_core::ItemizerOptions`, `tokenizer_core::Itemizer`, `tokenizer_core::Item { slice: &'a str, kind: ItemKind, location: Location, unescaped_value() -> Cow<'a, str> }`, `tokenizer_core::ErrorKind`, `tokenizer_core::ParseError`.
*   **Internal `RichItem` (e.g., in `src/item_adapter.rs` or `src/instruction.rs`):**
    ```rust
    #[derive(Debug, Clone)]
    pub struct RichItem<'a> {
        pub inner: strs_tools::string::tokenizer_core::Item<'a>, // Uses placeholder path
        pub segment_idx: Option<usize>, // None for single_str input, Some(idx) for slice input
    }
    impl<'a> RichItem<'a> {
        // Helper to get SourceLocation from this item
        pub fn source_location(&self) -> SourceLocation { /* ... */ }
    }
    ```
*   **Module Structure (Partially Implemented - `strs_tools` paths need update):**
    *   `src/lib.rs`, `src/instruction.rs` (OK)
    *   `src/error.rs`, `src/config.rs` (Need `strs_tools` path correction)
    *   `src/parser_engine.rs` (Parser struct OK, methods pending)

### Project Requirements (for Primary Target Component and interactions)
*   **R0: Valid Itemizer Dependency:** Must use a confirmed, working generic itemizer from `strs_tools` (or alternative).
*   **R1: Itemizer Usage:** Must use the confirmed itemizer (e.g., `strs_tools::string::tokenizer_core::Itemizer`).
*   **R2: Unilang Lexical Grammar Adherence (via ItemizerOptions):** `UnilangParserOptions` must configure the itemizer (e.g., `strs_tools::string::tokenizer_core::ItemizerOptions`) for:
    *   Quote pairs (e.g., `""`, `''`).
    *   Escape character (e.g., `\`) and supported escape sequences (as per `unilang/spec.md`).
    *   Delimiters (e.g., `::` for named args, `;;` for command separation).
    *   Operators (e.g., `?` for help).
    *   Comment prefix (e.g., `#`).
    *   Configuration to **discard** whitespace and comment items, so `analyze_items_to_instructions` receives only significant tokens.
    *   Implicit whitespace delimitation rules.
*   **R3: Unilang Syntactic Grammar Adherence:** Parser must strictly follow `unilang/spec.md` for:
    *   Command path structure (e.g., sequence of identifiers/unquoted values).
    *   Help operator (`?`) placement and meaning.
    *   Command separation (`;;`).
    *   Named argument syntax (`name::value`).
    *   Positional argument syntax.
    *   Rules for argument order (e.g., positional before named, if any).
    *   Handling of duplicate named arguments (e.g., error, or last one wins, per spec).
*   **R4: Dual Input Handling:** API supports `&str` and `&[&str]`.
*   **R5: Value Unescaping:** `Argument.value` is `Cow<'a, str>`, using itemizer's `unescaped_value()`. Command paths and arg names use raw `Item.slice`.
*   **R6: Precise Location-Aware Errors:** `ParseError.location` points to the exact `RichItem`(s) or span. For missing tokens, location points to where it was expected (e.g., zero-width span after preceding token).
*   **R7: No Panics on User Input:** Always return `Result`.
*   **R8: Zero-Copy (where feasible):** Minimize allocations.
*   **R9: No Command Definitions Dependency:** Purely syntactic.
*   **R10: Comprehensive Test Coverage:** Via Test Matrix.
*   **R11: API Clarity & Usability:** Well-documented public API.
*   **R12: Error Propagation:** Itemizer errors cleanly converted.
*   **R13: Lifetime Management:** Correct borrowing.
*   **R14: Idempotency:** Consistent results.
*   **R15: Clear Separation of Concerns:** Lexical (itemizer) vs. Syntactic (this parser).
*   **R16: Code Testability:** Internal logic testable.
*   **R17: Robustness to Malformed Input:** Gracefully return `ParseError`.
*   **R18: Performance Considerations:** Avoid gross inefficiencies.
*   **R19: Parser State:** The parser should be stateless across calls to `parse_single_str`/`parse_slice` (apart from its `options`). Each call is independent.
*   **R20: `GenericInstruction` Structure:** `command_path_slices` stores raw slices. `named_arguments` keys are raw name slices. `positional_arguments` stores `Argument`s in order.
*   **R21 (Existing):** Direct code modifications restricted to `unilang_instruction_parser`.
*   **R22 (Existing):** Verification commands non-interactive.
*   **R23 (Existing):** Files under ~1000 LoC.

### Expected Behavior Rules (Unilang Specific - to be confirmed against `unilang/spec.md`)
*   **E1 (Value Unescaping):** `Argument::value` stores unescaped `Cow<'a, str>`, using `strs_tools::string::tokenizer_core::Item::unescaped_value()`.
*   **E2 (Delimiters/Operators):** `;;` separates instructions. `::` separates named argument name and value. `?` (typically at end of command or path) requests help.
*   **E3 (Argument Types):** Supports named arguments (`name::value`) and positional arguments.
*   **E4 (Identifiers):** Command path segments and argument names are from `strs_tools::string::tokenizer_core::Item.slice` (typically `Identifier` or `UnquotedValue` kinds).
*   **E5 (Item Stream):** Itemizer (e.g., `strs_tools::string::tokenizer_core::Itemizer`) configured to discard whitespace/comment items. Parser processes significant `RichItem`s.
*   **E6 (Argument Order):** (To be defined by `unilang/spec.md`) e.g., "All positional arguments must appear before any named arguments." or "Positional arguments are not allowed after a named argument."
*   **E7 (Duplicate Named Args):** (To be defined by `unilang/spec.md`) e.g., "Duplicate named arguments result in a `ParseError::Syntax`." or "The last occurrence of a named argument overrides previous ones."
*   **E8 (Empty Instructions):** (To be defined by `unilang/spec.md`) e.g., Input like `cmd1 ;;;; cmd2` (empty instruction between `;;`) results in a `ParseError::Syntax` or is silently skipped. Default to error if unspecified.
*   **E9 (SourceLocation):** `SourceLocation` enum (`StrSpan`, `SliceSegment`) used.
*   **E10 (Error Granularity):** Errors should be specific (e.g., `ErrorKind::MissingNamedArgumentValue` vs. generic `Syntax`).

### Increments

#### Phase 1: Setup and Core Structures

*   ⏳ **Increment 1: Finalize Core Structures & Initial Configuration**
    *   Target Component(s): `unilang_instruction_parser`
    *   ❗ **Sub-Step 0: Resolve `strs_tools` Itemizer Dependency & API.**
        *   Action: User to confirm/provide the correct `strs_tools` feature and module path for generic itemization (e.g., `string_tokenizer` feature, `strs_tools::string::tokenizer_core` module) and the exact API of `Item`, `ItemKind`, `ItemizerOptions`, `Itemizer`, `ErrorKind`, `ParseError` from that module.
        *   If not available in `strs_tools`, this plan is blocked. For now, proceed assuming `strs_tools::string::tokenizer_core::` and its types.
    *   Detailed Plan Step 1: Update `unilang_instruction_parser/Cargo.toml` if Sub-Step 0 identifies a different feature for `strs_tools`.
    *   Detailed Plan Step 2: Correct `src/error.rs`:
        *   `ErrorKind::Itemization` wraps `strs_tools::string::tokenizer_core::ErrorKind`.
        *   `From` impl for `strs_tools::string::tokenizer_core::ParseError`.
    *   Detailed Plan Step 3: Correct `src/config.rs`:
        *   `UnilangParserOptions.itemizer_options` is `strs_tools::string::tokenizer_core::ItemizerOptions<'static>`.
        *   `Default` impl for `UnilangParserOptions` correctly initializes `tokenizer_core::ItemizerOptions` as per Project Requirement R2 (discard whitespace/comments, set delimiters, quotes, etc.).
    *   Detailed Plan Step 4: Define `RichItem<'a>` struct (e.g., in `src/item_adapter.rs` or `src/instruction.rs`) with `inner: strs_tools::string::tokenizer_core::Item<'a>` and `segment_idx: Option<usize>`. Add `Debug, Clone` derives and a helper method `source_location(&self) -> SourceLocation`.
    *   Verification Strategy: `cargo build --package unilang_instruction_parser`. Manual review of `config.rs` (itemizer options), `error.rs` (error wrapping), and `RichItem` against the (now assumed correct) `strs_tools::string::tokenizer_core` API.
    *   Commit Message: `fix(unilang_parser): Align core types with confirmed itemizer API and add RichItem`

#### Phase 2: Parsing Engine Implementation

*   ⚫ **Increment 2: Implement Parser Entry Points and Item Stream Generation**
    *   Target Component(s): `unilang_instruction_parser`
    *   Pre-Analysis: Assumes Increment 1 is complete.
    *   Detailed Plan Step 1: In `src/parser_engine.rs`, implement `pub fn parse_single_str<'a>(&self, input: &'a str) -> Result<Vec<GenericInstruction<'a>>, ParseError>`.
        *   Create `strs_tools::string::tokenizer_core::Itemizer::new(input, &self.options.itemizer_options)`.
        *   Call `itemize_all()`. Convert itemizer `ParseError` to `unilang_instruction_parser::ParseError` (location `SourceLocation::StrSpan`).
        *   Transform `Vec<strs_tools::string::tokenizer_core::Item<'a>>` into `Vec<RichItem<'a>>` (`segment_idx: None`).
        *   Pass to `analyze_items_to_instructions`.
    *   Detailed Plan Step 2: In `src/parser_engine.rs`, implement `pub fn parse_slice<'a>(&self, input_segments: &'a [&'a str]) -> Result<Vec<GenericInstruction<'a>>, ParseError>`.
        *   Initialize `Vec<RichItem<'a>>`. Loop `input_segments` with `seg_idx`.
        *   Itemize each `segment_str`. Convert itemizer `ParseError` using `SourceLocation::SliceSegment { segment_index: seg_idx, ... }`.
        *   Convert `tokenizer_core::Item<'a>` to `RichItem<'a>` with `segment_idx: Some(seg_idx)`.
        *   Pass combined `Vec<RichItem<'a>>` to `analyze_items_to_instructions`.
    *   Detailed Plan Step 3: Implement placeholder `fn analyze_items_to_instructions<'input>(&self, _items: Vec<RichItem<'input>>) -> Result<Vec<GenericInstruction<'input>>, ParseError>` in `parser_engine.rs` (returns `Ok(vec![])`).
    *   Detailed Plan Step 4: Add tests in `tests/parser_config_entry_tests.rs` for `parse_single_str` and `parse_slice`:
        *   Empty/whitespace/comment-only inputs (should yield `Ok(vec![])` as `analyze_items_to_instructions` is a stub).
        *   Inputs causing itemization errors (e.g., unterminated quote if itemizer detects it), verify `ParseError` propagation.
    *   Verification Strategy: `cargo test --package unilang_instruction_parser`. Relevant tests: `parser_config_entry_tests.rs`.
    *   Commit Message: `feat(unilang_parser): Implement parser entry points and RichItem stream generation`

*   ⚫ **Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries**
    *   Target Component(s): `unilang_instruction_parser`
    *   Detailed Plan Step 1: In `parser_engine.rs`, begin `analyze_items_to_instructions(self, items: Vec<RichItem<'input>>)` implementation.
    *   Detailed Plan Step 2: Iterate through `items`, splitting them into groups based on `RichItem` where `inner.kind == ItemKind::Delimiter && inner.slice == ";;"`. Each group of `RichItem`s will form one `GenericInstruction`.
    *   Detailed Plan Step 3: For each group:
        *   If a group is empty (e.g., from `cmd ;; ;; cmd2` or leading/trailing `;;`): Handle as per Expected Behavior E8 (e.g., return `ParseError` or skip).
        *   If non-empty, pass this group (a `&[RichItem<'input>]`) to a new private helper method, e.g., `parse_single_instruction_from_items(&self, instruction_items: &[RichItem<'input>]) -> Result<GenericInstruction<'input>, ParseError>`.
    *   Detailed Plan Step 4: Collect results from `parse_single_instruction_from_items`.
    *   Verification Strategy: Add tests in `tests/syntactic_analyzer_command_tests.rs` for:
        *   Single command (no `;;`).
        *   Multiple commands separated by `;;`.
        *   Edge cases: `cmd;;`, `;;cmd`, `;;`, `cmd1 ;;;; cmd2`. Verify correct number of `GenericInstruction`s or appropriate errors.
    *   Commit Message: `feat(unilang_parser): Implement command grouping by ';;' delimiter`

*   ⚫ **Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing**
    *   Target Component(s): `unilang_instruction_parser`
    *   Detailed Plan Step 1: Implement `parse_single_instruction_from_items(&self, instruction_items: &[RichItem<'input>]) -> Result<GenericInstruction<'input>, ParseError>`.
    *   Detailed Plan Step 2: Initialize a `GenericInstruction`. Determine its `overall_location` from the span of the first to the last `RichItem` in `instruction_items`.
    *   Detailed Plan Step 3: Parse Command Path:
        *   Iterate from the start of `instruction_items`. Consume `RichItem`s if `inner.kind` is `ItemKind::Identifier` or `ItemKind::UnquotedValue`, adding `inner.slice` to `GenericInstruction.command_path_slices`.
        *   Stop path parsing when a different `ItemKind` is met, or an item that could start an argument (e.g., `::` if it's a distinct token, or a potential argument name).
        *   If no path segments found and other items exist, it might be an error or a command-less instruction (e.g. only `?`).
    *   Detailed Plan Step 4: Parse Help Operator (`?`):
        *   After path parsing (or if no path), check if the *last remaining significant item* in `instruction_items` (before argument parsing would begin) is `RichItem` where `inner.kind == ItemKind::Operator && inner.slice == "?"`.
        *   If so, set `GenericInstruction.help_requested = true` and consume this item. This `?` should not be considered an argument.
        *   Handle cases where `?` might appear elsewhere (e.g., mid-arguments) – this should be a syntax error as per E2.
    *   Detailed Plan Step 5: Store remaining `RichItem`s from `instruction_items` (those not part of path or help operator) for argument parsing in the next increment.
    *   Verification Strategy: Update tests in `tests/syntactic_analyzer_command_tests.rs`:
        *   Verify `command_path_slices` for simple and multi-segment paths.
        *   Verify `help_requested` flag with `?` in various valid/invalid positions.
        *   Verify `overall_location` for parsed instructions.
    *   Commit Message: `feat(unilang_parser): Parse command path and help operator '?'`

*   ⚫ **Increment 5: Syntactic Analyzer - Argument Parsing (Named, Positional)**
    *   Target Component(s): `unilang_instruction_parser`
    *   Detailed Plan Step 1: Continue `parse_single_instruction_from_items`. Use the remaining `RichItem`s after path/help parsing.
    *   Detailed Plan Step 2: Iterate through these items. Adhere to argument order rules (E6).
        *   **Named Arguments:** Detect sequence: `RichItem` (name: `Identifier`|`UnquotedValue`) -> `RichItem` (delim: `Delimiter`, `"::"`) -> `RichItem` (value: `QuotedValue`|`UnquotedValue`).
            *   Create `Argument` with `name_slice` (raw `name_item.inner.slice`), `value` (from `value_item.inner.unescaped_value()`), and `SourceLocation`s from `RichItem`s.
            *   Handle duplicate named arguments as per E7 (error or override). Store in `GenericInstruction.named_arguments`.
            *   Report `ParseError` for malformations (e.g., `name::` then EOF, `::value`, name/value wrong `ItemKind`).
        *   **Positional Arguments:** Any `RichItem` (kind `QuotedValue`|`UnquotedValue`) not part of a valid named argument sequence (and respecting order E6).
            *   Create `Argument` with `value` (from `item.inner.unescaped_value()`) and `SourceLocation`. Store in `GenericInstruction.positional_arguments`.
    *   Detailed Plan Step 3: After iterating, if any `RichItem`s remain unconsumed, it's a syntax error (e.g. unexpected operator).
    *   Verification Strategy: Update tests in `tests/argument_parsing_tests.rs`. Test:
        *   Positional only, named only, mixed arguments (respecting E6).
        *   Quoted/unquoted values, values needing unescaping.
        *   Error conditions: malformed named args, duplicate named args (per E7), order violations (per E6).
        *   Verify `Argument.name_location` and `Argument.value_location`.
    *   Commit Message: `feat(unilang_parser): Implement named and positional argument parsing logic`

#### Phase 3: Refinements and Testing

*   ⚫ **Increment 6: Error Reporting Integration and Refinement**
    *   Target Component(s): `unilang_instruction_parser`
    *   Detailed Plan Step 1: Review all `ParseError` creation sites in `analyze_items_to_instructions`, `parse_single_instruction_from_items`, and entry points.
    *   Detailed Plan Step 2: Ensure `ParseError.location` is accurate. For missing tokens, location should be a zero-width span immediately after the preceding token (or at current EOF if applicable).
    *   Detailed Plan Step 3: Define more specific `ErrorKind` variants if useful (e.g., `MissingNamedArgumentValue`, `UnexpectedTokenInArguments`, `InvalidCommandPath`, `DuplicateNamedArgument`).
    *   Detailed Plan Step 4: Add/update tests in `tests/error_reporting_tests.rs` for specific syntax errors, verifying `ErrorKind` and `SourceLocation` for both `parse_single_str` and `parse_slice`.
    *   Verification Strategy: `cargo test --package unilang_instruction_parser`. Focus on `error_reporting_tests.rs`. Manually review error messages.
    *   Commit Message: `fix(unilang_parser): Refine error kinds and SourceLocation accuracy for all ParseErrors`

*   ⚫ **Increment 7: Comprehensive Test Suite (Test Matrix)**
    *   (Details as in V3 plan - Test Matrix covering inputs, structures, args, values, delimiters, operators, quoting, escapes, errors, edge cases, adhering to all Expected Behavior rules E1-E10)
    *   Verification Strategy: `cargo test --package unilang_instruction_parser --all-features`. Aim for high test coverage.
    *   Commit Message: `test(unilang_parser): Implement comprehensive test suite based on Test Matrix`

*   ⚫ **Increment 8: Documentation and Examples**
    *   (Details as in V3 plan - Crate/API docs, example file, Readme update)
    *   Verification Strategy: Manual review, `cargo test --doc --package unilang_instruction_parser`.
    *   Commit Message: `docs(unilang_parser): Add crate and API documentation, and usage example`

### Requirements (Task-Specific for Primary Target Component)
*   **TSR1:** The choice and API of the itemizer from `strs_tools` (or alternative) must be finalized before substantial work on Increments 2-5. The current plan assumes `strs_tools::string::tokenizer_core::` as a placeholder.
*   **TSR2:** `unilang/spec.md` must be consulted to finalize Expected Behavior rules E6, E7, E8 before implementing Increments 3 and 5.

### Notes & Insights
*   **CRITICAL BLOCKER: Itemizer.** The parser's implementation heavily depends on a suitable generic itemizer from `strs_tools` (or an alternative). This must be resolved first.
*   The plan is now more granular in parsing stages (command grouping, then path/help, then args).
*   Error kinds and locations need careful attention at each stage of syntactic analysis.
*   The existing test files will become progressively relevant as their corresponding functionalities are implemented in Increments 2-6.