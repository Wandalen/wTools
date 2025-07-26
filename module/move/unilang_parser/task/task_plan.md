# Task Plan: Fix Parser Errors and Align with Spec (v3)

### Goal
*   To fix the parser engine to correctly reject invalid tokens in arguments, align the implementation with the specification by adding support for kebab-case in argument names, and add comprehensive test coverage for these changes.

### Ubiquitous Language (Vocabulary)
*   **Command Path**: The hierarchical name of a command (e.g., `cmd.subcmd`).
*   **Argument Name**: The key for a named argument (e.g., `my-arg` in `my-arg::value`).
*   **Kebab-case**: An identifier style using hyphens to separate words (e.g., `kebab-case`).
*   **Unrecognized Token**: A token that is not a valid identifier, number, operator, or delimiter according to the language grammar.

### Progress
*   **Roadmap Milestone:** M2: Core Parser Refinement
*   **Primary Editable Crate:** `module/move/unilang_parser`
*   **Overall Progress:** 1/3 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Fix Invalid Token Handling in Arguments
    *   ⚫ Increment 2: Implement and Test Kebab-Case Argument Support
    *   ⚫ Increment 3: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/move/unilang` (Reason: To run integration tests to ensure no regressions)

### Relevant Context
*   Control Files to Reference:
    *   `./spec.md`
*   Files to Include:
    *   `module/move/unilang_parser/src/parser_engine.rs`
    *   `module/move/unilang_parser/src/item_adapter.rs`
    *   `module/move/unilang_parser/tests/error_reporting_tests.rs`
    *   `module/move/unilang_parser/tests/argument_parsing_tests.rs`
    *   `module/move/unilang_parser/tests/command_parsing_tests.rs`

### Expected Behavior Rules / Specifications
*   Rule 1: An `Unrecognized` token (like `!`) in an argument list must produce a `Syntax` error.
*   Rule 2: Argument names can contain hyphens (`-`), but not as leading or trailing characters (e.g., `my-arg` is valid).
*   Rule 3: Command path segments cannot contain hyphens.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `error_reporting_tests::unexpected_token_in_args` | Fixed (Monitored) | This was the primary bug. Fixed in attempt 1. |
| `spec_adherence_tests::s6_28_command_path_invalid_identifier_segment` | Fixed (Monitored) | Regression introduced by the first fix attempt, now resolved. |

### Crate Conformance Check Procedure
*   Run `timeout 90 cargo test -p unilang_parser --all-targets`.
*   Run `timeout 90 cargo test -p unilang --all-targets`.
*   Run `timeout 90 cargo clippy -p unilang_parser -- -D warnings`.
*   Run `timeout 90 cargo clippy -p unilang -- -D warnings`.

### Increments

##### Increment 1: Fix Invalid Token Handling in Arguments
*   **Goal:** Modify `parser_engine.rs` to correctly raise a syntax error when an `Unrecognized` token is found in the argument list, fixing the failing test.
*   **Specification Reference:** `spec.md` Appendix A.2 (Implicit: only `IDENTIFIER` or `QUOTED_STRING` are valid values).
*   **Steps:**
    1.  Use `read_file` to load `module/move/unilang_parser/src/parser_engine.rs`.
    2.  In the `parse_arguments` function, locate the `match item.kind` block.
    3.  Separate the `UnilangTokenKind::Unrecognized(ref s)` case from the `Identifier` and `Number` cases into its own match arm. This new arm must immediately return a `ParseError`.
    4.  Use `search_and_replace` to modify the `match` statement.
        *   **Search For:**
            ```rust
            UnilangTokenKind::Identifier( ref s ) | UnilangTokenKind::Number( ref s ) | UnilangTokenKind::Unrecognized( ref s ) =>
            ```
        *   **Replace With:**
            ```rust
            UnilangTokenKind::Unrecognized( ref s ) =>
            {
              return Err( ParseError::new( ErrorKind::Syntax( format!( "Unexpected token '{}' in arguments", s ) ), item.adjusted_source_location.clone() ) );
            }
            UnilangTokenKind::Identifier( ref s ) | UnilangTokenKind::Number( ref s ) =>
            ```
    5.  Perform Increment Verification.
*   **Increment Verification:**
    1.  Execute `timeout 90 cargo test --test error_reporting_tests -p unilang_parser` via `execute_command`.
    2.  Analyze the command output to confirm that the test `unexpected_token_in_args` now passes.
    3.  Execute `timeout 90 cargo test -p unilang_parser` via `execute_command` to ensure no regressions were introduced in other tests.
*   **Commit Message:** "fix(parser): Reject unrecognized tokens in arguments"

##### Increment 2: Implement and Test Kebab-Case Argument Support
*   **Goal:** Align the parser with `spec.md` by allowing `kebab-case` for argument names while rejecting it for command path segments, and add tests to verify this behavior.
*   **Specification Reference:** `spec.md` Section 2.2.
*   **Steps:**
    1.  **Refine Identifier Classification:** Modify `is_valid_identifier` in `module/move/unilang_parser/src/item_adapter.rs` to be more permissive, recognizing kebab-case as a valid "identifier-like" token.
        *   Use `search_and_replace` to replace the `is_valid_identifier_char` and `is_valid_identifier` functions with a single, more idiomatic function:
            ```rust
            fn is_valid_identifier(s: &str) -> bool {
                !s.is_empty()
                    && s.chars().next().map_or(false, |c| c.is_ascii_lowercase() || c == '_')
                    && !s.ends_with('-')
                    && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
            }
            ```
    2.  **Add Contextual Validation for Command Paths:** Modify `parse_command_path` in `module/move/unilang_parser/src/parser_engine.rs` to explicitly reject identifiers containing hyphens.
        *   Use `search_and_replace` to insert a validation check *before* the segment is pushed to the vector.
        *   **Search For:** `command_path_slices.push( s.clone() );`
        *   **Replace With:**
            ```rust
            if s.contains('-') {
                return Err(ParseError::new(
                    ErrorKind::Syntax(format!("Invalid character '-' in command path segment '{}'", s)),
                    item.adjusted_source_location.clone(),
                ));
            }
            command_path_slices.push( s.clone() );
            ```
    3.  **Add Test for Kebab-Case Argument:** Use `insert_content` to add a new test to `module/move/unilang_parser/tests/argument_parsing_tests.rs`.
        *   **New Test Code:**
            ```rust
            /// Tests that a named argument with kebab-case is parsed correctly.
            #[test]
            fn parses_kebab_case_named_argument() {
                let parser = Parser::new(UnilangParserOptions::default());
                let input = "cmd my-arg::value another-arg::true";
                let result = parser.parse_single_instruction(input);
                assert!(result.is_ok(), "Parse error: {:?}", result.err());
                let instruction = result.unwrap();
                assert_eq!(instruction.command_path_slices, vec!["cmd".to_string()]);
                assert_eq!(instruction.named_arguments.len(), 2);
                assert_eq!(instruction.named_arguments.get("my-arg").unwrap().value, "value");
                assert_eq!(instruction.named_arguments.get("another-arg").unwrap().value, "true");
            }
            ```
    4.  **Add Test for Kebab-Case in Command Path:** Use `insert_content` to add a new test to `module/move/unilang_parser/tests/command_parsing_tests.rs`.
        *   **New Test Code:**
            ```rust
            /// Tests that a command path with a hyphen (kebab-case) is rejected.
            #[test]
            fn rejects_kebab_case_in_command_path() {
                let parser = Parser::new(UnilangParserOptions::default());
                let input = "cmd.my-sub.command arg1";
                let result = parser.parse_single_instruction(input);
                assert!(result.is_err(), "Expected error for kebab-case in command path");
                if let Err(e) = result {
                    assert!(matches!(e.kind, ErrorKind::Syntax(_)));
                    assert!(e.to_string().contains("Invalid character '-' in command path segment 'my-sub'"));
                }
            }
            ```
    5.  Perform Increment Verification.
*   **Increment Verification:**
    1.  Execute `timeout 90 cargo test -p unilang_parser` via `execute_command`.
    2.  Analyze the output to confirm that all tests, including the two new ones, pass, and that no regressions were introduced.
*   **Commit Message:** "feat(parser): Implement and test kebab-case argument support"

##### Increment 3: Finalization
*   **Goal:** Perform a final, holistic review, cleanup, and verification of the entire task's output.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Use `read_file` to load `module/move/unilang_parser/changelog.md`.
    2.  Use `insert_content` to add entries for the bug fix and new feature at the top of the file.
        *   **Changelog Entry:**
            ```markdown
            * [2025-07-26] fix(parser): Reject unrecognized tokens (e.g., `!`) in argument lists.
            * [2025-07-26] feat(parser): Add support for kebab-case in argument names as per spec.
            ```
    3.  Perform the full Crate Conformance Check procedure by executing the defined commands via `execute_command`.
    4.  Review all changes made during this task to ensure they are correct and complete.
    5.  Execute `git status` via `execute_command` to ensure the working directory is clean.
*   **Increment Verification:**
    *   All checks in the Crate Conformance Check procedure must pass based on the output of the `execute_command` calls.
*   **Commit Message:** "chore(parser): Finalize parsing fixes and kebab-case support"

### Task Requirements
*   The solution must not introduce any regressions in parsing other valid instruction formats.
*   The fix must be fully contained within the `unilang_parser` crate.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook.

### Assumptions
*   The core of the issue lies within the token classification and parser engine logic.

### Out of Scope
*   Large-scale refactoring of the entire parser.

### External System Dependencies
*   None

### Notes & Insights
*   The key is to differentiate validation contexts: `kebab-case` is valid for argument names but invalid for command path segments. This contextual validation must happen in the `parser_engine`, not during the initial `classify_split` stage.

### Changelog
*
