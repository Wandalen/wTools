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
*   **Overall Progress:** 2/4 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Replicate the Bug with a Focused Test
    *   ✅ Increment 2: Implement the Parser Logic Fix
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
        let mut split_options_former = SplitOptionsFormer::new( delimiters_as_str_slice );
        split_options_former
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
*   **Goal:** Before any code is changed, we must have a reliable, automated way to prove the bug exists. This test will serve as the primary validation for the fix.
*   **Specification Reference:** Expected Behavior Rules 1 & 2.
*   **Test Matrix:**
    | ID   | Input String         | Expected `command_path_slices` | Expected `positional_arguments` | Notes                                   |
    |------|----------------------|--------------------------------|---------------------------------|-----------------------------------------|
    | T1.1 | `.test.command arg1` | `["test", "command"]`          | `["arg1"]`                      | The primary failing case.               |
    | T1.2 | `command arg1`       | `["command"]`                  | `["arg1"]`                      | Should already pass.                    |
    | T1.3 | `.command arg1`      | `["command"]`                  | `["arg1"]`                      | Should fail.                            |
    | T1.4 | `command.sub arg1`   | `["command", "sub"]`           | `["arg1"]`                      | Should fail.                            |
    | T1.5 | `command`            | `["command"]`                  | `[]`                            | Should already pass.                    |
*   **Steps:**
    *   Step 1: Create a new test file `tests/command_parsing_tests.rs` with the content below. This content includes the Test Matrix and test functions for each combination.
        ```rust
        //! ## Test Matrix for Command Path Parsing
        //!
        //! | ID   | Input String         | Expected `command_path_slices` | Expected `positional_arguments` | Notes                                   |
        //! |------|----------------------|--------------------------------|---------------------------------|-----------------------------------------|
        //! | T1.1 | `.test.command arg1` | `["test", "command"]`          | `["arg1"]`                      | The primary failing case.               |
        //! | T1.2 | `command arg1`       | `["command"]`                  | `["arg1"]`                      | Should already pass.                    |
        //! | T1.3 | `.command arg1`      | `["command"]`                  | `["arg1"]`                      | Should fail.                            |
        //! | T1.4 | `command.sub arg1`   | `["command", "sub"]`           | `["arg1"]`                      | Should fail.                            |
        //! | T1.5 | `command`            | `["command"]`                  | `[]`                            | Should already pass.                    |

        use unilang_instruction_parser::{ Parser, UnilangParserOptions };

        fn parse_and_assert( input : &str, expected_path : &[ &str ], expected_args : &[ &str ] )
        {
          let options = UnilangParserOptions::default();
          let parser = Parser::new( options );
          let instructions = parser.parse_single_str( input ).unwrap();
          assert_eq!( instructions.len(), 1 );
          let instruction = &instructions[ 0 ];
          assert_eq!( instruction.command_path_slices, expected_path );
          let positional_values: Vec<&str> = instruction.positional_arguments.iter().map(|arg| arg.value.as_str()).collect();
          assert_eq!( positional_values, expected_args );
        }

        /// Tests the primary failing case.
        /// Test Combination: T1.1
        #[test]
        fn parses_dotted_prefix_command_path_correctly()
        {
          parse_and_assert( ".test.command arg1", &["test", "command"], &["arg1"] );
        }

        /// Tests a simple command without dots.
        /// Test Combination: T1.2
        #[test]
        fn parses_simple_command_path_correctly()
        {
          parse_and_assert( "command arg1", &["command"], &["arg1"] );
        }

        /// Tests a command with a leading dot.
        /// Test Combination: T1.3
        #[test]
        fn parses_leading_dot_command_path_correctly()
        {
          parse_and_assert( ".command arg1", &["command"], &["arg1"] );
        }

        /// Tests a command with an infix dot.
        /// Test Combination: T1.4
        #[test]
        fn parses_infix_dot_command_path_correctly()
        {
          parse_and_assert( "command.sub arg1", &["command", "sub"], &["arg1"] );
        }

        /// Tests a command with no arguments.
        /// Test Combination: T1.5
        #[test]
        fn parses_command_only_correctly()
        {
          parse_and_assert( "command", &["command"], &[] );
        }
        ```
    *   Step 2: Use `write_to_file` to create `module/move/unilang_instruction_parser/tests/command_parsing_tests.rs`.
    *   Step 3: Read `module/move/unilang_instruction_parser/tests/tests.rs`.
    *   Step 4: Use `insert_content` to add `mod command_parsing_tests;` to `module/move/unilang_instruction_parser/tests/tests.rs`.
    *   Step 5: Perform Increment Verification.
*   **Increment Verification:**
    *   1.  Execute `timeout 90 cargo test -p unilang_instruction_parser --test command_parsing_tests` via `execute_command`.
    *   2.  Analyze the output. The tests `parses_dotted_prefix_command_path_correctly`, `parses_leading_dot_command_path_correctly`, and `parses_infix_dot_command_path_correctly` **must fail** with an assertion error. The other tests should pass. This confirms the bug is replicated.
*   **Commit Message:** "test(parser): Add failing test for incorrect command path parsing"

##### Increment 2: Implement the Parser Logic Fix
*   **Goal:** To fix the command path parsing by introducing a simple state machine into the `parse_single_instruction_from_rich_items` function.
*   **Specification Reference:** Expected Behavior Rules 1, 2, & 3.
*   **Steps:**
    *   1.  Read `src/parser_engine.rs`.
    *   2.  In `parse_single_instruction_from_rich_items`, replace the existing command path parsing logic with a new state-machine-based implementation.
        *   **Code to be replaced:** The `eprintln!` debugging statements and the `while` loop that currently attempts to parse the command path.
        *   **New Logic:**
            ```rust
            let mut command_path_slices = Vec::new();
            let mut items_cursor = 0;

            // Handle optional leading dot
            if let Some(first_item) = significant_items.get(0) {
                if let UnilangTokenKind::Delimiter(d) = &first_item.kind {
                    if d == "." {
                        items_cursor += 1; // Consume the leading dot
                    }
                }
            }

            // Consume command path segments
            while items_cursor < significant_items.len() {
                let current_item = significant_items[items_cursor];

                if let UnilangTokenKind::Identifier(s) = &current_item.kind {
                    command_path_slices.push(s.clone());
                    items_cursor += 1;

                    // After an identifier, we expect either a dot or the end of the command path.
                    // Any other token (including a space delimiter) should terminate the command path.
                    if let Some(next_item) = significant_items.get(items_cursor) {
                        if let UnilangTokenKind::Delimiter(d) = &next_item.kind {
                            if d == "." {
                                items_cursor += 1; // Consume the dot
                            } else {
                                // Any other delimiter (space, "::", "?") ends the command path.
                                break;
                            }
                        } else {
                            // Next item is not a delimiter, so command path ends.
                            break;
                        }
                    } else {
                        // End of significant items, command path ends naturally.
                        break;
                    }
                } else {
                    // Any non-identifier token (including unexpected delimiters) indicates the end of the command path.
                    break;
                }
            }
            ```
    *   Step 3: Use `search_and_replace` to perform the replacement in `src/parser_engine.rs`.
    *   Step 4: Remove the `eprintln!` statements from the function as they are no longer needed.
    *   Step 5: Perform Increment Verification.
*   **Increment Verification:**
    *   1.  Execute `timeout 90 cargo test -p unilang_instruction_parser --test command_parsing_tests` via `execute_command`.
    *   2.  Analyze the output. The tests from Increment 1 **must now pass**.
*   **Commit Message:** "fix(parser): Correctly distinguish command paths from arguments"

##### Increment 3: Verify No Regressions Incrementally
*   **Rationale:** To avoid the previous "stuck" state, we will verify the fix against the existing test suite in stages. This allows us to pinpoint the source of any regression immediately.
*   **Specification Reference:** Expected Behavior Rule 4.
*   **Steps:**
    *   1.  **Run Argument Parsing Tests:** Execute `timeout 90 cargo test -p unilang_instruction_parser --test argument_parsing_tests`. These are the most likely to be affected by the change.
    *   2.  **Analyze Output:** If the tests fail, the fix in Increment 2 was incorrect. Initiate Critical Log Analysis, revert the changes, and redesign the fix in Increment 2. **Do not proceed.**
    *   3.  **Run Syntactic Analyzer Tests:** If Step 1 passes, execute `timeout 90 cargo test -p unilang_instruction_parser --test syntactic_analyzer_command_tests`.
    *   4.  **Analyze Output:** If these tests fail, repeat the analysis and redesign process. **Do not proceed.**
    *   5.  **Run Full Suite:** If all previous steps pass, run the entire test suite to catch any other regressions: `timeout 90 cargo test -p unilang_instruction_parser --all-targets`.
*   **Increment Verification:**
    *   1.  The `execute_command` for each step must exit with code 0.
    *   2.  The output of each test run must show all tests passing.
*   **Commit Message:** "test(parser): Verify no regressions after command parsing fix"

##### Increment 4: Finalization
*   **Goal:** Perform a final review and verification of the entire task's output.
*   **Specification Reference:** N/A
*   **Steps:**
    *   1.  Perform a self-critique of all changes against the plan's goal and requirements.
    *   2.  Run the Crate Conformance Check one last time.
    *   3.  Execute `git status` to ensure the working directory is clean.
*   **Increment Verification:**
    *   1.  Execute the full `Crate Conformance Check Procedure`.
    *   2.  Execute `git status` via `execute_command` and confirm the output shows no uncommitted changes.
*   **Commit Message:** "chore: Finalize command parsing fix and verification"

### Changelog
*   [Initial] Plan created to methodically fix the command path parsing bug.
*   [Increment 1] Added a failing test case to replicate the command path parsing bug. Fixed initial compilation errors in the test setup.
*   [Increment 2] Correctly distinguished command paths from arguments by refining the parsing logic in `parser_engine.rs` and ensuring proper tokenization.
