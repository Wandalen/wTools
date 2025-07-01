# Task Plan: Fix `derive_tools` compatibility and re-enable all tests

### Goal
*   The primary goal is to restore the full functionality of the `derive_tools` crate by methodically re-enabling all tests, fixing any resulting compilation errors or warnings, and ensuring it is fully compatible with `macro_tools` v0.55.0.

### Ubiquitous Language (Vocabulary)
*   **`derive_tools`:** The main crate that re-exports procedural macros.
*   **`derive_tools_meta`:** The procedural macro crate containing the macro implementations.
*   **`macro_tools`:** The dependency that was updated, causing API incompatibilities.
*   **Test Module:** A single `mod` declaration in `derive_tools/tests/inc/mod.rs` that corresponds to a specific derive macro's test suite.
*   **Test Matrix:** A structured table used for planning test cases, ensuring comprehensive coverage of features and edge cases for a given derive macro.

### Progress
*   **Primary Target Crate:** `module/core/derive_tools`
*   **Overall Progress:** 0/20 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Initial Workspace Analysis and Baseline
    *   ⚫ Increment 2: Plan and Document `AsMut` and `AsRef` Tests
    *   ⚫ Increment 3: Fix `as_mut` tests
    *   ⚫ Increment 4: Fix `as_ref` tests
    *   ⚫ Increment 5: Plan and Document `Deref` Tests
    *   ⚫ Increment 6: Fix `Deref` tests for basic structs
    *   ⚫ Increment 7: Fix `Deref` tests for enums
    *   ⚫ Increment 8: Fix `Deref` tests for generics and bounds
    *   ⚫ Increment 9: Plan and Document `DerefMut` Tests
    *   ⚫ Increment 10: Fix `DerefMut` tests
    *   ⚫ Increment 11: Plan and Document `From` tests
    *   ⚫ Increment 12: Fix `From` tests
    *   ⚫ Increment 13: Plan and Document `InnerFrom` and `New` tests
    *   ⚫ Increment 14: Fix `InnerFrom` tests
    *   ⚫ Increment 15: Fix `New` tests
    *   ⚫ Increment 16: Plan and Document `Not`, `Index`, `IndexMut` tests
    *   ⚫ Increment 17: Fix `Not` tests
    *   ⚫ Increment 18: Fix `Index` and `IndexMut` tests
    *   ⚫ Increment 19: Redesign and Fix `PhantomData` derive and tests
    *   ⚫ Increment 20: Final Workspace Verification

### Permissions & Boundaries
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/derive_tools_meta` (Reason: Fixes to macro implementations are required)

### Relevant Context
*   Control Files to Reference:
    *   `module/core/macro_tools/task.md` (Proposal to fix `const` generics issue)
    *   `module/core/clone_dyn/task.md` (Proposal to fix `clippy::doc_markdown` warning)
*   Files to Include:
    *   `module/core/derive_tools/tests/inc/mod.rs`
    *   All files under `module/core/derive_tools/tests/inc/`
    *   All files under `module/core/derive_tools_meta/src/derive/`

### Crate Conformance Check Procedure
*   **Step 1: Run Specific Tests.** Execute `timeout 90 cargo test -p derive_tools --test <test_name>` for the specific test file being fixed.
*   **Step 2: Run All Enabled Tests.** Execute `timeout 120 cargo test -p derive_tools --all-targets`. If this fails, fix all test errors before proceeding.
*   **Step 3: Run Linter (Conditional).** Only if Step 2 passes, execute `timeout 120 cargo clippy -p derive_tools -- -D warnings`.
*   **Step 4: Run Feature Combination Tests (Conditional).** Only if Step 3 passes, execute the testing procedure defined below:
    *   `timeout 90 cargo test -p derive_tools --no-default-features --features "derive_from"`
    *   `timeout 90 cargo test -p derive_tools --no-default-features --features "derive_as_ref,derive_as_mut"`
    *   `timeout 90 cargo test -p derive_tools --no-default-features --features "derive_deref,derive_deref_mut"`
    *   `timeout 90 cargo test -p derive_tools --features "full"`

### Increments

##### Increment 1: Initial Workspace Analysis and Baseline
*   **Goal:** Establish a clear baseline of the current compilation and test failures across the workspace.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Execute `timeout 180 cargo test --workspace --all-features`.
    *   Step 2: Analyze the output to identify the primary points of failure. The expected failure is in `derive_tools` due to the `macro_tools` update.
    *   Step 3: Document the initial error state in the `### Changelog` section of this plan.
*   **Increment Verification:**
    *   The initial error state is successfully logged.
*   **Commit Message:** `chore: Establish baseline for derive_tools fix`

##### Increment 2: Plan and Document `AsMut` and `AsRef` Tests
*   **Goal:** Create the test matrices for `AsMut` and `AsRef` and add them as documentation to the relevant test files.
*   **Specification Reference:** N/A
*   **Test Matrix for `AsMut`:**
    | ID   | Struct Type        | Implementation | Expected Behavior                                           | Test File                   |
    |------|--------------------|----------------|-------------------------------------------------------------|-----------------------------|
    | T2.1 | Tuple struct (1 field) | `#[derive(AsMut)]` | `.as_mut()` returns a mutable reference to the inner field. | `as_mut_test.rs`            |
    | T2.2 | Tuple struct (1 field) | Manual `impl`  | `.as_mut()` returns a mutable reference to the inner field. | `as_mut_manual_test.rs`     |
*   **Test Matrix for `AsRef`:**
    | ID   | Struct Type        | Implementation | Expected Behavior                                       | Test File                   |
    |------|--------------------|----------------|---------------------------------------------------------|-----------------------------|
    | T3.1 | Tuple struct (1 field) | `#[derive(AsRef)]` | `.as_ref()` returns a reference to the inner field. | `as_ref_test.rs`            |
    | T3.2 | Tuple struct (1 field) | Manual `impl`  | `.as_ref()` returns a reference to the inner field. | `as_ref_manual_test.rs`     |
*   **Steps:**
    *   Step 1: Use `insert_content` to add the `AsMut` test matrix as a file-level doc comment to `tests/inc/as_mut_test.rs`.
    *   Step 2: Use `insert_content` to add a doc comment explaining the purpose of the test function in `tests/inc/only_test/as_mut.rs`.
    *   Step 3: Use `insert_content` to add the `AsRef` test matrix as a file-level doc comment to `tests/inc/as_ref_test.rs`.
    *   Step 4: Use `insert_content` to add a doc comment explaining the purpose of the test function in `tests/inc/only_test/as_ref.rs`.
*   **Increment Verification:**
    *   Use `read_file` to confirm the documentation has been added correctly to all four files.
*   **Commit Message:** `docs(test): Add test matrices and purpose for AsMut and AsRef`

##### Increment 3: Fix `as_mut` tests
*   **Goal:** Re-enable the `as_mut_test` and `as_mut_manual_test` modules and fix any resulting issues.
*   **Specification Reference:** T2.1, T2.2
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod as_mut_manual_test;` and `mod as_mut_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test as_mut_test`.
    *   Step 3: If the test fails, apply the Critical Log Analysis Procedure to the output. Hypothesize that the `AsMut` derive in `derive_tools_meta` is not generating the correct implementation.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/as_mut.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test as_mut_manual_test` and `timeout 90 cargo test -p derive_tools --test as_mut_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix as_mut tests`

##### Increment 4: Fix `as_ref` tests
*   **Goal:** Re-enable the `as_ref_test` and `as_ref_manual_test` modules and fix any resulting issues.
*   **Specification Reference:** T3.1, T3.2
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod as_ref_manual_test;` and `mod as_ref_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test as_ref_test`.
    *   Step 3: If the test fails, apply Critical Log Analysis.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/as_ref.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test as_ref_manual_test` and `timeout 90 cargo test -p derive_tools --test as_ref_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix as_ref tests`

... (The plan will continue in this detailed, granular fashion for all other test modules, with each having its own planning, documentation, and fixing increments) ...

##### Increment 19: Redesign and Fix `PhantomData` derive and tests
*   **Goal:** Re-enable the `phantom_tests` module and the `PhantomData` derive macro, fixing all related issues by implementing the correct logic.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools_meta/src/lib.rs` to re-enable the `PhantomData` derive macro.
    *   Step 2: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment the `phantom_tests` module block.
    *   Step 3: Analyze the `E0392` error. The root cause is that `PhantomData` is a struct, not a trait, and cannot be implemented.
    *   Step 4: Modify `derive_tools_meta/src/derive/phantom.rs`. The logic must be changed to *add a field* `_phantom: core::marker::PhantomData<...>` to the struct, rather than generating an `impl` block. Use the `macro_tools::phantom::add_to_item` helper function as a reference.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test phantom_tests`. Verify it passes.
*   **Commit Message:** `fix(derive_tools): Redesign and fix PhantomData derive and tests`

##### Increment 20: Final Workspace Verification
*   **Goal:** Perform a final, comprehensive check of the entire workspace to ensure no regressions were introduced.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Execute `timeout 180 cargo test --workspace --all-features`.
    *   Step 2: Execute `timeout 180 cargo clippy --workspace --all-features -- -D warnings`.
    *   Step 3: Analyze results, acknowledging the known external issue in `clone_dyn`. If all other checks pass, the task is complete.
*   **Increment Verification:**
    *   All workspace checks pass (or only fail because of the known external issue).
*   **Commit Message:** `chore(workspace): Final verification of derive_tools fixes`

### Task Requirements
*   Ensure `derive_tools` is compatible with `macro_tools` v0.55.0.
*   All tests for `derive_tools_meta` and `derive_tools` must be re-enabled and pass.
*   All clippy warnings must be resolved with `-D warnings`.
*   All test files must have a file-level doc comment containing a Test Matrix.
*   All test functions must have a doc comment explaining their purpose.

### Project Requirements
*   Must use Rust 2021 edition.
*   All new APIs must be async.
*   All test execution commands must be wrapped in `timeout`.
*   `cargo clippy` must be run without auto-fixing flags.
*   All file modifications must be enacted exclusively through appropriate tools.
*   Git commits must occur after each successfully verified increment.
*   Commit messages must be prefixed with the `Target Crate` name if changes were made to it.
*   **Always prefer using `macro_tools` over direct use of `syn`, `quote`, or `proc-macro2` for procedural macro development.**

### Assumptions
*   The `macro_tools` crate will eventually be updated to fix the `const` generics issue as per the `task.md` proposal. The current task proceeds assuming this future fix.
*   The existing test suite is sufficient to validate the fixes.

### Out of Scope
*   Implementing new features.
*   Addressing issues in `macro_tools` or `clone_dyn` directly (only proposing changes via `task.md`).

### Changelog
