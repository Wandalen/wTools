# Project Plan: `unilang_instruction_parser` (Revised V4)

### Goal
*   Implement a robust, non-panicking parser in `unilang_instruction_parser` for `unilang` CLI syntax, strictly adhering to `unilang/spec.md`.
*   Utilize `strs_tools::string::split` for lexical analysis/itemization.
*   Produce `Vec<GenericInstruction<'a>>` from `&str` or `&[&str]` input.
*   Provide precise, AST-node-level, location-aware error reporting using `SourceLocation`.

### Progress
*   Overall Task for unilang_instruction_parser: 🏗️ Foundational Setup - 20% Complete (Core types adapted to `strs_tools::string::split`)
*   Milestones Achieved:
    *   ✅ Increment 1: Core types adapted to `strs_tools::string::split` and `no_std` feature added.
*   Currently Working On:
    *   All steps for Increment 1 are complete.
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
(Increments 2-5 will need significant rework based on the new itemization approach. The parser will iterate `SplitIterator`, then classify each `Split` into `RichItem` with `UnilangTokenKind`, then process the stream of `RichItem`s. Comment and escape handling will need to be integrated into the parser logic.)

*   ⚫ **Increment 2: Implement Parser Entry Points and `RichItem` Stream Generation**
    *   (Plan to be revised: `parse_single_str` and `parse_slice` will use `SplitOptionsFormer::new(...).src(...).perform()`. The loop will take `Split<'a>`, classify it into `RichItem<'a> { inner, segment_idx, kind }`. Whitespace/comment `Split` items might need explicit filtering if not handled by `stripping` or if `SplitOptionsFormer` preserves them.)
*   ⚫ **Increment 3: Syntactic Analyzer - Command Grouping and Instruction Boundaries**
    *   (Plan to be revised: Will operate on `Vec<RichItem<'input>>`. Grouping by `RichItem` where `kind == UnilangTokenKind::Delimiter(";;".to_string())`.)
*   ⚫ **Increment 4: Syntactic Analyzer - Command Path and Help Operator Parsing**
    *   (Plan to be revised: Operates on `&[RichItem<'input>]`. Path from `UnilangTokenKind::Identifier` or `UnquotedValue`. Help from `UnilangTokenKind::Operator("?".to_string())`.)
*   ⚫ **Increment 5: Syntactic Analyzer - Argument Parsing (Named, Positional)**
    *   (Plan to be revised: Named args: `Identifier`/`UnquotedValue` -> `Delimiter("::".to_string())` -> `QuotedValue`/`UnquotedValue`. Unescaping is now parser's job.)

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
*   Increments 2-5 need substantial revision in their detailed steps once Increment 1 is complete and the classification mechanism is clearer.
