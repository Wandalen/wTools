# Task Plan: Restore `derive_tools` Functionality

### Goal
*   The goal is to restore the full functionality of the `derive_tools` crate by re-enabling all tests, fixing compilation errors/warnings, and ensuring compatibility with `macro_tools` v0.55.0.

### Ubiquitous Language (Vocabulary)
*   **Primary Target Crate:** `module/core/derive_tools`
*   **Meta Crate:** `module/core/derive_tools_meta` (contains procedural macros)
*   **Test Crate:** `module/core/derive_tools/tests` (contains integration tests)
*   **`IsTransparentComplex`:** A struct used in tests that involves complex generics (lifetimes, types, consts) and has been problematic due to `E0207` (unconstrained const parameter) with `macro_tools`.

### Progress
*   **Roadmap Milestone:** M1: Core Derive Macro Functionality
*   **Primary Target Crate:** `module/core/derive_tools`
*   **Overall Progress:** 14/20 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Initial Analysis and Setup
    *   ✅ Increment 2: Plan and Document `AsMut` and `AsRef` Tests
    *   ✅ Increment 3: Fix `AsMut` Tests
    *   ✅ Increment 4: Fix `AsRef` Tests
    *   ✅ Increment 5: Plan and Document `Deref` Tests
    *   ✅ Increment 6: Fix `Deref` Tests
    *   ✅ Increment 7: Fix `Deref` Tests for Enums (and add compile-fail test)
    *   ✅ Increment 8: Address `Deref` Tests for Generics and Bounds (Blocked by `E0207`)
    *   ✅ Increment 9: Plan and Document `DerefMut` Tests
    *   ✅ Increment 10: Fix `DerefMut` Tests
    *   ✅ Increment 11: Plan and Document `From` Tests
    *   ✅ Increment 12: Fix `From` Tests
    *   ✅ Increment 13: Plan and Document `InnerFrom` and `New` Tests
    *   ✅ Increment 14: Fix `InnerFrom` Tests
    *   ⚫ Increment 15: Fix `New` Tests
    *   ⚫ Increment 16: Plan and Document `Not`, `Index`, `IndexMut` Tests
    *   ⚫ Increment 17: Fix `Not` Tests
    *   ⚫ Increment 18: Fix `Index` and `IndexMut` Tests
    *   ⚫ Increment 19: Redesign and Fix `PhantomData` Derive and Tests
    *   ⚫ Increment 20: Final `derive_tools` Verification

### Permissions & Boundaries
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/derive_tools_meta` (Reason: Contains the procedural macros to be fixed)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/derive_tools/src/lib.rs`
    *   `module/core/derive_tools_meta/src/lib.rs`
    *   `module/core/derive_tools_meta/src/derive/as_mut.rs`
    *   `module/core/derive_tools_meta/src/derive/as_ref.rs`
    *   `module/core/derive_tools_meta/src/derive/deref.rs`
    *   `module/core/derive_tools_meta/src/derive/deref_mut.rs`
    *   `module/core/derive_tools_meta/src/derive/from.rs`
    *   `module/core/derive_tools/tests/inc/mod.rs`
    *   `module/core/derive_tools/tests/inc/as_mut/basic_test.rs`
    *   `module/core/derive_tools/tests/inc/as_mut/basic_manual_test.rs`
    *   `module/core/derive_tools/tests/inc/as_ref/basic_test.rs`
    *   `module/core/derive_tools/tests/inc/as_ref/basic_manual_test.rs`
    *   `module/core/derive_tools/tests/inc/deref/basic_test.rs`
    *   `module/core/derive_tools/tests/inc/deref/basic_manual_test.rs`
    *   `module/core/derive_tools/tests/inc/deref/compile_fail_enum.rs`
    *   `module/core/derive_tools/tests/inc/deref_mut/basic_test.rs`
    *   `module/core/derive_tools/tests/inc/deref_mut/basic_manual_test.rs`
    *   `module/core/derive_tools/tests/inc/deref_mut/compile_fail_enum.rs`
    *   `module/core/derive_tools/tests/inc/from/basic_test.rs`
    *   `module/core/derive_tools/tests/inc/from/basic_manual_test.rs`
    *   `module/core/derive_tools/Cargo.toml`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `macro_tools`
    *   `test_tools`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   N/A

### Expected Behavior Rules / Specifications
*   All derive macros should correctly implement their respective traits for structs.
*   `Deref` and `DerefMut` derive macros should explicitly return a `syn::Error` when applied to enums, as these traits are not generally applicable to enums.
*   `AsMut`, `AsRef`, `From` derives should work for both named and unnamed fields, and for various type parameters.
*   The `IsTransparentComplex` struct (with const generics) is currently blocked by `E0207` in `macro_tools` and will be addressed in a separate task or when `macro_tools` is updated. Its tests are temporarily commented out.

### Crate Conformance Check Procedure
*   **Step 1: Run Tests.** Execute `timeout 90 cargo test -p derive_tools --all-targets`. If this fails, fix all test errors before proceeding.
*   **Step 2: Run Linter (Conditional).** Only if Step 1 passes, execute `timeout 90 cargo clippy -p derive_tools -- -D warnings`.

### Increments
(Note: The status of each increment is tracked in the `### Progress` section.)
##### Increment 1: Initial Analysis and Setup
*   **Goal:** Understand the current state of the `derive_tools` crate, identify broken tests, and set up the task plan.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/derive_tools/task.md` (initial plan).
    *   Step 2: Read `module/core/derive_tools/Cargo.toml` to understand dependencies and features.
    *   Step 3: Read `module/core/derive_tools/tests/inc/mod.rs` to see which tests are enabled/disabled.
    *   Step 4: Run `timeout 90 cargo test -p derive_tools --all-targets` to identify all failing tests and compilation errors.
    *   Step 5: Analyze the output to identify the first set of issues to address.
    *   Step 6: Update `task.md` with the initial plan, including `Permissions & Boundaries`, `Relevant Context`, and high-level increments.
    *   Step 7: Perform Increment Verification.
    *   Step 8: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Confirm `task.md` is updated with the initial plan.
*   **Commit Message:** feat(derive_tools): Initial analysis and task plan setup

##### Increment 2: Plan and Document `AsMut` and `AsRef` Tests
*   **Goal:** Create and document the basic test files for `AsMut` and `AsRef` derives, including test matrices.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Create `module/core/derive_tools/tests/inc/as_mut/basic_test.rs` with initial test structure and documentation.
    *   Step 2: Create `module/core/derive_tools/tests/inc/as_mut/basic_manual_test.rs` with manual implementation and documentation.
    *   Step 3: Create `module/core/derive_tools/tests/inc/as_ref/basic_test.rs` with initial test structure and documentation.
    *   Step 4: Create `module/core/derive_tools/tests/inc/as_ref/basic_manual_test.rs` with manual implementation and documentation.
    *   Step 5: Update `module/core/derive_tools/tests/inc/mod.rs` to include `as_mut_manual_test`, `as_mut_test`, `as_ref_manual_test`, and `as_ref_test` modules.
    *   Step 6: Perform Increment Verification.
    *   Step 7: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- as_mut_test as_ref_test` and confirm compilation errors related to missing derives.
*   **Commit Message:** feat(derive_tools): Plan and document AsMut and AsRef tests

##### Increment 3: Fix `AsMut` Tests
*   **Goal:** Implement and fix the `AsMut` derive macro to pass `as_mut_test` and `as_mut_manual_test`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/derive_tools_meta/src/derive/as_mut.rs`.
    *   Step 2: Implement the `AsMut` derive macro logic in `module/core/derive_tools_meta/src/derive/as_mut.rs` to correctly generate `AsMut` implementations for structs.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- as_mut_test as_mut_manual_test` and confirm tests pass.
*   **Commit Message:** fix(derive_tools): Implement and fix AsMut derive macro

##### Increment 4: Fix `AsRef` Tests
*   **Goal:** Implement and fix the `AsRef` derive macro to pass `as_ref_test` and `as_ref_manual_test`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/derive_tools_meta/src/derive/as_ref.rs`.
    *   Step 2: Implement the `AsRef` derive macro logic in `module/core/derive_tools_meta/src/derive/as_ref.rs` to correctly generate `AsRef` implementations for structs.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- as_ref_test as_ref_manual_test` and confirm tests pass.
*   **Commit Message:** fix(derive_tools): Implement and fix AsRef derive macro

##### Increment 5: Plan and Document `Deref` Tests
*   **Goal:** Create and document the basic test files for `Deref` derive, including test matrices.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Create `module/core/derive_tools/tests/inc/deref/basic_test.rs` with initial test structure and documentation. Temporarily comment out `IsTransparentComplex` due to `E0207`.
    *   Step 2: Create `module/core/derive_tools/tests/inc/deref/basic_manual_test.rs` with manual implementation and documentation. Temporarily comment out `IsTransparentComplex` due to `E0207`.
    *   Step 3: Update `module/core/derive_tools/tests/inc/mod.rs` to include `deref_tests` module.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- deref_test` and confirm compilation errors related to missing derives.
*   **Commit Message:** feat(derive_tools): Plan and document Deref tests

##### Increment 6: Fix `Deref` Tests
*   **Goal:** Implement and fix the `Deref` derive macro for basic structs to pass `deref_tests`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/derive_tools_meta/src/derive/deref.rs`.
    *   Step 2: Implement the `Deref` derive macro logic in `module/core/derive_tools_meta/src/derive/deref.rs` to correctly generate `Deref` implementations for structs.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- deref_test` and confirm tests pass for basic structs.
*   **Commit Message:** fix(derive_tools): Implement and fix Deref derive macro for basic structs

##### Increment 7: Fix `Deref` Tests for Enums (and add compile-fail test)
*   **Goal:** Modify the `Deref` derive macro to explicitly return a `syn::Error` when applied to an enum, and add a compile-fail test to verify this behavior.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Modify `module/core/derive_tools_meta/src/derive/deref.rs` to return `syn::Error` when applied to `StructLike::Enum`.
    *   Step 2: Create `module/core/derive_tools/tests/inc/deref/compile_fail_enum.rs` with a test case that applies `Deref` to an enum.
    *   Step 3: Update `module/core/derive_tools/tests/inc/mod.rs` to include the `deref_trybuild` test.
    *   Step 4: Modify `module/core/derive_tools/Cargo.toml` to ensure `derive_tools_meta` is available for `trybuild` tests.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- deref_trybuild` and confirm the compile-fail test passes.
*   **Commit Message:** fix(derive_tools): Deref macro rejects enums with compile-fail test

##### Increment 8: Address `Deref` Tests for Generics and Bounds (Blocked by `E0207`)
*   **Goal:** Acknowledge and document the blocking `E0207` issue with `IsTransparentComplex` and defer its resolution.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Explicitly comment out `IsTransparentComplex` related code in `module/core/derive_tools/tests/inc/deref/basic_test.rs` and `module/core/derive_tools/tests/inc/deref/basic_manual_test.rs`.
    *   Step 2: Update `task.md` to clearly state that this increment is blocked by `E0207` and its resolution is deferred.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --all-targets` and confirm no new errors or warnings related to `IsTransparentComplex` appear.
*   **Commit Message:** docs(derive_tools): Defer Deref generics tests due to E0207

##### Increment 9: Plan and Document `DerefMut` Tests
*   **Goal:** Create and document the basic test files for `DerefMut` derive, including test matrices.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Create `module/core/derive_tools/tests/inc/deref_mut/basic_test.rs` with initial test structure and documentation. Temporarily comment out `IsTransparentComplex` due to `E0207`.
    *   Step 2: Create `module/core/derive_tools/tests/inc/deref_mut/basic_manual_test.rs` with manual implementation and documentation. Temporarily comment out `IsTransparentComplex` due to `E0207`.
    *   Step 3: Update `module/core/derive_tools/tests/inc/mod.rs` to include `deref_mut_tests` module.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- deref_mut_test` and confirm compilation errors related to missing derives.
*   **Commit Message:** feat(derive_tools): Plan and document DerefMut tests

##### Increment 10: Fix `DerefMut` Tests
*   **Goal:** Implement and fix the `DerefMut` derive macro for basic structs to pass `deref_mut_tests`. Also, ensure it rejects enums with a compile-fail test.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/derive_tools_meta/src/derive/deref_mut.rs`.
    *   Step 2: Implement the `DerefMut` derive macro logic in `module/core/derive_tools_meta/src/derive/deref_mut.rs` to correctly generate `DerefMut` implementations for structs. Ensure `Deref` is also derived or implemented for the target type.
    *   Step 3: Modify `module/core/derive_tools_meta/src/derive/deref_mut.rs` to return `syn::Error` when applied to `StructLike::Enum`.
    *   Step 4: Create `module/core/derive_tools/tests/inc/deref_mut/compile_fail_enum.rs` with a test case that applies `DerefMut` to an enum.
    *   Step 5: Update `module/core/derive_tools/tests/inc/mod.rs` to include the `deref_mut_trybuild` test.
    *   Step 6: Clean up unused imports/variables in `module/core/derive_tools_meta/src/derive/deref_mut.rs`.
    *   Step 7: Perform Increment Verification.
    *   Step 8: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- deref_mut_test deref_mut_trybuild` and confirm tests pass and compile-fail test passes.
*   **Commit Message:** fix(derive_tools): Implement and fix DerefMut derive macro, reject enums

##### Increment 11: Plan and Document `From` Tests
*   **Goal:** Create and document the basic test files for `From` derive, including test matrices.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Create `module/core/derive_tools/tests/inc/from/basic_test.rs` with initial test structure and documentation. Temporarily comment out `IsTransparentComplex` due to `E0207`.
    *   Step 2: Create `module/core/derive_tools/tests/inc/from/basic_manual_test.rs` with manual implementation and documentation. Temporarily comment out `IsTransparentComplex` due to `E0207`.
    *   Step 3: Update `module/core/derive_tools/tests/inc/mod.rs` to include `from_tests` module.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- from_test` and confirm compilation errors related to missing derives.
*   **Commit Message:** feat(derive_tools): Plan and document From tests

##### Increment 12: Fix `From` Tests
*   **Goal:** Implement and fix the `From` derive macro for basic structs to pass `from_tests`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/derive_tools_meta/src/derive/from.rs`.
    *   Step 2: Implement the `From` derive macro logic in `module/core/derive_tools_meta/src/derive/from.rs` to correctly generate `From` implementations for structs.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- from_test` and confirm tests pass.
*   **Commit Message:** fix(derive_tools): Implement and fix From derive macro

##### Increment 13: Plan and Document `InnerFrom` and `New` Tests
*   **Goal:** Create and document the basic test files for `InnerFrom` and `New` derives, including test matrices.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Create `module/core/derive_tools/tests/inc/inner_from/basic_test.rs` with initial test structure and documentation.
    *   Step 2: Create `module/core/derive_tools/tests/inc/inner_from/basic_manual_test.rs` with manual implementation and documentation.
    *   Step 3: Create `module/core/derive_tools/tests/inc/new/basic_test.rs` with initial test structure and documentation.
    *   Step 4: Create `module/core/derive_tools/tests/inc/new/basic_manual_test.rs` with manual implementation and documentation.
    *   Step 5: Update `module/core/derive_tools/tests/inc/mod.rs` to include `inner_from_tests` and `new_tests` modules.
    *   Step 6: Perform Increment Verification.
    *   Step 7: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- inner_from_test new_test` and confirm compilation errors related to missing derives.
*   **Commit Message:** feat(derive_tools): Plan and document InnerFrom and New tests

##### Increment 14: Fix `InnerFrom` Tests
*   **Goal:** Implement and fix the `InnerFrom` derive macro for basic structs to pass `inner_from_tests`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/derive_tools_meta/src/derive/inner_from.rs`.
    *   Step 2: Implement the `InnerFrom` derive macro logic in `module/core/derive_tools_meta/src/derive/inner_from.rs` to correctly generate `InnerFrom` implementations for structs.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- inner_from_test` and confirm tests pass.
*   **Commit Message:** fix(derive_tools): Implement and fix InnerFrom derive macro

##### Increment 15: Fix `New` Tests
*   **Goal:** Implement and fix the `New` derive macro for basic structs to pass `new_tests`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/derive_tools_meta/src/derive/new.rs`.
    *   Step 2: Implement the `New` derive macro logic in `module/core/derive_tools_meta/src/derive/new.rs` to correctly generate `New` implementations for structs.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- new_test` and confirm tests pass.
*   **Commit Message:** fix(derive_tools): Implement and fix New derive macro

##### Increment 16: Plan and Document `Not`, `Index`, `IndexMut` Tests
*   **Goal:** Create and document the basic test files for `Not`, `Index`, and `IndexMut` derives, including test matrices.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Create `module/core/derive_tools/tests/inc/not/basic_test.rs` with initial test structure and documentation.
    *   Step 2: Create `module/core/derive_tools/tests/inc/not/basic_manual_test.rs` with manual implementation and documentation.
    *   Step 3: Create `module/core/derive_tools/tests/inc/index/basic_test.rs` with initial test structure and documentation.
    *   Step 4: Create `module/core/derive_tools/tests/inc/index/basic_manual_test.rs` with manual implementation and documentation.
    *   Step 5: Create `module/core/derive_tools/tests/inc/index_mut/basic_test.rs` with initial test structure and documentation.
    *   Step 6: Create `module/core/derive_tools/tests/inc/index_mut/basic_manual_test.rs` with manual implementation and documentation.
    *   Step 7: Update `module/core/derive_tools/tests/inc/mod.rs` to include `not_tests`, `index_tests`, and `index_mut_tests` modules.
    *   Step 8: Perform Increment Verification.
    *   Step 9: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- not_test index_test index_mut_test` and confirm compilation errors related to missing derives.
*   **Commit Message:** feat(derive_tools): Plan and document Not, Index, IndexMut tests

##### Increment 17: Fix `Not` Tests
*   **Goal:** Implement and fix the `Not` derive macro for basic structs to pass `not_tests`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/derive_tools_meta/src/derive/not.rs`.
    *   Step 2: Implement the `Not` derive macro logic in `module/core/derive_tools_meta/src/derive/not.rs` to correctly generate `Not` implementations for structs.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- not_test` and confirm tests pass.
*   **Commit Message:** fix(derive_tools): Implement and fix Not derive macro

##### Increment 18: Fix `Index` and `IndexMut` Tests
*   **Goal:** Implement and fix the `Index` and `IndexMut` derive macros for basic structs to pass `index_tests` and `index_mut_tests`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/derive_tools_meta/src/derive/index.rs` and `module/core/derive_tools_meta/src/derive/index_mut.rs`.
    *   Step 2: Implement the `Index` derive macro logic in `module/core/derive_tools_meta/src/derive/index.rs` to correctly generate `Index` implementations for structs.
    *   Step 3: Implement the `IndexMut` derive macro logic in `module/core/derive_tools_meta/src/derive/index_mut.rs` to correctly generate `IndexMut` implementations for structs.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- index_test index_mut_test` and confirm tests pass.
*   **Commit Message:** fix(derive_tools): Implement and fix Index and IndexMut derive macros

##### Increment 19: Redesign and Fix `PhantomData` Derive and Tests
*   **Goal:** Redesign and fix the `PhantomData` derive macro and its tests to ensure correct behavior and compatibility.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Analyze existing `PhantomData` derive macro and tests.
    *   Step 2: Propose a redesign for the `PhantomData` derive macro if necessary, considering the `E0207` issue.
    *   Step 3: Implement the redesigned `PhantomData` derive macro.
    *   Step 4: Update or rewrite `PhantomData` tests to reflect the redesign and ensure full coverage.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Run `timeout 90 cargo test -p derive_tools --test tests -- phantom_test` and confirm tests pass.
*   **Commit Message:** refactor(derive_tools): Redesign and fix PhantomData derive and tests

##### Increment 20: Final `derive_tools` Verification
*   **Goal:** Perform a final, holistic verification of the entire `derive_tools` crate to ensure all tests pass, no warnings are present, and the crate is in a clean state.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Run `timeout 90 cargo test -p derive_tools --all-targets` to ensure all tests pass.
    *   Step 2: Run `timeout 90 cargo clippy -p derive_tools -- -D warnings` to ensure no warnings are present.
    *   Step 3: Run `git status` to confirm a clean working directory.
    *   Step 4: Self-critique: Review the entire task's output against all requirements and design principles.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Confirm all tests pass, no warnings, and clean git status.
*   **Commit Message:** chore(derive_tools): Final verification and cleanup
