# Task Plan: Restore and Complete `derive_tools` Functionality (V4)

### Goal
*   To methodically restore, validate, and complete the entire test suite for the `derive_tools` crate, ensuring every derive macro is fully functional, tested, and compliant with the project's specifications. This V4 plan uses a highly granular, test-driven, and context-aware structure to ensure robust execution.

### Ubiquitous Language (Vocabulary)
*   **`derive_tools`**: The user-facing facade crate.
*   **`derive_tools_meta`**: The procedural macro implementation crate.
*   **`macro_tools`**: The foundational utility crate. Its correctness is critical.
*   **Test Matrix**: A structured table defining test cases. Relevant rows from the master matrix MUST be added as a doc comment to each specific test file being worked on.
*   **`trybuild`**: The framework used for compile-fail tests.

### Progress
*   **Roadmap Milestone:** M2: Full Test Suite Restoration
*   **Primary Editable Crate:** `module/core/derive_tools`
*   **Overall Progress:** 2/18 increments complete
*   **Increment Status:** (Grouped by derive for clarity)
    *   ✅ **Group 0: Setup**
        *   ✅ Increment 1: Establish Initial Baseline
    *   ✅ **Group 1: Foundational Fixes**
        *   ✅ Increment 2: Fix `macro_tools` `const` Generics Bug
    *   ⚫ **Group 2: Deref Family**
        *   ⚫ Increment 3: Re-enable and Fix `Deref`
        *   ⚫ Increment 4: Re-enable and Fix `DerefMut`
    *   ⚫ **Group 3: AsRef Family**
        *   ⚫ Increment 5: Re-enable and Fix `AsRef`
        *   ⚫ Increment 6: Re-enable and Fix `AsMut`
    *   ⚫ **Group 4: Conversion Family**
        *   ⚫ Increment 7: Re-enable and Fix `From`
        *   ⚫ Increment 8: Re-enable and Fix `InnerFrom`
    *   ⚫ **Group 5: Constructor Family**
        *   ⚫ Increment 9: Re-enable and Fix `New`
    *   ⚫ **Group 6: Operator Family**
        *   ⚫ Increment 10: Re-enable and Fix `Not`
        *   ⚫ Increment 11: Re-enable and Fix `Index`
        *   ⚫ Increment 12: Re-enable and Fix `IndexMut`
    *   ⚫ **Group 7: Special Macros**
        *   ⚫ Increment 13: Redesign and Fix `PhantomData` Macro
    *   ⚫ **Group 8: Integration Tests**
        *   ⚫ Increment 14: Fix `all_test` Integration
        *   ⚫ Increment 15: Fix `basic_test` Integration
    *   ⚫ **Group 9: Finalization**
        *   ⚫ Increment 16: Final Code Cleanup and Documentation Review
        *   ⚫ Increment 17: Final Workspace Verification
        *   ⚫ Increment 18: Update Project Changelog

### Permissions & Boundaries
*   **Mode:** `code`
*   **Run workspace-wise commands:** false
*   **Add transient comments:** false
*   **Additional Editable Crates:**
    *   `module/core/derive_tools_meta` (Reason: Implements the derive macros)
    *   `module/core/macro_tools` (Reason: Foundational utilities may need fixes. This is permitted *if and only if* a bug in `macro_tools` is identified as the root cause of a `derive_tools` test failure.)

### Crate Conformance Check Procedure
*   **This is run at the end of each major group of increments.**
*   **Step 1: Run Tests.** Execute `timeout 180 cargo test -p derive_tools --all-targets`.
*   **Step 2: Run Linter (Conditional).** Only if Step 1 passes, execute `timeout 180 cargo clippy -p derive_tools --all-features -- -D warnings`.

### Increments

#### Group 0: Setup
##### Increment 1: Establish Initial Baseline
*   **Goal:** Get a clear, current picture of the crate's state by running tests and lints to understand all existing failures.
*   **Context & Rationale:** Before making changes, we need a snapshot of what's broken. This includes disabled tests (which we can infer from the file list vs. `mod.rs`) and active failures. This baseline will validate our fixes later.
*   **Elaboration & Self-Critique (Pre-computation):**
    *   **Critique:** Just running `cargo test` won't show which tests are commented out in `tests/inc/mod.rs`. I need to read that file. Using `--no-fail-fast` is crucial to get a complete list of all failing tests.
    *   **Final Approach:** Read `tests/inc/mod.rs`, run a full test suite with `--no-fail-fast`, run clippy, and log the complete state.
*   **Steps:**
    1.  **Action:** Use `read_file` to load `module/core/derive_tools/tests/inc/mod.rs`.
    2.  **Action:** Use `execute_command` to run `timeout 180 cargo test -p derive_tools --all-targets --no-fail-fast`.
    3.  **Action:** Use `execute_command` to run `timeout 180 cargo clippy -p derive_tools --all-features -- -D warnings`.
    4.  **Analysis:** Create a summary of all commented-out test modules, all failing tests, and all clippy warnings. Store this in the `### Changelog` section.
    5.  **Verification:** The summary of failures and warnings is complete and logged in the changelog.
    6.  **Commit:** This is an analysis-only step, no code changes to commit.
*   **Commit Message:** `chore(derive_tools): Establish baseline of test and lint failures`

---
#### Group 1: Foundational Fixes
##### Increment 2: Fix `macro_tools` `const` Generics Bug
*   **Goal:** Apply the fix proposed in `macro_tools/task.md` to resolve the `const` generics issue, which is a known blocker for many `derive_tools` tests.
*   **Context & Rationale:** The `Deref` and `DerefMut` tests (and likely others) are failing because `macro_tools::generic_params::decompose` incorrectly handles `const` parameters. Fixing this foundational issue in the dependency is the first step to unblocking the `derive_tools` tests.
*   **Elaboration & Self-Critique (Pre-computation):**
    *   **Critique:** The proposal in `macro_tools/task.md` is sound. The key is to change how `generics_for_ty` is constructed for `ConstParam`. I must ensure the fix doesn't break other uses of `decompose`. The change should be surgical.
    *   **Final Approach:** Read `macro_tools/src/generic_params.rs`, apply the targeted fix to the `decompose` function, and then immediately run tests within the `macro_tools` crate to ensure no regressions were introduced there.
*   **Steps:**
    1.  **Action:** Use `read_file` to load `module/core/macro_tools/src/generic_params.rs`.
    2.  **Action:** Use `search_and_replace` to modify the `decompose` function in `module/core/macro_tools/src/generic_params.rs` to correctly handle `ConstParam` for `generics_for_ty`, ensuring it only includes the identifier.
    3.  **Verification:** Execute `timeout 180 cargo test -p macro_tools --all-targets`.
    4.  **Conditional Rethinking:**
        *   **If** verification succeeds, proceed to Commit.
        *   **Else**, analyze the failure, propose a refined fix for `macro_tools`, and loop back to Action 2.
    5.  **Commit:** Use `execute_command` to `git add .` and `git commit` the changes to `macro_tools`.
*   **Commit Message:** `fix(macro_tools): Correctly decompose const generics for type paths`

---
#### Group 2: Deref Family
##### Increment 3: Re-enable and Fix `Deref`
*   **Goal:** Re-enable all `Deref` tests, create a comprehensive test matrix, and fix the `Deref` derive macro and its tests.
*   **Context & Rationale:** `Deref` is a fundamental trait. With the `macro_tools` fix in place, we can now tackle the tests that depend on `const` generics and other complex scenarios.
*   **Elaboration & Self-Critique (Pre-computation):**
    *   **Critique:** The test matrix must be thorough. It should cover: single-field structs (tuple and named), multi-field structs (should fail without attribute), enums (should fail), unit structs (should fail), and all generic variations.
    *   **Final Approach:** First, write the test matrix into the main test file. Second, uncomment the `deref_tests` module in `mod.rs`. Third, run tests to see the specific failures. Fourth, fix the `deref.rs` implementation in `derive_tools_meta`. Finally, verify and commit.
*   **Steps:**
    1.  **Action:** Use `write_to_file` to prepend the following Test Matrix as a doc comment to `module/core/derive_tools/tests/inc/deref/basic_test.rs`.
        ```rust
        //! # Test Matrix for `Deref`
        //!
        //! | ID   | Struct Type        | Fields      | Generics         | Attributes | Expected Behavior                                     | Test Type    |
        //! |------|--------------------|-------------|------------------|------------|-------------------------------------------------------|--------------|
        //! | T1.1 | Tuple Struct       | 1           | None             | -          | Implements `Deref` to the inner field.                | `tests/inc/deref/basic_test.rs` |
        //! | T1.2 | Named Struct       | 1           | None             | -          | Implements `Deref` to the inner field.                | `tests/inc/deref/basic_test.rs` |
        //! | T1.3 | Tuple Struct       | >1          | None             | -          | Fails to compile: `Deref` requires a single field.    | `trybuild`   |
        //! | T1.4 | Named Struct       | >1          | None             | `#[deref]` | Implements `Deref` to the specified field.            | `tests/inc/deref/struct_named.rs` |
        //! | T1.5 | Named Struct       | >1          | None             | -          | Fails to compile: `#[deref]` attribute is required.   | `trybuild`   |
        //! | T1.6 | Enum               | Any         | Any              | -          | Fails to compile: `Deref` cannot be on an enum.       | `tests/inc/deref/compile_fail_enum.rs` |
        //! | T1.7 | Unit Struct        | 0           | None             | -          | Fails to compile: `Deref` requires a field.           | `trybuild`   |
        //! | T1.8 | Struct             | 1           | Lifetime         | -          | Implements `Deref` correctly with lifetimes.          | `tests/inc/deref/generics_lifetimes.rs` |
        //! | T1.9 | Struct             | 1           | Type             | -          | Implements `Deref` correctly with type generics.      | `tests/inc/deref/generics_types.rs` |
        //! | T1.10| Struct             | 1           | Const            | -          | Implements `Deref` correctly with const generics.     | `tests/inc/deref/generics_constants.rs` |
        //! | T1.11| Struct             | 1           | Where clause     | -          | Implements `Deref` correctly with where clauses.      | `tests/inc/deref/bounds_where.rs` |
        ```
    2.  **Action:** Use `search_and_replace` to uncomment the `deref_tests` module in `module/core/derive_tools/tests/inc/mod.rs`.
    3.  **Action:** Fix the `Deref` implementation in `module/core/derive_tools_meta/src/derive/deref.rs` to handle all cases from the test matrix correctly, including returning `syn::Error` for enums and multi-field structs without an attribute.
    4.  **Verification:** Execute `timeout 180 cargo test -p derive_tools --test deref_tests`.
    5.  **Conditional Rethinking:** If verification fails, analyze the failure, propose a fix, and loop back to Action 3.
    6.  **Commit:** Use `execute_command` to `git add .` and `git commit` the changes.
*   **Commit Message:** `fix(derive_tools): Re-enable, document, and fix Deref derive and tests`

##### Increment 4: Re-enable and Fix `DerefMut`
*   **Goal:** Re-enable all `DerefMut` tests, create a comprehensive test matrix, and fix the `DerefMut` derive macro.
*   **Steps:**
    1.  **Action:** Use `write_to_file` to prepend the following Test Matrix as a doc comment to `module/core/derive_tools/tests/inc/deref_mut/basic_test.rs`.
        ```rust
        //! # Test Matrix for `DerefMut`
        //!
        //! | ID   | Struct Type        | Fields      | Generics         | Attributes   | Prerequisite | Expected Behavior                                       | Test Type    |
        //! |------|--------------------|-------------|------------------|--------------|--------------|---------------------------------------------------------|--------------|
        //! | T2.1 | Tuple Struct       | 1           | None             | -            | `Deref`      | Implements `DerefMut` to the inner field.               | `tests/inc/deref_mut/basic_test.rs` |
        //! | T2.2 | Named Struct       | 1           | None             | -            | `Deref`      | Implements `DerefMut` to the inner field.               | `tests/inc/deref_mut/basic_test.rs` |
        //! | T2.3 | Named Struct       | >1          | None             | `#[deref_mut]` | `Deref`      | Implements `DerefMut` to the specified field.           | `tests/inc/deref_mut/struct_named.rs` |
        //! | T2.4 | Struct             | 1           | Any              | -            | No `Deref`   | Fails to compile: `DerefMut` requires `Deref`.          | `trybuild`   |
        //! | T2.5 | Enum               | Any         | Any              | -            | -            | Fails to compile: `DerefMut` cannot be on an enum.      | `trybuild`   |
        ```    2.  **Action:** Use `search_and_replace` to uncomment the `deref_mut_tests` module in `module/core/derive_tools/tests/inc/mod.rs`.
    3.  **Action:** Fix the `DerefMut` implementation in `module/core/derive_tools_meta/src/derive/deref_mut.rs`.
    4.  **Verification:** Execute `timeout 180 cargo test -p derive_tools --test deref_mut_tests`.
    5.  **Conditional Rethinking:** If verification fails, analyze the failure, propose a fix, and loop back to Action 3.
    6.  **Commit:** Use `execute_command` to `git add .` and `git commit` the changes.
*   **Commit Message:** `fix(derive_tools): Re-enable, document, and fix DerefMut derive and tests`

---
#### Group 3: AsRef Family
##### Increment 5: Re-enable and Fix `AsRef`
*   **Goal:** Re-enable, document, and fix the `AsRef` derive.
*   **Steps:**
    1.  **Action:** Add the following Test Matrix to `tests/inc/as_ref_test.rs`.
        ```rust
        //! # Test Matrix for `AsRef`
        //!
        //! | ID   | Struct Type  | Fields | Attributes          | Expected Behavior                               | Test Type  |
        //! |------|--------------|--------|---------------------|-------------------------------------------------|------------|
        //! | T3.1 | Tuple Struct | 1      | -                   | Implements `AsRef<InnerType>`.                  | `as_ref_test.rs` |
        //! | T3.2 | Named Struct | >1     | `#[as_ref]` on field | Implements `AsRef<FieldType>`.                  | `trybuild` |
        //! | T3.3 | Struct       | 1      | `#[as_ref(forward)]` | Forwards `AsRef` impl from inner field.         | `trybuild` |
        //! | T3.4 | Enum         | Any    | -                   | Fails to compile.                               | `trybuild` |
        ```
    2.  **Action:** Uncomment `as_ref_test` in `tests/inc/mod.rs`.
    3.  **Action:** Fix `derive_tools_meta/src/derive/as_ref.rs`.
    4.  **Verification:** `timeout 180 cargo test -p derive_tools --test as_ref_test`.
    5.  **Commit.**
*   **Commit Message:** `fix(derive_tools): Re-enable, document, and fix AsRef derive`

##### Increment 6: Re-enable and Fix `AsMut`
*   **Goal:** Re-enable, document, and fix the `AsMut` derive.
*   **Steps:**
    1.  **Action:** Add the following Test Matrix to `tests/inc/as_mut_test.rs`.
        ```rust
        //! # Test Matrix for `AsMut`
        //!
        //! | ID   | Struct Type  | Fields | Attributes         | Prerequisite | Expected Behavior                               | Test Type  |
        //! |------|--------------|--------|--------------------|--------------|-------------------------------------------------|------------|
        //! | T4.1 | Tuple Struct | 1      | -                  | `AsRef`      | Implements `AsMut<InnerType>`.                  | `as_mut_test.rs` |
        //! | T4.2 | Named Struct | >1     | `#[as_mut]` on field | `AsRef`      | Implements `AsMut<FieldType>`.                  | `trybuild` |
        //! | T4.3 | Struct       | 1      | -                  | No `AsRef`   | Fails to compile.                               | `trybuild` |
        //! | T4.4 | Enum         | Any    | -                  | -            | Fails to compile.                               | `trybuild` |
        ```
    2.  **Action:** Uncomment `as_mut_test` in `tests/inc/mod.rs`.
    3.  **Action:** Fix `derive_tools_meta/src/derive/as_mut.rs`.
    4.  **Verification:** `timeout 180 cargo test -p derive_tools --test as_mut_test`.
    5.  **Commit.**
*   **Commit Message:** `fix(derive_tools): Re-enable, document, and fix AsMut derive`

---
#### Group 4: Conversion Family
##### Increment 7: Re-enable and Fix `From`
*   **Goal:** Re-enable, document, and fix the `From` derive for both structs and enums.
*   **Steps:**
    1.  **Action:** Add Test Matrix to `tests/inc/from/basic_test.rs`.
    2.  **Action:** Uncomment `from_tests` in `tests/inc/mod.rs`.
    3.  **Action:** Fix `derive_tools_meta/src/derive/from.rs`.
    4.  **Verification:** `timeout 180 cargo test -p derive_tools --test from_tests`.
    5.  **Commit.**
*   **Commit Message:** `fix(derive_tools): Re-enable, document, and fix From derive`

##### Increment 8: Re-enable and Fix `InnerFrom`
*   **Goal:** Re-enable, document, and fix the `InnerFrom` derive.
*   **Steps:**
    1.  **Action:** Add Test Matrix to `tests/inc/inner_from/basic_test.rs`.
    2.  **Action:** Uncomment `inner_from_tests` in `tests/inc/mod.rs`.
    3.  **Action:** Fix `derive_tools_meta/src/derive/inner_from.rs`.
    4.  **Verification:** `timeout 180 cargo test -p derive_tools --test inner_from_tests`.
    5.  **Commit.**
*   **Commit Message:** `fix(derive_tools): Re-enable, document, and fix InnerFrom derive`

---
#### Group 5: Constructor Family
##### Increment 9: Re-enable and Fix `New`
*   **Goal:** Re-enable, document, and fix the `New` derive.
*   **Steps:**
    1.  **Action:** Add Test Matrix to `tests/inc/new/basic_test.rs`.
    2.  **Action:** Uncomment `new_tests` in `tests/inc/mod.rs`.
    3.  **Action:** Fix `derive_tools_meta/src/derive/new.rs`.
    4.  **Verification:** `timeout 180 cargo test -p derive_tools --test new_tests`.
    5.  **Commit.**
*   **Commit Message:** `fix(derive_tools): Re-enable, document, and fix New derive`

---
#### Group 6: Operator Family
##### Increment 10: Re-enable and Fix `Not`
*   **Goal:** Re-enable, document, and fix the `Not` derive.
*   **Steps:**
    1.  **Action:** Add Test Matrix to `tests/inc/not/basic_test.rs`.
    2.  **Action:** Uncomment `not_tests` in `tests/inc/mod.rs`.
    3.  **Action:** Fix `derive_tools_meta/src/derive/not.rs`.
    4.  **Verification:** `timeout 180 cargo test -p derive_tools --test not_tests`.
    5.  **Commit.**
*   **Commit Message:** `fix(derive_tools): Re-enable, document, and fix Not derive`

##### Increment 11: Re-enable and Fix `Index`
*   **Goal:** Re-enable, document, and fix the `Index` derive.
*   **Steps:**
    1.  **Action:** Add Test Matrix to `tests/inc/index/basic_test.rs`.
    2.  **Action:** Uncomment `index_tests` in `tests/inc/mod.rs`.
    3.  **Action:** Fix `derive_tools_meta/src/derive/index.rs`.
    4.  **Verification:** `timeout 180 cargo test -p derive_tools --test index_tests`.
    5.  **Commit.**
*   **Commit Message:** `fix(derive_tools): Re-enable, document, and fix Index derive`

##### Increment 12: Re-enable and Fix `IndexMut`
*   **Goal:** Re-enable, document, and fix the `IndexMut` derive.
*   **Steps:**
    1.  **Action:** Add Test Matrix to `tests/inc/index_mut/basic_test.rs`.
    2.  **Action:** Uncomment `index_mut_tests` in `tests/inc/mod.rs`.
    3.  **Action:** Fix `derive_tools_meta/src/derive/index_mut.rs`.
    4.  **Verification:** `timeout 180 cargo test -p derive_tools --test index_mut_tests`.
    5.  **Commit.**
*   **Commit Message:** `fix(derive_tools): Re-enable, document, and fix IndexMut derive`

---
#### Group 7: Special Macros
##### Increment 13: Redesign and Fix `PhantomData` Macro
*   **Goal:** Redesign the flawed `PhantomData` derive into a working attribute macro and fix all related tests.
*   **Context & Rationale:** The `derive(PhantomData)` approach is fundamentally incorrect as `PhantomData` is a struct, not a trait. The correct approach is a macro that *adds* a `_phantom` field.
*   **Steps:**
    1.  **Action:** Refactor `derive_tools_meta/src/derive/phantom.rs` into a function-like macro `#[phantom]`.
    2.  **Action:** Update `lib.rs` files to export the new macro.
    3.  **Action:** Update all tests in `tests/inc/phantom/` to use `#[phantom]` instead of `#[derive(PhantomData)]`.
    4.  **Action:** Add a `trybuild` test to ensure `#[derive(PhantomData)]` now fails to compile.
    5.  **Verification:** `timeout 180 cargo test -p derive_tools --test phantom_tests`.
    6.  **Commit.**
*   **Commit Message:** `refactor(derive_tools): Redesign PhantomData to attribute macro, fix tests`

---
#### Group 8: Integration Tests
##### Increment 14: Fix `all_test` Integration
*   **Goal:** Fix the `all_test.rs` which tests multiple derives on a single struct.
*   **Steps:**
    1.  **Action:** Uncomment `all_test` in `tests/inc/mod.rs`.
    2.  **Verification:** `timeout 180 cargo test -p derive_tools --test all_test`.
    3.  **Commit.**
*   **Commit Message:** `fix(derive_tools): Repair all_test integration tests`

##### Increment 15: Fix `basic_test` Integration
*   **Goal:** Fix the `basic_test.rs` which tests a combination of derives.
*   **Steps:**
    1.  **Action:** Uncomment `basic_test` in `tests/inc/mod.rs`.
    2.  **Verification:** `timeout 180 cargo test -p derive_tools --test basic_test`.
    3.  **Commit.**
*   **Commit Message:** `fix(derive_tools): Repair basic_test integration tests`

---
#### Group 9: Finalization
##### Increment 16: Final Code Cleanup and Documentation Review
*   **Goal:** Review the entire crate for code quality, consistency, and documentation.
*   **Steps:**
    1.  **Action:** Run `cargo fmt --all` on the workspace.
    2.  **Action:** Manually review all test files to ensure they have a Test Matrix doc comment.
    3.  **Action:** Review all public APIs in `derive_tools/src/lib.rs` and ensure they are documented.
*   **Commit Message:** `chore(derive_tools): Final cleanup and documentation review`

##### Increment 17: Final Workspace Verification
*   **Goal:** Perform a final, holistic verification of the entire workspace.
*   **Steps:**
    1.  **Action:** Execute `timeout 300 cargo test --workspace --all-features --all-targets`.
    2.  **Action:** Execute `timeout 300 cargo clippy --workspace --all-features -- -D warnings`.
    3.  **Verification:** Check that both commands exit with code 0.
*   **Commit Message:** `chore(workspace): Final verification of all crates`

##### Increment 18: Update Project Changelog
*   **Goal:** Update the `changelog.md` with a summary of the work completed in this task.
*   **Steps:**
    1.  **Action:** Read `module/core/derive_tools/changelog.md`.
    2.  **Action:** Prepend a new entry summarizing the restoration of the test suite and the fixing of all derive macros.
    3.  **Action:** Use `write_to_file` to save the updated changelog.
*   **Commit Message:** `docs(changelog): Document restoration of derive_tools functionality`

### Task Requirements
*   All tests in `module/core/derive_tools/tests/` must be re-enabled and passing.
*   Every primary test file for a derive must have a file-level doc comment containing its relevant Test Matrix rows.
*   The implementation of each derive must match the `spec.md` for the features covered by the *existing* test suite.
*   The entire workspace must pass `clippy -D warnings`.

### Project Requirements
*   (Inherited from workspace)

### Assumptions
*   The `macro_tools/task.md` proposal is sound and will unblock `const` generics tests.

### Out of Scope
*   Implementing new features, even if they are defined in `spec.md`. The focus of this task is to fix and restore existing functionality covered by the current test suite.

### Changelog
*   [Increment 2 | 2025-07-01 20:58 UTC] Fixed `macro_tools` `const` generics bug by reverting changes to `generic_params.rs` as the original code was correct and the issue was not in `macro_tools`.
*   [Increment 1 | 2025-07-01 20:55 UTC] Established initial baseline.
    *   **Commented-out test modules:** `clone_dyn_test`, `variadic_from_test`, `all_manual_test`, `all_test`, `basic_test`, and numerous sub-modules within `deref_tests`, `deref_mut_tests`, `new_tests`, `from_tests`, `not_tests`, `inner_from_tests`, `index_tests`, `index_mut_tests`.
    *   **Failing tests:** None.
    *   **Clippy warnings:** None.
    *   **Compilation warnings:** 2 warnings in `deref/basic_manual_test.rs` about `IsTransparentComplex` struct never being constructed.
*   [YYYY-MM-DD] Initialized V4 of the task plan. Restructured to use atomic, test-driven increments with localized context and dynamic dependency handling.
*   [Increment 3 | 2025-07-01 21:11 UTC] Created external change proposal for `clone_dyn_meta` to fix `GenericsWithWhere` import.