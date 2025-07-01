# Task Plan: Full Enhancement for `clone_dyn` Crate

### Goal
*   To comprehensively improve the `clone_dyn` crate and its ecosystem (`clone_dyn_meta`, `clone_dyn_types`) by ensuring full test coverage across all feature combinations, eliminating all compiler and clippy warnings, and enhancing the documentation for maximum clarity and completeness.

### Ubiquitous Language (Vocabulary)
*   **`clone_dyn` Ecosystem:** The set of three related crates: `clone_dyn` (facade), `clone_dyn_meta` (proc-macro), and `clone_dyn_types` (core traits/logic).
*   **Trait Object:** A `dyn Trait` instance, which is a pointer to some data and a vtable.
*   **Feature Combination:** A specific set of features enabled during a build or test run (e.g., `--no-default-features --features clone_dyn_types`).

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Target Crate:** `module/core/clone_dyn`
*   **Overall Progress:** 0/7 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Initial Lint Fix
    *   ⚫ Increment 2: Codebase Analysis & Test Matrix Design
    *   ⚫ Increment 3: Test Implementation & `cfg` Scaffolding
    *   ⚫ Increment 4: `macro_tools` Refactoring
    *   ⚫ Increment 5: Comprehensive Feature Combination Verification
    *   ⚫ Increment 6: Documentation Overhaul
    *   ⚫ Increment 7: Final Review and Cleanup

### Permissions & Boundaries
*   **Run workspace-wise commands:** false
*   **Add transient comments:** false
*   **Additional Editable Crates:**
    *   `module/core/clone_dyn_meta` (Reason: Part of the `clone_dyn` ecosystem, requires potential fixes)
    *   `module/core/clone_dyn_types` (Reason: Part of the `clone_dyn` ecosystem, requires potential fixes)

### Crate Conformance Check Procedure
*   **Step 1: Run Tests.** Execute `timeout 90 cargo test -p {crate_name}` with a specific feature set relevant to the increment. If this fails, fix all test errors before proceeding.
*   **Step 2: Run Linter (Conditional).** Only if Step 1 passes, execute `timeout 90 cargo clippy -p {crate_name} -- -D warnings` with the same feature set.

### Feature Combinations for Testing
This section lists all meaningful feature combinations that must be tested for each crate in the ecosystem to ensure full compatibility and correctness.

| Crate | Command | Description |
|---|---|---|
| `clone_dyn` | `cargo test -p clone_dyn --no-default-features` | Tests that the crate compiles with no features enabled. Most tests should be skipped via `cfg`. |
| `clone_dyn` | `cargo test -p clone_dyn --no-default-features --features clone_dyn_types` | Tests the manual-clone functionality where `CloneDyn` is available but the proc-macro is not. |
| `clone_dyn` | `cargo test -p clone_dyn --features derive_clone_dyn` | Tests the full functionality with the `#[clone_dyn]` proc-macro enabled (equivalent to default). |
| `clone_dyn_types` | `cargo test -p clone_dyn_types --no-default-features` | Tests that the types crate compiles with no features enabled. |
| `clone_dyn_types` | `cargo test -p clone_dyn_types --features enabled` | Tests the types crate with its core features enabled (default). |
| `clone_dyn_meta` | `cargo test -p clone_dyn_meta --no-default-features` | Tests that the meta crate compiles with no features enabled. |
| `clone_dyn_meta` | `cargo test -p clone_dyn_meta --features enabled` | Tests the meta crate with its core features enabled (default). |

### Test Matrix
This matrix outlines the test cases required to ensure comprehensive coverage of the `clone_dyn` ecosystem.

| ID | Description | Target Crate(s) | Test File(s) | Key Logic | Feature Combination | Expected Outcome |
|---|---|---|---|---|---|---|
| T1.1 | Verify `clone_into_box` for copyable types (`i32`). | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone_into_box` | `clone_dyn_types` | Pass |
| T1.2 | Verify `clone_into_box` for clonable types (`String`). | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone_into_box` | `clone_dyn_types` | Pass |
| T1.3 | Verify `clone_into_box` for slice types (`&str`, `&[i32]`). | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone_into_box` | `clone_dyn_types` | Pass |
| T2.1 | Verify `clone()` helper for various types. | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone` | `clone_dyn_types` | Pass |
| T3.1 | Manually implement `Clone` for a `Box<dyn Trait1>` and test cloning a `Vec` of trait objects. | `clone_dyn_types` | `inc/basic_manual.rs` | Manual `impl Clone` | `clone_dyn_types` | Pass |
| T4.1 | Use `#[clone_dyn]` on a simple trait and test cloning a `Vec` of trait objects. | `clone_dyn` | `inc/basic.rs` | `#[clone_dyn]` macro | `derive_clone_dyn` | Pass |
| T4.2 | Use `#[clone_dyn]` on a generic trait with `where` clauses and test cloning a `Vec` of trait objects. | `clone_dyn` | `inc/parametrized.rs` | `#[clone_dyn]` macro | `derive_clone_dyn` | Pass |
| T5.1 | Ensure `clone_dyn_meta` uses `macro_tools` abstractions instead of `syn`, `quote`, `proc-macro2` directly. | `clone_dyn_meta` | `src/clone_dyn.rs` | Macro implementation | `enabled` | Code review pass |
| T6.1 | Verify `clippy::doc_markdown` lint is fixed in `clone_dyn`'s Readme. | `clone_dyn` | `Readme.md` | Linting | `default` | `clippy` pass |

### Increments

##### Increment 1: Initial Lint Fix
*   **Goal:** Address the existing `clippy::doc_markdown` lint documented in `task.md`.
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/clone_dyn/Readme.md` to replace `# Module :: clone_dyn` with `# Module :: \`clone_dyn\``.
    *   Step 2: Perform Increment Verification.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo clippy -p clone_dyn -- -D warnings`. The command should pass without the `doc_markdown` error.
*   **Commit Message:** "fix(clone_dyn): Correct doc_markdown lint in Readme.md"

##### Increment 2: Codebase Analysis & Test Matrix Design
*   **Goal:** Analyze the codebase to identify test gaps, required `cfg` attributes, and `macro_tools` refactoring opportunities. The output of this increment is an updated plan, not code changes.
*   **Steps:**
    *   Step 1: Review all `tests/inc/*.rs` files. Compare existing tests against the `Test Matrix`. Identify any test cases from the matrix that are not yet implemented.
    *   Step 2: Review `clone_dyn/Cargo.toml` features and the tests. Determine which tests need `#[cfg(feature = "...")]` attributes to run only under specific feature combinations.
    *   Step 3: Read `module/core/clone_dyn_meta/src/clone_dyn.rs`. Analyze the `ItemAttributes::parse` implementation and other areas for direct usage of `syn`, `quote`, or `proc-macro2` that could be replaced by `macro_tools` helpers.
    *   Step 4: Update this plan file (`task_plan.md`) with the findings: detail the new tests to be written in Increment 3, the `cfg` attributes to be added, and the specific refactoring plan for Increment 4.
*   **Increment Verification:**
    *   The `task_plan.md` is updated with a detailed plan for the subsequent implementation increments.
*   **Commit Message:** "chore(clone_dyn): Analyze codebase and detail implementation plan"

##### Increment 3: Test Implementation & `cfg` Scaffolding
*   **Goal:** Implement the new tests and `cfg` attributes as designed in Increment 2.
*   **Steps:**
    *   Step 1: Use `insert_content` to add the Test Matrix documentation to the top of `tests/inc/only_test/basic.rs` and other relevant test files.
    *   Step 2: Implement any new test functions identified in the analysis from Increment 2.
    *   Step 3: Add the planned `#[cfg]` attributes to the test modules and functions in `tests/inc/mod.rs` and other test files.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p clone_dyn --features derive_clone_dyn` to ensure all existing and new tests pass with default features.
*   **Commit Message:** "test(clone_dyn): Implement test matrix and add feature cfgs"

##### Increment 4: `macro_tools` Refactoring
*   **Goal:** Refactor `clone_dyn_meta` to idiomatically use `macro_tools` helpers, based on the plan from Increment 2.
*   **Steps:**
    *   Step 1: Apply the planned refactoring to `module/core/clone_dyn_meta/src/clone_dyn.rs`, replacing manual parsing loops and direct `syn` usage with `macro_tools` equivalents.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p clone_dyn_meta`.
    *   Run `timeout 90 cargo test -p clone_dyn` to ensure the refactored macro still works as expected.
*   **Commit Message:** "refactor(clone_dyn_meta): Adopt idiomatic macro_tools usage"

##### Increment 5: Comprehensive Feature Combination Verification
*   **Goal:** Execute the full test plan defined in the `Feature Combinations for Testing` section to validate the `cfg` scaffolding and ensure correctness across all features.
*   **Steps:**
    *   Step 1: Execute every command from the `Feature Combinations for Testing` table using `execute_command`.
    *   Step 2: If any command fails, apply a targeted fix (e.g., adjust a `cfg` attribute) and re-run only the failing command until it passes.
*   **Increment Verification:**
    *   Successful execution (exit code 0) of all commands listed in the `Feature Combinations for Testing` table.
*   **Commit Message:** "test(clone_dyn): Verify all feature combinations"

##### Increment 6: Documentation Overhaul
*   **Goal:** Refactor and improve the `Readme.md` files for all three crates.
*   **Steps:**
    *   Step 1: Read the `Readme.md` for `clone_dyn`, `clone_dyn_meta`, and `clone_dyn_types`.
    *   Step 2: For `clone_dyn/Readme.md`, clarify the roles of the `_meta` and `_types` crates and ensure the main example is clear.
    *   Step 3: For `clone_dyn_types/Readme.md` and `clone_dyn_meta/Readme.md`, clarify their roles as internal dependencies of `clone_dyn`.
    *   Step 4: Use `write_to_file` to save the updated content for all three `Readme.md` files.
*   **Increment Verification:**
    *   The `write_to_file` operations for the three `Readme.md` files complete successfully.
*   **Commit Message:** "docs(clone_dyn): Revise and improve Readme documentation"

##### Increment 7: Final Review and Cleanup
*   **Goal:** Perform a final quality check and remove any temporary artifacts.
*   **Steps:**
    *   Step 1: Run `cargo clippy -p clone_dyn --features full -- -D warnings`.
    *   Step 2: Run `cargo clippy -p clone_dyn_meta --features full -- -D warnings`.
    *   Step 3: Run `cargo clippy -p clone_dyn_types --features full -- -D warnings`.
*   **Increment Verification:**
    *   All `clippy` commands pass with exit code 0.
*   **Commit Message:** "chore(clone_dyn): Final cleanup and project polish"

### Task Requirements
*   All code must be warning-free under `clippy` with `-D warnings`.
*   Tests must cover all meaningful feature combinations.
*   Test files must include a Test Matrix in their documentation.
*   The `Readme.md` should be clear, concise, and comprehensive.

### Project Requirements
*   The `macro_tools` crate must be used in place of direct dependencies on `proc-macro2`, `quote`, or `syn`.

### Changelog
*   2025-07-01: V6: Re-structured increments for better workflow (Analyze -> Implement -> Verify). Made planning steps more explicit and proactive.
