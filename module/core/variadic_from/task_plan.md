
# Task Plan: Align `variadic_from` with Specification v1.1

### Goal
*   Refactor the `variadic_from` and `variadic_from_meta` crates to be fully compliant with `spec.md`. This involves correcting the derive macro's code generation, overhauling the test suite for comprehensive coverage, updating documentation to be accurate and testable, and ensuring all code adheres to the project's codestyle.

### Ubiquitous Language (Vocabulary)
*   **Variadic Constructor:** A constructor that can accept a variable number of arguments, implemented via the `from!` macro.
*   **`FromN` Traits:** A set of traits (`From1`, `From2`, `From3`) defining a contract for constructing a type from `N` arguments.
*   **`VariadicFrom` Trait:** A marker trait (`#[derive(VariadicFrom)]`) that triggers the automatic implementation of `FromN` and standard `From` traits.
*   **Convenience Implementation:** An `impl FromM for StructWithNFields` where `M < N`, generated only when field types are identical, for ergonomic single-argument construction.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/variadic_from`
*   **Overall Progress:** 0/4 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Refactor `variadic_from_meta` for Spec Compliance
    *   ⚫ Increment 2: Overhaul and Restructure Test Suite
    *   ⚫ Increment 3: Refactor `variadic_from` Library and Update `Readme.md`
    *   ⚫ Increment 4: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/variadic_from_meta`

### Relevant Context
*   **Specification:** `module/core/variadic_from/spec.md`
*   **Codestyle:** `code/rules/codestyle.md`
*   **Files to Modify:**
    *   `module/core/variadic_from/src/lib.rs`
    *   `module/core/variadic_from/src/variadic.rs`
    *   `module/core/variadic_from/Readme.md`
    *   `module/core/variadic_from/tests/inc/mod.rs`
    *   `module/core/variadic_from_meta/src/lib.rs`
    *   `module/core/variadic_from_meta/Cargo.toml`

### Crate Conformance Check Procedure
*   **Step 1: Run All Tests.** Execute `timeout 90 cargo test --workspace` and verify no failures.
*   **Step 2: Run Linter.** Execute `timeout 90 cargo clippy --workspace -- -D warnings` and verify no errors or warnings.
*   **Step 3: Run Doc Tests.** Execute `timeout 90 cargo test --workspace --doc` and verify no failures.
*   **Step 4: Check Git Status.** Execute `git status` to ensure no unexpected uncommitted files.

### Increments

##### Increment 1: Refactor `variadic_from_meta` for Spec Compliance
*   **Goal:** Correct the `VariadicFrom` derive macro to generate code that strictly adheres to `spec.md`.
*   **Specification Reference:** `spec.md` Section 3.1, 6.4
*   **Steps:**
    1.  Read `module/core/variadic_from_meta/src/lib.rs` and `module/core/variadic_from_meta/Cargo.toml`.
    2.  In `lib.rs`, remove `attributes(from)` from the `#[proc_macro_derive]` definition.
    3.  Refactor the code generation logic to be modular. Create helper functions to generate `FromN` impls and `From<tuple>` impls.
    4.  Modify the `From<T>` and `From<(T1, ...)>` generation to **delegate** to the corresponding `FromN` trait method (e.g., `fn from(src: T) -> Self { Self::from1(src) }`).
    5.  Implement conditional logic for generating convenience `FromN` implementations. This requires comparing `syn::Type` equality.
        *   For 2-field structs, generate `impl From1` only if `field_type_1 == field_type_2`.
        *   For 3-field structs, generate `impl From1` only if all three field types are identical.
        *   For 3-field structs, generate `impl From2` only if the second and third field types are identical.
    6.  Change all generated paths to `variadic_from` to be absolute (e.g., `::variadic_from::exposed::From1`).
    7.  Ensure the macro generates no code for structs with 0 or >3 fields by returning an empty `TokenStream`.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build -p variadic_from_meta`. Analyze output for success.
    *   Execute `timeout 90 cargo clippy -p variadic_from_meta -- -D warnings`. Analyze output for success.
*   **Commit Message:** `fix(variadic_from_meta): Align derive macro with spec v1.1`

##### Increment 2: Overhaul and Restructure Test Suite
*   **Goal:** Create a new, clean, and comprehensive test suite for `variadic_from` that validates all behaviors defined in `spec.md`.
*   **Specification Reference:** `spec.md` Section 10
*   **Steps:**
    1.  Delete the existing, outdated test files: `variadic_from_derive_test.rs`, `variadic_from_manual_test.rs`, `variadic_from_only_test.rs`, and all other test files in `tests/inc/` except `mod.rs` and `compile_fail/`.
    2.  In `tests/inc/mod.rs`, remove all old module declarations.
    3.  Create a new test file `tests/inc/derive_test.rs`.
    4.  In `derive_test.rs`, add comprehensive tests covering:
        *   **1-field structs:** Named and unnamed, `From<T>` and `from!` usage.
        *   **2-field structs (identical types):** Named and unnamed, `From2`, `From<(T,T)>`, and convenience `From1` usage.
        *   **2-field structs (different types):** Named and unnamed, `From2` and `From<(T1,T2)>` usage. Verify convenience `From1` is **not** generated.
        *   **3-field structs:** All combinations of identical/different types and their corresponding `FromN` and convenience impls.
        *   **Generics:** A test for a struct with generic parameters and a `where` clause.
    5.  Create two new compile-fail tests:
        *   `tests/inc/compile_fail/err_from_0_fields.rs`: `#[derive(VariadicFrom)] struct S; let _ : S = from!(1);`
        *   `tests/inc/compile_fail/err_from_4_fields.rs`: `#[derive(VariadicFrom)] struct S(i32,i32,i32,i32); let _ : S = from!(1,2);`
    6.  Update `tests/inc/mod.rs` to include `mod derive_test;`.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p variadic_from --all-targets`. Analyze output for success. The new tests should pass against the fixed macro from Increment 1.
*   **Commit Message:** `test(variadic_from): Overhaul test suite for spec compliance`

##### Increment 3: Refactor `variadic_from` Library and Update `Readme.md`
*   **Goal:** Clean up the `variadic_from` library structure and update its `Readme.md` to be accurate, runnable, and informative.
*   **Specification Reference:** `spec.md` Sections 4.1, 4.2
*   **Steps:**
    1.  Read `module/core/variadic_from/src/lib.rs` and `module/core/variadic_from/src/variadic.rs`.
    2.  Move the entire `mod variadic { ... }` block from `src/lib.rs` into the `src/variadic.rs` file.
    3.  In `src/lib.rs`, replace the inline module with `pub mod variadic;`.
    4.  In `src/lib.rs`, ensure `VariadicFrom` is correctly re-exported in the `exposed` and `prelude` modules.
    5.  Fix the codestyle of the `from!` macro definition in `src/variadic.rs` to use newlines for braces.
    6.  Read `module/core/variadic_from/Readme.md`.
    7.  Rewrite the "Quick Start" and "Expanded Code" examples to be accurate, spec-compliant, and runnable as doc tests (` ```rust `).
    8.  Remove the "Debugging" section that mentions the non-existent `#[debug]` attribute.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p variadic_from --doc`. Analyze output for success.
*   **Commit Message:** `refactor(variadic_from): Clean up lib, update and fix doc tests`

##### Increment 4: Finalization
*   **Goal:** Perform a final, holistic review and verification of the entire task's output, ensuring all requirements are met and the codebase is clean.
*   **Specification Reference:** `spec.md` Section 10
*   **Steps:**
    1.  Perform the full `Crate Conformance Check Procedure`.
    2.  Self-critique all changes against the `spec.md` and `codestyle.md`.
    3.  Ensure no commented-out code or temporary files remain.
    4.  Execute `git status` to confirm the working directory is clean.
*   **Increment Verification:**
    *   All steps of the `Crate Conformance Check Procedure` must pass with exit code 0 and no warnings.
*   **Commit Message:** `chore(variadic_from): Finalize and verify spec v1.1 implementation`

### Changelog
*   [New Plan | 2025-07-05 23:13 UTC] Created a new, comprehensive plan to address spec compliance, test suite overhaul, and documentation accuracy for `variadic_from` and `variadic_from_meta`.
