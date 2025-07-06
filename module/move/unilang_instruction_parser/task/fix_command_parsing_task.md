# Task Plan: Fix Command Path Parsing (Revised)

### Goal
*   To fix the critical bug in `unilang_instruction_parser` where the command path is incorrectly parsed as a positional argument. This plan is designed to be methodical, with small, verifiable steps to ensure the fix is correct and does not introduce regressions, which was an issue in the previous attempt.

### Ubiquitous Language (Vocabulary)
*   **`GenericInstruction`**: The primary output of the parser, representing a single parsed command.
*   **`command_path_slices`**: The field in `GenericInstruction` that should contain the components of the command name (e.g., `["test", "command"]` for `.test.command`).
*   **`strs_tools`**: The external dependency used for low-level string tokenization (splitting).
*   **`Split` / `SplitIterator`**: Core components from `strs_tools` that produce token-like items from a string.
*   **`Parser State Machine`**: The logic within `parser_engine.rs` that transitions between states (e.g., `ParsingCommand`, `ParsingArguments`) to interpret the token stream.

### Progress
*   **Roadmap Milestone:** N/A (This is a bug-fix task to unblock other work)
*   **Primary Editable Crate:** `module/move/unilang_instruction_parser`
*   **Overall Progress:** 0/4 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Replicate the Bug with a Focused Test
    *   ⚫ Increment 2: Implement the Parser Logic Fix
    *   ⚫ Increment 3: Verify No Regressions Incrementally
    *   ⚫ Increment 4: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:** None

### Relevant Context
*   Control Files to Reference:
    *   `./task.md` (The original change proposal outlining the bug)
    *   `./spec.md` (The formal specification for the `unilang` framework)
*   Files to Include:
    *   `src/parser_engine.rs` (The location of the core parsing logic)
    *   `src/instruction.rs` (Definition of `GenericInstruction`)
    *   `tests/argument_parsing_tests.rs` (Existing tests that must not be broken)
    *   `tests/syntactic_analyzer_command_tests.rs` (Existing tests that must not be broken)
    *   `tests/tests.rs` (To register the new test file)

### `strs_tools` API Guide
This section provides the necessary information to correctly use the `strs_tools` dependency for tokenization.

*   **Core Function:** `strs_tools::string::split::split()`
    *   This is the entry point. It returns a builder object called `SplitOptionsFormer`.
*   **Builder (`SplitOptionsFormer`):**
    *   You configure the parser using methods on this builder.
    *   `.src( &str )`: Sets the input string to parse.
    *   `.delimeter( D )`: Sets the delimiter(s). `D` can be `&str` or `Vec<&str>`.
    *   `.quoting( bool )`: Set to `true` to make the tokenizer treat quoted sections (e.g., `"hello world"`) as a single token.
    *   `.perform()`: Consumes the builder and returns a `SplitIterator`.
*   **Lifetime Pitfall with `.delimeter()`:**
    *   The `.delimeter()` method **borrows** the string slices. If you create a `Vec<&str>` from a `Vec<String>` on the same line you pass it to the builder, the compiler will raise an `E0716` error because the temporary vector is dropped while the builder is still borrowing it.
    *   **Problematic Pattern (Current Code):**
        ```rust
        // This pattern, if used, will cause a compilation error.
        // let split_iterator = SplitOptionsFormer::new( self.options.main_delimiters.iter().map(|s| s.as_str()).collect::<Vec<_>>() )
        //     .src( input )
        //     /* ... other options ... */
        //     .perform();
        ```
    *   **Correct Usage Pattern:**
        ```rust
        // In parser_engine.rs, inside tokenize_input:
        // You MUST bind the Vec<&str> to a variable that outlives the builder configuration.
        let delimiters_as_str_slice: Vec<&str> = self.options.main_delimiters.iter().map(|s| s.as_str()).collect();
        let split_options_former = SplitOptionsFormer::new( delimiters_as_str_slice )
            .src( input )
            .quoting( true );
        let split_iterator = split_options_former.perform();
        ```
*   **Iterator (`SplitIterator`):**
    *   This is the object you loop over. It yields `Split` structs.
*   **Output Item (`Split<'a>`):**
    *   `string: &'a str`: The raw string slice of the token.
    *   `typ: SplitType`: An enum, either `Delimited` (the content between delimiters) or `Delimiter` (the delimiter itself).
    *   `start: usize`, `end: usize`: The byte indices of the token in the original source string.

### Expected Behavior Rules / Specifications
*   Rule 1: Given an input string like `.test.command arg1`, the parser **must** populate `GenericInstruction.command_path_slices` with `["test", "command"]`.
*   Rule 2: The first contiguous sequence of identifiers, optionally separated by dots, **must** be treated as the command path.
*   Rule 3: All subsequent tokens **must** be treated as arguments (positional or named).
*   Rule 4: The fix **must not** cause any regressions. All tests in `argument_parsing_tests.rs` and `syntactic_analyzer_command_tests.rs` must continue to pass.

### Crate Conformance Check Procedure
*   Step 1: Execute `timeout 90 cargo test -p unilang_instruction_parser --all-targets` via `execute_command`.
*   Step 2: Analyze `execute_command` output. If it fails, initiate Critical Log Analysis.
*   Step 3: If tests pass, execute `timeout 90 cargo clippy -p unilang_instruction_parser -- -D warnings` via `execute_command`.
*   Step 4: Analyze `execute_command` output. If it fails, initiate Linter Fix & Regression Check Procedure.

### Increments

##### Increment 1: Replicate the Bug with a Focused Test
*   **Rationale:** Before any code is changed, we must have a reliable, automated way to prove the bug exists. This test will serve as the primary validation for the fix.
*   **Specification Reference:** Expected Behavior Rules 1 & 2.
*   **Steps:**
    1.  Create a new test file: `tests/command_parsing_tests.rs`.
    2.  Add a test matrix to the file-level documentation comment, outlining test cases for command path parsing.
    3.  Implement a test named `parses_command_path_correctly` in the new file. This test will parse the string `.test.command arg1` and assert that `instruction.command_path_slices` is `vec!["test", "command"]` and `instruction.positional_arguments` contains `"arg1"`.
    4.  Add more test cases to cover variations: `command arg1`, `.command arg1`, and `command.sub arg1`.
    5.  Modify `tests/tests.rs` to include the new test file: `mod command_parsing_tests;`.
*   **Increment Verification:**
    1.  Execute `timeout 90 cargo test -p unilang_instruction_parser --test command_parsing_tests` via `execute_command`.
    2.  Analyze the output. The test **must fail** with an assertion error showing that `command_path_slices` is empty and `positional_arguments` contains the command path.
*   **Commit Message:** "test(parser): Add failing test for incorrect command path parsing"

##### Increment 2: Implement the Parser Logic Fix
*   **Rationale:** The previous attempt to fix this caused regressions. This time, the fix will be more surgical, modifying the existing state machine in `parser_engine.rs` rather than attempting a wholesale replacement of logic.
*   **Specification Reference:** Expected Behavior Rules 1, 2, & 3.
*   **Steps:**
    1.  **Analyze:** Read the content of `src/parser_engine.rs`, specifically the `parse_single_instruction_from_rich_items` function.
    2.  **Hypothesize:** The current logic incorrectly identifies all initial `Identifier` tokens as positional arguments. The fix is to introduce a state machine. A simple approach is to use a boolean flag, e.g., `parsing_command_path`, which is initially `true`.
    3.  **Design the Logic Change:**
        *   While `parsing_command_path` is `true`, consume `Identifier` and `Delimiter(".")` tokens and append them to `command_path_slices`.
        *   Transition `parsing_command_path` to `false` as soon as a token that cannot be part of a command path is encountered (e.g., `Delimiter("::")`, `QuotedValue`, `Operator("?")`, or a non-dot delimiter).
        *   Once `parsing_command_path` is `false`, the rest of the function's logic for parsing named and positional arguments can proceed as it currently does.
    4.  **Implement:** Use `search_and_replace` to apply this targeted state-machine logic to `parse_single_instruction_from_rich_items`.
*   **Increment Verification:**
    1.  Execute `timeout 90 cargo test -p unilang_instruction_parser --test command_parsing_tests` via `execute_command`.
    2.  Analyze the output. The test from Increment 1 **must now pass**. This verifies the fix in isolation before checking for regressions.
*   **Commit Message:** "fix(parser): Correctly distinguish command paths from arguments"

##### Increment 3: Verify No Regressions Incrementally
*   **Rationale:** To avoid the previous "stuck" state, we will verify the fix against the existing test suite in stages. This allows us to pinpoint the source of any regression immediately.
*   **Specification Reference:** Expected Behavior Rule 4.
*   **Steps:**
    1.  **Run Argument Parsing Tests:** Execute `timeout 90 cargo test -p unilang_instruction_parser --test argument_parsing_tests`. These are the most likely to be affected by the change.
    2.  **Analyze Output:** If the tests fail, the fix in Increment 2 was incorrect. Initiate Critical Log Analysis, revert the changes, and redesign the fix in Increment 2. **Do not proceed.**
    3.  **Run Syntactic Analyzer Tests:** If Step 1 passes, execute `timeout 90 cargo test -p unilang_instruction_parser --test syntactic_analyzer_command_tests`.
    4.  **Analyze Output:** If these tests fail, repeat the analysis and redesign process. **Do not proceed.**
    5.  **Run Full Suite:** If all previous steps pass, run the entire test suite to catch any other regressions: `timeout 90 cargo test -p unilang_instruction_parser --all-targets`.
*   **Increment Verification:**
    1.  The `execute_command` for each step must exit with code 0.
    2.  The output of each test run must show all tests passing.
*   **Commit Message:** "test(parser): Verify no regressions after command parsing fix"

##### Increment 4: Finalization
*   **Goal:** Perform a final review and verification of the entire task's output.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Perform a self-critique of all changes against the plan's goal and requirements.
    2.  Run the Crate Conformance Check one last time.
    3.  Execute `git status` to ensure the working directory is clean.
*   **Increment Verification:**
    1.  Execute the full `Crate Conformance Check Procedure`.
    2.  Execute `git status` via `execute_command` and confirm the output shows no uncommitted changes.
*   **Commit Message:** "chore: Finalize command parsing fix and verification"

### Changelog
*   [Initial] Plan created to methodically fix the command path parsing bug.
