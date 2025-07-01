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
*   **Overall Progress:** 5/20 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Initial `derive_tools` Analysis and Baseline
    *   ✅ Increment 2: Plan and Document `AsMut` and `AsRef` Tests
    *   ✅ Increment 3: Fix `as_mut` tests
    *   ✅ Increment 4: Fix `as_ref` tests
    *   ✅ Increment 5: Plan and Document `Deref` Tests
    *   ⏳ Increment 6: Fix `Deref` tests for basic structs
    *   ⚫ Increment 7: Fix `Deref` tests for enums
    *   ⚫ Increment 8: Fix `Deref` tests for generics and bounds
    *   ⚫ Increment 9: Plan and Document `DerefMut` Tests
    *   ⚫ Increment 10: Fix `DerefMut` tests
    *   ⚫ Increment 11: Plan and Document `From` Tests
    *   ⚫ Increment 12: Fix `From` tests
    *   ⚫ Increment 13: Plan and Document `InnerFrom` and `New` tests
    *   ⚫ Increment 14: Fix `InnerFrom` tests
    *   ⚫ Increment 15: Fix `New` tests
    *   ⚫ Increment 16: Plan and Document `Not`, `Index`, `IndexMut` tests
    *   ⚫ Increment 17: Fix `Not` tests
    *   ⚫ Increment 18: Fix `Index` and `IndexMut` tests
    *   ⚫ Increment 19: Redesign and Fix `PhantomData` derive and tests
    *   ⚫ Increment 20: Final `derive_tools` Verification

### Permissions & Boundaries
*   **Run workspace-wise commands:** false
*   **Add transient comments:** false
*   **Additional Editable Crates:**
    *   `module/core/derive_tools_meta` (Reason: Fixes to macro implementations are required)

### Relevant Context
*   Control Files to Reference:
    *   `module/core/macro_tools/task.md` (Proposal to fix `const` generics issue)
    *   `module/core/clone_dyn/task.md` (Proposal to fix `clippy::doc_markdown` warning)
    *   `module/core/derive_tools/task/postpone_no_std_refactoring_task.md` (New task for `no_std` refactoring postponement)
    *   `module/move/willbe/task/remove_pth_std_feature_dependency_task.md` (New task proposal for `willbe` to resolve `pth` `std` feature conflict)
    *   `module/core/pth/task/no_std_refactoring_task.md` (New task for `pth` `no_std` refactoring postponement)
    *   `module/core/error_tools/task/no_std_refactoring_task.md` (New task for `error_tools` `no_std` refactoring postponement)
    *   `module/core/clone_dyn/task/fix_test_issues_task.md` (New task for `clone_dyn` test issues)
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

##### Increment 1: Initial `derive_tools` Analysis and Baseline
*   **Goal:** Establish a clear baseline of the current compilation and test failures for the `derive_tools` crate only.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Execute `timeout 120 cargo test -p derive_tools --all-targets`.
    *   Step 2: Analyze the output to identify the primary points of failure within `derive_tools`.
    *   Step 3: Document the initial error state in the `### Changelog` section of this plan.
*   **Increment Verification:**
    *   The initial error state for `derive_tools` is successfully logged.
*   **Commit Message:** `chore(derive_tools): Establish baseline for derive_tools fix`

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
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test tests -- as_mut_test`.
    *   Step 3: If the test fails, apply the Critical Log Analysis Procedure to the output. Hypothesize that the `AsMut` derive in `derive_tools_meta` is not generating the correct implementation.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/as_mut.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test tests -- as_mut_manual_test` and `timeout 90 cargo test -p derive_tools --test tests -- as_mut_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix as_mut tests`

##### Increment 4: Fix `as_ref` tests
*   **Goal:** Re-enable the `as_ref_test` and `as_ref_manual_test` modules and fix any resulting issues.
*   **Specification Reference:** T3.1, T3.2
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod as_ref_manual_test;` and `mod as_ref_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test tests -- as_ref_test`.
    *   Step 3: If the test fails, apply Critical Log Analysis.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/as_ref.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test tests -- as_ref_manual_test` and `timeout 90 cargo test -p derive_tools --test tests -- as_ref_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix as_ref tests`

##### Increment 5: Plan and Document `Deref` Tests
*   **Goal:** Create the test matrices for `Deref` and add them as documentation to the relevant test files.
*   **Specification Reference:** N/A
*   **Test Matrix for `Deref`:**
    | ID   | Struct Type        | Inner Type | Implementation | Expected Behavior                                       | Test File                   |
    |------|--------------------|------------|----------------|---------------------------------------------------------|-----------------------------|
    | T5.1 | Tuple struct (1 field) | `i32`      | `#[derive(Deref)]` | Dereferencing returns a reference to the inner `i32`. | `deref_test.rs`             |
    | T5.2 | Tuple struct (1 field) | `i32`      | Manual `impl`  | Dereferencing returns a reference to the inner `i32`. | `deref_manual_test.rs`      |
    | T5.3 | Named struct (1 field) | `String`   | `#[derive(Deref)]` | Dereferencing returns a reference to the inner `String`. | `deref_test.rs`             |
    | T5.4 | Named struct (1 field) | `String`   | Manual `impl`  | Dereferencing returns a reference to the inner `String`. | `deref_manual_test.rs`      |
*   **Steps:**
    *   Step 1: Create file `tests/inc/deref_test.rs` with initial content `include!( "./only_test/deref.rs" );`.
    *   Step 2: Create file `tests/inc/deref_manual_test.rs` with initial content `include!( "./only_test/deref.rs" );`.
    *   Step 3: Create file `tests/inc/only_test/deref.rs` with initial content `#[ test ] fn deref_test() { }`.
    *   Step 4: Use `insert_content` to add the `Deref` test matrix as a file-level doc comment to `tests/inc/deref_test.rs`.
    *   Step 5: Use `insert_content` to add a doc comment explaining the purpose of the test function in `tests/inc/only_test/deref.rs`.
    *   Step 6: Use `insert_content` to add the `Deref` test matrix as a file-level doc comment to `tests/inc/deref_manual_test.rs`.
*   **Increment Verification:**
    *   Use `read_file` to confirm the documentation has been added correctly to all three files.
*   **Commit Message:** `docs(test): Add test matrices and purpose for Deref`

##### Increment 6: Fix `Deref` tests for basic structs
*   **Goal:** Re-enable and fix `Deref` tests for basic structs.
*   **Specification Reference:** T5.1, T5.2, T5.3, T5.4
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod deref_manual_test;` and `mod deref_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test tests -- deref_test`.
    *   Step 3: If the test fails, apply Critical Log Analysis.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/deref.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test tests -- deref_manual_test` and `timeout 90 cargo test -p derive_tools --test tests -- deref_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix Deref tests for basic structs`

##### Increment 7: Fix `Deref` tests for enums
*   **Goal:** Re-enable and fix `Deref` tests for enums.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod deref_enum_test;` and `mod deref_enum_manual_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test tests -- deref_enum_test`.
    *   Step 3: If the test fails, apply Critical Log Analysis.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/deref.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test tests -- deref_enum_manual_test` and `timeout 90 cargo test -p derive_tools --test tests -- deref_enum_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix Deref tests for enums`

##### Increment 8: Fix `Deref` tests for generics and bounds
*   **Goal:** Re-enable and fix `Deref` tests for generics and bounds.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod deref_generics_test;` and `mod deref_generics_manual_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test tests -- deref_generics_test`.
    *   Step 3: If the test fails, apply Critical Log Analysis.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/deref.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test tests -- deref_generics_manual_test` and `timeout 90 cargo test -p derive_tools --test tests -- deref_generics_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix Deref tests for generics and bounds`

##### Increment 9: Plan and Document `DerefMut` Tests
*   **Goal:** Create the test matrices for `DerefMut` and add them as documentation to the relevant test files.
*   **Specification Reference:** N/A
*   **Test Matrix for `DerefMut`:**
    | ID   | Struct Type        | Inner Type | Implementation | Expected Behavior                                           | Test File                   |
    |------|--------------------|------------|----------------|-------------------------------------------------------------|-----------------------------|
    | T9.1 | Tuple struct (1 field) | `i32`      | `#[derive(DerefMut)]` | Dereferencing returns a mutable reference to the inner `i32`. | `deref_mut_test.rs`         |
    | T9.2 | Tuple struct (1 field) | `i32`      | Manual `impl`  | Dereferencing returns a mutable reference to the inner `i32`. | `deref_mut_manual_test.rs`  |
    | T9.3 | Named struct (1 field) | `String`   | `#[derive(DerefMut)]` | Dereferencing returns a mutable reference to the inner `String`. | `deref_mut_test.rs`         |
    | T9.4 | Named struct (1 field) | `String`   | Manual `impl`  | Dereferencing returns a mutable reference to the inner `String`. | `deref_mut_manual_test.rs`  |
*   **Steps:**
    *   Step 1: Use `insert_content` to add the `DerefMut` test matrix as a file-level doc comment to `tests/inc/deref_mut_test.rs`.
    *   Step 2: Use `insert_content` to add a doc comment explaining the purpose of the test function in `tests/inc/only_test/deref_mut.rs`.
*   **Increment Verification:**
    *   Use `read_file` to confirm the documentation has been added correctly to both files.
*   **Commit Message:** `docs(test): Add test matrices and purpose for DerefMut`

##### Increment 10: Fix `DerefMut` tests
*   **Goal:** Re-enable and fix `DerefMut` tests.
*   **Specification Reference:** T9.1, T9.2, T9.3, T9.4
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod deref_mut_manual_test;` and `mod deref_mut_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test tests -- deref_mut_test`.
    *   Step 3: If the test fails, apply Critical Log Analysis.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/deref_mut.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test tests -- deref_mut_manual_test` and `timeout 90 cargo test -p derive_tools --test tests -- deref_mut_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix DerefMut tests`

##### Increment 11: Plan and Document `From` Tests
*   **Goal:** Create the test matrices for `From` and add them as documentation to the relevant test files.
*   **Specification Reference:** N/A
*   **Test Matrix for `From`:**
    | ID    | Struct Type        | Source Type | Implementation | Expected Behavior                                       | Test File                   |
    |-------|--------------------|-------------|----------------|---------------------------------------------------------|-----------------------------|
    | T11.1 | Tuple struct (1 field) | `i32`       | `#[derive(From)]` | `From<i32>` is implemented, allowing conversion.        | `from_test.rs`              |
    | T11.2 | Tuple struct (1 field) | `i32`       | Manual `impl`  | `From<i32>` is implemented, allowing conversion.        | `from_manual_test.rs`       |
    | T11.3 | Named struct (1 field) | `String`    | `#[derive(From)]` | `From<String>` is implemented, allowing conversion.     | `from_test.rs`              |
    | T11.4 | Named struct (1 field) | `String`    | Manual `impl`  | `From<String>` is implemented, allowing conversion.     | `from_manual_test.rs`       |
*   **Steps:**
    *   Step 1: Use `insert_content` to add the `From` test matrix as a file-level doc comment to `tests/inc/from_test.rs`.
    *   Step 2: Use `insert_content` to add a doc comment explaining the purpose of the test function in `tests/inc/only_test/from.rs`.
*   **Increment Verification:**
    *   Use `read_file` to confirm the documentation has been added correctly to both files.
*   **Commit Message:** `docs(test): Add test matrices and purpose for From`

##### Increment 12: Fix `From` tests
*   **Goal:** Re-enable and fix `From` tests.
*   **Specification Reference:** T11.1, T11.2, T11.3, T11.4
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod from_manual_test;` and `mod from_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test tests -- from_test`.
    *   Step 3: If the test fails, apply Critical Log Analysis.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/from.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test tests -- from_manual_test` and `timeout 90 cargo test -p derive_tools --test tests -- from_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix From tests`

##### Increment 13: Plan and Document `InnerFrom` and `New` tests
*   **Goal:** Create the test matrices for `InnerFrom` and `New` and add them as documentation to the relevant test files.
*   **Specification Reference:** N/A
*   **Test Matrix for `InnerFrom`:**
    | ID    | Struct Type        | Inner Type | Implementation | Expected Behavior                                       | Test File                   |
    |-------|--------------------|------------|----------------|---------------------------------------------------------|-----------------------------|
    | T13.1 | Tuple struct (1 field) | `i32`      | `#[derive(InnerFrom)]` | `From<i32>` is implemented for the inner type.          | `inner_from_test.rs`        |
    | T13.2 | Tuple struct (1 field) | `i32`      | Manual `impl`  | `From<i32>` is implemented for the inner type.          | `inner_from_manual_test.rs` |
*   **Test Matrix for `New`:**
    | ID    | Struct Type        | Fields     | Implementation | Expected Behavior                                       | Test File                   |
    |-------|--------------------|------------|----------------|---------------------------------------------------------|-----------------------------|
    | T14.1 | Tuple struct (1 field) | 1          | `#[derive(New)]` | `new()` constructor is generated.                       | `new_test.rs`               |
    | T14.2 | Tuple struct (1 field) | 1          | Manual `impl`  | `new()` constructor is generated.                       | `new_manual_test.rs`        |
    | T14.3 | Named struct (1 field) | 1          | `#[derive(New)]` | `new()` constructor is generated.                       | `new_test.rs`               |
    | T14.4 | Named struct (1 field) | 1          | Manual `impl`  | `new()` constructor is generated.                       | `new_manual_test.rs`        |
*   **Steps:**
    *   Step 1: Use `insert_content` to add the `InnerFrom` test matrix as a file-level doc comment to `tests/inc/inner_from_test.rs`.
    *   Step 2: Use `insert_content` to add a doc comment explaining the purpose of the test function in `tests/inc/only_test/inner_from.rs`.
    *   Step 3: Use `insert_content` to add the `New` test matrix as a file-level doc comment to `tests/inc/new_test.rs`.
    *   Step 4: Use `insert_content` to add a doc comment explaining the purpose of the test function in `tests/inc/only_test/new.rs`.
*   **Increment Verification:**
    *   Use `read_file` to confirm the documentation has been added correctly to all four files.
*   **Commit Message:** `docs(test): Add test matrices and purpose for InnerFrom and New`

##### Increment 14: Fix `InnerFrom` tests
*   **Goal:** Re-enable and fix `InnerFrom` tests.
*   **Specification Reference:** T13.1, T13.2
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod inner_from_manual_test;` and `mod inner_from_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test tests -- inner_from_test`.
    *   Step 3: If the test fails, apply Critical Log Analysis.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/inner_from.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test tests -- inner_from_manual_test` and `timeout 90 cargo test -p derive_tools --test tests -- inner_from_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix InnerFrom tests`

##### Increment 15: Fix `New` tests
*   **Goal:** Re-enable and fix `New` tests.
*   **Specification Reference:** T14.1, T14.2, T14.3, T14.4
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod new_manual_test;` and `mod new_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test tests -- new_test`.
    *   Step 3: If the test fails, apply Critical Log Analysis.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/new.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test tests -- new_manual_test` and `timeout 90 cargo test -p derive_tools --test tests -- new_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix New tests`

##### Increment 16: Plan and Document `Not`, `Index`, `IndexMut` tests
*   **Goal:** Create the test matrices for `Not`, `Index`, and `IndexMut` and add them as documentation to the relevant test files.
*   **Specification Reference:** N/A
*   **Test Matrix for `Not`:**
    | ID    | Struct Type        | Inner Type | Implementation | Expected Behavior                                       | Test File                   |
    |-------|--------------------|------------|----------------|---------------------------------------------------------|-----------------------------|
    | T16.1 | Tuple struct (1 field) | `bool`     | `#[derive(Not)]` | `!` operator returns the logical NOT of the inner field. | `not_test.rs`               |
    | T16.2 | Tuple struct (1 field) | `bool`     | Manual `impl`  | `!` operator returns the logical NOT of the inner field. | `not_manual_test.rs`        |
*   **Test Matrix for `Index`:**
    | ID    | Struct Type        | Inner Type | Implementation | Expected Behavior                                       | Test File                   |
    |-------|--------------------|------------|----------------|---------------------------------------------------------|-----------------------------|
    | T17.1 | Tuple struct (1 field) | `Vec<i32>` | `#[derive(Index)]` | `[]` operator returns a reference to an element.        | `index_test.rs`             |
    | T17.2 | Tuple struct (1 field) | `Vec<i32>` | Manual `impl`  | `[]` operator returns a reference to an element.        | `index_manual_test.rs`      |
*   **Test Matrix for `IndexMut`:**
    | ID    | Struct Type        | Inner Type | Implementation | Expected Behavior                                           | Test File                   |
    |-------|--------------------|------------|----------------|-------------------------------------------------------------|-----------------------------|
    | T18.1 | Tuple struct (1 field) | `Vec<i32>` | `#[derive(IndexMut)]` | `[]` operator returns a mutable reference to an element.    | `index_mut_test.rs`         |
    | T18.2 | Tuple struct (1 field) | `Vec<i32>` | Manual `impl`  | `[]` operator returns a mutable reference to an element.    | `index_mut_manual_test.rs`  |
*   **Steps:**
    *   Step 1: Use `insert_content` to add the `Not` test matrix as a file-level doc comment to `tests/inc/not_test.rs`.
    *   Step 2: Use `insert_content` to add a doc comment explaining the purpose of the test function in `tests/inc/only_test/not.rs`.
    *   Step 3: Use `insert_content` to add the `Index` test matrix as a file-level doc comment to `tests/inc/index_test.rs`.
    *   Step 4: Use `insert_content` to add a doc comment explaining the purpose of the test function in `tests/inc/only_test/index.rs`.
    *   Step 5: Use `insert_content` to add the `IndexMut` test matrix as a file-level doc comment to `tests/inc/index_mut_test.rs`.
    *   Step 6: Use `insert_content` to add a doc comment explaining the purpose of the test function in `tests/inc/only_test/index_mut.rs`.
*   **Increment Verification:**
    *   Use `read_file` to confirm the documentation has been added correctly to all six files.
*   **Commit Message:** `docs(test): Add test matrices and purpose for Not, Index, IndexMut`

##### Increment 17: Fix `Not` tests
*   **Goal:** Re-enable and fix `Not` tests.
*   **Specification Reference:** T16.1, T16.2
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod not_manual_test;` and `mod not_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test tests -- not_test`.
    *   Step 3: If the test fails, apply Critical Log Analysis.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/not.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test tests -- not_manual_test` and `timeout 90 cargo test -p derive_tools --test tests -- not_test`. Verify both pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix Not tests`

##### Increment 18: Fix `Index` and `IndexMut` tests
*   **Goal:** Re-enable and fix `Index` and `IndexMut` tests.
*   **Specification Reference:** T17.1, T17.2, T18.1, T18.2
*   **Steps:**
    *   Step 1: Use `search_and_replace` on `module/core/derive_tools/tests/inc/mod.rs` to uncomment `mod index_manual_test;`, `mod index_test;`, `mod index_mut_manual_test;`, and `mod index_mut_test;`.
    *   Step 2: Execute `timeout 90 cargo test -p derive_tools --test tests -- index_test`.
    *   Step 3: If the test fails, apply Critical Log Analysis.
    *   Step 4: Propose and apply a fix to `derive_tools_meta/src/derive/index.rs` and `derive_tools_meta/src/derive/index_mut.rs`.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p derive_tools --test tests -- index_manual_test`, `timeout 90 cargo test -p derive_tools --test tests -- index_test`, `timeout 90 cargo test -p derive_tools --test tests -- index_mut_manual_test`, and `timeout 90 cargo test -p derive_tools --test tests -- index_mut_test`. Verify all pass.
*   **Commit Message:** `fix(derive_tools): Re-enable and fix Index and IndexMut tests`

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

##### Increment 20: Final `derive_tools` Verification
*   **Goal:** Perform a final, comprehensive check of the `derive_tools` crate to ensure no regressions were introduced.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Execute `timeout 120 cargo test -p derive_tools --all-targets`.
    *   Step 2: Execute `timeout 120 cargo clippy -p derive_tools -- -D warnings`.
    *   Step 3: Analyze results. If all checks pass, the task is complete.
*   **Increment Verification:**
    *   All `derive_tools` checks pass.
*   **Commit Message:** `chore(derive_tools): Final verification of derive_tools fixes`

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
*   **`no_std` compatibility for `pth` and `error_tools` (postponed to a new task).**

### External System Dependencies (Optional)
*   N/A

### Notes & Insights
*   The `no_std` compatibility issues in `pth` and `error_tools` have been formally postponed to new tasks. This task will proceed without addressing `no_std` for these external crates.

### Changelog
*   [Increment 5 | 2025-07-01 12:09 UTC] Created `deref_test.rs`, `deref_manual_test.rs`, and `only_test/deref.rs` and added test matrices and doc comments.
*   [Increment 1 | 2025-07-01 09:16 UTC] Initial workspace test run failed with errors in `pth` and `wca` crates, primarily related to missing `use` statements and conflicting trait implementations.
*   [Increment 1 | 2025-07-01 11:12 UTC] `cargo test -p derive_tools --all-targets` failed due to unresolved modules (`the_module`), missing macros (`a_id`), and unrecognized attributes (`clone_dyn`) originating from `clone_dyn` crate's tests, which are included in `derive_tools`'s test suite.
*   [2025-07-01 11:18 UTC] Updated test command syntax in plan to correctly target internal test modules.
