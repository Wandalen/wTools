# Task Plan: Rename `unilang_instruction_parser` to `unilang_parser`

### Goal
*   Rename the Rust crate `unilang_instruction_parser` to `unilang_parser` across the workspace, updating all references and ensuring the project builds and tests successfully.

### Ubiquitous Language (Vocabulary)
*   **Old Crate Name:** `unilang_instruction_parser`
*   **New Crate Name:** `unilang_parser`
*   **Workspace:** The root `wTools` directory containing multiple Rust crates.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/move/unilang_instruction_parser` (will become `module/move/unilang_parser`)
*   **Overall Progress:** 3/3 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Rename Crate Directory and `Cargo.toml`
    *   ✅ Increment 2: Update Dependent `Cargo.toml` Files
    *   ⏳ Increment 3: Update Source Code References and Final Checks

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/move/unilang` (Reason: Contains `tasks.md` and might have other references)
    *   `module/move/wca` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/strs_tools` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/diagnostics_tools` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/error_tools` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/former` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/former_meta` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/former_types` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/impls_index` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/impls_index_meta` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/inspect_type` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/iter_tools` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/mod_interface` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/mod_interface_meta` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/pth` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/test_tools` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/typing_tools` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/variadic_from` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/core/variadic_from_meta` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/move/willbe` (Reason: Might depend on `unilang_instruction_parser`)
    *   `module/alias/cargo_will` (Reason: Might depend on `unilang_instruction_parser`)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/move/unilang_parser/Cargo.toml`
    *   `module/move/unilang_parser/src/lib.rs`
    *   `module/move/unilang/Cargo.toml`
    *   `module/move/unilang/task/tasks.md`
    *   `Cargo.toml` (workspace root)
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `unilang_instruction_parser` (old name)
    *   `unilang_parser` (new name)
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   N/A

### Expected Behavior Rules / Specifications
*   The crate directory `module/move/unilang_instruction_parser` must be renamed to `module/move/unilang_parser`.
*   The `name` field in `Cargo.toml` for the renamed crate must be `unilang_parser`.
*   All `Cargo.toml` files in the workspace that depend on or reference `unilang_instruction_parser` must be updated to `unilang_parser`.
*   All `use` statements and other code references to `unilang_instruction_parser` within the source code must be updated to `unilang_parser`.
*   The project must compile and pass all tests (`cargo test --workspace`) without errors or new warnings after the renaming.
*   The `tasks.md` file must be updated to reflect the new crate name.

### Tests
| Test ID | Status | Notes |
|---|---|---|

### Crate Conformance Check Procedure
*   For all `Editable Crates`:
    1.  Execute `timeout 90 cargo test -p {crate_name} --all-targets`.
    2.  Analyze the output for any test failures. If failures occur, initiate `Critical Log Analysis`.
    3.  Execute `timeout 90 cargo clippy -p {crate_name} -- -D warnings`.
    4.  Analyze the output for any linter warnings. If warnings occur, initiate `Linter Fix & Regression Check Procedure`.
    5.  Execute `cargo clean -p {crate_name}` followed by `timeout 90 cargo build -p {crate_name}`. Critically analyze the build output for any unexpected debug prints from procedural macros. If any are found, the check fails; initiate the `Critical Log Analysis` procedure.

### Increments
(Note: The status of each increment is tracked in the `### Progress` section.)
##### Increment 1: Rename Crate Directory and `Cargo.toml`
*   **Goal:** Rename the `unilang_instruction_parser` crate directory and update its `Cargo.toml` file.
*   **Specification Reference:** User feedback.
*   **Steps:**
    *   Step 1: Use `git mv` to rename the directory `module/move/unilang_instruction_parser` to `module/move/unilang_parser`.
    *   Step 2: Read the `Cargo.toml` file of the newly renamed crate (`module/move/unilang_parser/Cargo.toml`).
    *   Step 3: Update the `name` field in `module/move/unilang_parser/Cargo.toml` from `unilang_instruction_parser` to `unilang_parser`.
    *   Step 4: Update the `documentation`, `repository`, and `homepage` fields in `module/move/unilang_parser/Cargo.toml`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo check -p unilang_parser` to ensure the renamed crate can be checked. (Note: This may fail due to workspace inconsistencies, which will be addressed in the next increment.)
*   **Commit Message:** `refactor(unilang_parser): Rename crate directory and Cargo.toml`

##### Increment 2: Update Dependent `Cargo.toml` Files
*   **Goal:** Update all `Cargo.toml` files in the workspace that depend on or reference `unilang_instruction_parser`.
*   **Specification Reference:** User feedback.
*   **Steps:**
    *   Step 1: Search for all `Cargo.toml` files in the workspace that contain the string `unilang_instruction_parser`.
    *   Step 2: For each identified `Cargo.toml` file, replace all occurrences of `unilang_instruction_parser` with `unilang_parser`.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo check --workspace` to ensure the entire workspace can be checked.
*   **Commit Message:** `refactor(unilang_parser): Update Cargo.toml dependencies`

##### Increment 3: Update Source Code References and Final Checks
*   **Goal:** Update all source code references to the old crate name and perform final verification.
*   **Specification Reference:** User feedback.
*   **Steps:**
    *   Step 1: Search for all Rust source files (`.rs`) in the workspace that contain the string `unilang_instruction_parser`.
    *   Step 2: For each identified `.rs` file, replace all occurrences of `unilang_instruction_parser` with `unilang_parser`.
    *   Step 3: Update the `tasks.md` file in `module/move/unilang/task/tasks.md` to reflect the new crate name in the completed task entry.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test --workspace` to ensure all tests pass. (Note: This may fail due to external system dependencies.)
    *   Run `timeout 90 cargo clippy --workspace -- -D warnings` to ensure no new lints. (Note: This may fail due to external system dependencies.)
    *   Run `git status` to ensure the working directory is clean.
*   **Commit Message:** `refactor(unilang_parser): Update source code references and finalize rename`

### Task Requirements
*   The crate `unilang_instruction_parser` must be fully renamed to `unilang_parser`.
*   All references to the old name must be updated.
*   The project must compile and pass all tests without errors or new warnings.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   All new APIs must be async.
*   All new or modified production code must be accompanied by automated tests within the same increment.
*   All automated test files must be placed within the canonical `tests` directory at the crate root.
*   Prefer writing integration-style tests within the `tests` directory to validate the public-facing API of a crate.
*   Each test must be focused and verify only a single, specific aspect of behavior.
*   All functional tests for a code unit that accepts parameters must explicitly provide a value for every parameter.
*   If a code unit has parameters with default values, their behavior must be verified in a dedicated, isolated test (`Default Value Equivalence Testing`).
*   When an increment explicitly involves writing automated tests, the Detailed Planning phase for that increment must include the creation of a Test Matrix.
*   Each test file must begin with a file-level doc comment containing the relevant Test Matrix from the plan file.
*   Each individual test function must have a doc comment that clearly states its specific purpose and provides a mandatory link back to the Test Combination ID it covers.
*   Use a consistent alias `the_module` to refer to the aggregating crate itself within the test context to prevent `E0433: failed to resolve` errors.
*   Root-level test files must begin with `#![ allow( unused_imports ) ]`.
*   Non-root (Included) test files must begin with `use super::*;`.
*   When creating a new module file, always add the corresponding module declaration (`mod my_module;`) to its parent module file *first*.
*   Strive to keep files under approximately 1000 lines of code.
*   Code generated by procedural macros must use paths that correctly resolve within the target crate's specific module structure.
*   Structure your crate's modules primarily by feature or by architectural layer.
*   Documentation should add extra value by explaining why and what for—not by repeating how the code works.
*   When implementing a feature composed of several distinct but related sub-tasks or components within an increment, fully complete one sub-task before beginning the next step.
*   Developing procedural macros effectively involves ensuring the generated code is correct and behaves as expected *before* writing the macro itself.
*   Use strictly 2 spaces over tabs for consistent indentation.
*   When chaining method calls, start each method on a new line directly below the chain start, without additional indentation.
*   When breaking a line due to a method chain (using `.`) or namespace access (using `::`), maintain the same indentation as the first line.
*   Include a space before and after `:`, `=`, and operators, excluding the namespace operator `::`.
*   Space After Opening Symbols: After opening `{`, `(`, `<`, `[`, and `|`, insert a space if they are followed by content on the same line.
*   Space Before Closing Symbols: Before closing `|`, `]`, `}`, `)`, and `>`, insert a space if they are preceded by content on the same line.
*   No Spaces Around Angle Brackets: When using angle brackets `<` and `>` for generic type parameters, do not include spaces between the brackets and the type parameters.
*   Attributes: Place each attribute on its own line; ensure spaces immediately inside both `[]` and `()` if present; ensure a space between the attribute name and the opening parenthesis.
*   Each attribute must be placed on its own line, and the entire block of attributes must be separated from the item itself by a newline.
*   The `where` keyword should start on a new line; each parameter in the `where` clause should start on a new line.
*   When defining a trait implementation (`impl`) for a type, if the trait and the type it is being implemented for do not fit on the same line, the trait should start on a new line.
*   Function parameters should be listed with one per line; the return type should start on a new line; the `where` clause should start on a new line.
*   When using `match` expressions, place the opening brace `{` for multi-line blocks on a new line after the match arm.
*   No spaces between `&` and the lifetime specifier.
*   Avoid complex, multi-level inline nesting.
*   Keep lines under 110 characters.
*   Inline comments (`//`) should start with a space following the slashes.
*   Comments should primarily explain the "why" or clarify non-obvious aspects of the *current* code. Do not remove existing task-tracking comments.
*   Use structured `Task Markers` in source code comments to track tasks, requests, and their resolutions.
*   When addressing an existing task comment, add a new comment line immediately below it, starting with `// aaa:`.
*   For declarative macros, `=>` token should reside on a separate line from macro pattern.
*   For declarative macros, allow `{{` and `}}` on the same line to improve readability.
*   For declarative macros, you can place the macro pattern and its body on the same line if they are short enough.
*   All dependencies must be defined in `[workspace.dependencies]` in the root `Cargo.toml` without features; individual crates inherit and specify features.
*   Lint configurations must be defined centrally in the root `Cargo.toml` using `[workspace.lints]`; individual crates inherit via `[lints] workspace = true`.
*   Avoid using attributes for documentation; use ordinary doc comments `//!` and `///`.

### Assumptions
*   The `pkg-config` issue is an environment configuration problem and not a code issue within the target crates.
*   The `unilang_instruction_parser` crate is the only one being renamed.

### Out of Scope
*   Resolving the `pkg-config` system dependency issue.
*   Any other refactoring or feature implementation not directly related to the renaming.

### External System Dependencies
*   `pkg-config` (required for `yeslogic-fontconfig-sys` which is a transitive dependency of `wtools`)

### Notes & Insights
*   N/A

### Changelog
*   `[User Feedback | 2025-07-20 21:31 UTC]` User requested renaming `unilang_instruction_parser` to `unilang_parser`.
*   `[Increment 1 | 2025-07-20 21:34 UTC]` Renamed crate directory `module/move/unilang_instruction_parser` to `module/move/unilang_parser`.
*   `[Increment 1 | 2025-07-20 21:35 UTC]` Updated `name`, `documentation`, `repository`, and `homepage` fields in `module/move/unilang_parser/Cargo.toml`.
*   `[Increment 2 | 2025-07-20 21:36 UTC]` Updated `module/move/unilang/Cargo.toml` to reference `unilang_parser`.
*   `[Increment 2 | 2025-07-20 21:37 UTC]` Updated root `Cargo.toml` to explicitly list `module/move` members, including `unilang_parser`.
*   `[Increment 3 | 2025-07-20 21:39 UTC]` Updated references in `module/move/unilang/tests/inc/integration_tests.rs`.
*   `[Increment 3 | 2025-07-20 21:39 UTC]` Updated references in `module/move/unilang/tests/inc/phase1/full_pipeline_test.rs`.
*   `[Increment 3 | 2025-07-20 21:39 UTC]` Updated references in `module/move/unilang/src/semantic.rs`.
*   `[Increment 3 | 2025-07-20 21:39 UTC]` Updated references in `module/move/unilang/src/error.rs`.
*   `[Increment 3 | 2025-07-20 21:39 UTC]` Updated references in `module/move/unilang/tests/inc/phase2/runtime_command_registration_test.rs`.
*   `[Increment 3 | 2025-07-20 21:39 UTC]` Updated references in `module/move/unilang/tests/inc/phase2/collection_types_test.rs`.
*   `[Increment 3 | 2025-07-20 21:39 UTC]` Updated references in `module/move/unilang/tests/inc/phase2/argument_types_test.rs`.
*   `[Increment 3 | 2025-07-20 21:40 UTC]` Updated references in `module/move/unilang/tests/inc/phase2/complex_types_and_attributes_test.rs`.
*   `[Increment 3 | 2025-07-20 21:40 UTC]` Updated references in `module/move/unilang/src/bin/unilang_cli.rs`.
*   `[Increment 3 | 2025-07-20 21:40 UTC]` Updated references in `module/move/unilang_parser/tests/tests.rs`.
*   `[Increment 3 | 2025-07-20 21:40 UTC]` Updated references in `module/move/unilang_parser/tests/parser_config_entry_tests.rs`.
*   `[Increment 3 | 2025-07-20 21:40 UTC]` Updated references in `module/move/unilang_parser/tests/error_reporting_tests.rs`.
*   `[Increment 3 | 2025-07-20 21:40 UTC]` Updated references in `module/move/unilang_parser/tests/syntactic_analyzer_command_tests.rs`.
*   `[Increment 3 | 2025-07-20 21:41 UTC]` Updated references in `module/move/unilang_parser/tests/comprehensive_tests.rs`.
*   `[Increment 3 | 2025-07-20 21:41 UTC]` Updated references in `module/move/unilang_parser/tests/command_parsing_tests.rs`.
*   `[Increment 3 | 2025-07-20 21:41 UTC]` Updated references in `module/move/unilang_parser/tests/argument_parsing_tests.rs`.
*   `[Increment 3 | 2025-07-20 21:41 UTC]` Updated references in `module/move/unilang_parser/tests/spec_adherence_tests.rs`.
*   `[Increment 3 | 2025-07-20 21:41 UTC]` Renamed `module/move/unilang_parser/examples/unilang_instruction_parser_basic.rs` to `module/move/unilang_parser/examples/unilang_parser_basic.rs`.
*   `[Increment 3 | 2025-07-20 21:41 UTC]` Updated references in `module/move/unilang_parser/examples/unilang_parser_basic.rs`.
*   `[Increment 3 | 2025-07-20 21:42 UTC]` Updated references in `module/move/unilang_parser/src/lib.rs`.
*   `[Increment 3 | 2025-07-20 21:42 UTC]` Updated references in `module/move/unilang/task/tasks.md`.