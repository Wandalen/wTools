# Project Plan: `unilang_instruction_parser` (Revised V4)

### Goal
*   Implement a robust, non-panicking parser in `unilang_instruction_parser` for `unilang` CLI syntax, strictly adhering to `unilang/spec.md`.
*   Utilize `strs_tools::string::split` for lexical analysis/itemization.
*   Produce `Vec<GenericInstruction<'a>>` from `&str` or `&[&str]` input.
*   Provide precise, AST-node-level, location-aware error reporting using `SourceLocation`.

### Progress
*   Overall Task for unilang_instruction_parser: 🏗️ Foundational Setup - 30% Complete (Parser entry points and RichItem stream generation implemented)
*   Milestones Achieved:
    *   ✅ Increment 1: Core types adapted to `strs_tools::string::split` and `no_std` feature added.
    *   ✅ Increment 2: Parser entry points and `RichItem` stream generation implemented.
*   Currently Working On:
    *   All steps for Increment 2 are complete.
*   Up Next:
    *   ⚫🚀 Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries (Needs plan revision due to itemizer change)
    *   ⚫🚀 Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing (Needs plan revision due to itemizer change)
    *   ⚫🚀 Increment 5: Syntactic Analyzer - Argument Parsing (Named, Positional) (Needs plan revision due to itemizer change)
    *   ⚫🚀 Increment 6: Error Reporting Integration and Refinement
    *   ⚫🚀 Increment 7: Comprehensive Test Suite (Test Matrix)
    *   ⚫🚀 Increment 8: Documentation and Examples

### Relevant Context
*   **Primary Target Component:** `unilang_instruction_parser`
*   **Primary Language(s):** Rust
*   **Dependencies:** `strs_tools` (specifically `strs_tools::string::split`), `error_tools`, `iter_tools`.
*   **Itemizer:** `strs_tools::string::split` module.
    *   Key types: `strs_tools::string::split::Split<'a>`, `strs_tools::string::split::SplitType`, `strs_tools::string::split::SplitOptionsFormer<'a>`, `strs_tools::string::split::SplitIterator<'a>`.
    *   Note: This itemizer is simpler than the previously assumed `tokenizer_core`. It does not provide detailed `ItemKind` classification (like Identifier, Operator) or unescaping. These will be responsibilities of `unilang_instruction_parser`.
*   `unilang/spec.md`: The authoritative source for `unilang` lexical and syntactic grammar.
*   **Workspace:** Yes
*   **Internal `RichItem` (defined in `src/item_adapter.rs`):**
    ```rust
    #[derive(Debug, Clone)]
    pub struct RichItem<'a> {
        pub inner: strs_tools::string::split::Split<'a>,
        pub segment_idx: Option<usize>,
        pub kind: UnilangTokenKind<'a>,
    }
    impl<'a> RichItem<'a> {
        pub fn source_location(&self) -> SourceLocation { /* ... uses inner.start, inner.end ... */ }
    }
    ```
*   **Internal `UnilangTokenKind` (defined in `src/item_adapter.rs`):**
    ```rust
    pub enum UnilangTokenKind<'a> {
        Identifier( Cow<'a, str> ),
        Operator( Cow<'a, str> ),
        Delimiter( Cow<'a, str> ),
        QuotedValue( Cow<'a, str> ),
        UnquotedValue( Cow<'a, str> ),
        Unrecognized( Cow<'a, str> ),
    }
    ```
*   **Module Structure:**
    *   `src/lib.rs`, `src/instruction.rs`, `src/error.rs`, `src/config.rs`, `src/parser_engine.rs`, `src/item_adapter.rs`

### Project Requirements (for Primary Target Component and interactions)
*   **R0: Valid Itemizer Usage:** Must use `strs_tools::string::split`.
*   **R1: Item Classification:** `unilang_instruction_parser` must classify `strs_tools::string::split::Split.string` into `UnilangTokenKind`.
*   **R2: Unilang Lexical Grammar Adherence (via SplitOptionsFormer & Parser Logic):** `UnilangParserOptions` must configure `SplitOptionsFormer` for:
    *   Quote pairs (e.g., `""`, `''`) via `quoting_prefixes`, `quoting_postfixes`.
    *   Delimiters (e.g., `::` for named args, `;;` for command separation) via `delimeter` option.
    *   Operators (e.g., `?` for help) will likely be treated as delimiters by `SplitOptionsFormer` or classified by the parser.
    *   Comment prefix (e.g., `#`) handling will be a parser responsibility (post-split).
    *   Whitespace discarding: Use `stripping : true` in `SplitOptionsFormer` and/or filter in parser.
*   **R3-R23:** (Largely as before, but implications of new itemizer to be considered, e.g., R5 unescaping is now fully parser's job).
*   **R5 (Revised): Value Unescaping:** `Argument.value` is `Cow<'a, str>`. Unescaping logic must be implemented in `unilang_instruction_parser`.
*   **R12 (Revised): Error Propagation:** Errors from `SplitIterator` (if any, it doesn't seem to return `Result`) or from the parser's own classification/syntax analysis need to be handled.

### Expected Behavior Rules (Unilang Specific - to be confirmed against `unilang/spec.md`)
*   **E1 (Value Unescaping):** `Argument::value` stores unescaped `Cow<'a, str>`. `unilang_instruction_parser` implements unescaping.
*   **E2 (Delimiters/Operators):** `;;` separates instructions. `::` separates named argument name and value. `?` requests help. These will be configured as delimiters for `SplitOptionsFormer` or classified by the parser.
*   **E4 (Identifiers):** Command path segments and argument names are derived from `strs_tools::string::split::Split.string` after classification.
*   **E5 (Item Stream):** `SplitOptionsFormer` configured to manage delimiters. Parser filters/classifies `Split` items into `RichItem`s with `UnilangTokenKind`. Whitespace/comments handled by `stripping` or parser logic.
*   (E3, E6-E10 remain largely the same in principle, but implementation details will adapt to the new itemizer)

### Increments

#### Phase 1: Setup and Core Structures

*   ✅ **Increment 1: Adapt to `strs_tools::string::split` & Define Core Structures**
    *   Target Component(s): `unilang_instruction_parser`
    *   Pre-Analysis: User has directed to use `strs_tools::string::split`. This is a significant API change from the placeholder `tokenizer_core`. The parser will need to handle more token classification.
    *   Detailed Plan Step 1: Update `unilang_instruction_parser/Cargo.toml`:
        *   Ensure `strs_tools` dependency is correctly specified. The `string/split.rs` module is part of the main `strs_tools` library, so no special feature flag should be needed for it beyond the base dependency.
        *   Add `"no_std"` to the `[features]` section of `unilang_instruction_parser/Cargo.toml` to resolve the `unexpected_cfgs` warning.
            ```toml
            # In unilang_instruction_parser/Cargo.toml
            [features]
            default = []
            no_std = []
            ```
    *   Detailed Plan Step 2: Modify `src/error.rs`:
        *   Remove or significantly re-evaluate `ErrorKind::Itemization` as `strs_tools::string::split::SplitIterator` does not return `Result` and thus doesn't have its own `ErrorKind` or `ParseError` to wrap. Parsing errors will primarily originate from `unilang_instruction_parser`'s own logic.
        *   Remove the `From<...ParseError>` impl related to the previous itemizer.
        *   Ensure `ErrorKind::Syntax(String)`, `UnterminatedQuote`, `InvalidEscapeSequence` are robust.
    *   Detailed Plan Step 3: Modify `src/config.rs`:
        *   `UnilangParserOptions` should store high-level options.
        *   The `Default` impl for `UnilangParserOptions` will set these high-level options. A method on `UnilangParserOptions` (e.g., `to_split_options_former<'s>(&'s self, src: &'s str) -> strs_tools::string::split::SplitOptionsFormer<'s>`) will translate these into `SplitOptionsFormer` settings when an iterator is needed.
        *   This translation will configure delimiters (`;;`, `::`, `?`), quote pairs (`""`, `''` via `quoting_prefixes`/`postfixes`), and `stripping : true`.
        *   Comment/escape char logic is now a parser responsibility.
    *   Detailed Plan Step 4: Define/Modify `RichItem<'a>` struct in a new file `src/item_adapter.rs` (or `src/instruction.rs` if preferred, but `item_adapter.rs` is better for separation):
        *   `pub inner: strs_tools::string::split::Split<'a>`
        *   `pub segment_idx: Option<usize>`
        *   `pub kind: UnilangTokenKind<'a>` (see next step)
        *   `source_location(&self) -> SourceLocation` method using `self.inner.start` and `self.inner.end`.
    *   Detailed Plan Step 5: In `src/item_adapter.rs`, define:
        *   `pub enum UnilangTokenKind<'a> { Identifier( Cow<'a, str> ), Operator( Cow<'a, str> ), Delimiter( Cow<'a, str> ), QuotedValue( Cow<'a, str> ), UnquotedValue( Cow<'a, str> ), Unrecognized( Cow<'a, str> ) }`
        *   `pub fn classify_split<'a>(split: &strs_tools::string::split::Split<'a>, options: &UnilangParserOptions) -> UnilangTokenKind<'a>`
            *   This function will look at `split.string` and `split.typ`.
            *   If `split.typ == SplitType::Delimeter`, it's `UnilangTokenKind::Delimiter` or `Operator` based on `options`.
            *   If `split.typ == SplitType::Delimeted`, it needs further classification.
    *   Detailed Plan Step 6: Ensure `src/lib.rs` declares `mod item_adapter;` and re-exports its contents in prelude.
    *   Verification Strategy: `cargo build --package unilang_instruction_parser`. Manual review of changes against `strs_tools::string::split` API and new classification logic.
    *   Commit Message: `refactor(unilang_parser): Adapt core types to strs_tools::string::split API and add RichItem`

#### Phase 2: Parsing Engine Implementation

*   ✅ **Increment 2: Implement Parser Entry Points and `RichItem` Stream Generation**
    *   Target Component(s): `unilang_instruction_parser`
    *   Pre-Analysis: Increment 1 is complete. `strs_tools::string::split` is the itemizer. `item_adapter::classify_split` provides initial token classification.
    *   Crucial Design Rules: [Error Handling: Use a Centralized Approach](#error-handling-use-a-centralized-approach), [Implementation: Complete One Sub-Task Before Starting Another](#implementation-complete-one-sub-task-before-starting-another).
    *   Relevant Behavior Rules: E4 (Identifiers), E5 (Item Stream).
    *   Detailed Plan Step 1: **Refine `item_adapter::classify_split` function.**
        *   Ensure it correctly identifies `Delimiter("::")`, `Delimiter(";;")`, and `Operator("?")` based on `split.string` when `split.typ == SplitType::Delimeter`.
        *   For `SplitType::Delimeted` content:
            *   If `UnilangParserOptions` is configured to preserve quotes by `SplitOptionsFormer` (e.g., by setting `preserving_quoting: true` in `to_split_options_former`), then `classify_split` must check if `split.string` starts/ends with configured quote characters. If so, classify as `UnilangTokenKind::QuotedValue` (containing the *inner* string, without the quotes).
            *   Otherwise (not quoted or quotes already stripped by `SplitOptionsFormer`), classify as `UnilangTokenKind::Identifier` or `UnilangTokenKind::UnquotedValue`. The distinction might be heuristic for now (e.g., based on `unilang/spec.md` rules for identifiers if available, otherwise assume `UnquotedValue` or a more general `PotentialIdentifierOrValue`).
            *   Empty `Delimeted` strings should probably be `UnilangTokenKind::Unrecognized("")` or filtered out before classification if `SplitOptionsFormer`'s `preserving_empty` is false.
        *   Add basic tests for `classify_split` within `item_adapter.rs` (e.g., in a `#[cfg(test)] mod tests { ... }`).
    *   Detailed Plan Step 2: In `src/parser_engine.rs`, implement `pub fn parse_single_str<'input>(&self, input: &'input str) -> Result<Vec<GenericInstruction<'input>>, ParseError>`.
        *   Create a `SplitIterator` using `self.options.to_split_options_former(input).perform()`.
        *   Iterate through the `Split<'input>` items from the iterator.
        *   For each `Split` item:
            *   Call `item_adapter::classify_split` to get `UnilangTokenKind<'input>`.
            *   Construct `RichItem<'input> { inner: split_item, segment_idx: None, kind: classified_kind }`.
            *   Collect these `RichItem`s into a `Vec`.
        *   Pass the `Vec<RichItem<'input>>` to `analyze_items_to_instructions`.
        *   Handle potential errors from `analyze_items_to_instructions`.
    *   Detailed Plan Step 3: In `src/parser_engine.rs`, implement `pub fn parse_slice<'input>(&self, input_segments: &'input [&'input str]) -> Result<Vec<GenericInstruction<'input>>, ParseError>`.
        *   Initialize an empty `Vec<RichItem<'input>>`.
        *   Loop through `input_segments` with `enumerate()` to get `seg_idx` and `segment_str`.
        *   For each `segment_str`:
            *   Create a `SplitIterator` using `self.options.to_split_options_former(segment_str).perform()`.
            *   Iterate, classify each `Split`, and construct `RichItem<'input> { inner: split_item, segment_idx: Some(seg_idx), kind: classified_kind }`.
            *   Append to the main `Vec<RichItem<'input>>`.
        *   Pass the combined `Vec<RichItem<'input>>` to `analyze_items_to_instructions`.
    *   Detailed Plan Step 4: In `src/parser_engine.rs`, implement a placeholder for `fn analyze_items_to_instructions<'input>(&self, items: Vec<RichItem<'input>>) -> Result<Vec<GenericInstruction<'input>>, ParseError>`.
        *   This function will take `items: Vec<RichItem<'input>>`.
        *   For now, it should just return `Ok(vec![])`.
        *   Add a `// TODO: Implement full syntactic analysis` comment.
    *   Detailed Plan Step 5: Create `tests/parser_config_entry_tests.rs` (if not existing) and add tests for `parse_single_str` and `parse_slice`:
        *   Test with empty input: `""`, `&[]` -> `Ok(vec![])`.
        *   Test with whitespace/comment-only input (assuming `SplitOptionsFormer` with `stripping:true` and parser filtering will result in no significant `RichItem`s): `"   # comment   "` -> `Ok(vec![])`.
        *   Test with a single simple token, e.g., `"command"` -> `Ok(vec![])` (as `analyze_items_to_instructions` is a stub, but ensures item stream generation and classification runs). Verify that `classify_split` produces an expected `UnilangTokenKind` for "command".
        *   Test with multiple segments: `&["cmd1", "arg1"]` -> `Ok(vec![])`.
    *   Verification Strategy: `cargo build --package unilang_instruction_parser`, then `cargo test --package unilang_instruction_parser --test parser_config_entry_tests`. Review `item_adapter::classify_split` logic.
    *   Commit Message: `feat(unilang_parser): Implement parser entry points and RichItem stream generation using string::split`

*   ⚫ **Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries**
    *   (Plan to be revised: Will operate on `Vec<RichItem<'input>>`. Grouping by `RichItem` where `kind == UnilangTokenKind::Delimiter(";;".into())`.)
    *   **(Needs plan revision due to itemizer change)**
*   ⚫ **Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing**
    *   (Plan to be revised: Operates on `&[RichItem<'input>]`. Path from `UnilangTokenKind::Identifier` or `UnquotedValue`. Help from `UnilangTokenKind::Operator("?".into())`.)
    *   **(Needs plan revision due to itemizer change)**
*   ⚫ **Increment 5: Syntactic Analyzer - Argument Parsing (Named, Positional)**
    *   (Plan to be revised: Named args: `Identifier`/`UnquotedValue` -> `Delimiter("::".into())` -> `QuotedValue`/`UnquotedValue`. Unescaping is now parser's job.)
    *   **(Needs plan revision due to itemizer change)**

#### Phase 3: Refinements and Testing
*   ⚫ **Increment 6: Error Reporting Integration and Refinement**
*   ⚫ **Increment 7: Comprehensive Test Suite (Test Matrix)**
*   ⚫ **Increment 8: Documentation and Examples**

### Requirements (Task-Specific for Primary Target Component)
*   **TSR1:** The API of `strs_tools::string::split` is now known. The parser must adapt.
*   **TSR2:** `unilang/spec.md` must be consulted to finalize Expected Behavior rules E6, E7, E8 and to guide the new classification logic and unescaping.

### Notes & Insights
*   **Itemizer Change Impact:** Switching to `strs_tools::string::split` is a major change. The parser now has more responsibilities:
    *   Token classification (Identifier, Operator, etc.) based on `Split.string`.
    *   Value unescaping.
    *   Potentially comment handling if not fully managed by `SplitOptionsFormer`.
*   The `UnilangTokenKind` and `classify_split` function will be central to the new approach.
*   Increments 2-5 need substantial revision in their detailed steps once Increment 1 is complete and the classification mechanism is clearer. The current text for Inc 2 is a first pass.
