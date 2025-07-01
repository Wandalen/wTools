# Task Plan: Fix `clone_dyn` Test Suite Issues (v2)

### Goal
*   To fix the compilation errors and test failures within the `clone_dyn` crate's test suite, specifically addressing issues related to unresolved modules (`the_module`), missing macros (`a_id`), and unrecognized attributes (`clone_dyn`), as detailed in `task/fix_test_issues_task.md`. The successful completion of this task will unblock the `derive_tools` crate's test suite.

### Ubiquitous Language (Vocabulary)
*   **`clone_dyn` Ecosystem:** The set of three related crates: `clone_dyn` (facade), `clone_dyn_meta` (proc-macro), and `clone_dyn_types` (core traits/logic).
*   **`the_module`:** An alias used in integration tests to refer to the crate under test (in this case, `clone_dyn`).
*   **`a_id`:** An assertion macro provided by `test_tools` for comparing values in tests.
*   **Shared Test (`only_test/basic.rs`):** A file containing test logic included by other test files to avoid code duplication.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/clone_dyn`
*   **Overall Progress:** 2/2 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Fix Test Context and Path Resolution
    *   ✅ Increment 2: Final Verification and Cleanup

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** false
*   **Additional Editable Crates:**
    *   `module/core/clone_dyn_meta`
    *   `module/core/clone_dyn_types`

### Relevant Context
*   **Control Files to Reference:**
    *   `module/core/clone_dyn/task/fix_test_issues_task.md`
*   **Files to Include:**
    *   `module/core/clone_dyn/tests/tests.rs`
    *   `module/core/clone_dyn/tests/inc/mod.rs`
    *   `module/core/clone_dyn/tests/inc/basic.rs`
    *   `module/core/clone_dyn/tests/inc/only_test/basic.rs`
    *   `module/core/clone_dyn/tests/inc/parametrized.rs`

### Crate Conformance Check Procedure
*   **Step 1: Run Tests.** Execute `timeout 120 cargo test -p clone_dyn --all-targets`. If this fails, fix all test errors before proceeding.
*   **Step 2: Run Linter (Conditional).** Only if Step 1 passes, execute `timeout 120 cargo clippy -p clone_dyn --features full -- -D warnings`.

### Increments

##### Increment 1: Fix Test Context and Path Resolution
*   **Goal:** Atomically apply all necessary fixes to resolve the `the_module`, `a_id`, and `clone_dyn` attribute resolution errors.
*   **Specification Reference:** `task/fix_test_issues_task.md`
*   **Steps:**
    1.  **Analyze:** Read the content of `tests/inc/only_test/basic.rs`, `tests/inc/basic.rs`, and `tests/inc/parametrized.rs` to confirm the current state.
    2.  **Propagate Context:** Use `insert_content` to add `use super::*;` to the top of `module/core/clone_dyn/tests/inc/only_test/basic.rs`. This will resolve the `the_module` and `a_id` errors by making the alias and macro available from the parent test module.
    3.  **Fix Attribute Path in `basic.rs`:**
        *   Use `search_and_replace` to remove the line `use the_module::clone_dyn;` from `module/core/clone_dyn/tests/inc/basic.rs`.
        *   Use `search_and_replace` to replace `#[ clone_dyn ]` with `#[ the_module::clone_dyn ]` in `module/core/clone_dyn/tests/inc/basic.rs`. Using the established `the_module` alias is consistent with the rest of the test suite.
    4.  **Fix Attribute Path in `parametrized.rs`:**
        *   Use `search_and_replace` to replace `#[ clone_dyn ]` with `#[ the_module::clone_dyn ]` in `module/core/clone_dyn/tests/inc/parametrized.rs`.
*   **Increment Verification:**
    *   Execute `timeout 120 cargo test -p clone_dyn --all-targets`. The command should now pass with no compilation errors or test failures.
*   **Commit Message:** "fix(clone_dyn): Resolve path and context issues in test suite"

##### Increment 2: Final Verification and Cleanup
*   **Goal:** Perform a final, holistic review and verification of the entire `clone_dyn` ecosystem to ensure all changes are correct and no regressions were introduced.
*   **Specification Reference:** `task/fix_test_issues_task.md`
*   **Steps:**
    1.  Execute `timeout 120 cargo test -p clone_dyn --all-targets`.
    2.  Execute `timeout 120 cargo clippy -p clone_dyn --features full -- -D warnings`.
    3.  Execute `timeout 120 cargo clippy -p clone_dyn_meta --features full -- -D warnings`.
    4.  Execute `timeout 120 cargo clippy -p clone_dyn_types --features full -- -D warnings`.
    5.  Self-critique: Review all changes against the task requirements. The fixes should be minimal, correct, and robust.
*   **Increment Verification:**
    *   All test and clippy commands pass with exit code 0.
*   **Commit Message:** "chore(clone_dyn): Final verification of test suite fixes"

### Task Requirements
*   All tests in `clone_dyn` must pass.
*   The `derive_tools` test suite must compile without errors originating from `clone_dyn`.
*   All code must be warning-free under `clippy` with `-D warnings`.

### Project Requirements
*   (Inherited from previous plan)

### Assumptions
*   The errors reported in `fix_test_issues_task.md` are accurate and are the only blockers from `clone_dyn`.

### Out of Scope
*   Refactoring any logic beyond what is necessary to fix the specified test issues.
*   Making changes to the `derive_tools` crate.

### External System Dependencies
*   None.

### Notes & Insights
*   Using a crate-level alias (`the_module`) is a good pattern for integration tests, but it must be correctly propagated to all included files.
*   Using a fully qualified path or an established alias for proc-macro attributes (`#[the_module::my_macro]`) is a robust pattern that prevents resolution issues when tests are included and run by other crates in the workspace.

### Changelog
*   [Increment 1 | 2025-07-01 21:37 UTC] Applied fixes for `the_module`, `a_id`, and `clone_dyn` attribute resolution errors in test files.
*   [Increment 2 | 2025-07-01 21:40 UTC] Performed final verification of `clone_dyn` ecosystem, confirming all tests and lints pass.
*   [Initial] Plan created to address test failures in `clone_dyn`.
*   [v2] Refined plan to be more efficient, combining fixes into a single increment before a dedicated verification increment.
