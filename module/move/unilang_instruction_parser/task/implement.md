# Task Plan: Refactor Parser for Robustness and Specification Adherence

### Goal
*   To refactor the `unilang_instruction_parser` to be more robust, maintainable, and strictly compliant with the parsing rules in `spec.md`. This involves simplifying the parser engine by improving the token classification layer and then implementing a correct state machine driven by specific, specification-based tests.

### Critique of Previous Plan & Codebase
*   **Architectural Contradiction:** The current `parser_engine.rs` implements a complex manual tokenizer, which contradicts the `spec.md` mandate to use `strs_tools` as the core tokenization engine. This adds unnecessary complexity and potential for bugs.
*   **Insufficient Abstraction:** The parser engine's state machine is not fully driven by the token `kind` from `item_adapter.rs`, often inspecting raw strings instead. This makes the logic less clear and harder to maintain.
*   **Vague Testing Strategy:** The previous plan lacked specific, failing test cases for each rule in the specification, making it difficult to verify full compliance.

### Ubiquitous Language (Vocabulary)
*   **`GenericInstruction`**: The primary output of the parser.
*   **`Command Path`**: The initial sequence of dot-separated identifiers that names the command.
*   **`RichItem` / `UnilangTokenKind`**: The classified token produced by `item_adapter.rs`. This should be the primary input for the parser's state machine.
*   **`spec.md`**: The canonical source of truth for parsing rules.

### Progress
*   **Roadmap Milestone:** N/A (Bug fix to unblock `unilang`'s M3.1)
*   **Primary Editable Crate:** `module/move/unilang_instruction_parser`
*   **Overall Progress:** 0/4 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Refactor Token Classification and Simplify Engine
    *   ⚫ Increment 2: Add Comprehensive, Failing Spec-Adherence Tests
    *   ⚫ Increment 3: Implement Correct Parser State Machine
    *   ⚫ Increment 4: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:** None

### Relevant Context
*   Control Files to Reference:
    *   `module/move/unilang/spec.md`
*   Files to Include:
    *   `src/parser_engine.rs`
    *   `src/item_adapter.rs`
    *   `tests/`

### Expected Behavior Rules / Specifications
*   The parser must correctly implement all rules in `spec.md`, Section 2.4 "Parsing Rules and Precedence".
*   **Rule 1 (Command Path):** The longest possible sequence of dot-separated identifiers at the start of an expression is the command path.
*   **Rule 2 (Transition to Args):** The path ends when a non-identifier/non-dot token is found (e.g., `::`, `?`, quoted string).
*   **Rule 3 (Dots):** Leading dots are ignored. Trailing dots on a command path are a syntax error.
*   **Rule 4 (Help):** `?` must be the final token.
*   All existing tests must continue to pass.

### Crate Conformance Check Procedure
*   Step 1: Execute `timeout 90 cargo test -p unilang_instruction_parser --all-targets` via `execute_command`.
*   Step 2: Analyze `execute_command` output. If it fails, initiate Critical Log Analysis.
*   Step 3: If tests pass, execute `timeout 90 cargo clippy -p unilang_instruction_parser -- -D warnings` via `execute_command`.
*   Step 4: Analyze `execute_command` output. If it fails, initiate Linter Fix & Regression Check Procedure.

### Increments

##### Increment 1: Refactor Token Classification and Simplify Engine
*   **Goal:** To simplify the parser by replacing the manual, error-prone tokenizer in `parser_engine.rs` with the architecturally-mandated `strs_tools` crate. This creates a clean, simple foundation for implementing the correct parsing logic.
*   **Rationale:** The current `tokenize_input` function contradicts the project's architecture (`spec.md` Section 1.1) and is a source of complexity. By delegating raw tokenization to `strs_tools` and refining the `item_adapter` to classify these tokens, we create a clean separation of concerns. The main parser engine will no longer deal with raw strings, only with classified `RichItem`s, making its logic much simpler and more robust.
*   **Steps:**
    1.  Use `read_file` to load the current content of `module/move/unilang_instruction_parser/src/parser_engine.rs` and `module/move/unilang_instruction_parser/src/item_adapter.rs`.
    2.  Refactor `item_adapter.rs::classify_split` function. Its new logic will be simpler. It will receive a `Split` item from `strs_tools`. It will first check if the `split.string` is a known operator (`::`, `?`) or delimiter (`.`). If not, it will check if the string starts and ends with a quote (`"` or `'`). If so, it classifies it as `UnilangTokenKind::QuotedValue`. Otherwise, it will be classified as an `Identifier` or `Unrecognized`.
    3.  Refactor `parser_engine.rs::tokenize_input` function. Delete the entire manual tokenization logic. The new function will:
        *   Define the full set of delimiters: `vec![ " ", ".", "::", "?" ]`.
        *   Use `strs_tools::string::split::split()` to create a `SplitOptionsFormer`.
        *   Configure the splitter with `.src(input)`, `.delimeter(...)`, `.preserving_delimeters(true)`, and crucially, `.quoting(false)`.
        *   Call `.perform()` to get the `SplitIterator`.
        *   Map each `Split` item from the iterator into a `RichItem` by calling the newly refactored `item_adapter::classify_split`.
        *   Return the `Vec<RichItem>`.
    4.  Use `write_to_file` to save the updated contents of `src/parser_engine.rs` and `src/item_adapter.rs`.
    5.  Perform Increment Verification.
*   **Increment Verification:**
    1.  Execute `timeout 90 cargo test -p unilang_instruction_parser --all-targets` via `execute_command`.
    2.  Analyze the output. All existing tests must pass. This refactoring should be behaviorally equivalent for the existing tests, but if any fail, they will be fixed before proceeding.
*   **Commit Message:** "refactor(parser): Simplify tokenization via item_adapter"

##### Increment 2: Add Comprehensive, Failing Spec-Adherence Tests
*   **Goal:** To create a new test suite that codifies the specific parsing rules from `spec.md`, Section 2.4. These tests are designed to fail with the current logic, proving its non-conformance and providing clear targets for the next increment.
*   **Rationale:** A test-driven approach is the most reliable way to ensure full compliance with a specification. By writing tests that fail first, we define the exact required behavior and can be confident the implementation is correct when the tests pass.
*   **Steps:**
    1.  Use `write_to_file` to create a new file at `module/move/unilang_instruction_parser/tests/spec_adherence_tests.rs`. The content will include:
        *   A `test_path_ends_at_quoted_string` function that parses `.command "arg"` and asserts the path is `["command"]` and that a positional argument `"arg"` was found.
        *   A `test_path_ends_at_named_argument_delimiter` function that parses `.command ::arg` and asserts the path is `["command"]` and that a named argument `arg` is being parsed.
        *   A `test_trailing_dot_is_error` function that parses `command.sub. arg` and asserts that it returns a `ParseError` with `ErrorKind::Syntax`.
        *   A `test_help_operator_must_be_final` function that parses `.command ? arg` and asserts it returns a `ParseError` with `ErrorKind::Syntax`.
    2.  Use `read_file` to get the content of `module/move/unilang_instruction_parser/tests/tests.rs`.
    3.  Use `insert_content` to add `mod spec_adherence_tests;` to `tests/tests.rs`.
    4.  Perform Increment Verification.
*   **Increment Verification:**
    1.  Execute `timeout 90 cargo test -p unilang_instruction_parser --test spec_adherence_tests` via `execute_command`.
    2.  Analyze the output. It is critical that these tests **fail**. The failure messages will confirm that the current parser logic does not adhere to the specification.
*   **Commit Message:** "test(parser): Add failing tests for spec adherence"

##### Increment 3: Implement Correct Parser State Machine
*   **Goal:** To modify the state machine in `src/parser_engine.rs` to correctly implement the specification rules, making the new tests pass.
*   **Rationale:** This is the core fix. With a simplified token stream from Increment 1 and clear failing tests from Increment 2, we can now implement the correct parsing logic with confidence.
*   **Steps:**
    1.  Use `read_file` to load `src/parser_engine.rs`.
    2.  Refactor the `parse_single_instruction_from_rich_items` function, focusing on the `while let Some(item) = ...` loop and the `match state` block for `ParserState::ParsingCommandPath`.
    3.  The decision-making logic must be driven by `item.kind` (`UnilangTokenKind`), not the raw string content.
    4.  If the state is `ParsingCommandPath` and the token `kind` is `Identifier` or `Delimiter(".")`, continue parsing the command path.
    5.  If the state is `ParsingCommandPath` and the token `kind` is `QuotedValue`, `Operator("::")`, or `Operator("?")`, the state must transition to `ParsingArguments`. The current `item` must then be re-processed by the argument parsing logic in the next loop iteration.
    6.  Add a check after the loop to handle a trailing dot on the command path, which should result in a `Syntax` error.
    7.  Use `write_to_file` to save the updated `src/parser_engine.rs`.
    8.  Perform Increment Verification.
*   **Increment Verification:**
    1.  Execute `timeout 90 cargo test -p unilang_instruction_parser --all-targets` via `execute_command`.
    2.  Analyze the output. All tests in the crate, including the new `spec_adherence_tests`, must now pass.
*   **Commit Message:** "fix(parser): Refactor engine to align with spec parsing rules"

##### Increment 4: Finalization
*   **Goal:** Perform a final, holistic review and verification of the entire task's output, ensuring all tests pass and the crate is clean.
*   **Rationale:** This final quality gate ensures that the fixes did not introduce any regressions and that the crate meets all project standards.
*   **Steps:**
    1.  Execute `timeout 90 cargo test -p unilang_instruction_parser --all-targets` via `execute_command`. Analyze the output to confirm all tests pass.
    2.  Execute `timeout 90 cargo clippy -p unilang_instruction_parser -- -D warnings` via `execute_command`. Analyze the output and fix any reported warnings.
    3.  Execute `git status` via `execute_command` to ensure there are no uncommitted changes.
    4.  Perform a self-critique of all changes against the plan's goal and the specification to confirm full compliance.
*   **Increment Verification:**
    1.  Execute the full `Crate Conformance Check Procedure`.
    2.  Execute `git status` via `execute_command` and confirm the output shows no uncommitted changes.
*   **Commit Message:** "chore(parser): Finalize spec adherence refactor"

### Changelog
*   [Initial] Plan created to refactor the parser to strictly adhere to the official specification.