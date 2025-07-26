# Task Plan: Phase 3 - Architectural Unification (Elaborated)

### Goal
*   To execute Phase 3 of the `unilang` roadmap. This involves a critical refactoring to unify the framework's architecture by removing all legacy parsing components and making the `unilang_parser` crate the single source of truth for syntactic analysis. The plan also includes aligning the core data models (`CommandDefinition`, `ArgumentDefinition`) with the formal specification, updating the help generator, enhancing test coverage for the new features, and updating the `spec.md` document to reflect the final, as-built architecture.

### Ubiquitous Language (Vocabulary)
*   **`unilang_parser`**: The modern, low-level crate for lexical and syntactic analysis.
*   **`GenericInstruction`**: The output of `unilang_parser`, representing a semantically unaware command structure.
*   **`SemanticAnalyzer`**: The component in the `unilang` crate that validates a `GenericInstruction` against the `CommandRegistry`.
*   **`CommandDefinition` / `ArgumentDefinition`**: The core data models representing the command interface.
*   **Architectural Unification**: The process of migrating the entire framework to use the `unilang_parser`.

### Progress
*   **Roadmap Milestone:** Phase 3: Architectural Unification
*   **Primary Editable Crate:** `module/move/unilang`
*   **Overall Progress:** 3/12 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Pre-computation - Reconcile Data Models and Plan Tests
    *   ✅ Increment 2: Refactor `SemanticAnalyzer` to Consume `GenericInstruction`
    *   ✅ Increment 3: Update `unilang_cli` Binary and Core Integration Tests
    *   ⏳ Increment 4: Implement Full Data Models in `unilang/src/data.rs`
    *   ⚫ Increment 5: Update All Code to Use New Data Models
    *   ⚫ Increment 6: Write Failing Integration Test for Command Aliasing
    *   ⚫ Increment 7: Implement Command Alias Resolution in CLI
    *   ⚫ Increment 8: Update `HelpGenerator` and Write Failing Help Tests
    *   ⚫ Increment 9: Implement New Help Output and Fix Tests
    *   ⚫ Increment 10: Create Comprehensive Crate Example
    *   ⚫ Increment 11: Update Formal Specification (`spec.md`)
    *   ⚫ Increment 12: Finalization and Legacy Code Removal

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/move/unilang_parser` (Reason: May require minor adjustments or bug fixes discovered during integration)

### Relevant Context
*   Control Files to Reference:
    *   `module/move/unilang/spec.md`
    *   `module/move/unilang/roadmap.md`
*   Files to Include (for AI's reference):
    *   `module/move/unilang/src/semantic.rs`
    *   `module/move/unilang/src/bin/unilang_cli.rs`
    *   `module/move/unilang/src/data.rs`
    *   `module/move/unilang/src/help.rs`
    *   `module/move/unilang/tests/inc/phase1/full_pipeline_test.rs`
    *   `module/move/unilang/tests/inc/phase2/cli_integration_test.rs`
    *   `module/move/unilang/tests/inc/phase2/help_generation_test.rs`
    *   `module/move/unilang_parser/src/instruction.rs` (to understand `GenericInstruction`)

### Expected Behavior Rules / Specifications
*   The `unilang` crate must exclusively use the `unilang_parser` crate for all command string parsing.
*   The data models in `unilang/src/data.rs` must be updated to match the fields defined in `unilang/spec.md`, Section 3.2 and 3.3.
*   All existing tests must pass after the refactoring, and new tests must be added to cover the new data model fields and behaviors.
*   The `spec.md` file must be updated to reflect the final architecture and data models.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `full_pipeline_test` | Fixed (Monitored) | Was `Failing (New)`, now passing. |
| `cli_integration_test` | Fixed (Monitored) | Was `Failing (New)`, now passing. |

### Crate Conformance Check Procedure
*   Run `timeout 180 cargo test --workspace` and verify it passes with no warnings.
*   Run `timeout 180 cargo clippy --workspace -- -D warnings` and verify it passes with no warnings.

### Increments

##### Increment 1: Pre-computation - Reconcile Data Models and Plan Tests
*   **Goal:** To analyze the codebase, resolve the data model inconsistencies between `spec.md` and `data.rs`, and create a comprehensive Test Matrix for all new features in this phase before writing any implementation code.
*   **Specification Reference:** `spec.md` Sections 3.2, 3.3
*   **Steps:**
    1.  **Analysis:** Compare `unilang/spec.md`, `unilang/src/data.rs`, and the `former` usage in `unilang/src/bin/unilang_cli.rs`. Identify all missing fields in the `CommandDefinition` and `ArgumentDefinition` structs (e.g., `hint`, `status`, `version`, `aliases`, `tags`, etc.).
    2.  **Decision:** Conclude that `data.rs` must be updated to be the single source of truth, fully matching the specification.
    3.  **Test Planning:** Create a detailed Test Matrix in this plan file. The matrix will define test cases for:
        *   Command invocation via alias.
        *   Help output displaying `status`, `version`, `aliases`, and `tags`.
        *   Behavior of `interactive` and `sensitive` argument attributes (conceptual tests for now).
*   **Increment Verification:**
    1.  The Test Matrix is complete and present in this plan file.
    2.  The analysis of data model inconsistencies is documented in the `### Notes & Insights` section.
*   **Commit Message:** "chore(planning): Reconcile data models and create test plan for Phase 3"

##### Increment 2: Refactor `SemanticAnalyzer` to Consume `GenericInstruction`
*   **Goal:** To refactor `unilang::semantic::SemanticAnalyzer` to accept `&[unilang_parser::GenericInstruction]` as input, making it the first core component to adopt the new parser.
*   **Specification Reference:** `spec.md` Section 2.1
*   **Steps:**
    1.  In `unilang/src/semantic.rs`, modify the `SemanticAnalyzer::new` signature to `pub fn new( instructions : &'a [GenericInstruction], registry : &'a CommandRegistry ) -> Self`.
    2.  Update the `SemanticAnalyzer::analyze` method to iterate over `&[GenericInstruction]`.
    3.  Adapt the logic inside `analyze` and `bind_arguments` to read the command path (`instruction.command_path_slices.join(".")`), positional arguments (`instruction.positional_arguments`), and named arguments (`instruction.named_arguments`) from the `GenericInstruction` struct.
    4.  Update the `unilang/tests/inc/phase1/full_pipeline_test.rs` to use `unilang_parser::Parser` to generate `GenericInstruction`s for its test cases, fixing any compilation errors in the test file.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo test -p unilang --test full_pipeline_test`. All tests in this file must pass.
*   **Commit Message:** "refactor(unilang): Migrate SemanticAnalyzer to use unilang_parser::GenericInstruction"

##### Increment 3: Update `unilang_cli` Binary and Core Integration Tests
*   **Goal:** To migrate the main CLI binary and its integration tests to the new unified parsing pipeline.
*   **Specification Reference:** `roadmap.md` M3.1.3, M3.1.4
*   **Steps:**
    1.  In `unilang/src/bin/unilang_cli.rs`, remove any old parsing logic.
    2.  Instantiate `unilang_parser::Parser` and use it to parse the command-line arguments into `GenericInstruction`s.
    3.  Feed the resulting instructions into the now-refactored `SemanticAnalyzer`.
    4.  Fix any compilation errors that arise in the `main` function.
    5.  Run the `cli_integration_test.rs` suite. It is expected to fail.
    6.  Update the assertions in `unilang/tests/inc/phase2/cli_integration_test.rs` to match any changes in error messages or behavior resulting from the new parser.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo test -p unilang --test cli_integration_test`. All tests must pass.
*   **Commit Message:** "refactor(unilang): Migrate unilang_cli and integration tests to new parser"

##### Increment 4: Implement Full Data Models in `unilang/src/data.rs`
*   **Goal:** To update the `CommandDefinition` and `ArgumentDefinition` structs in `data.rs` to be the single source of truth, fully matching the formal specification.
*   **Specification Reference:** `spec.md` Sections 3.2, 3.3
*   **Steps:**
    1.  In `unilang/src/data.rs`, add all missing fields to `CommandDefinition`: `hint`, `status`, `version`, `tags`, `aliases`, `permissions`, `idempotent`.
    2.  In `unilang/src/data.rs`, add all missing fields to `ArgumentDefinition`: `hint`, `default_value`, `aliases`, `tags`.
    3.  In `unilang/src/data.rs`, add the `interactive` and `sensitive` fields to `ArgumentAttributes`.
    4.  Ensure the `former::Former` derive is correctly configured for all new fields, especially `Option` and `Vec` types.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo check -p unilang`. The crate must compile without errors. Compilation errors in other files are expected.
*   **Commit Message:** "feat(unilang): Implement full data models for Command and Argument definitions"

##### Increment 5: Update All Code to Use New Data Models
*   **Goal:** To update all instantiations of `CommandDefinition` and `ArgumentDefinition` across the entire crate to use the new, complete structs.
*   **Steps:**
    1.  In `unilang/src/bin/unilang_cli.rs`, update the `CommandDefinition::former()` calls to include all the new fields (`hint`, `status`, `aliases`, etc.) with sensible default values.
    2.  In all test files (e.g., `full_pipeline_test.rs`, `command_loader_test.rs`, etc.), update the `CommandDefinition` and `ArgumentDefinition` initializations to match the new struct definitions.
*   **Increment Verification:**
    1.  Perform the Crate Conformance Check. All existing tests must pass.
*   **Commit Message:** "refactor(unilang): Update all call sites to use new data models"

##### Increment 6: Write Failing Integration Test for Command Aliasing
*   **Goal:** To create a new, failing integration test that verifies the behavior of command aliases as specified in the Test Matrix (T-ALIAS-1).
*   **Steps:**
    1.  Create a new test file: `unilang/tests/inc/phase3/data_model_features_test.rs`.
    2.  In this file, add a test case that registers a command with an alias (e.g., `e` for `echo`) in `unilang_cli.rs`.
    3.  Write an `assert_cmd` test that invokes the command using its alias (`unilang_cli e`).
    4.  Assert that the command fails, as the alias resolution logic is not yet implemented.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo test --test data_model_features_test -- --nocapture`. The new test `T-ALIAS-1` must fail.
*   **Commit Message:** "test(unilang): Add failing integration test for command aliasing"

##### Increment 7: Implement Command Alias Resolution in CLI
*   **Goal:** To implement the logic that allows commands to be invoked via their aliases, making the failing test from the previous increment pass.
*   **Steps:**
    1.  In `unilang/src/bin/unilang_cli.rs`, before parsing, iterate through the `CommandRegistry` to build a mapping from aliases to canonical command names.
    2.  Check if the first user-provided argument is an alias. If it is, replace it with the canonical command name before passing the arguments to the parser.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo test --test data_model_features_test`. The alias test must now pass.
    2.  Perform the full Crate Conformance Check to ensure no regressions.
*   **Commit Message:** "feat(unilang): Implement command alias resolution in CLI"

##### Increment 8: Update `HelpGenerator` and Write Failing Help Tests
*   **Goal:** To update the help generation tests to expect the new metadata fields, causing them to fail.
*   **Specification Reference:** `roadmap.md` M3.2.3
*   **Steps:**
    1.  In `unilang/tests/inc/phase2/help_generation_test.rs`, update the assertions to check for the presence of "Aliases:", "Status:", and "Version:" in the help output.
    2.  Run the test suite. The `help_generation_test` is now expected to fail because the `HelpGenerator` does not yet produce this output.
    3.  Update the `unilang/tests/inc/phase2/help_generation_test.rs` to use `unilang_parser::Parser` to generate `GenericInstruction`s for its test cases, fixing any compilation errors in the test file.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo test --test help_generation_test`. The tests must fail with assertion errors related to the missing new fields.
*   **Commit Message:** "test(unilang): Update help tests to expect new metadata fields"

##### Increment 9: Implement New Help Output and Fix Tests
*   **Goal:** To enhance the `HelpGenerator` to display the new metadata, making the failing help tests pass.
*   **Steps:**
    1.  In `unilang/src/help.rs`, modify `HelpGenerator::command` to include the new fields (`aliases`, `status`, `version`, etc.) in the formatted string.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo test --test help_generation_test`. All tests must now pass.
*   **Commit Message:** "feat(unilang): Enhance HelpGenerator to display new metadata"

##### Increment 10: Create Comprehensive Crate Example
*   **Goal:** To provide a clear, real-world usage example for developers, demonstrating how to use the framework with its updated features.
*   **Steps:**
    1.  Create a new example file: `unilang/examples/full_cli_example.rs`.
    2.  In this file, define several commands using the full `CommandDefinition` struct, demonstrating namespaces, aliases, various argument kinds, and default values.
    3.  Write a `main` function that registers these commands, parses arguments from `std::env::args()`, and runs the full interpreter pipeline.
    4.  Add clear comments explaining each step of the process.
    5.  Update `Readme.md` to point to the new, more comprehensive example.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo run --example full_cli_example -- .math.add a::5 b::10`. The command should execute successfully and print the correct result.
    2.  Execute `timeout 180 cargo run --example full_cli_example -- help .math.add`. It must show the new, detailed help format.
*   **Commit Message:** "docs(unilang): Add comprehensive example for crate usage"

##### Increment 11: Update Formal Specification (`spec.md`)
*   **Goal:** To update the `spec.md` document to be the single source of truth for the now-unified architecture and complete data models.
*   **Specification Reference:** `roadmap.md` M3.3
*   **Steps:**
    1.  In `unilang/spec.md`, update the tables in sections 3.2 and 3.3 to include all the newly added fields for `CommandDefinition` and `ArgumentDefinition`.
    2.  Revise section 2.1 to formally document the three-phase processing pipeline (Syntactic Analysis -> Semantic Analysis -> Execution).
    3.  Add new top-level sections (e.g., "Global Arguments", "Extensibility Model", "Cross-Cutting Concerns") as placeholders or with initial content as described in the roadmap.
*   **Increment Verification:**
    1.  Manual review of `unilang/spec.md` to confirm it aligns with the current codebase and roadmap goals.
*   **Commit Message:** "docs(unilang): Update spec.md with unified architecture and complete data models"

##### Increment 12: Finalization and Legacy Code Removal
*   **Goal:** To perform a final, holistic review, remove any legacy code, and verify the entire task's output.
*   **Specification Reference:** `roadmap.md` M3.1.1
*   **Steps:**
    1.  Execute `git rm -r module/move/unilang/src/ca/` if the directory exists.
    2.  Search for and remove any `mod ca;` declarations in `unilang/src/lib.rs` or other modules.
    3.  Perform a final self-critique of all changes against the plan's `Goal`.
    4.  Execute the full Crate Conformance Check procedure one last time.
    5.  Run `git status` to ensure the working directory is clean.
*   **Increment Verification:**
    1.  All steps of the Crate Conformance Check must pass.
*   **Commit Message:** "chore(unilang): Finalize architectural unification and remove legacy code"

### Notes & Insights
*   **Data Model Discrepancy:** Initial analysis revealed a significant inconsistency between `spec.md`, `data.rs`, and `unilang_cli.rs`. The `data.rs` structs are missing many fields required by the spec and used by the CLI's builder. This plan prioritizes fixing this by making `data.rs` the source of truth first.
*   **`CommandDefinition.status` Type:** The `spec.md` defines `status` as an `Enum`, but `data.rs` currently uses `String`. For now, the plan will keep it as `String` to avoid widespread changes, but this is noted as a potential future refinement to align strictly with the `Enum` type.

### Test Matrix for New Features
| ID | Feature | Test Case | Expected Behavior |
|---|---|---|---|
| T-ALIAS-1 | Alias Invocation | `unilang_cli e` (where `e` is alias for `echo`) | Executes the `echo` command successfully. |
| T-HELP-1 | Help - Aliases | `unilang_cli help echo` | Help output contains a line like "Aliases: e". |
| T-HELP-2 | Help - Status | `unilang_cli help echo` | Help output contains a line like "Status: stable". |
| T-HELP-3 | Help - Version | `unilang_cli help echo` | Help output contains the version string, e.g., "(v1.0.0)". |
| T-ARG-ATTR-1 | Argument Attributes - Interactive | Command with `interactive: true` argument, argument missing | Modality prompts user for input (conceptual). |
| T-ARG-ATTR-2 | Argument Attributes - Sensitive | Command with `sensitive: true` argument, value provided | Value is masked/redacted in logs/UI (conceptual). |

### Changelog
*   [Initial] Created a highly elaborated task plan for Phase 3, enforcing strict TDD and providing explicit implementation details.
*   [Increment 1 | 2025-07-26T12:59:59.681Z] Completed pre-computation, reconciled data models, and updated test plan.
*   [Increment 2 | 2025-07-26T13:02:39.110Z] Refactored SemanticAnalyzer to use unilang_parser::GenericInstruction.
*   [Increment 3 | 2025-07-26T13:04:14.149Z] Updated unilang_cli binary and core integration tests.
