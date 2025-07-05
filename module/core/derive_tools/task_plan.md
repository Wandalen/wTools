# Task Plan: Fix errors in derive_tools and derive_tools_meta

### Goal
*   To identify and resolve all compilation errors in the `derive_tools` and `derive_tools_meta` crates, ensuring they compile successfully.

### Ubiquitous Language (Vocabulary)
*   **derive_tools**: The primary crate providing derive macros.
*   **derive_tools_meta**: The proc-macro crate implementing the logic for the derive macros in `derive_tools`.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/derive_tools`
*   **Overall Progress:** 1/3 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Targeted Diagnostics - Identify compilation errors
    *   ✅ Increment 2: Fix E0597, unused_assignments warning, and typo in derive_tools_meta
    *   ⚫ Increment 3: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/derive_tools_meta` (Reason: Proc-macro implementation for the primary crate)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/derive_tools/Cargo.toml`
    *   `module/core/derive_tools_meta/Cargo.toml`
    *   `module/core/derive_tools_meta/src/derive/from.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `derive_tools`
    *   `derive_tools_meta`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   None identified yet.

### Expected Behavior Rules / Specifications
*   The `derive_tools` and `derive_tools_meta` crates should compile without any errors or warnings.

### Crate Conformance Check Procedure
*   Step 1: Run `cargo check -p derive_tools_meta` and `cargo check -p derive_tools` via `execute_command`. Analyze output for success.
*   Step 2: If Step 1 passes, run `cargo test -p derive_tools_meta` and `cargo test -p derive_tools` via `execute_command`. Analyze output for success.
*   Step 3: If Step 2 passes, run `cargo clippy -p derive_tools_meta -- -D warnings` and `cargo clippy -p derive_tools -- -D warnings` via `execute_command`. Analyze output for success.

### Increments
##### Increment 1: Targeted Diagnostics - Identify compilation errors
*   **Goal:** To run targeted checks on `derive_tools_meta` and `derive_tools` to capture all compilation errors.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Execute `cargo check -p derive_tools_meta` to get errors from the meta crate.
    *   Step 2: Execute `cargo check -p derive_tools` to get errors from the main crate.
    *   Step 3: Analyze the output to identify all errors.
    *   Step 4: Update `Increment 2` with a detailed plan to fix the identified errors.
*   **Increment Verification:**
    *   Step 1: The `execute_command` for both `cargo check` commands complete.
    *   Step 2: The output logs containing the errors are successfully analyzed.
*   **Commit Message:** "chore(diagnostics): Capture initial compilation errors per-crate"

##### Increment 2: Fix E0597, unused_assignments warning, and typo in derive_tools_meta
*   **Goal:** To fix the `E0597: `where_clause` does not live long enough` error, the `unused_assignments` warning, and the `predates` typo in `derive_tools_meta/src/derive/from.rs`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read the file `module/core/derive_tools_meta/src/derive/from.rs`.
    *   Step 2: Modify the code to directly assign the `Option<syn::WhereClause>` to `where_clause_owned` and then take a reference to it, resolving both the lifetime issue and the `unused_assignments` warning.
    *   Step 3: Correct the typo `predates` to `predicates` on line 515.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Step 1: Execute `cargo clippy -p derive_tools_meta -- -D warnings` via `execute_command`.
    *   Step 2: Analyze the output to confirm that all errors and warnings are resolved.
*   **Commit Message:** "fix(derive_tools_meta): Resolve lifetime, unused assignment warning, and typo in From derive"

##### Increment 3: Finalization
*   **Goal:** To perform a final, holistic review and verification of the entire task's output, ensuring all errors are fixed and the crates are fully compliant.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Perform a final self-critique against all requirements.
    *   Step 2: Execute the full `Crate Conformance Check Procedure`.
    *   Step 3: Execute `git status` to ensure the working directory is clean.
*   **Increment Verification:**
    *   Step 1: All checks in the `Crate Conformance Check Procedure` pass successfully based on `execute_command` output.
    *   Step 2: `git status` output shows a clean working tree.
*   **Commit Message:** "chore(ci): Final verification of derive_tools fixes"

### Task Requirements
*   All fixes must adhere to the project's existing code style.
*   No new functionality should be introduced; the focus is solely on fixing existing errors.
*   Do not run commands with the `--workspace` flag.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   Must use Rust 2021 edition.

### Assumptions
*   The errors are confined to the `derive_tools` and `derive_tools_meta` crates.
*   The existing test suite is sufficient to catch regressions introduced by the fixes.

### Out of Scope
*   Refactoring code that is not directly related to a compilation error.
*   Updating dependencies unless required to fix an error.

### External System Dependencies
*   None.

### Notes & Insights
*   The errors in the meta crate will likely need to be fixed before the errors in the main crate can be fully resolved.

### Changelog
*   [Initial] Plan created.
*   [2025-07-05] Updated plan to avoid workspace commands per user instruction.
*   [2025-07-05] Identified E0716 in `derive_tools_meta` and planned fix.
*   [2025-07-05] Identified E0597 in `derive_tools_meta` and planned fix.
*   [2025-07-05] Corrected `timeout` command syntax for Windows.
*   [2025-07-05] Removed `timeout` wrapper from commands due to Windows compatibility issues.
*   [2025-07-05] Planned fix for `unused_assignments` warning in `derive_tools_meta`.
*   [2025-07-05] Planned fix for `predates` typo in `derive_tools_meta`.