# Task Plan: Refactor `variadic_from` and `variadic_from_meta` for Spec v1.1 Compliance

### Goal
Systematically refactor the `variadic_from` and `variadic_from_meta` crates to be fully compliant with `spec.md` v1.1. This includes correcting the derive macro's code generation using best practices from `macro_tools`, overhauling the test suite for comprehensive and accurate validation, and updating all documentation to reflect the correct behavior.

### Ubiquitous Language (Vocabulary)
*   **Variadic Constructor:** A constructor that can accept a variable number of arguments, implemented via the `from!` macro.
*   **`FromN` Traits:** A set of traits (`From1`, `From2`, `From3`) defining a contract for constructing a type from `N` arguments.
*   **`VariadicFrom` Trait:** A marker trait (`#[derive(VariadicFrom)]`) that triggers the automatic implementation of `FromN` and standard `From` traits.
*   **Convenience Implementation:** An `impl FromM for StructWithNFields` where `M < N`, generated only when specific field types are identical, for ergonomic single-argument construction.
*   **`macro_tools`:** The foundational library for building procedural macros, providing helpers for parsing, analysis, and code generation.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/variadic_from`
*   **Overall Progress:** 1/7 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Audit, Cleanup, and Initial Setup
    *   ⚫ Increment 2: Refactor Macro Input Parsing using `macro_tools`
    *   ⚫ Increment 3: Implement Core `FromN` and `From<Tuple>` Generation
    *   ⚫ Increment 4: Implement Conditional Convenience `FromN` Generation
    *   ⚫ Increment 5: Implement and Validate the New Test Suite
    *   ⚫ Increment 6: Refactor `variadic_from` Library and Update Documentation
    *   ⚫ Increment 7: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/variadic_from_meta`

### Relevant Context
*   **Specification:** `module/core/variadic_from/spec.md`
*   **Codestyle:** `code/rules/codestyle.md`
*   **Key `macro_tools` components to use:**
    *   `struct_like::StructLike`: To parse the `DeriveInput` into a unified struct representation.
    *   `struct_like::FieldOrVariant`: To iterate over fields.
    *   `generic_params::GenericsRef`: To handle generics (`impl_generics`, `ty_generics`, `where_clause`).
    *   `quote` (aliased as `qt`): For generating token streams.
    *   `syn_err!`, `return_syn_err!`: For error handling.
    *   `typ::*`: For type analysis, like checking for `Option` or getting type parameters.
*   **Files to Modify:**
    *   `module/core/variadic_from/src/lib.rs`
    *   `module/core/variadic_from/src/variadic.rs`
    *   `module/core/variadic_from/Readme.md`
    *   `module/core/variadic_from/tests/inc/mod.rs`
    *   `module/core/variadic_from/tests/inc/derive_test.rs` (new)
    *   `module/core/variadic_from/tests/inc/compile_fail/*.rs` (new)
    *   `module/core/variadic_from_meta/src/lib.rs`

### Crate Conformance Check Procedure
*   **Step 1: Build Crates.** Execute `timeout 90 cargo build -p variadic_from -p variadic_from_meta`. Verify no errors.
*   **Step 2: Run Linter.** Execute `timeout 90 cargo clippy --workspace -- -D warnings`. Verify no errors or warnings.
*   **Step 3: Run All Tests.** Execute `timeout 90 cargo test --workspace`. Verify no failures.
*   **Step 4: Run Doc Tests.** Execute `timeout 90 cargo test --workspace --doc`. Verify no failures.

### Increments

##### Increment 1: Audit, Cleanup, and Initial Setup
*   **Goal:** Establish a clean baseline by removing outdated test files and ensuring the project compiles, even if tests fail.
*   **Specification Reference:** `spec.md` Section 6.6 (Test Organization)
*   **Steps:**
    1.  Use `list_files` recursively on `module/core/variadic_from/tests/inc/` to confirm the existence of files to be deleted.
    2.  Delete the `only_test` directory using `execute_command` with `git rm -r module/core/variadic_from/tests/inc/only_test`.
    3.  Delete `derive_test.rs` using `execute_command` with `git rm module/core/variadic_from/tests/inc/derive_test.rs`. If it doesn't exist, ignore the error.
    4.  Delete `variadic_from_derive_test.rs` using `execute_command` with `git rm module/core/variadic_from/tests/inc/variadic_from_derive_test.rs`.
    5.  Delete `variadic_from_manual_test.rs` using `execute_command` with `git rm module/core/variadic_from/tests/inc/variadic_from_manual_test.rs`.
    6.  Delete `variadic_from_only_test.rs` using `execute_command` with `git rm module/core/variadic_from/tests/inc/variadic_from_only_test.rs`.
    7.  Use `write_to_file` to clear the contents of `module/core/variadic_from/tests/inc/mod.rs`, leaving only `use super::*;\nuse test_tools::exposed::*;`.
    8.  Read the content of `module/core/variadic_from/src/lib.rs`.
    9.  Extract the `mod variadic { ... }` block from the content of `lib.rs`.
    10. Create a new file `module/core/variadic_from/src/variadic.rs` and write the extracted `mod variadic` block into it, adding `use super::*;` at the top.
    11. In the `from!` macro definition within `src/variadic.rs`, replace `$crate::variadic::` with `::variadic_from::variadic::`.
    12. Update `module/core/variadic_from/src/lib.rs` to replace the inline module with `pub mod variadic;`.
    13. Perform Increment Verification.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build -p variadic_from -p variadic_from_meta`. The build must succeed. Analyze output for any errors.
*   **Commit Message:** `chore(variadic_from): Clean up test directory and refactor lib structure`

##### Increment 2: Refactor Macro Input Parsing using `macro_tools`
*   **Goal:** Rewrite the `variadic_from_meta` derive macro to use the robust parsing utilities from `macro_tools`, improving maintainability and correctness.
*   **Specification Reference:** `spec.md` Section 3.1, 6.5
*   **Steps:**
    1.  Read `module/core/variadic_from_meta/src/lib.rs`.
    2.  Replace the existing `VariadicFromContext` struct and its `new` function with a more streamlined version.
    3.  In the main `variadic_from_derive` function, parse the input `TokenStream` into a `macro_tools::struct_like::StructLike` instance.
    4.  Handle the case where the input is not a struct (e.g., an enum) by returning an appropriate compile error using `macro_tools::return_syn_err!`.
    5.  Extract the struct's identifier, generics, and fields using the methods on `StructLike` (e.g., `.ident()`, `.generics()`, `.fields()`).
    6.  Collect field types into a `Vec<&syn::Type>` using `struct_like.field_types().collect()`.
    7.  Collect field names (or indices for tuple structs) into a `Vec<proc_macro2::TokenStream>`. Use `field.ident.as_ref().unwrap().to_token_stream()` for named fields and `syn::Index::from(i).to_token_stream()` for unnamed fields.
    8.  Store this information in the new `VariadicFromContext` struct.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build -p variadic_from_meta`. The macro crate should still compile successfully.
*   **Commit Message:** `refactor(variadic_from_meta): Use macro_tools::struct_like for input parsing`

##### Increment 3: Implement Core `FromN` and `From<Tuple>` Generation
*   **Goal:** Generate the primary `FromN` and `From<(...)>` implementations as defined in the specification.
*   **Specification Reference:** `spec.md` Section 3.1, 6.4
*   **Steps:**
    1.  Create a helper function `fn generate_from_n_impl(...)` that takes the `VariadicFromContext`.
    2.  Inside this function, use a `match` on the number of fields (1, 2, or 3).
    3.  For each case, generate the `impl<...> FromN<...> for StructName<...>` block.
        *   Use `generics.split_for_impl()` to correctly handle `impl_generics`, `ty_generics`, and `where_clause`.
        *   Use `quote!` to construct the implementation. The body should construct `Self` using the field names/indices from the context.
        *   Use absolute paths for traits: `::variadic_from::exposed::From1`, etc.
    4.  Create a helper function `fn generate_from_tuple_impl(...)`.
    5.  Inside this function, use a `match` on the number of fields (1, 2, or 3).
    6.  For each case, generate the `impl<...> From<TupleType> for StructName<...>` block.
        *   The body of the `from` function **must** delegate to the corresponding `FromN` trait method (e.g., `Self::from2(a, b)`).
    7.  In the main derive function, call these helpers and combine their `TokenStream` outputs.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build -p variadic_from_meta`. The macro crate should compile.
*   **Commit Message:** `feat(variadic_from_meta): Implement FromN and From<Tuple> generation`

##### Increment 4: Implement Conditional Convenience `FromN` Generation
*   **Goal:** Implement the logic for generating convenience `FromN` implementations based on field type equality.
*   **Specification Reference:** `spec.md` Section 3.1
*   **Steps:**
    1.  Create a helper function `fn generate_convenience_impls(...)`.
    2.  Inside this function, check the number of fields.
    3.  **For 2-field structs:**
        *   Compare the types of the two fields. `syn::Type` does not implement `PartialEq`, so compare their token streams: `field1_type.to_token_stream().to_string() == field2_type.to_token_stream().to_string()`.
        *   If they are identical, generate `impl<...> From1<T1> for StructName<...>` where the single argument is used for both fields.
    4.  **For 3-field structs:**
        *   Check if all three field types are identical. If so, generate `impl<...> From1<T1> for StructName<...>` where the argument is used for all three fields.
        *   Check if the second and third field types are identical. If so, generate `impl<...> From2<T1, T2> for StructName<...>` where `arg1` goes to the first field and `arg2` goes to the second and third fields.
    5.  Integrate this function's output into the main derive function.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo build -p variadic_from_meta`. The macro crate should compile.
*   **Commit Message:** `feat(variadic_from_meta): Implement conditional convenience FromN impls`

##### Increment 5: Implement and Validate the New Test Suite
*   **Goal:** Create and pass a new, comprehensive test suite that validates all behaviors defined in `spec.md`.
*   **Specification Reference:** `spec.md` Section 10
*   **Steps:**
    1.  Follow the `Test Implementation and Validation Sequence` defined below to incrementally build the test suite and fix the macro implementation.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p variadic_from --all-targets`. The command must exit with code 0.
*   **Commit Message:** `test(variadic_from): Implement and pass new spec-compliant test suite`

##### Increment 6: Refactor `variadic_from` Library and Update Documentation
*   **Goal:** Clean up the library code and update the `Readme.md` to be accurate, runnable, and informative.
*   **Specification Reference:** `spec.md` Sections 2.2, 4.1, 4.2
*   **Steps:**
    1.  Read `module/core/variadic_from/src/variadic.rs`.
    2.  Add the blanket implementations from `spec.md` section 2.2 into the `variadic` module.
    3.  Read `module/core/variadic_from/Readme.md`.
    4.  Rewrite the "Quick Start" and "Expanded Code" examples to be accurate, spec-compliant, and runnable as doc tests (` ```rust `). Ensure they reflect the new convenience impl logic.
    5.  Update the "Macro Behavior Details" section to accurately describe the logic for 1, 2, and 3-field structs, including the conditions for convenience impls.
    6.  Remove the "Debugging" section that mentions the non-existent `#[debug]` attribute.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p variadic_from --doc`. The command must exit with code 0.
*   **Commit Message:** `docs(variadic_from): Update Readme.md and add blanket impls`

##### Increment 7: Finalization
*   **Goal:** Perform a final, holistic review and verification of the entire task's output.
*   **Specification Reference:** `spec.md` Section 10
*   **Steps:**
    1.  Perform the full `Crate Conformance Check Procedure`.
    2.  Self-critique all changes against the `spec.md` and `codestyle.md`.
    3.  Ensure no commented-out code or temporary files remain.
    4.  Execute `git status` to confirm the working directory is clean.
*   **Increment Verification:**
    *   All steps of the `Crate Conformance Check Procedure` must pass with exit code 0 and no warnings.
*   **Commit Message:** `chore(variadic_from): Finalize and verify spec v1.1 implementation`

### Test Implementation and Validation Sequence
This sequence should be followed during **Increment 5**. Each step involves adding a small piece of test code, running the tests, and fixing any issues in `variadic_from_meta` before proceeding.

**Phase 1: Foundation & Simplest Case (1-Field Structs)**
1.  **Create Test File:** Create `module/core/variadic_from/tests/inc/derive_test.rs`.
2.  **Populate Initial Content:** Add the Test Matrix from the plan and necessary `use` statements to `derive_test.rs`.
3.  **Update `mod.rs`:** Add `mod derive_test;` to `module/core/variadic_from/tests/inc/mod.rs`.
4.  **Add 1-Field Tests:** Add the `test_named_struct_1_field` and `test_tuple_struct_1_field` functions to `derive_test.rs`.
5.  **Verify & Fix:** Run `cargo test -p variadic_from --test derive_test`. Analyze failures and fix the `variadic_from_meta` implementation until these tests pass.

**Phase 2: Two-Field Structs**
1.  **Add Identical-Type Tests:** Add `test_named_struct_2_identical_fields` and `test_tuple_struct_2_identical_fields` to `derive_test.rs`.
2.  **Verify & Fix:** Run tests. Fix issues related to `From2` and the convenience `From1` implementation.
3.  **Add Different-Type Tests:** Add `test_named_struct_2_different_fields` and `test_tuple_struct_2_different_fields` to `derive_test.rs`.
4.  **Verify & Fix:** Run tests. The primary goal is to ensure these compile and pass, which implicitly tests that the convenience `From1` was *not* generated.

**Phase 3: Three-Field Structs**
1.  **Add Identical-Type Tests:** Add `test_named_struct_3_identical_fields` and `test_tuple_struct_3_identical_fields`.
2.  **Verify & Fix:** Run tests. Fix issues related to `From3` and convenience `From1`/`From2` impls.
3.  **Add Mixed-Type Tests:** Add `test_named_struct_3_fields_last_different` and `test_tuple_struct_3_fields_last_two_identical`.
4.  **Verify & Fix:** Run tests. Ensure the correct convenience impls are generated (or not generated) based on type equality.

**Phase 4: Generic Structs**
1.  **Add Generic Tests:** Add `test_named_struct_1_generic_field` and `test_tuple_struct_2_generic_fields` to `derive_test.rs`.
2.  **Verify & Fix:** Run tests. This specifically validates the `generics.split_for_impl()` logic in the macro.

**Phase 5: Compile-Fail Tests**
1.  **Add 0-Field Test:** Create `tests/inc/compile_fail/err_from_0_fields.rs`. Add it to `tests/inc/mod.rs`. Run `cargo test -p variadic_from`. Verify it fails as expected by `trybuild`.
2.  **Add 4-Field Test:** Create `tests/inc/compile_fail/err_from_4_fields.rs`. Add it to `mod.rs`. Run tests. Verify it fails as expected.
3.  **Add `from!` Macro Test:** Create `tests/inc/compile_fail/err_from_too_many_args.rs` with `from!(1,2,3,4)`. Add it to `mod.rs`. Run tests. Verify it fails with the `compile_error!` message.

### Task Requirements
*   All code must be compliant with `spec.md` version 1.1.
*   The `variadic_from_meta` crate must be refactored to use `macro_tools` utilities where appropriate.
*   The test suite must be overhauled to be comprehensive and directly test the specification.
*   All tests, including doc tests and compile-fail tests, must pass.
*   The `Readme.md` must be updated to be accurate and contain runnable examples.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   Must use Rust 2021 edition.

### Assumptions
*   The `macro_tools` crate is sufficiently stable and provides the necessary utilities for this refactoring.
*   The `spec.md` is the single source of truth for the expected behavior.

### Out of Scope
*   Adding support for more than 3 variadic arguments.
*   Introducing new attributes or configuration options to the derive macro.
*   Refactoring other crates in the workspace unless absolutely necessary to unblock `variadic_from`.

### Notes & Insights
*   The core of this task is a careful, step-by-step reimplementation and validation of the derive macro.
*   Comparing `syn::Type` requires converting them to strings, which is a known workaround for the lack of a direct `PartialEq` implementation. This should be encapsulated in the context/helper functions.
*   The `from!` macro's pathing (`$crate::` vs `::variadic_from::`) is a subtle but important detail for ensuring it works correctly when used from other crates.

### Changelog
*   [New Plan | 2025-07-06 15:41 UTC] Created a new, more detailed plan to address spec compliance, test failures, and explicit `macro_tools` usage.
*   [Plan Update | 2025-07-06 16:00 UTC] Added a detailed, phased testing sequence to Increment 5 to ensure a methodical and robust validation process.
*   [Increment 1 | 2025-07-06 15:53 UTC] Cleaned up test directory and refactored library structure.
