# Project Plan: `unilang_instruction_parser` (Revised)

## Goal
*   Implement a parser in `unilang_instruction_parser` for `unilang` CLI syntax, leveraging `strs_tools::string::parser` for itemization.
*   Produce `Vec<GenericInstruction<'a>>` from `&str` or `&[&str]` input, adhering to `spec.md`.
*   Provide precise, location-aware error reporting using a custom `SourceLocation`.

## Relevant Context
*   **Target Crate:** `unilang_instruction_parser`
*   **Dependencies:** `strs_tools` (for itemization), `error_tools`, `iter_tools`.
*   `unilang/spec.md` (or equivalent spec for `unilang` grammar).
*   **Workspace:** Yes
*   **Module Structure:**
    *   `src/lib.rs`
    *   `src/instruction.rs` (`GenericInstruction`, `Argument`)
    *   `src/error.rs` (`ParseError`, `ErrorKind`, `SourceLocation`)
    *   `src/parser_engine.rs` (`Parser`, syntactic analysis logic)
    *   `src/config.rs` (for `UnilangParserOptions` wrapping `ItemizerOptions`)

### Expected Behavior Rules (Unilang Specific)
*   (E0-E10 from previous plan, with clarifications below)
*   **E1 Clarified:** `Argument::value` will store unescaped content as `Cow<'a, str>`.
*   **E4 Clarified:** Command path segments and argument names are derived from `strs_tools::Item.slice`.
*   **E5 Clarified:** `strs_tools::Itemizer` configured to discard whitespace/comment items. `unilang_instruction_parser` processes a clean stream of significant items. Unquoted values with spaces (single string input) become multiple `Item`s from `strs_tools`, which `unilang_instruction_parser` must then interpret (e.g., as a multi-part command path or a sequence of positional arguments).
*   **E9 Clarified:** `SourceLocation` enum (`StrSpan`, `SliceSegment`) used for error reporting.

## Increments

### Phase 1: Setup and Core Structures

*   ⚫ **Increment 1: Initialize Crate, Define Core Structures & Location Handling**
    *   Target Crate(s): `unilang_instruction_parser`
    *   Detailed Plan Step 1: Setup `Cargo.toml` with dependencies:
        *   `strs_tools = { workspace = true, features = ["string_parser"] }` (Verify feature name).
        *   `error_tools = { workspace = true, features = [ "enabled", "error_typed" ] }`.
        *   `iter_tools = { workspace = true, features = [ "enabled" ] }`.
    *   Detailed Plan Step 2: Create `src/error.rs`:
        *   Define `pub enum SourceLocation { StrSpan { start: usize, end: usize }, SliceSegment { segment_index: usize, start_in_segment: usize, end_in_segment: usize } }`. Add `Debug`, `PartialEq`, `Clone`.
        *   Define `pub enum ErrorKind { Itemization(strs_tools::string::parser::ErrorKind), Syntax(String), UnterminatedQuote, InvalidEscapeSequence }`.
        *   Define `pub struct ParseError { pub kind: ErrorKind, pub location: Option<SourceLocation> }`. Implement `Debug`, `std::error::Error`, `Display`.
        *   Implement `From<strs_tools::string::parser::ParseError>` for `ParseError` (will require mapping `strs_tools::Location` to a temporary/partial `SourceLocation` or deciding how to handle this translation globally).
    *   Detailed Plan Step 3: Create `src/instruction.rs`:
        *   Define `pub struct Argument<'a> { pub name_slice: Option<&'a str> /* raw name */, pub value: std::borrow::Cow<'a, str> /* unescaped */, pub name_location: Option<SourceLocation>, pub value_location: SourceLocation }`.
        *   Define `pub struct GenericInstruction<'a> { pub command_path_slices: Vec<&'a str>, pub named_arguments: std::collections::HashMap<&'a str, Argument<'a>>, pub positional_arguments: Vec<Argument<'a>>, pub help_requested: bool, pub overall_location: SourceLocation }`.
        *   Add `Debug`, `PartialEq` to both.
    *   Detailed Plan Step 4: Create `src/lib.rs`, `src/config.rs`, `src/parser_engine.rs` with basic module structure.
    *   Detailed Plan Step 5: Add `pub mod error; pub mod instruction; pub mod config; pub mod parser_engine;` to `src/lib.rs`. Re-export key types.
    *   Verification Strategy: `cargo build --package unilang_instruction_parser`. Manual review.
    *   Commit Message: `feat(unilang_parser): Define core structures, error, and location types`

### Phase 2: Parsing Engine Implementation

*   ⚫ **Increment 2: Implement Parser Configuration and Entry Points**
    *   Target Crate(s): `unilang_instruction_parser`
    *   Detailed Plan Step 1: In `src/config.rs`, define `pub struct UnilangParserOptions { pub itemizer_options: strs_tools::string::parser::ItemizerOptions<'static> }` (using `'static` for default delimiters/operators defined as consts).
    *   Detailed Plan Step 2: Implement `impl Default for UnilangParserOptions` which configures `itemizer_options` for `unilang` syntax:
        *   `quote_pairs: vec![("\"", "\""), ("'", "'")]`, `escape_char: Some('\\')`.
        *   `delimiters: vec!["::", ";;"]`, `operators: vec!["?"]`.
        *   `comment_prefix: Some("#")` (or as per unilang spec).
        *   `keep_whitespace_items: false`, `keep_comment_items: false`.
        *   `implicit_whitespace_delimit: true`.
    *   Detailed Plan Step 3: In `src/parser_engine.rs`, define `pub struct Parser { options: UnilangParserOptions }`.
    *   Detailed Plan Step 4: Implement `impl Parser { pub fn new(options: UnilangParserOptions) -> Self; ... }`.
    *   Detailed Plan Step 5: Implement `pub fn parse_single_str<'a>(&self, input: &'a str) -> Result<Vec<GenericInstruction<'a>>, ParseError>`.
        *   Create `strs_tools::string::parser::Itemizer::new(input, &self.options.itemizer_options)`.
        *   Call `itemize_all()`. Map `strs_tools::ParseError` to `unilang_instruction_parser::ParseError`, converting location to `SourceLocation::StrSpan`.
        *   Pass `Vec<strs_tools::string::parser::Item<'a>>` to `analyze_items_to_instructions`.
    *   Detailed Plan Step 6: Implement `pub fn parse_slice<'a>(&self, input_segments: &'a [&'a str]) -> Result<Vec<GenericInstruction<'a>>, ParseError>`.
        *   Initialize an empty `Vec<strs_tools::string::parser::Item<'a>>` for all items.
        *   Loop `input_segments` with index `seg_idx`:
            *   Itemize `segment_str` using `strs_tools::Itemizer`.
            *   For each `item` from `strs_tools`, create a new `strs_tools::Item` but replace its `item.location` (which is relative to `segment_str`) with a *temporary representation* or directly map to `unilang_instruction_parser::SourceLocation::SliceSegment { segment_index: seg_idx, start_in_segment: item.location.start, ... }` if you adapt `Item` or pass `seg_idx` around. *This is tricky. Simpler: `strs_tools::Item` remains as is. The `unilang_instruction_parser::ParseError` created during syntactic analysis will need to know which original segment an `Item` came from to build the final `SourceLocation`.*
            *   *Revised approach for `parse_slice` item location:* The `strs_tools::Item<'a>` will have locations relative to their individual segment. The `analyze_items_to_instructions` function will need to be aware of segment boundaries if it needs to report errors spanning multiple original segments, or the `Parser` will need to pass `seg_idx` to error creation. For now, assume `analyze_items_to_instructions` receives a flat `Vec<Item<'a>>` and error locations are based on these items' local spans. The final `ParseError` constructor will need `seg_idx` if the error is tied to an item from a slice.
            *   A simpler way for `parse_slice`: itemize each segment, then in `analyze_items_to_instructions`, if an error occurs with an `Item`, its original `item.location` (from `strs_tools`) is used along with the `segment_index` (which needs to be tracked alongside items from slices) to form the `SourceLocation::SliceSegment`.
        *   Pass the combined `Vec<Item<'a>>` (potentially with segment origin info) to `analyze_items_to_instructions`.
    *   Detailed Plan Step 7: Add basic tests for `parse_single_str` and `parse_slice` (empty input, single command name).
    *   Relevant Behavior Rules: E0, E9, E10.
    *   Verification Strategy: `cargo test --package unilang_instruction_parser`.
    *   Commit Message: `feat(unilang_parser): Impl parser config, entry points, and initial input handling`

*   ⚫ **Increment 3: Syntactic Analyzer - Command Structure (Path, Help, Command Separation)**
    *   Target Crate(s): `unilang_instruction_parser`
    *   Detailed Plan Step 1: In `parser_engine.rs`, implement `fn analyze_items_to_instructions<'input>(&self, items: Vec<strs_tools::string::parser::Item<'input>>, input_origin: InputOrigin /* enum { SingleStr, Slice(&'input [&'input str]) } */ ) -> Result<Vec<GenericInstruction<'input>>, ParseError>`. (InputOrigin helps map error locations).
        *   *Alternative for location*: Pass `seg_idx: Option<usize>` if processing items from a single segment of a slice, or handle location mapping when `ParseError` is constructed.
    *   Detailed Plan Step 2: Filter out `Whitespace` and `PotentialComment` items from `strs_tools`.
    *   Detailed Plan Step 3: Split the flat `items` list into sub-lists, where each sub-list represents one potential `GenericInstruction`. The separator is `Item { kind: Delimiter, slice: ";;" }`.
    *   Detailed Plan Step 4: For each sub-list of items:
        *   Parse command path: Consume leading `Identifier` or `UnquotedValue` items. Store their `slice`s. Record start/end `Item` for `overall_location`.
        *   Check for trailing `Item { kind: Operator, slice: "?" }` for `help_requested`.
        *   Store remaining items for argument parsing.
    *   Relevant Behavior Rules: E2 (`;;`, `?`), E4, E5.
    *   Verification Strategy: `cargo test --package unilang_instruction_parser` for command paths, help.
    *   Commit Message: `feat(unilang_parser): Parse command paths, help operator, and command separation`

*   ⚫ **Increment 4: Syntactic Analyzer - Argument Parsing (Named, Positional)**
    *   Target Crate(s): `unilang_instruction_parser`
    *   Detailed Plan Step 1: Within the loop for each command's items (after path/help):
        *   **Named Arguments:** Look for `Identifier`|`UnquotedValue` (name) -> `Delimiter("::")` -> `QuotedValue`|`UnquotedValue` (value).
            *   Use `item.unescaped_value()` for the value, store as `Cow<'a, str>` in `Argument`.
            *   Store `name.slice` and locations.
        *   **Positional Arguments:** Other `QuotedValue`|`UnquotedValue` items.
            *   Use `item.unescaped_value()`. Store locations.
        *   Handle errors for malformed named args (e.g., name without `::` or value).
    *   Relevant Behavior Rules: E1, E2 (`::`), E3.
    *   Verification Strategy: `cargo test --package unilang_instruction_parser` for arguments.
    *   Commit Message: `feat(unilang_parser): Implement named and positional argument parsing`

### Phase 3: Refinements and Testing

*   ⚫ **Increment 5: Error Reporting and `SourceLocation` Integration**
    *   Target Crate(s): `unilang_instruction_parser`
    *   Detailed Plan Step 1: Ensure all paths in `analyze_items_to_instructions` that generate `ParseError` correctly populate `ParseError::location` with a `SourceLocation`.
        *   If processing items from `parse_single_str`, use `SourceLocation::StrSpan` based on `item.location`.
        *   If processing items from `parse_slice`, this is where the `segment_index` associated with the failing `item` is crucial to construct `SourceLocation::SliceSegment`. The `analyze_items_to_instructions` might need to receive items as `Vec<(Item<'input>, Option<usize>/*seg_idx*/)>` or the `Parser` needs a way to map a global item index back to its original segment if `parse_slice` flattens everything.
        *   *Decision for Slice Location:* `parse_slice` should probably not flatten items immediately. It could call `analyze_items_to_instructions` per segment, or `analyze_items_to_instructions` needs to be more aware. A simpler start: `parse_slice` itemizes segment by segment. If an itemization error occurs within a segment, its location is already relative. If a syntactic error occurs later with items from a slice, the `Item` itself should carry enough info (or be wrappable) to trace back to its original segment_index and its local location.
        *   *Revised approach for Slice Location in `analyze_items_to_instructions`*: The `Item` struct from `strs_tools` only has `start/end` byte offsets. When `parse_slice` calls `itemize_all` on each segment, it gets `Item`s whose locations are relative to *that segment*. `parse_slice` must then transform these `Item`s (or wrap them) to include the `segment_index` before passing them to a flattened analysis stage, OR the analysis must happen per-segment and results aggregated.
        *   **Let's simplify:** `analyze_items_to_instructions` takes `items: Vec<strs_tools::string::parser::Item<'input>>` and `segment_index: Option<usize>`. `parse_single_str` calls it with `None`. `parse_slice` calls it for *each segment's items* with `Some(seg_idx)`. This means `analyze_items_to_instructions` might produce partial `GenericInstruction`s if a unilang command spans multiple shell arguments, which then need to be stitched together. This is getting complex.
        *   **Alternative for `parse_slice`:** Concatenate all string segments from the slice into one temporary owned `String` (with a special, non-printable separator if needed to map locations back accurately, or by tracking original segment lengths). Then parse this single string. This simplifies location tracking to always be `StrSpan` but introduces an allocation and copying.
        *   **Chosen Path (Compromise):** `parse_slice` will itemize each segment. The `Vec<Item<'a>>` passed to `analyze_items_to_instructions` will be flat. Each `Item` needs to be augmented or wrapped to carry its original `segment_idx`.
            ```rust
            // In unilang_instruction_parser, perhaps in input_adapter.rs or alongside Item
            struct RichItem<'a> {
                inner: strs_tools::string::parser::Item<'a>,
                segment_idx: Option<usize>, // None for single_str input
            }
            ```
            `analyze_items_to_instructions` works on `Vec<RichItem<'a>>`.
    *   Verification Strategy: Tests for errors in both input modes, checking `ParseError.location`.
    *   Commit Message: `fix(unilang_parser): Integrate SourceLocation for precise error reporting`

*   ⚫ **Increment 6: Comprehensive Test Suite (Test Matrix)**
    *   (As per previous plan: cover input types, command structures, arg types, value types, delimiters, operators, quoting, errors, edge cases).
    *   Verification Strategy: `cargo test --package unilang_instruction_parser --all-features`.
    *   Commit Message: `test(unilang_parser): Implement comprehensive test suite`

*   ⚫ **Increment 7: Documentation and Examples**
    *   (As per previous plan: crate-level, public API docs, example file).
    *   Verification Strategy: Manual review, `cargo test --doc --package unilang_instruction_parser`.
    *   Commit Message: `docs(unilang_parser): Add documentation and examples`

## Requirements (for `unilang_instruction_parser` - Expanded)
*   **R1: Dependency on `strs_tools::string::parser`:** Must use the itemizer from `strs_tools`.
*   **R2: Unilang Specific Syntax:** Syntactic analyzer implements `unilang` grammar from spec.
*   **R3: Dual Input Handling & Abstraction:** Public API supports `&str` and `&[&str]`. Internal logic must correctly map locations for both.
*   **R4: Value Unescaping:** Argument values in `GenericInstruction` must be unescaped, likely using `Cow<'a, str>`.
*   **R5: Precise Location-Aware Errors:** `ParseError` uses `SourceLocation` (distinguishing `StrSpan` and `SliceSegment`).
*   **R6: No Command Definitions Dependency:** Purely syntactic.
*   **R7: Comprehensive Test Coverage:** Including Test Matrix for various scenarios.
*   **R8: Adherence to Workspace Rules:** Standard project cargo command rules.
*   **R9: API Clarity:** Public API of `unilang_instruction_parser` is clear.
*   **R10: Correct `ItemizerOptions` Configuration:** `Parser::new()` must correctly configure `strs_tools::ItemizerOptions` for `unilang`'s specific lexemes (quotes, escapes, delimiters, operators, comments).
*   **R11: Handling of `strs_tools` Items:** The syntactic analyzer must correctly interpret the stream of `strs_tools::Item`s, typically ignoring `Whitespace` and `PotentialComment` kinds.
*   **R12: Lifetime Management:** All `&'a str` and `Cow<'a, str>` in output structures must correctly borrow from the original input.
*   **R13: Error Propagation:** Errors from `strs_tools::Itemizer` must be cleanly converted and propagated as `unilang_instruction_parser::ParseError`.

## Notes & Insights
*   The `strs_tools::string::parser::Item` struct should ideally contain `kind: ItemKind` where `ItemKind` itself can store the matched delimiter/operator string (e.g., `Delimiter(&'static str)`), making the `unilang_parser`'s job easier. This was noted for the `strs_tools` plan.
*   The most complex part of this new plan is handling `SourceLocation` correctly, especially when itemizing `&[&str]` and then performing syntactic analysis on a potentially flattened list of `RichItem`s. The `RichItem` wrapper approach seems like a good way to associate `segment_idx` with items originating from slices.
*   The decision for `Argument::value` to be `Cow<'a, str>` (unescaped) is a good balance for correctness and performance.

This revised plan for `unilang_instruction_parser` is more detailed about its interaction with `strs_tools` and the challenges of dual input source location tracking.