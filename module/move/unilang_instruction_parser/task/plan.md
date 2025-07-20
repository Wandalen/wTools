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
*   **Whitespace Separation:** The rule that whitespace acts as a separator between tokens, not part of the token's value unless the token is explicitly quoted.
*   **Trailing Dot:** A syntax error where a command path ends with a dot (`.`).
*   **Empty Instruction Segment:** An error occurring when a segment between `;;` delimiters is empty or contains only whitespace.
*   **Trailing Delimiter:** An error occurring when the input ends with a `;;` delimiter.
*   **Fragile Test:** A test that is overly sensitive to unrelated changes in the production code, often leading to failures even when the core functionality under test remains correct.
*   **Default Value Equivalence Testing:** A specific and isolated type of testing designed to verify that a function or component behaves identically when a parameter is omitted (and its default value is used implicitly) and when that same parameter is provided explicitly with the default value.

### Progress
*   **Roadmap Milestone:** M1: Core API Implementation
*   **Primary Editable Crate:** `module/move/unilang_instruction_parser`
*   **Overall Progress:** 8/10 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Deep Integration with `strs_tools`
    *   ✅ Increment 2: Multi-Instruction Parsing and Error Handling
    *   ✅ Increment 3: Parser Engine Simplification and Refactoring
    *   ✅ Increment 4: Reintroduce Parser Engine Helper Functions
    *   ✅ Increment 5: Address Doc Tests, Warnings, and Add Test Matrices
    *   ✅ Increment 5.1: Focused Debugging: Fix `strs_tools` compilation error
    *   ✅ Increment 5.2: External Crate Change Proposal: `strs_tools` `Split::was_quoted`
    *   ✅ Increment 6: Comprehensive Test Coverage for `spec.md` Rules
    *   ✅ Increment 6.1: Focused Debugging: Fix `s6_21_transition_by_non_identifier_token`
    *   ✅ Increment 7: Patch `strs_tools` and Fix Stuck Tests
    *   ✅ Increment 7.1: Focused Debugging: Fix `strs_tools` `Split` struct initialization errors
    *   ⚫ Increment 8: Final Code Review and Documentation
    *   ⚫ Increment 9: Finalization

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
    *   `module/core/strs_tools` (Reason: Need `Split::was_quoted` field for `spec.md` Rule 2. Proposal: `module/core/strs_tools/task.md`)

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
| `module/move/unilang_instruction_parser/src/lib.rs - (line 33)` | Fixed (Monitored) | Doc test fails due to `expected item after doc comment`. Fixed by correcting malformed doc comment. |
| `module/core/strs_tools/tests/smoke_test::debug_strs_tools_trailing_semicolon_space` | Fixed (Monitored) | Was failing because `strs_tools::string::split` produced an extra empty split at the end when there was trailing whitespace after a delimiter. Compilation also failed with `expected `{` after struct name, found keyword `let`ls` due to incorrect insertion of `let skip = ...` into `SplitOptions`'s `where` clause. Fixed by removing the misplaced code and re-inserting it correctly into `SplitIterator::next` after the `STRIPPING` logic. |
| `s6_16_duplicate_named_arg_last_wins` | Fixed (Monitored) | Parser returned error for duplicate named arguments. Fixed by setting `error_on_duplicate_named_arguments` to `false` by default in `UnilangParserOptions`. |
| `s6_21_transition_by_non_identifier_token` | Fixed (Monitored) | Parser was treating `!` as part of the command path. Fixed by making `parse_command_path` `break` on `Unrecognized` tokens, and reverting `parse_arguments` to only accept `Identifier` for positional arguments. |
| `s6_28_command_path_invalid_identifier_segment` | Fixed (Monitored) | Parser was treating `123` as a valid command path segment. Fixed by updating `is_valid_identifier` to disallow starting with a digit, and making `parse_command_path` return `Invalid identifier` error for `Unrecognized` tokens after a dot. |
| `s6_7_consecutive_dots_syntax_error` | Fixed (Monitored) | Error message mismatch. Fixed by updating the error message in `parser_engine.rs`. |
| `s6_13_named_arg_quoted_value_with_spaces` | Fixed (Monitored) | Parser failed to parse quoted named argument value. Fixed by allowing `Unrecognized` tokens as named argument values in `parser_engine.rs`. |
| `s6_24_named_arg_value_with_double_colon` | Fixed (Monitored) | Parser failed to parse named argument value with `::`. Fixed by allowing `Unrecognized` tokens as named argument values in `parser_engine.rs`. |
| `s6_25_named_arg_value_with_commas` | Fixed (Monitored) | Parser failed to parse named argument value with commas. Fixed by allowing `Unrecognized` tokens as named argument values in `parser_engine.rs`. |
| `s6_26_named_arg_value_with_key_value_pair` | Fixed (Monitored) | Parser failed to parse named argument value with key-value pairs. Fixed by allowing `Unrecognized` tokens as named argument values in `parser_engine.rs`. |
| `s6_2_whitespace_in_quoted_positional_arg` | Fixed (Monitored) | Parser returns `Unexpected token 'val with spaces'` for a quoted positional argument. This is because `parse_arguments` is not correctly handling `Unrecognized` tokens for positional arguments, and `item_adapter` cannot distinguish quoted strings from invalid identifiers without `strs_tools::Split::was_quoted`. This test requires the `strs_tools` change proposal to be implemented. |
| `tm2_11_named_arg_with_comma_separated_value` | Fixed (Monitored) | Parser failed to parse named argument value with commas. Fixed by allowing `Unrecognized` tokens as named argument values in `parser_engine.rs`. |
| `tm2_12_named_arg_with_key_value_pair_string` | Fixed (Monitored) | Parser failed to parse named argument value with key-value pairs. Fixed by allowing `Unrecognized` tokens as named argument values in `parser_engine.rs`. |
| `tm2_8_named_arg_with_simple_quoted_value` | Fixed (Monitored) | Parser failed to parse simple quoted named argument value. Fixed by allowing `Unrecognized` tokens as named argument values in `parser_engine.rs`. |
| `tm2_9_named_arg_with_quoted_value_containing_double_colon` | Fixed (Monitored) | Parser failed to parse named argument value with `::`. Fixed by allowing `Unrecognized` tokens as named argument values in `parser_engine.rs`. |
| `positional_arg_with_quoted_escaped_value_location` | Fixed (Monitored) | Parser returns `Unexpected token 'a\b"c'd\ne\tf'` for a quoted positional argument. This is because `parse_arguments` is not correctly handling `Unrecognized` tokens for positional arguments, and `item_adapter` cannot distinguish quoted strings from invalid identifiers without `strs_tools::Split::was_quoted`. This test requires the `strs_tools` change proposal to be implemented. |
| `unescaping_works_for_positional_arg_value` | Fixed (Monitored) | Parser returns `Unexpected token 'a\b"c'd\ne\tf'` for a quoted positional argument. This is because `parse_arguments` is not correctly handling `Unrecognized` tokens for positional arguments, and `item_adapter` cannot distinguish quoted strings from invalid identifiers without `strs_tools::Split::was_quoted`. This test requires the `strs_tools` change proposal to be implemented. |

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
    *   Step 1: Read `module/move/unilang_instruction_parser/src/parser_engine.rs`.
    *   Step 2: Modify `parser_engine.rs` to use `strs_tools::split` for initial tokenization, ensuring `preserving_delimeters(true)`, `quoting(true)`, and `preserving_quoting(false)`.
    *   Step 3: Modify `item_adapter.rs` to classify `strs_tools::Split` items into `UnilangTokenKind` and adjust `SourceLocation` for quoted strings.
    *   Step 4: Add a temporary test file `module/move/unilang_instruction_parser/tests/temp_unescape_test.rs` to verify `strs_tools::unescape_str` correctly handles `\'`.
    *   Step 5: If `temp_unescape_test.rs` fails, modify `module/core/strs_tools/src/string/split.rs` to fix `unescape_str` for `\'`.
    *   Step 6: Update `parse_single_instruction_from_rich_items` in `parser_engine.rs` to correctly handle empty input (after filtering whitespace) by returning an empty `GenericInstruction`.
    *   Step 7: Update `parse_single_instruction_from_rich_items` to correctly consume a leading dot (`.`) as per `spec.md` Rule 3.1.
    *   Step 8: Perform Increment Verification.
    *   Step 9: Perform Crate Conformance Check.
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
*   **Commit Message:** `feat(unilang_instruction_parser): Implement multi-instruction parsing and refine error handling`

##### Increment 3: Parser Engine Simplification and Refactoring
*   **Goal:** Refactor `src/parser_engine.rs` for simplicity, clarity, and maintainability, leveraging the safety provided by the now-passing test suite. This includes addressing the persistent "unexpected closing delimiter" error by reverting to a monolithic function and then carefully reintroducing helper functions.
*   **Specification Reference:** N/A (Internal refactoring).
*   **Steps:**
    *   Step 1: Revert `src/parser_engine.rs` to a monolithic `parse_single_instruction_from_rich_items` function, ensuring the `rich_items.is_empty()` check and corrected trailing dot location logic are present.
    *   Step 2: Perform Increment Verification (full test suite).
    *   Step 3: If tests pass, proceed to re-introduce helper functions in a new increment. If tests fail, initiate `Critical Log Analysis` and `Stuck Resolution Process`.
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
*   **Commit Message:** `fix(unilang_instruction_parser): Resolve doc test failures, warnings, and add test matrices`

##### Increment 5.1: Focused Debugging: Fix `strs_tools` compilation error
*   **Goal:** Diagnose and fix the `Failing (Stuck)` test: `module/core/strs_tools/tests/smoke_test::debug_strs_tools_trailing_semicolon_space` and the associated compilation error.
*   **Specification Reference:** N/A.
*   **Steps:**
    *   Step A: Apply Problem Decomposition. The problem is a compilation error, which is blocking the test fix. The immediate problem is the compiler error `expected `{` after struct name, found keyword `let`ls` at line 518.
    *   Step B: Isolate the test case. The test case is `debug_strs_tools_trailing_semicolon_space` in `module/core/strs_tools/tests/smoke_test.rs`. The compilation error is in `module/core/strs_tools/src/string/split.rs`.
    *   Step C: Add targeted debug logging. (Not directly applicable for compilation errors, but will keep in mind for runtime issues).
    *   Step D: Review related code changes since the test last passed. The last change was moving the `skip` logic.
    *   Step E: Formulate and test a hypothesis. The hypothesis is that the compiler is getting confused by the placement of the `let skip = ...` statement, even though it appears syntactically correct within the `next` function. This might be due to some subtle interaction with the `loop` or `match` statements, or a compiler bug/state issue.
    *   Step F: Revert the last change to `split.rs` (already done).
    *   Step G: Re-insert the `skip` logic, but this time, I will try to simplify the `if current_split.typ == SplitType::Delimiter` block to see if that helps the compiler. If not, I will try to move the `let skip = ...` to a separate helper function or a different scope within `next`.
    *   Step H: Upon successful fix, document the root cause and solution in the `### Notes & Insights` section.
*   **Commit Message:** `fix(strs_tools): Resolve stuck test module/core/strs_tools/tests/smoke_test::debug_strs_tools_trailing_semicolon_space`

##### Increment 5.2: External Crate Change Proposal: `strs_tools` `Split::was_quoted`
*   **Goal:** Create a formal change proposal (`task.md`) for the `strs_tools` crate to add a `was_quoted: bool` field to its `Split` struct. This is necessary for `unilang_instruction_parser` to correctly implement `spec.md` Rule 2 regarding quoted strings.
*   **Specification Reference:** `spec.md` Rule 2.
*   **Steps:**
    *   Step 1: Create `module/core/strs_tools/task.md` with the detailed change proposal, including problem statement, proposed solution (API changes, behavioral changes, internal changes), expected behavior, acceptance criteria, and potential impact.
*   **Commit Message:** `chore(unilang_instruction_parser): Propose strs_tools Split::was_quoted field`

##### Increment 6: Comprehensive Test Coverage for `spec.md` Rules
*   **Goal:** Ensure comprehensive test coverage for all rules defined in `spec.md`, especially those not fully covered by existing tests. This involves creating new tests in `tests/spec_adherence_tests.rs` based on a detailed `Test Matrix`.
*   **Specification Reference:** All rules in `spec.md`.
*   **Steps:**
    *   Step 1: Define a comprehensive `Test Matrix` for all `spec.md` rules, identifying test factors, combinations, and expected outcomes. This matrix will be added to the plan.
    *   Step 2: Create `tests/spec_adherence_tests.rs` and add tests based on the `Test Matrix`.
    *   Step 3: Implement any missing parser logic or fix bugs identified by the new tests.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Commit Message:** `test(unilang_instruction_parser): Add comprehensive spec.md adherence tests`

##### Increment 6.1: Focused Debugging: Fix `s6_21_transition_by_non_identifier_token`
*   **Goal:** Diagnose and fix the `Failing (Stuck)` test: `s6_21_transition_by_non_identifier_token`.
*   **Specification Reference:** N/A.
*   **Steps:**
    *   Step A: Apply Problem Decomposition. The problem is that `parse_command_path` is not correctly handling `Unrecognized` tokens, leading to an incorrect error or behavior.
    *   Step B: Isolate the test case. The test is `s6_21_transition_by_non_identifier_token` in `tests/spec_adherence_tests.rs`.
    *   Step C: Add targeted debug logging. I will add `println!` statements in `item_adapter::classify_split`, `parser_engine::parse_single_instruction`, and `parser_engine::parse_command_path` to trace the `item.kind` and the flow.
    *   Step D: Review related code changes since the test last passed. The test has never passed with the expected behavior. The relevant changes are in `item_adapter.rs` (identifier validation) and `parser_engine.rs` (handling `Unrecognized` in `parse_command_path`).
    *   Step E: Formulate and test a hypothesis. The hypothesis is that `parse_command_path` is not correctly breaking on `Unrecognized` tokens, or that `item_adapter` is not classifying `!` as `Unrecognized` in a way that `parse_command_path` expects.
    *   Step F: Upon successful fix, document the root cause and solution in the `### Notes & Insights` section.
*   **Commit Message:** `fix(unilang_instruction_parser): Resolve stuck test s6_21_transition_by_non_identifier_token`

##### Increment 7: Patch `strs_tools` and Fix Stuck Tests
*   **Goal:** To unblock the `Failing (Stuck)` tests by locally patching the `strs_tools` crate with the proposed `was_quoted` feature, and then implementing the necessary logic in `unilang_instruction_parser` to fix the tests.
*   **Specification Reference:** `spec.md` Rule 2.
*   **Steps:**
    *   Step 1: Read `module/core/strs_tools/src/string/split.rs` and `module/move/unilang_instruction_parser/src/item_adapter.rs`.
    *   Step 2: In `module/core/strs_tools/src/string/split.rs`, modify the `Split` struct to include `pub was_quoted : bool,`.
    *   Step 3: In the `SplitIterator::next` method within `split.rs`, track when a split is generated from a quoted string and set the `was_quoted` field to `true` on the returned `Split` instance. For all other cases, set it to `false`.
    *   Step 4: In `module/move/unilang_instruction_parser/src/item_adapter.rs`, modify the `classify_split` function. Add a condition to check `if split.was_quoted`. If it is `true`, classify the token as `UnilangTokenKind::Identifier`, regardless of its content. This ensures quoted strings are treated as single identifiers.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo test -p unilang_instruction_parser --test spec_adherence_tests -- --exact s6_2_whitespace_in_quoted_positional_arg` and analyze the output for success.
    *   Step 2: Execute `timeout 90 cargo test -p unilang_instruction_parser --test argument_parsing_tests -- --exact positional_arg_with_quoted_escaped_value_location` and analyze the output for success.
    *   Step 3: Execute `timeout 90 cargo test -p unilang_instruction_parser --test temp_unescape_test -- --exact unescaping_works_for_positional_arg_value` and analyze the output for success.
    *   Step 4: If all tests pass, the verification is successful.
*   **Commit Message:** `fix(parser): Implement was_quoted in strs_tools and fix quoted argument parsing`

##### Increment 7.1: Focused Debugging: Fix `strs_tools` `Split` struct initialization errors
*   **Goal:** Diagnose and fix the `Failing (Stuck)` compilation errors in `module/core/strs_tools/src/string/split.rs` related to missing `was_quoted` field initializations.
*   **Specification Reference:** N/A.
*   **Steps:**
    *   Step A: Apply Problem Decomposition. The problem is a compilation error due to missing field initializations. I need to find all `Split` struct instantiations and add `was_quoted: false` to them.
    *   Step B: Isolate the problem. The problem is in `module/core/strs_tools/src/string/split.rs`.
    *   Step C: Read `module/core/strs_tools/src/string/split.rs` to get the latest content.
    *   Step D: Search for all instances of `Split { ... }` and ensure `was_quoted: false` is present.
    *   Step E: Apply `search_and_replace` for any missing initializations.
    *   Step F: Perform Increment Verification.
    *   Step G: Upon successful fix, document the root cause and solution in the `### Notes & Insights` section.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo build -p strs_tools` and analyze the output for success (no compilation errors).
    *   Step 2: Execute `timeout 90 cargo test -p strs_tools --all-targets` and analyze the output for success.
*   **Commit Message:** `fix(strs_tools): Resolve Split struct initialization errors`

##### Increment 8: Final Code Review and Documentation
*   **Goal:** Conduct a thorough code review of the entire `unilang_instruction_parser` crate, ensuring adherence to all codestyle and design rules. Improve internal and external documentation.
*   **Specification Reference:** N/A (Code quality and documentation).
*   **Steps:**
    *   Step 1: Review all code for adherence to `codestyle.md` and `design.md` rules.
    *   Step 2: Add/improve doc comments for all public structs, enums, functions, and modules.
    *   Step 3: Ensure all `TODO`, `xxx`, `qqq` markers are addressed or annotated with `aaa` comments.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Commit Message:** `docs(unilang_instruction_parser): Improve documentation and code quality`

##### Increment 9: Finalization
*   **Goal:** Perform a final, holistic review and verification of the entire task's output, including a self-critique against all requirements and a full run of the `Crate Conformance Check`.
*   **Specification Reference:** N/A.
*   **Steps:**
    *   Step 1: Self-Critique: Review all changes against `Goal`, `Task Requirements`, `Project Requirements`.
    *   Step 2: Execute Test Quality and Coverage Evaluation.
    *   Step 3: Full Conformance Check: Run `Crate Conformance Check Procedure` on all `Editable Crates`.
    *   Step 4: Final Output Cleanliness Check.
    *   Step 5: Dependency Cleanup: Since `strs_tools` was directly modified as an editable crate, no `[patch]` section needs to be reverted. This step is complete.
    *   Step 6: Final Status Check: `git status`.
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
*   **[Increment 5.1 | 2025-07-20 19:17 UTC]** The `let skip = ...` compilation error in `strs_tools/src/string/split.rs` at line 518 is a persistent and unusual syntax error, suggesting a deeper compiler issue or corrupted file state. This was due to the `let skip = ...` statement being incorrectly inserted into the `where` clause of `SplitOptions` instead of the `next` function of `SplitIterator`.
*   **[Increment 5.1 | 2025-07-20 19:19 UTC]** The `module/core/strs_tools/tests/smoke_test::debug_strs_tools_trailing_semicolon_space` test was failing because `strs_tools::string::split` produced an extra empty split at the end when there was trailing whitespace after a delimiter, and the `STRIPPING` logic was applied before the `skip` logic. The fix involved moving the `skip` logic to *after* the `STRIPPING` logic in `SplitIterator::next`, ensuring that empty strings resulting from stripping are correctly skipped if `PRESERVING_EMPTY` is false.
*   **[Increment 6.1 | 2025-07-20 19:34 UTC]** The `s6_21_transition_by_non_identifier_token` test was failing because `parse_command_path` was incorrectly returning an `Invalid identifier` error for `Unrecognized` tokens (like `!`). The fix involved making `parse_command_path` `break` on `Unrecognized` tokens, and reverting `parse_arguments` to only accept `Identifier` for positional arguments.
*   **[Increment 6.1 | 2025-07-20 19:34 UTC]** The `s6_28_command_path_invalid_identifier_segment` test was failing because `is_valid_identifier` was not correctly disallowing identifiers starting with digits, and `parse_command_path` was not handling `Unrecognized` tokens after a dot correctly. The fix involved updating `is_valid_identifier` to disallow starting with a digit, and making `parse_command_path` return `Invalid identifier` error for `Unrecognized` tokens after a dot.

### Changelog
*   [Increment 1 | 2025-07-20 14:39 UTC] Integrated `strs_tools` for tokenization and unescaping. Fixed `strs_tools::unescape_str` to correctly handle `\'`. Updated `parse_single_instruction_from_rich_items` to handle empty input and leading dots.
*   [Increment 2 | 2025-07-20 14:39 UTC] Implemented `parse_multiple_instructions` with error handling for empty instruction segments and trailing delimiters. Refined `ParseError` display. Aligned test expectations in `syntactic_analyzer_command_tests.rs` and `argument_parsing_tests.rs` with `spec.md` rules.
*   [Increment 3 | 2025-07-20 14:46 UTC] Reverted `parser_engine.rs` to a monolithic function and fixed the "Empty instruction" error for input ".".
*   [Increment 4 | 2025-07-20 14:47 UTC] Reintroduced `parse_command_path` and `parse_arguments` helper functions into `parser_engine.rs`.
*   [Increment 5 | 2025-07-20 17:38 UTC] Addressed doc tests, resolved warnings, and added test matrices to all test files.
*   [Increment 5.1 | 2025-07-20 19:19 UTC] Resolved compilation error and fixed `strs_tools` trailing semicolon space test.
*   [Increment 5.2 | 2025-07-20 19:28 UTC] Created change proposal for `strs_tools` to add `Split::was_quoted` field.
*   [Increment 6.1 | 2025-07-20 19:34 UTC] Fixed `s6_21_transition_by_non_identifier_token` and `s6_28_command_path_invalid_identifier_segment` tests.
*   [Increment 7 | 2025-07-20 19:39 UTC] Reviewed code for adherence to codestyle/design rules, improved doc comments, and ensured no unaddressed markers.
*   [Increment 7.1 | 2025-07-20 20:05 UTC] Resolved `Split` struct initialization errors in `strs_tools` test files.
