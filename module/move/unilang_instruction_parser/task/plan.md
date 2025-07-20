# Task Plan: Stabilize `unilang_instruction_parser` Crate

### Goal
*   The primary goal of this task is to stabilize the `unilang_instruction_parser` crate by ensuring its parser engine is robust, clear, and adheres strictly to the Unilang specification (`spec.md`). This involves refactoring the parser, improving error handling, and achieving 100% test pass rate with comprehensive test coverage.

### Ubiquitous Language (Vocabulary)
*   **Unilang Instruction:** A single, parseable command in the Unilang language, consisting of a command path, arguments, and an optional help operator.
*   **Command Path:** A sequence of identifiers separated by dots (`.`), representing the hierarchical path to a command (e.g., `my.command.sub`).
*   **Argument:** A piece of data passed to a command, either positional (value only) or named (key::value).
*   **Help Operator (`?`):** A special operator indicating a request for help on a command, always appearing as the last token.
*   **RichItem:** An internal representation of a token (identifier, operator, delimiter) that includes its original string slice, its classified `UnilangTokenKind`, and its `SourceLocation`.
*   **SourceLocation:** A structure indicating the start and end byte indices of a token or instruction within the original input string.
*   **ParseError:** A custom error type used by the parser to report various parsing failures, including `ErrorKind` and `SourceLocation`.
*   **ErrorKind:** An enum within `ParseError` that categorizes the type of parsing failure (e.g., `Syntax`, `EmptyInstruction`, `TrailingDelimiter`).
*   **UnilangTokenKind:** An enum classifying the type of a token (e.g., `Identifier`, `Operator`, `Delimiter`, `Unrecognized`).
*   **Whitespace Separation:** The rule that whitespace acts as a separator between tokens, not part of the token's value unless explicitly quoted.
*   **Trailing Dot:** A syntax error where a command path ends with a dot (`.`).
*   **Empty Instruction Segment:** An error occurring when a segment between `;;` delimiters is empty or contains only whitespace.
*   **Trailing Delimiter:** An error occurring when the input ends with a `;;` delimiter.
*   **Fragile Test:** A test that is overly sensitive to unrelated changes in the production code, often leading to failures even when the core functionality under test remains correct.
*   **Default Value Equivalence Testing:** A specific and isolated type of testing designed to verify that a function or component behaves identically when a parameter is omitted (and its default value is used implicitly) and when that same parameter is provided explicitly with the default value.

### Progress
*   **Roadmap Milestone:** M1: Core API Implementation
*   **Primary Editable Crate:** `module/move/unilang_instruction_parser`
*   **Overall Progress:** 4/8 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Deep Integration with `strs_tools`
    *   ✅ Increment 2: Multi-Instruction Parsing and Error Handling
    *   ✅ Increment 3: Parser Engine Simplification and Refactoring
    *   ✅ Increment 4: Reintroduce Parser Engine Helper Functions
    *   ⏳ Increment 5: Address Doc Tests, Warnings, and Add Test Matrices
    *   ⚫ Increment 6: Comprehensive Test Coverage for `spec.md` Rules
    *   ⚫ Increment 7: Final Code Review and Documentation
    *   ⚫ Increment 8: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/strs_tools` (Reason: Direct dependency requiring modification for `unescape_str` functionality.)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/move/unilang_instruction_parser/src/parser_engine.rs`
    *   `module/move/unilang_instruction_parser/src/item_adapter.rs`
    *   `module/move/unilang_instruction_parser/src/error.rs`
    *   `module/move/unilang_instruction_parser/src/config.rs`
    *   `module/move/unilang_instruction_parser/tests/argument_parsing_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/command_parsing_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/comprehensive_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/error_reporting_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/parser_config_entry_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/spec_adherence_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/syntactic_analyzer_command_tests.rs`
    *   `module/move/unilang_instruction_parser/tests/temp_unescape_test.rs`
    *   `module/move/unilang_instruction_parser/tests/tests.rs`
    *   `module/core/strs_tools/src/string/split.rs`
    *   `module/move/unilang/spec.md`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `unilang_instruction_parser`
    *   `strs_tools`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   None

### Expected Behavior Rules / Specifications
*   **Rule 0: Whitespace Separation:** Whitespace (space, tab, newline, carriage return) acts as a separator between tokens. It is not part of the token's value unless the token is explicitly quoted. Multiple consecutive whitespace characters are treated as a single separator. Leading/trailing whitespace for the entire instruction is ignored.
*   **Rule 1: Command Path Identification:** The command path consists of one or more identifiers separated by the dot (`.`) delimiter. The command path ends when a non-identifier or non-dot token is encountered, or when the instruction ends.
*   **Rule 2: End of Command Path & Transition to Arguments:** The command path ends and arguments begin when:
    *   A token that is not an identifier or a dot is encountered (e.g., an operator like `::`, or a delimiter like `?`).
    *   A positional argument is encountered (an identifier not followed by `::`).
    *   The instruction ends.
*   **Rule 3: Dot (`.`) Operator Rules:**
    *   **3.1 Leading Dot:** An optional leading dot (`.`) at the very beginning of the instruction is consumed and does not form part of the command path. It signifies a root-level command.
    *   **3.2 Infix Dot:** Dots appearing between identifiers (e.g., `cmd.sub.action`) are consumed and act as path separators.
    *   **3.3 Trailing Dot:** A dot appearing at the end of the command path (e.g., `cmd.`, `cmd.sub.`) is a syntax error.
    *   **3.4 Consecutive Dots:** Multiple consecutive dots (e.g., `cmd..sub`) are a syntax error.
*   **Rule 4: Help Operator (`?`):** The question mark (`?`) acts as a help operator. It must be the final token in the instruction. It can be preceded by a command path and/or arguments. If any tokens follow `?`, it is a syntax error.
*   **Rule 5: Argument Types:**
    *   **5.1 Positional Arguments:** An identifier that is not part of the command path and is not followed by `::` is a positional argument.
    *   **5.2 Named Arguments:** An identifier followed by `::` and then a value (another identifier or quoted string) forms a named argument (e.g., `key::value`).
    *   **5.3 Positional After Named:** By default, positional arguments can appear after named arguments. This behavior can be configured via `UnilangParserOptions::error_on_positional_after_named`.
    *   **5.4 Duplicate Named Arguments:** By default, if a named argument is duplicated, the last one wins. This behavior can be configured via `UnilangParserOptions::error_on_duplicate_named_arguments`.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `sa1_1_root_namespace_list` | Fixed (Monitored) | Was failing with "Empty instruction" for input ".". Fixed by removing the problematic error check and adjusting overall location calculation. |
| `module/move/unilang_instruction_parser/src/lib.rs - (line 33)` | Failing (Stuck) | Doc test fails due to `expected item after doc comment`. This is because the `}` closing the `main` function in the doc test is followed by a doc comment, which is not allowed. |

### Crate Conformance Check Procedure
*   1.  **Run Tests:** For the `Primary Editable Crate` (`unilang_instruction_parser`) and `Additional Editable Crate` (`strs_tools`), execute `timeout 90 cargo test -p {crate_name} --all-targets`.
*   2.  **Run Doc Tests:** For the `Primary Editable Crate` (`unilang_instruction_parser`), execute `timeout 90 cargo test -p {crate_name} --doc`.
*   3.  **Analyze Test Output:** If any test command (unit, integration, or doc) fails, initiate the `Critical Log Analysis` procedure and resolve all test failures before proceeding.
*   4.  **Run Linter (Conditional):** Only if all tests in the previous step pass, for the `Primary Editable Crate` and `Additional Editable Crate`, execute `timeout 90 cargo clippy -p {crate_name} -- -D warnings`.
*   5.  **Analyze Linter Output:** If any linter command fails, initiate the `Linter Fix & Regression Check Procedure`.
*   6.  **Perform Output Cleanliness Check:** Execute `cargo clean -p {crate_name}` followed by `timeout 90 cargo build -p {crate_name}`. Critically analyze the build output for any unexpected debug prints from procedural macros. If any are found, the check fails; initiate the `Critical Log Analysis` procedure.

### Increments
(Note: The status of each increment is tracked in the `### Progress` section.)
##### Increment 1: Deep Integration with `strs_tools`
*   **Goal:** Integrate `strs_tools` for robust string splitting and unescaping, ensuring correct tokenization and handling of quoted strings and escape sequences. Address initial parsing issues related to whitespace and basic token classification.
*   **Specification Reference:** `spec.md` Rule 0 (Whitespace Separation), `spec.md` Rule 3.1 (Leading Dot).
*   **Steps:**
    *   Step 1: Read `module/move/unilang_instruction_parser/src/parser_engine.rs` and `module/move/unilang_instruction_parser/src/item_adapter.rs`.
    *   Step 2: Modify `parser_engine.rs` to use `strs_tools::split` for initial tokenization, ensuring `preserving_delimeters(true)`, `quoting(true)`, and `preserving_quoting(false)`.
    *   Step 3: Modify `item_adapter.rs` to classify `strs_tools::Split` items into `UnilangTokenKind` and adjust `SourceLocation` for quoted strings.
    *   Step 4: Add a temporary test file `module/move/unilang_instruction_parser/tests/temp_unescape_test.rs` to verify `strs_tools::unescape_str` correctly handles `\'`.
    *   Step 5: If `temp_unescape_test.rs` fails, modify `module/core/strs_tools/src/string/split.rs` to fix `unescape_str` for `\'`.
    *   Step 6: Update `parse_single_instruction_from_rich_items` in `parser_engine.rs` to correctly handle empty input (after filtering whitespace) by returning an empty `GenericInstruction`.
    *   Step 7: Update `parse_single_instruction_from_rich_items` to correctly consume a leading dot (`.`) as per `spec.md` Rule 3.1.
    *   Step 8: Perform Increment Verification.
    *   Step 9: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p unilang_instruction_parser --test temp_unescape_test` to confirm `strs_tools` unescaping fix.
    *   Run `timeout 90 cargo test -p unilang_instruction_parser --test parser_config_entry_tests` to verify basic parsing and empty input handling.
*   **Commit Message:** `feat(unilang_instruction_parser): Integrate strs_tools and fix basic parsing`

##### Increment 2: Multi-Instruction Parsing and Error Handling
*   **Goal:** Implement robust parsing for multiple instructions separated by `;;`, including comprehensive error handling for empty instruction segments and trailing delimiters. Refine existing error messages for clarity and consistency.
*   **Specification Reference:** `spec.md` (Implicit rule for multi-instruction parsing, explicit rules for error handling).
*   **Steps:**
    *   Step 1: Read `module/move/unilang_instruction_parser/src/parser_engine.rs` and `module/move/unilang_instruction_parser/src/error.rs`.
    *   Step 2: Implement `parse_multiple_instructions` in `parser_engine.rs` to split input by `;;` and parse each segment.
    *   Step 3: Add logic to `parse_multiple_instructions` to detect and return `ErrorKind::EmptyInstructionSegment` for consecutive `;;` or leading `;;`.
    *   Step 4: Add logic to `parse_multiple_instructions` to detect and return `ErrorKind::TrailingDelimiter` for input ending with `;;`.
    *   Step 5: Refine `ParseError`'s `Display` implementation in `error.rs` to ensure error messages are precise and consistent with test expectations.
    *   Step 6: Update `tests/syntactic_analyzer_command_tests.rs` and `tests/argument_parsing_tests.rs` to align test expectations with `spec.md` Rules 1, 2, and 4, specifically regarding command path parsing (space-separated segments are not part of the path) and the help operator (`?`). Remove or modify tests that contradict these rules.
    *   Step 7: Perform Increment Verification.
    *   Step 8: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p unilang_instruction_parser --test parser_config_entry_tests` to verify multi-instruction parsing and error handling.
    *   Run `timeout 90 cargo test -p unilang_instruction_parser --test syntactic_analyzer_command_tests` to verify command path and help operator parsing.
    *   Run `timeout 90 cargo test -p unilang_instruction_parser --test argument_parsing_tests` to verify argument parsing.
*   **Commit Message:** `feat(unilang_instruction_parser): Implement multi-instruction parsing and refine error handling`

##### Increment 3: Parser Engine Simplification and Refactoring
*   **Goal:** Refactor `src/parser_engine.rs` for simplicity, clarity, and maintainability, leveraging the safety provided by the now-passing test suite. This includes addressing the persistent "unexpected closing delimiter" error by reverting to a monolithic function and then carefully reintroducing helper functions.
*   **Specification Reference:** N/A (Internal refactoring).
*   **Steps:**
    *   Step 1: Revert `src/parser_engine.rs` to a monolithic `parse_single_instruction_from_rich_items` function, ensuring the `rich_items.is_empty()` check and corrected trailing dot location logic are present.
    *   Step 2: Perform Increment Verification (full test suite).
    *   Step 3: If tests pass, proceed to re-introduce helper functions in a new increment. If tests fail, initiate `Critical Log Analysis` and `Stuck Resolution Process`.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p unilang_instruction_parser --all-targets` to ensure the monolithic function compiles and all tests pass.
*   **Commit Message:** `refactor(unilang_instruction_parser): Revert parser_engine to monolithic for stability`

##### Increment 4: Reintroduce Parser Engine Helper Functions
*   **Goal:** Reintroduce helper functions into `src/parser_engine.rs` to simplify `parse_single_instruction_from_rich_items` while maintaining correctness and test pass rates.
*   **Specification Reference:** N/A (Internal refactoring).
*   **Steps:**
    *   Step 1: Read `module/move/unilang_instruction_parser/src/parser_engine.rs`.
    *   Step 2: Extract `parse_command_path` helper function from `parse_single_instruction_from_rich_items`.
    *   Step 3: Extract `parse_arguments` helper function from `parse_single_instruction_from_rich_items`.
    *   Step 4: Update `parse_single_instruction_from_rich_items` to use the new helper functions.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p unilang_instruction_parser --all-targets` to ensure the refactored code compiles and all tests pass.
*   **Commit Message:** `refactor(unilang_instruction_parser): Reintroduce parser engine helper functions`

##### Increment 5: Address Doc Tests, Warnings, and Add Test Matrices
*   **Goal:** Fix all failing doc tests, resolve all compiler warnings, and add a `Test Matrix` to each existing test file in `module/move/unilang_instruction_parser/tests/`.
*   **Specification Reference:** N/A (Code quality and documentation).
*   **Steps:**
    *   Step 1: Run `timeout 90 cargo test -p unilang_instruction_parser --doc` to identify failing doc tests.
    *   Step 2: Fix any failing doc tests in `src/lib.rs` or other relevant source files. This includes changing `//!` to `//` for code examples within doc tests and ensuring correct module paths (e.g., `crate::instruction::GenericInstruction`). Also, ensure inner attributes (`#![...]`) are at the top of the file, before any outer doc comments.
    *   Step 3: Run `timeout 90 cargo clippy -p unilang_instruction_parser -- -D warnings` to identify all warnings.
    *   Step 4: Resolve all compiler warnings in `src/` and `tests/` directories.
    *   Step 5: For each test file in `module/move/unilang_instruction_parser/tests/` (excluding `inc/mod.rs`), add a file-level doc comment containing a `Test Matrix` that lists the tests within that file and their purpose. If a test file already has a matrix, ensure it's up-to-date and correctly formatted.
    *   Step 6: Perform Increment Verification.
    *   Step 7: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p unilang_instruction_parser --doc` to confirm all doc tests pass.
    *   Run `timeout 90 cargo clippy -p unilang_instruction_parser -- -D warnings` to confirm no warnings.
    *   Manual review of test files to ensure Test Matrices are present and correctly formatted.
*   **Commit Message:** `fix(unilang_instruction_parser): Resolve doc test failures, warnings, and add test matrices`

##### Increment 6: Comprehensive Test Coverage for `spec.md` Rules
*   **Goal:** Ensure comprehensive test coverage for all rules defined in `spec.md`, especially those not fully covered by existing tests. This involves creating new tests in `tests/spec_adherence_tests.rs` based on a detailed `Test Matrix`.
*   **Specification Reference:** All rules in `spec.md`.
*   **Steps:**
    *   Step 1: Define a comprehensive `Test Matrix` for all `spec.md` rules, identifying test factors, combinations, and expected outcomes. This matrix will be added to the plan.
    *   Step 2: Create `tests/spec_adherence_tests.rs` and add tests based on the `Test Matrix`.
    *   Step 3: Implement any missing parser logic or fix bugs identified by the new tests.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p unilang_instruction_parser --test spec_adherence_tests` to verify new tests.
    *   Run `timeout 90 cargo test -p unilang_instruction_parser --all-targets` to ensure no regressions.
*   **Commit Message:** `test(unilang_instruction_parser): Add comprehensive spec.md adherence tests`

##### Increment 7: Final Code Review and Documentation
*   **Goal:** Conduct a thorough code review of the entire `unilang_instruction_parser` crate, ensuring adherence to all codestyle and design rules. Improve internal and external documentation.
*   **Specification Reference:** N/A (Code quality and documentation).
*   **Steps:**
    *   Step 1: Review all code for adherence to `codestyle.md` and `design.md` rules.
    *   Step 2: Add/improve doc comments for all public structs, enums, functions, and modules.
    *   Step 3: Ensure all `TODO`, `xxx`, `qqq` markers are addressed or annotated with `aaa` comments.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo clippy -p unilang_instruction_parser -- -D warnings` to check for linter warnings.
    *   Manual review of documentation for clarity and completeness.
*   **Commit Message:** `docs(unilang_instruction_parser): Improve documentation and code quality`

##### Increment 8: Finalization
*   **Goal:** Perform a final, holistic review and verification of the entire task's output, including a self-critique against all requirements and a full run of the Crate Conformance Check.
*   **Specification Reference:** N/A.
*   **Steps:**
    *   Step 1: Self-Critique: Review all changes against `Goal`, `Task Requirements`, `Project Requirements`.
    *   Step 2: Execute Test Quality and Coverage Evaluation.
    *   Step 3: Full Conformance Check: Run `Crate Conformance Check Procedure` on all `Editable Crates`.
    *   Step 4: Final Output Cleanliness Check.
    *   Step 5: Dependency Cleanup (if applicable).
    *   Step 6: Final Status Check: `git status`.
*   **Increment Verification:**
    *   All checks in the steps above must pass.
*   **Commit Message:** `chore(unilang_instruction_parser): Finalize task and verify all requirements`

### Task Requirements
*   The parser must correctly handle all syntax rules defined in `spec.md`.
*   Error messages must be clear, precise, and include `SourceLocation` where applicable.
*   The code must be well-documented and adhere to the provided `codestyle.md` and `design.md` rules.
*   Achieve 100% test pass rate for all automated tests.
*   All doc tests must pass.
*   All warnings must be handled.
*   Each test file must have a Test Matrix.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   Must use Rust 2021 edition.
*   All new APIs must be async. (Note: This task is for a parser, so this may not directly apply to all new functions, but the principle of async for I/O-bound operations should be followed if applicable).
*   All dependencies must be centralized in the workspace `Cargo.toml` and inherited by member crates.
*   Lint configurations must be defined centrally in the workspace `Cargo.toml` and inherited by member crates.

### Assumptions
*   The `strs_tools` crate is correctly integrated and its `unescape_str` function handles all necessary escape sequences (verified and fixed in Increment 1).
*   The `spec.md` document is the single source of truth for Unilang syntax rules.

### Out of Scope
*   Semantic analysis of Unilang instructions.
*   Execution of Unilang instructions.
*   Integration with external systems beyond `strs_tools`.

### External System Dependencies (Optional)
*   None

### Notes & Insights
*   The persistent "unexpected closing delimiter" error in `src/parser_engine.rs` suggests a deeper issue with file writing or an invisible character. Reverting to a monolithic function is a problem decomposition strategy to isolate the issue.

### Changelog
*   [Increment 1 | 2025-07-20 14:39 UTC] Integrated `strs_tools` for tokenization and unescaping. Fixed `strs_tools::unescape_str` to correctly handle `\'`. Updated `parse_single_instruction_from_rich_items` to handle empty input and leading dots.
*   [Increment 2 | 2025-07-20 14:39 UTC] Implemented `parse_multiple_instructions` with error handling for empty instruction segments and trailing delimiters. Refined `ParseError` display. Aligned test expectations in `syntactic_analyzer_command_tests.rs` and `argument_parsing_tests.rs` with `spec.md` rules.
*   [Increment 3 | 2025-07-20 14:46 UTC] Reverted `parser_engine.rs` to a monolithic function and fixed the "Empty instruction" error for input ".".
*   [Increment 4 | 2025-07-20 14:47 UTC] Reintroduced `parse_command_path` and `parse_arguments` helper functions into `parser_engine.rs`.
