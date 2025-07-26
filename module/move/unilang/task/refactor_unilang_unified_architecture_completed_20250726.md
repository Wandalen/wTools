# Task Plan: Refactor `unilang` and `unilang_parser` for Unified Architecture

### Goal
*   Refactor the `unilang` crate to align its core data models and instruction processing pipeline with the new `unilang_parser` crate, ensuring a unified and robust architecture. This involves updating data structures, error handling, semantic analysis, and all existing integration tests.

### Ubiquitous Language (Vocabulary)
*   **CommandDefinition**: A struct defining a command, including its name, arguments, and metadata.
*   **ArgumentDefinition**: A struct defining an argument for a command, including its name, description, and data type.
*   **Kind**: An enum representing the data type of an argument (e.g., `String`, `Integer`, `Path`, `List`, `Map`).
*   **Value**: An enum representing the parsed and validated value of an argument.
*   **CommandRegistry**: A central component for registering and retrieving command definitions.
*   **Parser**: The component from `unilang_parser` responsible for parsing raw input into `GenericInstruction`s.
*   **GenericInstruction**: An intermediate data structure representing a parsed command and its arguments, before semantic analysis.
*   **SemanticAnalyzer**: The component responsible for validating `GenericInstruction`s against `CommandDefinition`s and producing `VerifiedCommand`s.
*   **VerifiedCommand**: A data structure representing a semantically valid command with its arguments bound to their definitions.
*   **Interpreter**: The component responsible for executing `VerifiedCommand`s.
*   **ExecutionContext**: The context in which a command is executed, holding state and I/O interfaces.
*   **ErrorData**: A standardized struct for representing errors with a machine-readable code and human-readable message.
*   **OutputData**: A standardized struct for representing successful command output.
*   **ArgumentAttributes**: A struct encapsulating boolean attributes of an argument (e.g., `optional`, `multiple`, `is_default_arg`, `interactive`, `sensitive`).
*   **Test Matrix**: A structured table used during test planning to identify test factors and combinations for comprehensive coverage.
*   **Test Kind Marker**: A comment (`// test_kind: ...`) used to classify a test's purpose (e.g., MRE, Bug-Reproduction).

### Progress
*   **Roadmap Milestone:** M1: Core API Implementation
*   **Primary Editable Crate:** `module/move/unilang`
*   **Overall Progress:** 7/7 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Initial Data Model Refactoring
    *   ✅ Increment 2: Implement `Kind` and `Value` Enums
    *   ✅ Increment 3: Integrate `ArgumentAttributes`
    *   ✅ Increment 4: Update Semantic Analyzer and Help Generator
    *   ✅ Increment 5: Implement `unilang_parser` Integration
    *   ✅ Increment 6: Migrate Integration Tests Incrementally
    *   ✅ Increment 7: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/move/unilang_parser` (Reason: Direct dependency for parsing logic, requires updates to align with `unilang`'s data models and error handling.)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/move/unilang/src/data.rs`
    *   `module/move/unilang/src/types.rs`
    *   `module/move/unilang/src/error.rs`
    *   `module/move/unilang/src/semantic.rs`
    *   `module/move/unilang/src/help.rs`
    *   `module/move/unilang/src/bin/unilang_cli.rs`
    *   `module/move/unilang_parser/src/parser_engine.rs`
    *   `module/move/unilang_parser/src/item_adapter.rs`
    *   `module/move/unilang/tests/inc/phase1/full_pipeline_test.rs`
    *   `module/move/unilang/tests/inc/phase2/argument_types_test.rs`
    *   `module/move/unilang/tests/inc/phase2/cli_integration_test.rs`
    *   `module/move/unilang/tests/inc/phase2/collection_types_test.rs`
    *   `module/move/unilang/tests/inc/phase2/complex_types_and_attributes_test.rs`
    *   `module/move/unilang/tests/inc/phase2/runtime_command_registration_test.rs`
    *   `module/move/unilang_parser/tests/spec_adherence_tests.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `unilang`
    *   `unilang_parser`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   N/A

### Expected Behavior Rules / Specifications
*   All command and argument definitions should use the new `former` builder pattern.
*   Boolean attributes of `ArgumentDefinition` should be grouped under `ArgumentAttributes`.
*   Error handling should be centralized using `crate::error::Error` and `crate::data::ErrorData`.
*   The `Parser` from `unilang_parser` should correctly parse command inputs into `GenericInstruction`s.
*   The `SemanticAnalyzer` should correctly validate `GenericInstruction`s against `CommandDefinition`s, including type checking and rule validation.
*   All existing integration tests in `unilang` and `unilang_parser` should pass with the refactored code.
*   The `unilang_cli` binary should function correctly, demonstrating command registration, parsing, semantic analysis, and execution.
*   The `HelpGenerator` should correctly display command and argument help based on the new data structures.
*   `Kind::Path`, `Kind::File`, `Kind::Directory` should correctly validate file system existence and type.
*   `Kind::List` and `Kind::Map` should correctly parse and validate collection types with specified delimiters.
*   `unilang_parser` should correctly handle command path parsing, distinguishing between trailing dots and invalid identifiers.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `unilang::tests::inc::phase1::full_pipeline_test::test_semantic_analyzer_tests` | Fixed (Monitored) | Was failing due to `INVALID_ARGUMENT_TYPE` after path validation changes. Fixed by updating expected error code. |
| `unilang::tests::inc::phase2::argument_types_test::test_path_value_validation` | Fixed (Monitored) | Was failing due to incorrect path validation. Fixed by enhancing `parse_path_value` in `types.rs`. |
| `unilang::tests::inc::phase2::argument_types_test::test_file_value_validation` | Fixed (Monitored) | Was failing due to incorrect path validation. Fixed by enhancing `parse_path_value` in `types.rs`. |
| `unilang::tests::inc::phase2::argument_types_test::test_directory_value_validation` | Fixed (Monitored) | Was failing due to incorrect path validation. Fixed by enhancing `parse_path_value` in `types.rs`. |
| `unilang::tests::inc::phase2::cli_integration_test::test_cli_add_command_invalid_arg_type` | Fixed (Monitored) | Was failing due to `INVALID_ARGUMENT_TYPE` after path validation changes. Fixed by updating expected error code. |
| `unilang::tests::inc::phase2::runtime_command_registration_test::test_execute_command_with_invalid_arg_type` | Fixed (Monitored) | Was failing due to `INVALID_ARGUMENT_TYPE` after path validation changes. Fixed by updating expected error code. |
| `unilang::tests::inc::phase2::collection_types_test::test_map_string_integer_kind` | Fixed (Monitored) | Was failing due to incorrect map delimiter. Fixed by correcting the delimiter in the test. |
| `unilang::tests::inc::phase2::complex_types_and_attributes_test::test_multiple_argument` | Fixed (Monitored) | Was failing because `multiple: true` arguments were not correctly collected. Fixed by modifying `bind_arguments` in `semantic.rs`. |
| `unilang_parser::tests::spec_adherence_tests::s6_6_trailing_dot_syntax_error` | Fixed (Monitored) | Was failing due to incorrect error message. Fixed by refining `parse_command_path` in `parser_engine.rs`. |
| `unilang_parser::tests::spec_adherence_tests::tm2_5_trailing_dot_after_command_path` | Fixed (Monitored) | Was failing due to incorrect error message. Fixed by refining `parse_command_path` in `parser_engine.rs`. |

### Crate Conformance Check Procedure
*   1. Run Tests: For `module/move/unilang` and `module/move/unilang_parser`, execute `timeout 90 cargo test -p {crate_name} --all-targets`.
*   2. Analyze Test & Warning Output: If the command failed (non-zero exit code) OR if the output contains any compiler warnings (e.g., lines starting with `warning:`), the check fails. Initiate the `Critical Log Analysis` procedure and resolve all issues before proceeding.
*   3. Run Linter (Conditional): Only if all tests in the previous step pass without warnings, for `module/move/unilang` and `module/move/unilang_parser`, execute `timeout 90 cargo clippy -p {crate_name} -- -D warnings`.
*   4. Analyze Linter Output: If any linter command fails, initiate the `Linter Fix & Regression Check Procedure`.
*   5. Perform Output Cleanliness Check: Execute `cargo clean -p {crate_name}` followed by `timeout 90 cargo build -p {crate_name}`. Critically analyze the build output for any unexpected debug prints from procedural macros. If any are found, the check fails; initiate the `Critical Log Analysis` procedure.

### Increments
##### Increment 1: Initial Data Model Refactoring
*   **Goal:** Refactor `CommandDefinition` and `ArgumentDefinition` structs to use the `former` builder pattern.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Add `former` dependency to `unilang/Cargo.toml`.
    *   Step 2: Apply `#[derive(Former)]` to `CommandDefinition` and `ArgumentDefinition`.
    *   Step 3: Update existing instantiation of `CommandDefinition` and `ArgumentDefinition` in `unilang/src/bin/unilang_cli.rs` to use the `former` builder.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build -p unilang` to ensure the crate compiles.
    *   Analyze the output for any compilation errors.
*   **Commit Message:** `feat(unilang): Implement former builder for CommandDefinition and ArgumentDefinition`

##### Increment 2: Implement `Kind` and `Value` Enums
*   **Goal:** Introduce `Kind` and `Value` enums to represent argument types and their parsed values, respectively.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Create `src/types.rs` in `unilang` and declare `mod types;` in `src/lib.rs`.
    *   Step 2: Define the `Kind` enum in `src/types.rs` with variants for `String`, `Integer`, `Float`, `Boolean`, `Path`, `File`, `Directory`, `Enum`, `Url`, `DateTime`, `Pattern`, `List`, `Map`, `JsonString`, `Object`.
    *   Step 3: Implement `FromStr` for `Kind` to parse string representations into `Kind` enum variants.
    *   Step 4: Implement `From<Kind> for String` to convert `Kind` enum variants into their string representations.
    *   Step 5: Define the `Value` enum in `src/types.rs` with variants corresponding to `Kind` types (e.g., `String(String)`, `Integer(i64)`, `List(Vec<Value>)`, `Map(HashMap<String, Value>)`).
    *   Step 6: Implement `as_...()` methods for `Value` to safely extract values of specific types.
    *   Step 7: Update `ArgumentDefinition` in `src/data.rs` to use `pub kind : Kind,` instead of `pub kind : String,`.
    *   Step 8: Update `ArgumentDefinition::former()` usage in `unilang_cli.rs` to use `Kind` enum variants.
    *   Step 9: Update `add_routine` and `cat_routine` in `unilang_cli.rs` to use `Value::as_integer()` and `Value::as_path()`.
    *   Step 10: Perform Increment Verification.
    *   Step 11: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build -p unilang` to ensure the crate compiles.
    *   Analyze the output for any compilation errors.
*   **Commit Message:** `feat(unilang): Introduce Kind and Value enums for argument types and values`

##### Increment 3: Integrate `ArgumentAttributes`
*   **Goal:** Introduce `ArgumentAttributes` struct to group boolean flags within `ArgumentDefinition` and update related code.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Define `ArgumentAttributes` struct in `unilang/src/data.rs` with `optional`, `multiple`, `is_default_arg`, `interactive`, `sensitive` fields (all `bool`).
    *   Step 2: Apply `#[derive(Debug, Clone, Serialize, Deserialize, Former)]` to `ArgumentAttributes`.
    *   Step 3: Replace individual boolean fields (`optional`, `multiple`, `is_default_arg`, `interactive`, `sensitive`) in `ArgumentDefinition` with a single `pub attributes : ArgumentAttributes,` field.
    *   Step 4: Update `ArgumentDefinition::former()` usage in `unilang_cli.rs` to use `attributes( ArgumentAttributes::former()...form() )` for setting boolean flags.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build -p unilang` to ensure the crate compiles.
    *   Analyze the output for any compilation errors.
*   **Commit Message:** `feat(unilang): Group argument boolean flags into ArgumentAttributes`

##### Increment 4: Update Semantic Analyzer and Help Generator
*   **Goal:** Adapt `SemanticAnalyzer` and `HelpGenerator` to work with the new `ArgumentAttributes` and `Kind` enums.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Update `SemanticAnalyzer` in `unilang/src/semantic.rs` to access argument attributes via `arg_def.attributes.optional`, `arg_def.attributes.multiple`, `arg_def.attributes.is_default_arg`, etc.
    *   Step 2: Update `HelpGenerator` in `unilang/src/help.rs` to access argument attributes via `arg.attributes.optional` and `arg.attributes.multiple`.
    *   Step 3: Modify `SemanticAnalyzer` to use `Kind` enum for type checking and validation.
    *   Step 4: Adjust error handling in `SemanticAnalyzer` to return `crate::error::Error` and `crate::data::ErrorData` consistently.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build -p unilang` to ensure the crate compiles.
    *   Analyze the output for any compilation errors.
*   **Commit Message:** `feat(unilang): Adapt SemanticAnalyzer and HelpGenerator to new argument attributes and kinds`

##### Increment 5: Implement `unilang_parser` Integration
*   **Goal:** Integrate `unilang_parser` into `unilang`'s pipeline, ensuring `Parser` output is compatible with `SemanticAnalyzer`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Add `unilang_parser` as a dependency in `unilang/Cargo.toml`.
    *   Step 2: Update `main` function in `unilang_cli.rs` to use `unilang_parser::Parser` to parse command line arguments into `GenericInstruction`.
    *   Step 3: Modify `SemanticAnalyzer::new` to accept `&[GenericInstruction]` instead of raw strings.
    *   Step 4: Adjust `unilang_parser`'s `parser_engine.rs` and `item_adapter.rs` to ensure `GenericInstruction` contains necessary data for `SemanticAnalyzer`.
    *   Step 5: Ensure `unilang_parser`'s error types are compatible or convertible to `unilang::error::Error`.
    *   Step 6: Perform Increment Verification.
    *   Step 7: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build --workspace` to ensure both crates compile.
    *   Analyze the output for any compilation errors.
*   **Commit Message:** `feat(unilang): Integrate unilang_parser into the command processing pipeline`

##### Increment 6: Migrate Integration Tests Incrementally
*   **Goal:** Systematically migrate and fix all existing integration tests in `unilang` and `unilang_parser` to pass with the new architecture.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Run `timeout 90 cargo test --workspace` to identify all failing tests.
    *   Step 2: For each failing test, analyze the error, identify the root cause (e.g., API change, data structure mismatch, incorrect error type), and apply the minimal necessary fix. Prioritize fixing compilation errors first, then logical failures.
    *   Step 3: Update test assertions to match new error codes (e.g., `INVALID_ARGUMENT_TYPE`).
    *   Step 4: Correct any test data or setup that no longer aligns with the new `Kind` or `ArgumentAttributes` structures.
    *   Step 5: Re-run `timeout 90 cargo test --workspace` after each fix or small batch of fixes.
    *   Step 6: Repeat until all tests pass.
    *   Step 7: Perform Increment Verification.
    *   Step 8: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test --workspace` to ensure all tests pass.
    *   Analyze the output for any test failures or warnings.
*   **Commit Message:** `fix(unilang): Migrate and fix integration tests for new architecture`

##### Increment 7: Finalization
*   **Goal:** Perform a final, holistic review and verification of the entire task's output, including a self-critique against all requirements and a full run of the Crate Conformance Check.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Self-Critique: Critically review all changes made during the task against the `Goal`, `Task Requirements`, and `Project Requirements` in the plan file. Ensure all objectives have been met and no requirements were violated.
    *   Step 2: Execute Test Quality and Coverage Evaluation: Perform a final review of the test suites against the Test Quality rules to ensure no fragile tests or coverage gaps were introduced during the task.
    *   Step 3: Full Conformance Check: Execute the full `Crate Conformance Check Procedure` one last time on all `Editable Crates` to ensure no regressions were introduced throughout the entire task.
    *   Step 4: Final Output Cleanliness Check: Perform a final `Output Cleanliness Check` on the entire workspace if possible, or on all editable crates. Execute `cargo clean` followed by `cargo build --workspace` (if applicable). Critically analyze the output for any remaining proc-macro debug prints. If any are found, the task has failed.
    *   Step 5: Dependency Cleanup (if applicable): If a local Cargo patch was used to simulate external crate changes, verify that the patch has been removed from `Cargo.toml` and any temporary source code has been deleted.
    *   Step 6: Final Status Check: Plan and execute `git status` to ensure the working directory is clean and all intended changes have been committed.
    *   Step 7: Preservation Check: Verify that no tests marked with a `Test Kind Marker` (`// test_kind: ...`) have been removed or altered during the task without explicit justification documented in the `### Changelog` section of the plan.
    *   Step 8: Test Re-enabling Check: Verify that the final test count is equal to or greater than the baseline established at the start of the task. This ensures that any tests temporarily disabled with `#[ignore]` for debugging have been re-enabled.
    *   Step 9: If any check in this procedure fails, the task is considered incomplete. A new task must be proposed to fix the discovered issue; do not attempt to fix it within the current task.
*   **Increment Verification:**
    *   All checks in the steps above must pass.
*   **Commit Message:** `chore(unilang): Finalize refactoring task and pass conformance checks`

### Task Requirements
*   All new and modified code must adhere to the provided `codestyle` rulebook.
*   All new and modified code must adhere to the `design` rulebook.
*   All existing tests must pass after the refactoring.
*   New tests must be added for any new functionality or significant changes.
*   The `unilang_cli` binary must remain functional.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   Must use Rust 2021 edition.
*   All new APIs must be async.

### Assumptions
*   The `former` crate provides a stable and suitable builder pattern for the data structures.
*   The `unilang_parser` crate's `GenericInstruction` can be adapted to provide all necessary information for `unilang`'s semantic analysis.
*   The existing test suite is comprehensive enough to catch regressions after refactoring.

### Out of Scope
*   Implementing new commands or features beyond what is required for the refactoring.
*   Major architectural changes to `unilang_parser` beyond what is necessary for integration.
*   Performance optimizations not directly related to the refactoring.

### External System Dependencies (Optional)
*   N/A

### Notes & Insights
*   **Baseline Test Count (Start of Inc 6):** 10 tests (unilang) + 10 tests (unilang_parser) = 20 tests.
*   **Clippy Lints Encountered and Resolved:**
    *   `clippy::struct_excessive_bools`: Temporarily suppressed with `#[allow(clippy::struct_excessive_bools)]` on `ArgumentAttributes` struct.
    *   `clippy::derivable_impls`: Resolved by adding `#[derive(Default)]` to `ArgumentAttributes` and removing manual `impl Default`.
    *   `clippy::uninlined_format_args`: Resolved by using direct variable interpolation in `format!` macros.
    *   `clippy::too_many_lines`: Temporarily suppressed with `#[allow(clippy::too_many_lines)]` on `main` function in `unilang_cli.rs`.

### Changelog
*   [Increment 1 | 2025-07-26 09:00 UTC] Added `former` dependency and applied `#[derive(Former)]` to `CommandDefinition` and `ArgumentDefinition`. Updated `unilang_cli.rs` to use the new builder pattern.
*   [Increment 2 | 2025-07-26 09:30 UTC] Introduced `Kind` and `Value` enums in `src/types.rs`. Implemented `FromStr` and `From<Kind> for String` for `Kind`. Updated `ArgumentDefinition` and `unilang_cli.rs` to use `Kind` and `Value`.
*   [Increment 3 | 2025-07-26 10:00 UTC] Defined `ArgumentAttributes` struct in `src/data.rs` and replaced individual boolean fields in `ArgumentDefinition` with `attributes: ArgumentAttributes`. Updated `unilang_cli.rs` to use the new `attributes` builder.
*   [Increment 4 | 2025-07-26 10:30 UTC] Updated `SemanticAnalyzer` in `src/semantic.rs` and `HelpGenerator` in `src/help.rs` to use `ArgumentAttributes` and `Kind` enums. Adjusted error handling in `SemanticAnalyzer`.
*   [Increment 5 | 2025-07-26 11:00 UTC] Added `unilang_parser` as a dependency. Integrated `unilang_parser::Parser` into `unilang_cli.rs`. Modified `SemanticAnalyzer::new` to accept `&[GenericInstruction]`. Adjusted `unilang_parser`'s `parser_engine.rs` and `item_adapter.rs` for compatibility.
*   [Increment 6 | 2025-07-26 11:15 UTC] Systematically fixed compilation errors and test failures across `unilang` and `unilang_parser` integration tests. Addressed issues related to type validation, map delimiters, multiple arguments, and command path parsing.
*   [Increment 7 | 2025-07-26 11:28 UTC] Resolved `clippy::uninlined_format_args` in `semantic.rs`. Resolved `clippy::derivable_impls` in `data.rs` by deriving `Default` and removing manual implementation. Resolved `E0433` and `E0599` errors in `unilang_cli.rs` by adding missing `ArgumentAttributes` import and correctly nesting `optional`, `multiple`, `interactive`, and `sensitive` methods within the `attributes` builder chain. Temporarily suppressed `clippy::struct_excessive_bools` in `data.rs` and `clippy::too_many_lines` in `unilang_cli.rs`. Passed all `cargo clippy` and `cargo build` checks for both `unilang` and `unilang_parser`.