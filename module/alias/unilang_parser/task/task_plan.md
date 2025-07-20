# Task Plan: Convert `unilang_instruction_parser` to Alias and Relocate `unilang_parser`

### Goal
*   Move the `unilang_parser` crate from `module/move` to `module/alias`.
*   Create a new alias crate named `unilang_instruction_parser` in `module/alias` that re-exports `unilang_parser`.
*   Ensure all workspace references are updated and the project builds and tests successfully.

### Ubiquitous Language (Vocabulary)
*   **Old Location:** `module/move/unilang_parser`
*   **New Location:** `module/alias/unilang_parser`
*   **Alias Crate:** `unilang_instruction_parser` (will be created in `module/alias`)
*   **Target Crate:** `unilang_parser`
*   **Workspace:** The root `wTools` directory containing multiple Rust crates.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/move/unilang_parser` (will become `module/alias/unilang_parser`)
*   **Overall Progress:** 2/3 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Relocate `unilang_parser` and Update References
    *   ✅ Increment 2: Create `unilang_instruction_parser` Alias Crate
    *   ⏳ Increment 2.1: Focused Debugging: `git mv` "source directory is empty" error
    *   ⚫ Increment 3: Finalize and Clean Up

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/move/unilang` (Reason: Contains `tasks.md` and depends on `unilang_parser`)
    *   `module/move/wca` (Reason: Might depend on `unilang_parser`)
    *   `module/core/strs_tools` (Reason: Might depend on `unilang_parser`)
    *   `module/core/diagnostics_tools` (Reason: Might depend on `unilang_parser`)
    *   `module/core/error_tools` (Reason: Might depend on `unilang_parser`)
    *   `module/core/former` (Reason: Might depend on `unilang_parser`)
    *   `module/core/former_meta` (Reason: Might depend on `unilang_parser`)
    *   `module/core/former_types` (Reason: Might depend on `unilang_parser`)
    *   `module/core/impls_index` (Reason: Might depend on `unilang_parser`)
    *   `module/core/impls_index_meta` (Reason: Might depend on `unilang_parser`)
    *   `module/core/inspect_type` (Reason: Might depend on `unilang_parser`)
    *   `module/core/iter_tools` (Reason: Might depend on `unilang_parser`)
    *   `module/core/mod_interface` (Reason: Might depend on `unilang_parser`)
    *   `module/core/mod_interface_meta` (Reason: Might depend on `unilang_parser`)
    *   `module/core/pth` (Reason: Might depend on `unilang_parser`)
    *   `module/core/test_tools` (Reason: Might depend on `unilang_parser`)
    *   `module/core/typing_tools` (Reason: Might depend on `unilang_parser`)
    *   `module/core/variadic_from` (Reason: Might depend on `unilang_parser`)
    *   `module/core/variadic_from_meta` (Reason: Might depend on `unilang_parser`)
    *   `module/move/willbe` (Reason: Might depend on `unilang_parser`)
    *   `module/alias/cargo_will` (Reason: Might depend on `unilang_parser`)
    *   `module/alias/unilang_instruction_parser` (Reason: New alias crate to be created)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/alias/unilang_parser/Cargo.toml`
    *   `module/alias/unilang_parser/src/lib.rs`
    *   `module/move/unilang/Cargo.toml`
    *   `module/move/unilang/task/tasks.md`
    *   `Cargo.toml` (workspace root)
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `unilang_parser`
    *   `unilang_instruction_parser` (alias)
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   N/A

### Expected Behavior Rules / Specifications
*   The `unilang_parser` crate directory must be moved from `module/move/unilang_parser` to `module/alias/unilang_parser`.
*   A new crate `module/alias/unilang_instruction_parser` must be created.
*   The `module/alias/unilang_instruction_parser` crate must re-export `unilang_parser`.
*   All `Cargo.toml` files and source code references must be updated to reflect the new locations and alias.
*   The project must compile and pass all tests (`cargo test --workspace`) without errors or new warnings after the changes.
*   The `tasks.md` file must be updated to reflect the new alias structure.

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
##### Increment 1: Relocate `unilang_parser` and Update References
*   **Goal:** Move `unilang_parser` to `module/alias` and update direct path references.
*   **Specification Reference:** User feedback.
*   **Steps:**
    *   Step 1: Use `git mv` to rename the directory `module/move/unilang_parser` to `module/alias/unilang_parser`.
    *   Step 2: Read the root `Cargo.toml` file.
    *   Step 3: Update the `members` list in the root `Cargo.toml` to reflect the new path for `unilang_parser`.
    *   Step 4: Search for all `Cargo.toml` files in the workspace that contain the string `module/move/unilang_parser`.
    *   Step 5: For each identified `Cargo.toml` file, replace `module/move/unilang_parser` with `module/alias/unilang_parser`.
    *   Step 6: Perform Increment Verification.
    *   Step 7: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo check --workspace` to ensure the entire workspace can be checked.
*   **Commit Message:** `refactor(unilang_parser): Relocate to module/alias and update path references`

##### Increment 2: Create `unilang_instruction_parser` Alias Crate
*   **Goal:** Create the `unilang_instruction_parser` alias crate that re-exports `unilang_parser`.
*   **Specification Reference:** User feedback.
*   **Steps:**
    *   Step 1: Create a new directory `module/alias/unilang_instruction_parser`.
    *   Step 2: Create `module/alias/unilang_instruction_parser/Cargo.toml` with `name = "unilang_instruction_parser"` and a dependency on `unilang_parser`.
    *   Step 3: Create `module/alias/unilang_instruction_parser/src/lib.rs` and add `pub use unilang_parser::*;` to re-export the target crate.
    *   Step 4: Add `module/alias/unilang_instruction_parser` to the `members` list in the root `Cargo.toml`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo check --workspace` to ensure the entire workspace can be checked.
*   **Commit Message:** `feat(unilang_instruction_parser): Create alias crate for unilang_parser`

##### Increment 2.1: Focused Debugging: `git mv` "source directory is empty" error
*   **Goal:** Diagnose and fix the `git mv` error "fatal: source directory is empty" when moving `module/move/unilang_parser`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Apply Problem Decomposition: The problem is that `git mv` reports the source directory as empty, preventing the move.
    *   Step 2: Isolate the test case: The specific directory is `module/move/unilang_parser`.
    *   Step 3: Add targeted debug logging: Use `ls -la module/move/unilang_parser/` to see its contents, including hidden files, and `du -sh module/move/unilang_parser/` to check its size.
    *   Step 4: Formulate and test a hypothesis:
        *   Hypothesis 1: The directory is indeed empty, and `ls` is showing a stale cache or a symbolic link.
        *   Hypothesis 2: The directory contains files, but they are somehow untracked or in a state that `git mv` considers "empty" for its operation (e.g., all files are staged for deletion, or there's a `.git` subdirectory that's causing issues).
        *   Hypothesis 3: There's a deeper file system corruption or git repository issue.
    *   Step 5: Based on the output of `ls -la` and `du -sh`, determine the actual state of the directory.
    *   Step 6: If the directory is not empty, attempt to force `git add` all its contents to ensure they are tracked, then re-attempt `git mv`. If `git add` fails or reports no changes, consider a manual move and then `git add` the new location and `git rm` the old.
    *   Step 7: Upon successful fix, document the root cause and solution in the `### Notes & Insights` section.
*   **Increment Verification:**
    *   Run `ls -F module/alias/unilang_parser/` to confirm the directory exists and contains files.
    *   Run `ls -F module/move/unilang_parser/` to confirm the old directory is gone.
*   **Commit Message:** `fix(debug): Resolve git mv 'source directory is empty' error`

##### Increment 3: Finalize and Clean Up
*   **Goal:** Perform final verification and clean up any remaining redundant files or references.
*   **Specification Reference:** User feedback.
*   **Steps:**
    *   Step 1: Search for any remaining source code references to `unilang_instruction_parser` that are not part of the new alias crate and update them to `unilang_parser`. (This should ideally be minimal after previous steps).
    *   Step 2: Update the `tasks.md` file in `module/move/unilang/task/tasks.md` to reflect the new alias structure and completed tasks.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test --workspace` to ensure all tests pass. (Note: This may still fail due to external system dependencies.)
    *   Run `timeout 90 cargo clippy --workspace -- -D warnings` to ensure no new lints. (Note: This may still fail due to external system dependencies.)
    *   Run `git status` to ensure the working directory is clean.
*   **Commit Message:** `chore(unilang_parser): Finalize alias conversion and cleanup`

### Task Requirements
*   `unilang_parser` must be moved to `module/alias`.
*   `unilang_instruction_parser` must become an alias crate re-exporting `unilang_parser`.
*   All references must be updated.
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

### Out of Scope
*   Resolving the `pkg-config` system dependency issue.
*   Any other refactoring or feature implementation not directly related to the alias conversion and relocation.

### External System Dependencies
*   `pkg-config` (required for `yeslogic-fontconfig-sys` which is a transitive dependency of `wtools`)

### Notes & Insights
*   N/A

### Changelog
*   `[User Feedback | 2025-07-20 21:47 UTC]` User requested moving `unilang_parser` to `module/alias` and making `unilang_instruction_parser` an alias crate.
*   `[Increment 1 | 2025-07-20 21:47 UTC]` Renamed crate directory `module/move/unilang_parser` to `module/alias/unilang_parser`.
*   `[Increment 1 | 2025-07-20 21:48 UTC]` Removed `module/move/unilang_parser` from the `members` list in the root `Cargo.toml`.
*   `[Increment 2 | 2025-07-20 21:48 UTC]` Created directory `module/alias/unilang_instruction_parser`.
*   `[Increment 2 | 2025-07-20 21:48 UTC]` Created `module/alias/unilang_instruction_parser/Cargo.toml`.
*   `[Increment 2 | 2025-07-20 21:49 UTC]` Created `module/alias/unilang_instruction_parser/src/lib.rs`.
*   `[Increment 2 | 2025-07-20 21:49 UTC]` Added `module/alias/unilang_instruction_parser` to the `members` list in the root `Cargo.toml`.
*   `[Increment 2 | 2025-07-20 21:49 UTC]` Updated path for `unilang_parser` in `module/move/unilang/Cargo.toml`.