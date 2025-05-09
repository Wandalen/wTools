# Project Plan: Verify Former Derive for Tuple Enum Variants (Incremental Activation)

## Goal
*   Ensure the `#[derive(Former)]` macro correctly generates the expected constructors and subformers for **tuple enum variants** (`V()`, `V(T1)`, `V(T1, T2, ...)`) according to the defined behavior rules.
*   Verify the implementation handles `#[scalar]`, `#[subform_scalar]`, `#[standalone_constructors]`, and `#[arg_for_constructor]` attributes correctly for tuple variants.
*   **Incrementally activate** and ensure relevant tuple variant tests pass, grouping related `_derive`, `_manual`, and `_only_test` files for each variant type. **Verify manual tests pass before activating corresponding derive tests.**
*   Keep tests related to unit or struct enum variants commented out.
*   Add the Tuple Variant Test Matrix documentation to `former_enum_tests/mod.rs` while **preserving the existing matrix documentation**.
*   Ensure all code modifications adhere strictly to `code/gen` instructions, Design Rules, and Codestyle Rules.
*   Avoid using `cargo clippy`.

## Relevant Context

**Important:** Before starting implementation, thoroughly review the `Readme.md` and `advanced.md` files for the `former` crate, and the `Readme.md` for `former_meta` to ensure a full understanding of the existing design, features, and intended behaviors.

*   **Primary Test Files (Tuple Variants - to be handled incrementally):**
    *   `module/core/former/tests/inc/former_enum_tests/mod.rs` (Uncomment relevant `mod` declarations incrementally, add tuple matrix docs, preserve existing docs)
    *   **Zero-Field Tuple (`V()`):**
        *   `enum_named_fields_derive.rs` (Relevant Variants: `VariantZeroUnnamedDefault`, `VariantZeroUnnamedScalar`)
        *   `enum_named_fields_manual.rs` (Manual impl for above)
        *   `enum_named_fields_only_test.rs` (Tests for above, focusing on tuple variants)
        *   `compile_fail/tuple_zero_subform_scalar_error.rs`
    *   **Single-Field Tuple (`V(T1)`):**
        *   `basic_derive.rs`, `basic_manual.rs`, `basic_only_test.rs`
        *   `generics_in_tuple_variant_derive.rs`, `generics_in_tuple_variant_manual.rs`, `generics_in_tuple_variant_only_test.rs`
        *   `generics_shared_tuple_derive.rs`, `generics_shared_tuple_manual.rs`, `generics_shared_tuple_only_test.rs`
        *   `generics_independent_tuple_derive.rs`, `generics_independent_tuple_manual.rs`, `generics_independent_tuple_only_test.rs`
        *   `scalar_generic_tuple_derive.rs`, `scalar_generic_tuple_manual.rs`, `scalar_generic_tuple_only_test.rs`
        *   `standalone_constructor_derive.rs` (Relevant Variant: `TupleVariant`)
        *   `standalone_constructor_manual.rs` (Manual impl for `TupleVariant`)
        *   `standalone_constructor_only_test.rs` (Test for `TupleVariant`)
        *   `standalone_constructor_args_derive.rs` (Relevant Variant: `TupleVariantArgs`)
        *   `standalone_constructor_args_manual.rs` (Manual impl for `TupleVariantArgs`)
        *   `standalone_constructor_args_only_test.rs` (Test for `TupleVariantArgs`)
        *   `keyword_variant_derive.rs` (Relevant Variants: `r#Break(StringFormerStub)`, `r#Let(u32)`)
        *   `keyword_variant_only_test.rs` (Tests for above)
        *   `usecase1.rs` (Contains multiple `V(T1)` variants)
        *   `compile_fail/tuple_single_subform_non_former_error.rs`
    *   **Multi-Field Tuple (`V(T1, T2, ...)`):**
        *   `keyword_variant_derive.rs` (Relevant Variants: `r#If(bool, i32)`, `r#For(usize, &'static str)`)
        *   `keyword_variant_only_test.rs` (Tests for above)
        *   `standalone_constructor_args_derive.rs` (Relevant Variant: `MultiTupleArgs`)
        *   `standalone_constructor_args_manual.rs` (Manual impl for `MultiTupleArgs`)
        *   `standalone_constructor_args_only_test.rs` (Test for `MultiTupleArgs`)
        *   `compile_fail/tuple_multi_subform_scalar_error.rs`

*   **Macro Implementation (Tuple Variant Handlers):**
    *   `module/core/former_meta/src/derive_former/former_enum/tuple_zero_fields_handler.rs`
    *   `module/core/former_meta/src/derive_former/former_enum/tuple_single_field_scalar.rs`
    *   `module/core/former_meta/src/derive_former/former_enum/tuple_single_field_subform.rs`
    *   `module/core/former_meta/src/derive_former/former_enum/tuple_multi_fields_scalar.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (Main dispatcher)

*   **Core Crate Files:**
    *   `module/core/former/src/lib.rs`
    *   `module/core/former_meta/src/lib.rs`
    *   `module/core/former_types/src/lib.rs`

*   **Documentation:**
    *   `module/core/former/Readme.md`
    *   `module/core/former/advanced.md`
    *   `module/core/former_meta/Readme.md`

*   **Irrelevant Files (To remain commented out/ignored for this plan):**
    *   `unit_variant_*` files.
    *   Struct variant tests within `enum_named_fields_*`, `standalone_constructor_*`, `keyword_variant_*`.
    *   `generics_independent_struct_*`, `generics_shared_struct_*` files.
    *   `subform_collection_test.rs`.
    *   Handler files: `unit_variant_handler.rs`, `struct_*_handler.rs`.
*   **Main Test Module File (Parent):** `module/core/former/tests/inc/mod.rs`.


### Expected Enum Former Behavior Rules (Full Set for Context)

(Same as previous plan - retained for reference)
1.  **`#[scalar]` Attribute (on variant):** ...
2.  **`#[subform_scalar]` Attribute (on variant):** ...
3.  **Default Behavior (No `#[scalar]` or `#[subform_scalar]` on variant):** ...
4.  **`#[standalone_constructors]` Attribute (on enum):** ...

### Test Matrix Coverage (Tuple Variants)

This plan focuses on verifying the behavior for **Tuple Variants**. The relevant factors and combinations tested by the relevant files are:

*   **Factors:**
    1.  Variant Type: Tuple (Implicitly selected)
    2.  Number of Fields: Zero (`V()`), One (`V(T1)`), Multiple (`V(T1, T2, ...)`)
    3.  Field Type `T1` (for Single-Field): Derives `Former`, Does NOT derive `Former`
    4.  Variant-Level Attribute: None (Default), `#[scalar]`, `#[subform_scalar]`
    5.  Enum-Level Attribute: None, `#[standalone_constructors]`
    6.  Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context): N/A, On single field, On all/some/no fields (multi)

*   **Combinations Covered (Mapped to Rules & Test Files):**
    *   **Zero-Field (`V()`):**
        *   T0.1 (Default): Rule 3b (`enum_named_fields_*`)
        *   T0.2 (`#[scalar]`): Rule 1b (`enum_named_fields_*`)
        *   T0.3 (Default + Standalone): Rule 3b, 4 (`enum_named_fields_*`)
        *   T0.4 (`#[scalar]` + Standalone): Rule 1b, 4 (`enum_named_fields_*`)
        *   T0.5 (`#[subform_scalar]`): Rule 2b (Error - `compile_fail/tuple_zero_subform_scalar_error.rs`)
    *   **Single-Field (`V(T1)`):**
        *   T1.1 (Default, T1 derives Former): Rule 3d.i (`basic_*`, `generics_in_tuple_variant_*`, `generics_shared_tuple_*`, `usecase1.rs`)
        *   T1.2 (Default, T1 not Former): Rule 3d.ii (Needs specific test file if not covered implicitly)
        *   T1.3 (`#[scalar]`): Rule 1d (`generics_independent_tuple_*`, `scalar_generic_tuple_*`, `keyword_variant_*`)
        *   T1.4 (`#[subform_scalar]`, T1 derives Former): Rule 2d (Needs specific test file if not covered implicitly)
        *   T1.5 (`#[subform_scalar]`, T1 not Former): Rule 2d (Error - `compile_fail/tuple_single_subform_non_former_error.rs`)
        *   T1.6 (Default, T1 derives Former + Standalone): Rule 3d.i, 4 (`standalone_constructor_*`)
        *   T1.7 (Default, T1 not Former + Standalone): Rule 3d.ii, 4 (Needs specific test file if not covered implicitly)
        *   T1.8 (`#[scalar]` + Standalone): Rule 1d, 4 (`standalone_constructor_args_*`)
        *   T1.9 (`#[subform_scalar]`, T1 derives Former + Standalone): Rule 2d, 4 (Needs specific test file if not covered implicitly)
        *   T1.10 (`#[subform_scalar]`, T1 not Former + Standalone): Rule 2d (Error - Covered by T1.5)
    *   **Multi-Field (`V(T1, T2, ...)`):**
        *   TN.1 (Default): Rule 3f (Needs specific test file if not covered implicitly by TN.4)
        *   TN.2 (`#[scalar]`): Rule 1f (`keyword_variant_*`, `standalone_constructor_args_*`)
        *   TN.3 (`#[subform_scalar]`): Rule 2f (Error - `compile_fail/tuple_multi_subform_scalar_error.rs`)
        *   TN.4 (Default + Standalone): Rule 3f, 4 (Needs specific test file, potentially `standalone_constructor_args_*` if adapted)
        *   TN.5 (`#[scalar]` + Standalone): Rule 1f, 4 (`standalone_constructor_args_*`)

### Failure Diagnosis Algorithm
*   (Standard algorithm as previously defined, focusing on relevant `tuple_*_handler.rs` if `_derive` fails and `_manual` passes).
*   **Widespread Failure Strategy:** If uncommenting a test group causes numerous failures, propose selectively commenting out (using `//`) only the failing `#[test]` functions or problematic `include!` lines. Avoid commenting out entire files or modules unless absolutely necessary. Re-enable tests incrementally (one or small groups at a time) to isolate the root cause, following Rule 9.d.i of the Proc Macro Development Workflow.

## Increments

*   [⚫] **Increment 1: Document Tuple Variant Matrix**
    *   **Goal:** Add the Tuple Variant Test Matrix documentation to `former_enum_tests/mod.rs`, preserving existing matrices. Keep all tuple test modules commented out for now.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/mod.rs`:
        *   Add the "Test Matrix Coverage (Tuple Variants)" section from this plan as a module-level doc comment (`//!`), **after** any existing Unit Variant matrix and **before** any existing Named Variant matrix.
        *   Ensure all `mod` declarations related to tuple variants remain commented out.
    *   **Verification Strategy:** Request user to apply changes and run `cargo check --tests --package former`. Confirm no *new* compilation errors related to documentation.
    *   **Commit Message:** `docs(former): Add test matrix for tuple enum variants`

*   [⚫] **Increment 2: Verify Zero-Field Tuple Variants (`V()`)**
    *   **Goal:** Activate and verify `#[derive(Former)]` for zero-field tuple variants (Rules 1b, 3b, 4) using tests in `enum_named_fields_*`. Verify compile error for Rule 2b.
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/mod.rs` to uncomment `mod enum_named_fields_manual;`.
    *   **Detailed Plan Step 2:** Verify manual implementation for `VariantZeroUnnamedDefault` and `VariantZeroUnnamedScalar` in `enum_named_fields_manual.rs` passes tests (`cargo test ... enum_named_fields_manual`). Fix if needed.
    *   **Detailed Plan Step 3:** Modify `module/core/former/tests/inc/former_enum_tests/mod.rs` to uncomment `mod enum_named_fields_derive;`.
    *   **Detailed Plan Step 4:** Verify derived implementation for `VariantZeroUnnamedDefault` and `VariantZeroUnnamedScalar` in `enum_named_fields_derive.rs` passes tests (`cargo test ... enum_named_fields_derive`). Debug `tuple_zero_fields_handler.rs` if needed. *Handle widespread failures selectively if they occur.*
    *   **Detailed Plan Step 5:** Modify `module/core/former/tests/inc/former_enum_tests/mod.rs` to uncomment `mod compile_fail;` (if not already active).
    *   **Detailed Plan Step 6:** Verify `compile_fail/tuple_zero_subform_scalar_error.rs` fails compilation as expected (`cargo test --package former former_enum_tests::compile_fail::tuple_zero_subform_scalar_error`).
    *   **Crucial Design Rules:** Expected Behavior Rules 1b, 2b, 3b, 4. [Proc Macro: Development Workflow](#proc-macro-development-workflow).
    *   **Verification Strategy:** Tests pass for manual/derive, compile-fail test fails compilation.
    *   **Commit Message:** `feat(former): Verify zero-field tuple enum variant support`

*   [⚫] **Increment 3: Verify Single-Field Tuple Variants (`V(T1)`) - Scalar**
    *   **Goal:** Activate and verify `#[derive(Former)]` for single-field tuple variants with `#[scalar]` (Rules 1d, 4) using relevant test groups.
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1 (Manual):** In `module/core/former/tests/inc/former_enum_tests/mod.rs`, uncomment:
        *   `mod generics_independent_tuple_manual;`
        *   `mod scalar_generic_tuple_manual;`
        *   `mod standalone_constructor_args_manual;` (ensure only tuple variant parts are tested if it contains struct variants)
    *   **Detailed Plan Step 2 (Manual Verification):** Verify manual implementations pass tests (`cargo test ... <manual_module>`). Fix if needed.
    *   **Detailed Plan Step 3 (Derive):** In `module/core/former/tests/inc/former_enum_tests/mod.rs`, uncomment:
        *   `mod generics_independent_tuple_derive;`
        *   `mod scalar_generic_tuple_derive;`
        *   `mod keyword_variant_derive;` (ensure only relevant tuple variants are tested)
        *   `mod standalone_constructor_args_derive;` (ensure only tuple variant parts are tested)
    *   **Detailed Plan Step 4 (Derive Verification):** Verify derived implementations pass tests (`cargo test ... <derive_module>`). Debug `tuple_single_field_scalar.rs` if needed. *Handle widespread failures selectively.*
    *   **Crucial Design Rules:** Expected Behavior Rules 1d, 4. [Proc Macro: Development Workflow](#proc-macro-development-workflow).
    *   **Verification Strategy:** All relevant manual and derive tests pass.
    *   **Commit Message:** `feat(former): Verify #[scalar] single-field tuple enum variant support`

*   [⚫] **Increment 4: Verify Single-Field Tuple Variants (`V(T1)`) - Subform/Default**
    *   **Goal:** Activate and verify `#[derive(Former)]` for single-field tuple variants with default/`#[subform_scalar]` behavior (Rules 2d, 3d.i, 3d.ii, 4). Verify compile error for Rule 2d (T1 not Former).
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1 (Manual):** In `module/core/former/tests/inc/former_enum_tests/mod.rs`, uncomment:
        *   `mod basic_manual;`
        *   `mod generics_in_tuple_variant_manual;`
        *   `mod generics_shared_tuple_manual;`
        *   `mod standalone_constructor_manual;` (ensure only tuple variant parts are tested)
    *   **Detailed Plan Step 2 (Manual Verification):** Verify manual implementations pass tests. Fix if needed.
    *   **Detailed Plan Step 3 (Derive):** In `module/core/former/tests/inc/former_enum_tests/mod.rs`, uncomment:
        *   `mod basic_derive;`
        *   `mod generics_in_tuple_variant_derive;`
        *   `mod generics_shared_tuple_derive;`
        *   `mod usecase1;`
        *   `mod standalone_constructor_derive;` (ensure only tuple variant parts are tested)
    *   **Detailed Plan Step 4 (Derive Verification):** Verify derived implementations pass tests. Debug `tuple_single_field_subform.rs` and `tuple_single_field_scalar.rs` (for Rule 3d.ii) if needed. *Handle widespread failures selectively.*
    *   **Detailed Plan Step 5 (Compile Fail):** Verify `compile_fail/tuple_single_subform_non_former_error.rs` fails compilation.
    *   **Crucial Design Rules:** Expected Behavior Rules 2d, 3d.i, 3d.ii, 4. [Proc Macro: Development Workflow](#proc-macro-development-workflow).
    *   **Verification Strategy:** All relevant manual/derive tests pass, compile-fail test fails compilation.
    *   **Commit Message:** `feat(former): Verify default/subform single-field tuple enum variant support`

*   [⚫] **Increment 5: Verify Multi-Field Tuple Variants (`V(T1, T2, ...)` )**
    *   **Goal:** Activate and verify `#[derive(Former)]` for multi-field tuple variants (Rules 1f, 3f, 4). Verify compile error for Rule 2f.
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1 (Manual):** In `module/core/former/tests/inc/former_enum_tests/mod.rs`, ensure `standalone_constructor_args_manual` is active (focus on `MultiTupleArgs`).
    *   **Detailed Plan Step 2 (Manual Verification):** Verify manual implementations pass tests. Fix if needed.
    *   **Detailed Plan Step 3 (Derive):** In `module/core/former/tests/inc/former_enum_tests/mod.rs`, ensure `keyword_variant_derive` and `standalone_constructor_args_derive` are active (focus on relevant multi-field tuple variants).
    *   **Detailed Plan Step 4 (Derive Verification):** Verify derived implementations pass tests. Debug `tuple_multi_fields_scalar.rs` if needed. *Handle widespread failures selectively.*
    *   **Detailed Plan Step 5 (Compile Fail):** Verify `compile_fail/tuple_multi_subform_scalar_error.rs` fails compilation.
    *   **Crucial Design Rules:** Expected Behavior Rules 1f, 2f, 3f, 4. [Proc Macro: Development Workflow](#proc-macro-development-workflow).
    *   **Verification Strategy:** All relevant manual/derive tests pass, compile-fail test fails compilation.
    *   **Commit Message:** `feat(former): Verify multi-field tuple enum variant support`

*   [⚫] **Increment 6: Address TODOs/Issues in Tuple Variant Files**
    *   **Goal:** Review and address any outstanding `// xxx :` or `// qqq :` comments within the **activated** tuple variant test files.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Search for `xxx :` and `qqq :` comments in all activated `_derive.rs`, `_manual.rs`, `_only_test.rs` files related to tuple variants.
    *   **Detailed Plan Step 2:** Propose solutions or code changes for each identified comment based on its content.
    *   **Crucial Design Rules:** [Comments: Add Tasks and Label Simplifications](#comments-add-tasks-and-label-simplifications), [Comments: Annotate Addressed Tasks](#comments-annotate-addressed-tasks).
    *   **Verification Strategy:** Request user to apply changes. Run `cargo check --tests --package former` and `cargo test --package former --test tests -- --test-threads=1 --nocapture former_enum_tests`. Ensure tests still pass and comments are addressed appropriately.
    *   **Commit Message:** `chore(former): Address TODOs in tuple variant enum tests`

*   [⚫] **Increment 7: Final Focused Verification**
    *   **Goal:** Ensure all activated tuple tests pass and the `former` crate is healthy after the focused changes.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Run `cargo check --all-targets --package former`. Address any errors or warnings.
    *   **Detailed Plan Step 2:** Run `cargo test --package former --test tests -- --test-threads=1 --nocapture former_enum_tests`. Ensure all activated tests pass and compile-fail tests fail as expected.
    *   **Verification Strategy:** Zero errors/warnings from `check`. All activated tests pass.
    *   **Commit Message:** `test(former): Verify tuple variant enum tests pass`

### Requirements
*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules.
*   **Focus:** Only uncomment and address code related to **tuple enum variants**. Leave unit and struct variant tests commented out.
*   **Preserve Docs:** When adding the Tuple Variant Test Matrix to `former_enum_tests/mod.rs`, ensure the existing matrix documentation is **not removed**.
*   **Incremental Activation:** Uncomment test modules (`mod ...;`) only in the increment where they are first needed for verification.
*   **Incremental Verification:** Verify compilation and test success after each relevant increment. Verify `_manual` tests before `_derive` tests. Handle widespread failures by selectively commenting out only failing tests.
*   **Failure Analysis:** Follow the "Failure Diagnosis Algorithm".
*   **Approval Gates:** Obtain user approval before starting each increment and after successful verification.

## Notes & Insights
*   This plan focuses on tuple enum variants, activating tests incrementally.
*   It assumes the necessary infrastructure (`former_enum_tests/mod.rs`) exists.
*   Verification steps target only the relevant tuple tests until the final step.
*   The full "Expected Enum Former Behavior Rules" are kept for context.
*   Test Matrix coverage for tuple variants is explicitly noted and will be added to `mod.rs`.
*   `cargo clippy` check is excluded.
*   Verification strategy updated to test `_manual` before `_derive`.
*   Widespread failure handling strategy refined to be selective.
*   Relevant context expanded to include core crate files and documentation, with an emphasis on pre-reading.
