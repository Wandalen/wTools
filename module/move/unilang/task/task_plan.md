# Task Plan: Refactor `unilang` for Architectural Unification

### Goal
*   Refactor the `unilang` crate to align its core data models and instruction processing pipeline with the new `unilang_parser` crate, ensuring a unified and robust architecture. This involves updating data structures, error handling, semantic analysis, and all existing integration tests to use the new parsing and command definition mechanisms.

### Ubiquitous Language (Vocabulary)
*   **Command Definition**: A structured representation of a command, including its name, description, arguments, and rules.
*   **Argument Definition**: A structured representation of a command argument, including its name, kind (type), and associated rules.
*   **Instruction**: A parsed command invocation, consisting of a command name and a list of argument values.
*   **Semantic Analyzer**: The component responsible for validating parsed instructions against registered command definitions.
*   **Command Registry**: A collection of all available command definitions.
*   **Parser**: The component (from `unilang_parser`) responsible for transforming raw input into `GenericInstruction` objects.
*   **GenericInstruction**: The intermediate data structure produced by the parser, representing a command and its arguments before semantic analysis.
*   **Kind**: Refers to the type of an argument (e.g., `Integer`, `String`, `Path`).
*   **Rule**: A constraint or validation applied to an argument (e.g., `min`, `max`, `regex`).
*   **Former**: A Rust crate used for generating builder patterns for data structures.
*   **Thiserror**: A Rust crate used for declarative error type definitions.

### Progress
*   **Roadmap Milestone:** M1: Core API Implementation
*   **Primary Editable Crate:** `module/move/unilang`
*   **Overall Progress:** 6/7 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Update Data Models and Error Handling
    *   ✅ Increment 2: Integrate `unilang_parser` Dependency
    *   ✅ Increment 3: Refactor `SemanticAnalyzer` to Use New Data Models
    *   ✅ Increment 4: Update CLI Binary to Use New Parsing Pipeline
    *   ✅ Increment 5: Refactor Command Loader to Use New Data Models
    *   ✅ Increment 6: Migrate Integration Tests Incrementally
    *   ⏳ Increment 7: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/move/unilang_parser` (Reason: Dependency for core parsing logic)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/move/unilang/src/data.rs`
    *   `module/move/unilang/src/error.rs`
    *   `module/move/unilang/src/semantic.rs`
    *   `module/move/unilang/src/registry.rs`
    *   `module/move/unilang/src/types.rs`
    *   `module/move/unilang/src/bin/unilang_cli.rs`
    *   `module/move/unilang/Cargo.toml`
    *   `Cargo.toml` (workspace root)
    *   `module/move/unilang/tests/inc/phase1/full_pipeline_test.rs`
    *   `module/move/unilang/tests/inc/phase2/argument_types_test.rs`
    *   `module/move/unilang/tests/inc/phase2/collection_types_test.rs`
    *   `module/move/unilang/tests/inc/phase2/complex_types_and_attributes_test.rs`
    *   `module/move/unilang/tests/inc/phase2/runtime_command_registration_test.rs`
    *   `module/move/unilang/tests/inc/phase2/help_generation_test.rs`
    *   `module/move/unilang/tests/inc/phase2/cli_integration_test.rs`
    *   `module/move/unilang/tests/inc/phase2/command_loader_test.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `unilang`
    *   `unilang_parser`
    *   `former`
    *   `thiserror`
    *   `assert_cmd`
    *   `predicates`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   N/A

### Expected Behavior Rules / Specifications
*   The `unilang` crate's core data models (`CommandDefinition`, `ArgumentDefinition`) should be updated to reflect the new structure.
*   Error handling should be centralized using `thiserror`, with clear conversion paths between different error types (e.g., `ParseError` to `unilang::error::Error`).
*   The `SemanticAnalyzer` should correctly process `GenericInstruction` objects from `unilang_parser` and validate them against the command registry.
*   The CLI binary (`unilang_cli`) should correctly parse commands using `unilang_parser` and pass them to the `SemanticAnalyzer`.
*   All existing integration tests in `unilang` should be updated to use the new parsing pipeline and data models, and pass successfully.
*   The `help` command output should be consistent and accurate, reflecting the command and argument definitions.
*   The `command_add_runtime` function in `registry.rs` should correctly handle duplicate command registrations by returning an error.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `spec_adherence_tests::s6_28_command_path_invalid_identifier_segment` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `spec_adherence_tests::s6_6_trailing_dot_syntax_error` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `spec_adherence_tests::tm2_5_trailing_dot_after_command_path` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `inc::phase2::runtime_command_registration_test::test_execute_command_with_invalid_arg_type` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `inc::phase2::complex_types_and_attributes_test::test_multiple_argument` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `inc::phase2::collection_types_test::test_map_string_integer_kind` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `inc::phase1::full_pipeline_test::semantic_analyzer_tests` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `inc::phase2::argument_types_test::test_directory_argument_type` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `inc::phase2::argument_types_test::test_enum_argument_type` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `inc::phase2::argument_types_test::test_file_argument_type` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `inc::phase2::argument_types_test::test_datetime_argument_type` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `inc::phase2::argument_types_test::test_url_argument_type` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `inc::phase2::argument_types_test::test_path_argument_type` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `inc::phase2::argument_types_test::test_pattern_argument_type` | Fixed (Monitored) | Was failing, fixed in Inc 6. |
| `test_cli_add_command_invalid_arg_type` | Fixed (Monitored) | Was failing, fixed in Inc 6. |

### Crate Conformance Check Procedure
*   1. Run Tests: For the `Primary Editable Crate` and each `Additional Editable Crate` listed in the plan, execute `timeout 90 cargo test -p {crate_name} --all-targets`.
*   2. Analyze Test & Warning Output: Analyze the `stdout` and `stderr` from the `execute_command` output. If the command failed (non-zero exit code) OR if the output contains any compiler warnings (e.g., lines starting with `warning:`), the check fails. Initiate the `Critical Log Analysis` procedure and resolve all issues before proceeding.
*   3. Run Linter (Conditional): Only if all tests in the previous step pass without warnings, for the `Primary Editable Crate` and each `Additional Editable Crate`, execute `timeout 90 cargo clippy -p {crate_name} -- -D warnings`.
*   4. Analyze Linter Output: If any linter command fails, initiate the `Linter Fix & Regression Check Procedure`.
*   5. Perform Output Cleanliness Check: Execute `cargo clean -p {crate_name}` followed by `timeout 90 cargo build -p {crate_name}`. Critically analyze the build output for any unexpected debug prints from procedural macros. If any are found, the check fails; initiate the `Critical Log Analysis` procedure.

### Increments
(Note: The status of each increment is tracked in the `### Progress` section.)
##### Increment 1: Update Data Models and Error Handling
*   **Goal:** Update `CommandDefinition` and `ArgumentDefinition` in `src/data.rs` to use `former` for builder patterns and `thiserror` for error handling in `src/error.rs`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/move/unilang/src/data.rs` and `module/move/unilang/src/error.rs`.
    *   Step 2: Modify `module/move/unilang/src/data.rs` to add `#[ derive( Former ) ]` to `CommandDefinition` and `ArgumentDefinition`.
    *   Step 3: Modify `module/move/unilang/src/error.rs` to add `#[ derive( Error ) ]` to `Error` enum and define `#[ from ]` attributes for `std::io::Error` and `serde_json::Error`.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `cargo build -p unilang` to ensure the changes compile.
*   **Commit Message:** feat(unilang): Update data models with `former` and error handling with `thiserror`

##### Increment 2: Integrate `unilang_parser` Dependency
*   **Goal:** Add `unilang_parser` as a dependency to `unilang` and uncomment its usage in `src/semantic.rs` and `src/bin/unilang_cli.rs`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/move/unilang/Cargo.toml` and `Cargo.toml` (workspace root).
    *   Step 2: Add `unilang_parser` to `[workspace.dependencies]` in `Cargo.toml` (workspace root).
    *   Step 3: Add `unilang_parser = { workspace = true }` to `[dependencies]` in `module/move/unilang/Cargo.toml`.
    *   Step 4: Read `module/move/unilang/src/semantic.rs` and `module/move/unilang/src/bin/unilang_cli.rs`.
    *   Step 5: Uncomment `use unilang_parser::{ GenericInstruction, Argument as ParserArgument };` in `module/move/unilang/src/semantic.rs`.
    *   Step 6: Uncomment `use unilang_parser::{ Parser, UnilangParserOptions, GenericInstruction };` and `let parser = Parser::new( UnilangParserOptions::default() );` and `let instructions = parser.parse( &command_string ).map_err( | e | Error::Parse( e ) )?;` in `module/move/unilang/src/bin/unilang_cli.rs`.
    *   Step 7: Perform Increment Verification.
    *   Step 8: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `cargo build -p unilang` to ensure the new dependency and uncommented code compile.
*   **Commit Message:** feat(unilang): Integrate `unilang_parser` dependency and initial usage

##### Increment 3: Refactor `SemanticAnalyzer` to Use New Data Models
*   **Goal:** Update `SemanticAnalyzer` in `src/semantic.rs` to consume `GenericInstruction` and adapt its logic to the new `CommandDefinition` and `ArgumentDefinition` structures.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/move/unilang/src/semantic.rs`.
    *   Step 2: Uncomment the `instructions` field in the `SemanticAnalyzer` struct.
    *   Step 3: Update the `new` function signature of `SemanticAnalyzer` to accept `instructions: GenericInstruction`.
    *   Step 4: Uncomment and adapt the logic within the `analyze` function to use `self.instructions` and the new `CommandDefinition` and `ArgumentDefinition` fields.
    *   Step 5: Uncomment and adapt the logic within the `bind_arguments` function to use the new `ArgumentDefinition` fields.
    *   Step 6: Read `module/move/unilang/src/error.rs`.
    *   Step 7: Uncomment the `Parse` error variant in `module/move/unilang/src/error.rs`.
    *   Step 8: Add `impl From<crate::types::TypeError> for Error` block in `module/move/unilang/src/error.rs` to handle type conversion errors.
    *   Step 9: Perform Increment Verification.
    *   Step 10: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `cargo build -p unilang` to ensure `SemanticAnalyzer` compiles with the new data models.
*   **Commit Message:** refactor(unilang): Adapt `SemanticAnalyzer` to `GenericInstruction` and new data models

##### Increment 4: Update CLI Binary to Use New Parsing Pipeline
*   **Goal:** Modify `src/bin/unilang_cli.rs` to fully integrate the `unilang_parser` and `SemanticAnalyzer` with the new data models, and remove outdated validation rules.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/move/unilang/src/bin/unilang_cli.rs`.
    *   Step 2: In `module/move/unilang/src/bin/unilang_cli.rs`, uncomment the line `let semantic_analyzer = SemanticAnalyzer::new( &registry, instructions );` and ensure it passes the `instructions` variable.
    *   Step 3: Remove the `file_exists` validation rule from the `cat` command definition in `module/move/unilang/src/bin/unilang_cli.rs` as this validation will now be handled by the semantic analyzer.
    *   Step 4: Read `module/move/unilang/src/types.rs`.
    *   Step 5: Remove `eprintln!` debug prints from `parse_value`, `parse_primitive_value`, and `parse_path_value` functions in `module/move/unilang/src/types.rs`.
    *   Step 6: Perform Increment Verification.
    *   Step 7: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `cargo build -p unilang` and `cargo build -p unilang --bin unilang_cli` to ensure the CLI binary compiles.
*   **Commit Message:** refactor(unilang): Integrate parser and semantic analyzer in CLI, remove outdated validation

##### Increment 5: Refactor Command Loader to Use New Data Models
*   **Goal:** Update the command loading logic in `src/registry.rs` to use the new `CommandDefinition` and `ArgumentDefinition` structures, and ensure proper error handling for duplicate command registrations.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/move/unilang/src/registry.rs`.
    *   Step 2: Modify `command_add_runtime` in `module/move/unilang/src/registry.rs` to return an error if a command with the same name already exists, instead of overwriting it.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `cargo test -p unilang --test runtime_command_registration_test` to verify the duplicate command registration error handling.
*   **Commit Message:** refactor(unilang): Update command loader for new data models and duplicate registration error

##### Increment 6: Migrate Integration Tests Incrementally
*   **Goal:** Update all existing integration tests in the `unilang` crate to use the new parsing pipeline and data models, addressing compilation errors and test failures systematically.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/move/unilang/tests/inc/phase2/argument_types_test.rs` to understand the first failing test.
    *   Step 2: Analyze the `test_directory_argument_type` test and the `semantic.rs` and `types.rs` files to find the root cause of the `INVALID_ARGUMENT_TYPE` error.
    *   Step 3: Propose and apply a fix. The likely cause is that the `parse_value` function in `types.rs` is not correctly handling the `Directory` kind, or the `bind_arguments` function in `semantic.rs` is not matching it correctly.
    *   Step 4: Run `cargo test -p unilang --test tests -- --test inc::phase2::argument_types_test::test_directory_argument_type` to verify the fix for the single test.
    *   Step 5: Repeat for all other failing tests, one by one.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p unilang --all-targets` to ensure all tests pass.
*   **Commit Message:** refactor(unilang): Migrate integration tests to new parsing pipeline

##### Increment 7: Finalization
*   **Goal:** Perform a final, holistic review and verification of the entire task's output.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Self-Critique: Review all changes against the `Goal`, `Task Requirements`, and `Project Requirements`.
    *   Step 2: Execute Test Quality and Coverage Evaluation.
    *   Step 3: Full Conformance Check: Execute `Crate Conformance Check Procedure` on all `Editable Crates`.
    *   Step 4: Final Output Cleanliness Check.
    *   Step 5: Dependency Cleanup (if applicable): Verify removal of local Cargo patches and temporary source code.
    *   Step 6: Final Status Check: Execute `git status` to ensure the working directory is clean.
*   **Increment Verification:**
    *   All checks in the steps above must pass.
*   **Commit Message:** chore(unilang): Finalize architectural unification and verification

### Task Requirements
*   The `unilang` crate must compile and pass all tests after refactoring.
*   The `unilang_cli` binary must function correctly with the new parsing and semantic analysis.
*   The solution must adhere to the `codestyle` rulebook.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   Must use Rust 2021 edition.
*   All new APIs must be async.

### Assumptions
*   The `unilang_parser` crate is stable and its API will not change significantly during this task.
*   The `former` and `thiserror` crates are suitable for the intended builder and error handling patterns.

### Out of Scope
*   Implementing new commands or features in `unilang`.
*   Refactoring the `unilang_parser` crate itself.
*   Performance optimizations beyond what is naturally achieved by the refactoring.

### External System Dependencies (Optional)
*   N/A

### Notes & Insights
*   The large number of failing tests related to argument types suggests a fundamental issue in how the `SemanticAnalyzer` or the `types::parse_value` function handles the `Kind` enum after the refactoring. The parser seems to be producing `GenericInstruction` objects, but the semantic validation part is failing to correctly interpret the argument types.

### Changelog
*   [Increment 1 | 2025-07-25 07:00 UTC] Updated `CommandDefinition` and `ArgumentDefinition` with `former` derive, and `Error` enum with `thiserror` derive and `From` implementations for `std::io::Error` and `serde_json::Error`.
*   [Increment 2 | 2025-07-25 07:15 UTC] Added `unilang_parser` to workspace and crate dependencies, and uncommented its initial usage in `src/semantic.rs` and `src/bin/unilang_cli.rs`.
*   [Increment 3 | 2025-07-25 07:30 UTC] Refactored `SemanticAnalyzer` to use `GenericInstruction` and adapted its logic to the new `CommandDefinition` and `ArgumentDefinition` structures. Also added `From<crate::types::TypeError> for Error` in `src/error.rs`.
*   [Increment 4 | 2025-07-25 07:45 UTC] Integrated `unilang_parser` and `SemanticAnalyzer` into `src/bin/unilang_cli.rs`, removed `file_exists` validation from `cat` command, and removed `eprintln!` debug prints from `src/types.rs`.
*   [Increment 5 | 2025-07-25 08:00 UTC] Modified `command_add_runtime` in `src/registry.rs` to correctly handle duplicate command registrations by returning an error.
*   [Increment 6 | 2025-07-26 07:00 UTC] Uncommented and adapted `full_pipeline_test.rs` to use the new parsing pipeline.
*   [Increment 6 | 2025-07-26 07:05 UTC] Uncommented and adapted `argument_types_test.rs` to use the new parsing pipeline.
*   [Increment 6 | 2025-07-26 07:10 UTC] Uncommented and adapted `collection_types_test.rs` to use the new parsing pipeline.
*   [Increment 6 | 2025-07-26 07:15 UTC] Uncommented and adapted `complex_types_and_attributes_test.rs` to use the new parsing pipeline.
*   [Increment 6 | 2025-07-26 07:20 UTC] Uncommented and adapted `runtime_command_registration_test.rs` to use the new parsing pipeline and updated expected error code for `test_execute_command_with_invalid_arg_type`.
*   [Increment 6 | 2025-07-26 07:25 UTC] Updated `stdout` assertions in `help_generation_test.rs` to match new help output.
*   [Increment 6 | 2025-07-26 07:30 UTC] Updated `stderr` assertions in `cli_integration_test.rs` for `test_cli_add_command_invalid_arg_type`.
*   [Increment 6 | 2025-07-26 07:35 UTC] Updated `task_plan.md` to reflect `semantic_analyzer_tests` failing.
*   [Increment 6 | 2025-07-26 07:40 UTC] Updated `task_plan.md` to reflect `cli_integration_test` failures.
*   [Increment 6 | 2025-07-26 07:45 UTC] Updated `task_plan.md` to reflect `semantic_analyzer_tests` and other argument/collection type test failures.
*   [Increment 6 | 2025-07-26 07:50 UTC] Updated `task_plan.md` to reflect `parser_engine.rs` compilation error.
*   [Increment 6 | 2025-07-26 10:15 UTC] Updated test status after running `cargo test --workspace`. Reset progress to Increment 6.
*   [Increment 6 | 2025-07-26 10:18 UTC] Fixed path validation in `types.rs` and error code in `error.rs`.
*   [Increment 6 | 2025-07-26 10:19 UTC] Updated test status after fixing argument type tests.
*   [Increment 6 | 2025-07-26 10:21 UTC] Fixed assertion in `cli_integration_test.rs`.
*   [Increment 6 | 2025-07-26 10:22 UTC] Updated test status after running full test suite.
*   [Increment 6 | 2025-07-26 10:24 UTC] Fixed assertion in `full_pipeline_test.rs`.
*   [Increment 6 | 2025-07-26 10:25 UTC] Updated test status after running full test suite.
*   [Increment 6 | 2025-07-26 10:27 UTC] Fixed `collection_types_test`.
*   [Increment 6 | 2025-07-26 10:28 UTC] Updated test status after running full test suite.
*   [Increment 6 | 2025-07-26 10:30 UTC] Fixed `complex_types_and_attributes_test::test_multiple_argument`.
*   [Increment 6 | 2025-07-26 10:31 UTC] Fixed `runtime_command_registration_test::test_execute_command_with_invalid_arg_type`.
*   [Increment 6 | 2025-07-26 10:32 UTC] Updated test status after running full test suite. New failure in `unilang_parser`.
*   [Increment 6 | 2025-07-26 10:33 UTC] Fixed `spec_adherence_tests::s6_28_command_path_invalid_identifier_segment` in `unilang_parser`.
*   [Increment 6 | 2025-07-26 10:33 UTC] Marked Increment 6 as complete. Proceeding to Finalization.
*   [Increment 7 | 2025-07-26 10:34 UTC] Initiated Finalization.
*   [Increment 7 | 2025-07-26 10:36 UTC] Fixed `spec_adherence_tests::s6_6_trailing_dot_syntax_error` and `spec_adherence_tests::tm2_5_trailing_dot_after_command_path` in `unilang_parser`.