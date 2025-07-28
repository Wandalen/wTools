# Task Plan: Phase 3 - Audit, Enhance, and Finalize

### Goal
*   To rigorously audit and complete Phase 3 of the `unilang` roadmap. This involves verifying the architectural unification, resolving any remaining bugs, significantly enhancing test coverage to be comprehensive, improving documentation for clarity and completeness, and ensuring the final product is robust and maintainable before removing all legacy code.

### Ubiquitous Language (Vocabulary)
*   **`unilang_parser`**: The modern, low-level crate for lexical and syntactic analysis.
*   **`GenericInstruction`**: The output of `unilang_parser`, representing a semantically unaware command structure.
*   **`SemanticAnalyzer`**: The component in the `unilang` crate that validates a `GenericInstruction` against the `CommandRegistry`.
*   **`CommandDefinition` / `ArgumentDefinition`**: The core data models representing the command interface.
*   **Architectural Unification**: The process of migrating the entire framework to use the `unilang_parser`.

### Progress
*   **Roadmap Milestone:** Phase 3: Architectural Unification (Audit & Completion)
*   **Primary Editable Crate:** `module/move/unilang`
*   **Overall Progress:** 2/12 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Audit Existing Codebase and Test Structure
    *   ✅ Increment 2: Audit Core Refactoring (Increments 1-5)
    *   ⏳ Increment 3: Audit Feature Implementation (Increments 6-10)
    *   ⚫ Increment 4: Audit Documentation and Examples (Increments 11-12)
    *   ⚫ Increment 5: Focused Debugging for `diagnostics_tools` Doctest
    *   ⚫ Increment 6: Enhance Test Coverage for Data Models
    *   ⚫ Increment 7: Add Tests for Argument Attributes
    *   ⚫ Increment 8: Enhance Crate and Module Documentation
    *   ⚫ Increment 9: Implement Missing `From` Trait for `Error`
    *   ⚫ Increment 10: Remove Legacy `ca` Module
    *   ⚫ Increment 11: Final Conformance and Verification
    *   ⚫ Increment 12: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/move/unilang_parser` (Reason: May require minor adjustments or bug fixes discovered during integration)

### Relevant Context
*   Control Files to Reference:
    *   `module/move/unilang/spec.md`
    *   `module/move/unilang/roadmap.md`
    *   `module/move/unilang/task/phase3.md` (for auditing purposes)
*   Files to Include (for AI's reference):
    *   `module/move/unilang/src/lib.rs`
    *   `module/move/unilang/src/semantic.rs`
    *   `module/move/unilang/src/bin/unilang_cli.rs`
    *   `module/move/unilang/src/data.rs`
    *   `module/move/unilang/src/help.rs`
    *   `module/move/unilang/src/registry.rs`
    *   `module/move/unilang/tests/` (directory)

### Expected Behavior Rules / Specifications
*   The `unilang` crate must exclusively use the `unilang_parser` crate for all command string parsing.
*   All legacy parsing code (specifically the `ca` module) must be removed.
*   Test coverage must be comprehensive for all public APIs and features, including data model fields, aliases, and argument attributes.
*   All documentation must be accurate, concise, and provide clear usage examples.
*   All tests must pass, and all linter checks must be clean before completion.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `diagnostics_tools` doctest | Failing (New) | From previous plan: `Test executable succeeded, but it's marked should_panic`. |
| `unilang::tests::inc::phase1::full_pipeline_test` | Fixed (Monitored) | Was `Failing (New)`. Test target issue resolved by running `cargo test -p unilang --test tests`. |

### Crate Conformance Check Procedure
*   Run `timeout 180 cargo test -p unilang -- --nocapture` and verify it passes with no warnings.
*   Run `timeout 180 cargo test -p unilang_parser -- --nocapture` and verify it passes with no warnings.
*   Run `timeout 180 cargo clippy -p unilang -- -D warnings -A clippy::too-many-lines` and verify it passes with no warnings.
*   Run `timeout 180 cargo clippy -p unilang_parser -- -D warnings -A clippy::too-many-lines` and verify it passes with no warnings.

### Increments

##### Increment 1: Audit Existing Codebase and Test Structure
*   **Goal:** To get a baseline understanding of the current state of the `unilang` crate by reviewing its structure, dependencies, and existing test suites.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Use `list_files` to recursively list the contents of `module/move/unilang/src/`.
    2.  Use `list_files` to recursively list the contents of `module/move/unilang/tests/`.
    3.  Use `read_file` to read `module/move/unilang/Cargo.toml`.
    4.  Use `read_file` to read `module/move/unilang/src/lib.rs`.
    5.  Use `read_file` to read `module/move/unilang/tests/inc/mod.rs`.
    6.  Based on the output of the previous steps, formulate an analysis of the project structure, dependencies, and test organization.
    7.  Use `insert_content` to add the analysis to the `### Notes & Insights` section of `task_plan.md`.
    8.  Perform Increment Verification.
*   **Increment Verification:**
    1.  Confirm that the `list_files` and `read_file` commands were executed successfully.
    2.  Confirm that the analysis has been added to the `### Notes & Insights` section by reading the plan file.
*   **Commit Message:** "chore(audit): Review unilang crate structure and tests"

##### Increment 2: Audit Core Refactoring (Increments 1-5)
*   **Goal:** To verify the completion and correctness of the core refactoring work described in Increments 1-5 of the original `phase3.md` plan.
*   **Specification Reference:** `phase3.md` (Increments 1-5)
*   **Steps:**
    1.  **Audit `SemanticAnalyzer`:**
        *   Read `module/move/unilang/src/semantic.rs`.
        *   Read `module/move/unilang/tests/inc/phase1/full_pipeline_test.rs`.
        *   Verify that `SemanticAnalyzer`'s `new` method accepts `&[GenericInstruction]` and that `analyze` iterates over it.
        *   Verify that `full_pipeline_test.rs` uses `unilang_parser::Parser` to generate `GenericInstruction`s.
    2.  **Audit `unilang_cli`:**
        *   Read `module/move/unilang/src/bin/unilang_cli.rs`.
        *   Verify that it instantiates `unilang_parser::Parser` and feeds `GenericInstruction`s to `SemanticAnalyzer`.
    3.  **Audit Data Models:**
        *   Read `module/move/unilang/src/data.rs`.
        *   Read `module/move/unilang_meta/spec.md`.
        *   Compare `CommandDefinition` and `ArgumentDefinition` structs in `data.rs` against sections 3.2 and 3.3 of `spec.md` to ensure all fields are present.
    4.  **Audit Call Sites:**
        *   Perform a `search_files` for `CommandDefinition::former()` within `module/move/unilang/src/` with `file_pattern` `*.rs`.
    5.  Use `insert_content` to add any discrepancies or incomplete work found during the audit to `### Notes & Insights`.
    6.  Perform Increment Verification.
*   **Increment Verification:**
    1.  Confirm that all audit steps were executed and findings documented.
    2.  Execute `timeout 180 cargo test -p unilang --test tests -- --nocapture`. All tests must pass.
*   **Commit Message:** "chore(audit): Verify completion of core refactoring"

##### Increment 3: Audit Feature Implementation (Increments 6-10)
*   **Goal:** To verify the completion and correctness of the feature work (aliasing, help generation, bug fixes) from Increments 6-10 of the original plan.
*   **Specification Reference:** `phase3.md` (Increments 6-10)
*   **Steps:**
    1.  **Audit Aliasing:**
        *   Read `module/move/unilang/tests/inc/phase3/data_model_features_test.rs`.
        *   Read `module/move/unilang/src/bin/unilang_cli.rs`.
        *   Verify that the alias test exists and that the resolution logic is implemented as described in the original plan (lines 152-154 of `phase3.md`).
    3.  **Audit Help Generator:**
        *   Read `module/move/unilang/src/help.rs`.
        *   Read `module/move/unilang/tests/inc/phase2/help_generation_test.rs`.
        *   Verify that the help output includes the new metadata fields (`Aliases:`, `Status:`, `Version:`) and that tests assert this. (Note: The original plan's `Notes & Insights` already stated these tests were passing, so this is a re-verification).
    4.  **Audit Registry Fix:**
        *   Read `module/move/unilang/src/registry.rs`.
        *   Verify that the key generation logic for `commands` and `routines` is consistent and correct, as described in the original plan's notes (lines 250-252 of `phase3.md`).
    5.  Use `insert_content` to add any discrepancies or incomplete work found during the audit to `### Notes & Insights`.
    6.  Perform Increment Verification.
*   **Increment Verification:**
    1.  Confirm that all audit steps were executed and findings documented.
    2.  Execute `timeout 180 cargo test -p unilang --test data_model_features_test --test help_generation_test -- --nocapture`. All tests must pass.
*   **Commit Message:** "chore(audit): Verify completion of feature implementations"

##### Increment 4: Audit Documentation and Examples (Increments 11-12)
*   **Goal:** To verify the completion and quality of the documentation and examples from Increments 11-12 of the original plan.
*   **Specification Reference:** `phase3.md` (Increments 11-12)
*   **Steps:**
    1.  **Audit Example:** Read `unilang/examples/full_cli_example.rs`. Verify it is comprehensive and demonstrates the new features.
    2.  **Audit `Readme.md`:** Read `unilang/Readme.md`. Verify it points to the new example.
    3.  **Audit `spec.md`:** Read `unilang/spec.md`. Verify it has been updated with the new architecture and data models as described.
    4.  Document any discrepancies.
*   **Increment Verification:**
    1.  The audit is complete and findings are documented.
    2.  Run `timeout 180 cargo run --example full_cli_example -- help`. The command must execute successfully.
*   **Commit Message:** "chore(audit): Verify completion of documentation and examples"

##### Increment 5: Focused Debugging for `diagnostics_tools` Doctest
*   **Goal:** To diagnose and fix the `Failing (Stuck)` doctest in `diagnostics_tools`.
*   **Specification Reference:** `phase3.md` (Tests section)
*   **Steps:**
    1.  Locate the `diagnostics_tools` doctest. Based on the file list, this is likely in `crates_tools`. I will search for it.
    2.  Analyze the test code and the `should_panic` attribute. The error "Test executable succeeded, but it's marked should_panic" means the code inside the test *did not* panic as expected.
    3.  Hypothesize the cause: The underlying code has been fixed and no longer panics, but the test was not updated.
    4.  Propose a fix: Remove the `#[should_panic]` attribute and adjust the test to assert the successful (non-panicking) outcome.
    5.  Apply the fix using `search_and_replace`.
*   **Increment Verification:**
    1.  Perform the Crate Conformance Check. The previously failing doctest must now pass.
*   **Commit Message:** "fix(diagnostics_tools): Correct doctest that no longer panics"

##### Increment 6: Enhance Test Coverage for Data Models
*   **Goal:** To add new integration tests that explicitly cover the behavior of the new fields in `CommandDefinition` and `ArgumentDefinition`.
*   **Specification Reference:** `spec.md` Sections 3.2, 3.3
*   **Steps:**
    1.  In `unilang/tests/inc/phase3/data_model_features_test.rs`, add new test cases.
    2.  **Test `hint`:** Add a test to verify the `hint` for a command and an argument appears in the help output.
    3.  **Test `tags`:** Add a test that registers a command with `tags` and conceptually verifies they are stored (e.g., by checking the `CommandDefinition` struct).
    4.  **Test `version`:** Verify the command's `version` appears in the help output.
    5.  **Test `status`:** Verify the command's `status` appears in the help output.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo test -p unilang --test data_model_features_test`. All tests, including the new ones, must pass.
*   **Commit Message:** "test(unilang): Add integration tests for new data model fields"

##### Increment 7: Add Tests for Argument Attributes
*   **Goal:** To add conceptual or unit tests for the `interactive` and `sensitive` argument attributes.
*   **Specification Reference:** `spec.md` Section 3.3
*   **Steps:**
    1.  In `unilang/tests/inc/phase3/data_model_features_test.rs`, add new test cases.
    2.  **Test `interactive`:** Create a test that defines a command with an `interactive` argument. The test will not be able to test the actual prompting, but it can verify that the `interactive` flag is correctly set on the `ArgumentDefinition` struct after registration.
    3.  **Test `sensitive`:** Create a test similar to the one for `interactive`, verifying the `sensitive` flag is correctly set.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo test -p unilang --test data_model_features_test`. All tests must pass.
*   **Commit Message:** "test(unilang): Add tests for interactive and sensitive argument attributes"

##### Increment 8: Enhance Crate and Module Documentation
*   **Goal:** To review and improve the documentation for the `unilang` crate, ensuring it is clear, concise, and reflects the new architecture.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Read `unilang/src/lib.rs`. Add or update the crate-level documentation (`//!`) to explain the three-phase pipeline and the purpose of the crate.
    2.  Read `unilang/src/data.rs`. Add doc comments (`///`) to the `CommandDefinition` and `ArgumentDefinition` structs and their fields, explaining their purpose.
    3.  Read `unilang/src/semantic.rs` and `unilang/src/help.rs`. Add module-level documentation explaining their roles.
*   **Increment Verification:**
    1.  Run `timeout 180 cargo doc -p unilang --no-deps`. The command should complete without errors or warnings.
*   **Commit Message:** "docs(unilang): Enhance crate and module-level documentation"

##### Increment 9: Implement Missing `From` Trait for `Error`
*   **Goal:** To implement `From<wtools::error::BasicError>` for `unilang::Error` to improve error handling ergonomics.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Read `unilang/src/lib.rs` to locate the `Error` enum/struct.
    2.  Add a new variant to the `Error` enum, for example `Basic( wtools::error::BasicError )`.
    3.  Implement `From<wtools::error::BasicError>` for `Error`.
    4.  Search for `?` operators that could be simplified by this implementation and refactor them.
*   **Increment Verification:**
    1.  Perform the Crate Conformance Check.
*   **Commit Message:** "feat(unilang): Implement From<BasicError> for unilang::Error"

##### Increment 10: Remove Legacy `ca` Module
*   **Goal:** To remove the legacy `ca` module and all its related code from the `unilang` crate.
*   **Specification Reference:** `roadmap.md` M3.1.1
*   **Steps:**
    1.  Check if the directory `module/move/unilang/src/ca/` exists using `list_files`.
    2.  If it exists, execute `git rm -r module/move/unilang/src/ca/`.
    3.  In `unilang/src/lib.rs`, use `search_and_replace` to remove the `pub mod ca;` declaration.
*   **Increment Verification:**
    1.  Perform the Crate Conformance Check. The build must succeed, proving the `ca` module is no longer needed.
*   **Commit Message:** "refactor(unilang): Remove legacy 'ca' module"

##### Increment 11: Final Conformance and Verification
*   **Goal:** To perform a final, holistic check of the entire crate to ensure everything is correct and no regressions have been introduced.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Perform a final self-critique of all changes against the plan's `Goal`.
    2.  Execute the full Crate Conformance Check procedure one last time.
    3.  Run `git status` to ensure the working directory is clean.
*   **Increment Verification:**
    1.  All steps of the Crate Conformance Check must pass.
*   **Commit Message:** "chore(unilang): Final conformance check"

##### Increment 12: Finalization
*   **Goal:** To finalize the task.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Perform the `Finalization Increment Verification` procedure from the design rules.
*   **Increment Verification:**
    1.  All checks must pass.
*   **Commit Message:** "chore(task): Complete Phase 3 audit and finalization"

### Notes & Insights
*   This plan is an "audit and enhance" plan. It assumes the previous `phase3.md` plan was mostly executed but requires verification and supplementation.
*   The `diagnostics_tools` doctest failure is a high-priority fix.
*   Test coverage for the new data model fields is critical for ensuring the framework is robust.
*   **Audit Finding (Structure):** The `unilang` crate source has a flat module structure (`data`, `error`, `help`, etc.) and a single binary `unilang_cli`. The legacy `ca` module mentioned in the original plan does not appear to be declared in `src/lib.rs`.
*   **Audit Finding (Dependencies):** `Cargo.toml` shows a dependency on `unilang_parser` with a comment indicating it was "Temporarily removed due to Cargo resolution issues". This is a critical point to investigate during the audit of the core refactoring.
*   **Audit Finding (Tests):** Tests are well-organized into `phase1`, `phase2`, and `phase3` modules, reflecting the project's roadmap. This structure will be useful for auditing progress.
*   **Audit Finding (Data Models):** `CommandDefinition` in `module/move/unilang/src/data.rs` is missing `deprecation_message`, `http_method_hint`, and `examples` fields compared to `module/move/unilang_meta/spec.md`. The `namespace` and `version` fields are `Option<String>` in `data.rs` but `String` in `spec.md`. The `status` discrepancy is already noted.
*   **Audit Finding (Call Sites):** The `CommandDefinition::former()` calls in `module/move/unilang/src/bin/unilang_cli.rs` for `math_add_def`, `math_sub_def`, `greet_def`, and `config_set_def` are not fully updated with all new fields (`tags`, `permissions`, `idempotent`, and `namespace`/`aliases` for `greet_def`). This indicates Increment 5 of the original plan was incomplete.

### Changelog
*   [Initial] Created a new, comprehensive plan to audit, enhance, and finalize Phase 3.
*   [Increment 1 | 2025-07-28T17:54:17.725Z] Reviewed unilang crate structure and tests.
*   [Increment 2 | 2025-07-28T17:56:34.391Z] Identified `full_pipeline_test` as not being a direct test target.
*   [Increment 2 | 2025-07-28T17:57:44.823Z] Verified core refactoring (SemanticAnalyzer, unilang_cli, Data Models, Call Sites) and confirmed all tests pass.