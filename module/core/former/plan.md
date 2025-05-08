<!-- module/core/former/plan.md -->
# Project Plan: Verify Former Derive for Unit Enum Variants

## Goal
*   Ensure the `#[derive(Former)]` macro correctly generates the expected constructors for **unit enum variants** according to the defined behavior rules.
*   Verify the implementation handles the `#[scalar]` (which is the default for unit variants) and `#[standalone_constructors]` attributes correctly for unit variants.
*   Activate and ensure the `unit_variant_*` tests pass.
*   Keep tests related to tuple or struct enum variants commented out.
*   Add the Unit Variant Test Matrix documentation to `former_enum_tests/mod.rs` while **preserving the existing matrix documentation** for other variant types.
*   Ensure all code modifications adhere strictly to `code/gen` instructions, Design Rules, and Codestyle Rules.

## Relevant Context

*   **Relevant Files (To be uncommented/verified/modified):**
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/mod.rs` (Uncomment `mod unit_variant_*;`, add unit matrix docs, preserve existing docs)
    *   `module/core/former_meta/src/derive_former/former_enum/unit_variant_handler.rs` (Implementation)
*   **Irrelevant Files (To remain commented out/ignored for this plan):**
    *   All other test files/modules within `module/core/former/tests/inc/former_enum_tests/` (e.g., `basic_*`, `enum_named_fields_*`, `generics_*`, `keyword_*`, `scalar_generic_tuple_*`, `standalone_constructor_*`, `usecase1.rs`, `subform_collection_test.rs`).
    *   All other handler files within `module/core/former_meta/src/derive_former/former_enum/` (e.g., `tuple_*`, `struct_*`).
*   **Main Test Module File (Parent):** `module/core/former/tests/inc/mod.rs`.
*   **Core Types & Traits:** `module/core/former_types/src/lib.rs`.
*   **Documentation:**
    *   `module/core/former/advanced.md` (Specifically attribute sections)
    *   `module/core/former/Readme.md`

### Expected Enum Former Behavior Rules (Full Set for Context)

These rules define the expected code generation behavior for `#[derive(Former)]` on enums, based on variant structure and attributes.

1.  **`#[scalar]` Attribute (on variant):**
    *   **Unit Variant (`V`):** Generates `Enum::variant() -> Enum`. (Rule 1a)
    *   **Zero-Field Tuple Variant (`V()`):** Generates `Enum::variant() -> Enum`. (Rule 1b)
    *   **Zero-Field Struct Variant (`V {}`):** Generates `Enum::variant() -> Enum`. (Rule 1c)
    *   **Single-Field Tuple Variant (`V(T1)`):** Generates `Enum::variant(T1) -> Enum`. (Rule 1d)
    *   **Single-Field Struct Variant (`V { f1: T1 }`):** Generates `Enum::variant { f1: T1 } -> Enum`. (Rule 1e)
    *   **Multi-Field Tuple Variant (`V(T1, T2, ...)`):** Generates `Enum::variant(T1, T2, ...) -> Enum`. (Rule 1f)
    *   **Multi-Field Struct Variant (`V { f1: T1, f2: T2, ... }`):** Generates `Enum::variant() -> Enum`. (Rule 1g)
    *   *Error Cases:* Cannot be combined with `#[subform_scalar]` on the same variant.

2.  **`#[subform_scalar]` Attribute (on variant):**
    *   **Unit Variant:** Error. (Rule 2a)
    *   **Zero-Field Variant (Tuple or Struct):** Error. (Rule 2b, 2c)
    *   **Single-Field Tuple Variant (`V(T1)` where `T1` derives `Former`):** Generates `Enum::variant() -> T1Former<...>` (former for the field's type). (Rule 2d)
    *   **Single-Field Tuple Variant (`V(T1)` where `T1` does NOT derive `Former`):** Error. (Rule 2d)
    *   **Single-Field Struct Variant (`V { f1: T1 }`):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Rule 2e)
    *   **Multi-Field Tuple Variant:** Error. (Rule 2f)
    *   **Multi-Field Struct Variant (`V { f1: T1, f2: T2, ... }`):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Rule 2g)

3.  **Default Behavior (No `#[scalar]` or `#[subform_scalar]` on variant):**
    *   **Unit Variant (`V`):** Generates `Enum::variant() -> Enum`. (Rule 3a)
    *   **Zero-Field Tuple Variant (`V()`):** Generates `Enum::variant() -> Enum`. (Rule 3b)
    *   **Zero-Field Struct Variant (`V {}`):** Error (requires `#[scalar]` or fields). (Rule 3c)
    *   **Single-Field Tuple Variant (`V(T1)` where `T1` derives `Former`):** Generates `Enum::variant() -> T1Former<...>`. (Rule 3d.i)
    *   **Single-Field Tuple Variant (`V(T1)` where `T1` does NOT derive `Former`):** Generates `Enum::variant(T1) -> Enum`. (Rule 3d.ii)
    *   **Single-Field Struct Variant (`V { f1: T1 }`):** Generates `Enum::variant() -> VariantFormer<...>`. (Rule 3e)
    *   **Multi-Field Tuple Variant (`V(T1, T2, ...)`):** Generates `Enum::variant(T1, T2, ...) -> Enum`. (Rule 3f)
    *   **Multi-Field Struct Variant (`V { f1: T1, f2: T2, ... }`):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Rule 3g)

4.  **`#[standalone_constructors]` Attribute (on enum):**
    *   Generates top-level constructor functions for each variant (e.g., `fn my_variant(...)`).
    *   **Return Type & Arguments (Option 2 Logic):**
        *   If **all** fields of a variant are marked `#[arg_for_constructor]`: `fn my_variant(arg1: T1, ...) -> Enum`.
        *   If **zero or some** fields are marked `#[arg_for_constructor]`:
            *   If the variant's default/scalar behavior yields `Enum::variant(all_args) -> Enum`: `fn my_variant(marked_args...) -> EnumVariantFormerForRemainingArgs`. (Requires implicit variant former).
            *   If the variant's default/scalar behavior yields `Enum::variant() -> Enum` (Unit/Zero-Tuple/Scalar-Zero-Struct): `fn my_variant() -> Enum`.
            *   If the variant's default/subform behavior yields `Enum::variant() -> SpecificFormer`: `fn my_variant(marked_args...) -> SpecificFormer` (with args pre-set).

### Test Matrix Coverage (Unit Variants)

This plan focuses on verifying the behavior for **Unit Variants**. The relevant factors and combinations tested by the `unit_variant_*` files are:

*   **Factors:**
    1.  Variant Type: Unit (Implicitly selected)
    2.  Variant-Level Attribute: None (Default), `#[scalar]`
    3.  Enum-Level Attribute: None, `#[standalone_constructors]`

*   **Combinations Covered by `unit_variant_only_test.rs`:**
    *   Unit + Default + None (Rule 3a) -> Tested via `Status::pending()` / `Status::complete()` in `unit_variant_constructors()` test.
    *   Unit + `#[scalar]` + None (Rule 1a) -> Tested via `Status::pending()` / `Status::complete()` in `unit_variant_constructors()` test (as default is scalar).
    *   Unit + Default + `#[standalone_constructors]` (Rule 3a, 4) -> Tested via `pending()` / `complete()` in `unit_variant_standalone_constructors()` test.
    *   Unit + `#[scalar]` + `#[standalone_constructors]` (Rule 1a, 4) -> Tested via `pending()` / `complete()` in `unit_variant_standalone_constructors()` test.

### Failure Diagnosis Algorithm
*   (Standard algorithm as previously defined, focusing on `unit_variant_handler.rs` if `_derive` fails and `_manual` passes).

## Increments

*   [✅] **Increment 1: Activate Unit Variant Tests and Document Matrix**
    *   **Goal:** Ensure only the `unit_variant_*` test modules are active within the `former_enum_tests` module and add the Unit Variant Test Matrix documentation to the module file, preserving existing matrix documentation.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/mod.rs`:
        *   Add the "Test Matrix Coverage (Unit Variants)" section from this plan as a module-level doc comment (`//!`) at the top of the file, **before** the existing matrix documentation for Tuple and Named variants.
        *   Uncomment `mod unit_variant_derive;`.
        *   Uncomment `mod unit_variant_manual;`.
        *   Ensure all other `mod` declarations within this file remain commented out.
    *   **Verification Strategy:** Request user to apply changes and run `cargo check --tests --package former`. Confirm no *new* compilation errors related to module declarations or documentation.
    *   **Commit Message:** `chore(former): Activate unit variant enum tests and document matrix`

*   [✅] **Increment 2: Verify Manual Unit Variant Implementation**
    *   **Goal:** Confirm the manual implementation in `unit_variant_manual.rs` compiles and passes tests, aligning with the expected behavior rules.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Review `module/core/former/tests/inc/former_enum_tests/unit_variant_manual.rs` and `unit_variant_only_test.rs`. (Done)
    *   **Detailed Plan Step 2:** **Align Test Setup with User Instructions (using apply_diff):**
        - **Modify `former_enum_tests/mod.rs` using `apply_diff`:**
            - Search for the current content of the file.
            - Replace it with the content that includes the test matrix documentation as doc comments at the top, and the `use` statements and `mod unit_variant_derive;` and `mod unit_variant_manual;` declarations outside the doc comment block.
        - **Modify `unit_variant_manual.rs` using `apply_diff`:**
            - Search for the `#[cfg(test)] mod tests { include!("unit_variant_only_test.rs"); }` block.
            - Replace it with the original `include!("unit_variant_only_test.rs");` line at the top level.
        - **Modify `unit_variant_only_test.rs` using `apply_diff`:**
            - Search for the `#[cfg(test)]` attribute and `use super::*;` lines at the top.
            - Replace them with just the original content (starting with the commented-out matrix or whatever was there originally before I added `#[cfg(test)]`).
    *   **Detailed Plan Step 3:** Request user to run `cargo check --tests --package former --features enabled`. Confirm no new compilation errors. (Done)
    *   **Detailed Plan Step 4:** Request user to run `cargo test --package former --test tests --features enabled -- --test-threads=1 --nocapture former_enum_tests::unit_variant_manual`. Analyze results against expected behavior. Address any failures by correcting the manual implementation or test logic. (Done)
    *   **Pre-Analysis:** The manual implementation should provide static methods `Status::pending()` and `Status::complete()` returning `Status`. If `standalone_constructors` were manually implemented (as they are in the provided context), top-level functions `pending()` and `complete()` returning `Status` should also exist. This aligns with Rules 1a/3a and 4. The `_only_test.rs` file should call these.
    *   **Crucial Design Rules:** Expected Behavior Rules 1a, 3a, 4.
    *   **Verification Strategy:** Request user to run `cargo test --package former --test tests --features enabled -- --test-threads=1 --nocapture former_enum_tests::unit_variant_manual`. Analyze results against expected behavior. Address any failures by correcting the manual implementation or test logic.
    *   **Commit Message:** `fix(former): Correct manual unit variant enum implementation` (if fixes needed) or `refactor(former): Verify manual unit variant enum implementation` (if no needed).

*   [✅] **Increment 3: Verify Derived Unit Variant Implementation**
    *   **Goal:** Confirm the `#[derive(Former)]` macro correctly generates code for unit variants, including handling `#[standalone_constructors]`, by ensuring `unit_variant_derive.rs` compiles and passes tests.
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1:** Review `module/core/former/tests/inc/former_enum_tests/unit_variant_derive.rs`. It derives `Former` and `standalone_constructors`.
    *   **Detailed Plan Step 2:** Review `module/core/former_meta/src/derive_former/former_enum/unit_variant_handler.rs`.
    *   **Pre-Analysis:** Expect the macro (via `unit_variant_handler.rs`) to generate code equivalent to the manual implementation based on Rules 1a/3a and 4. The handler should produce both the static methods and the standalone constructors.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rules 1a, 3a, 4.
    *   **Verification Strategy:**
        1.  Request user run `cargo check --tests --package former --features enabled`. Fix any compilation errors originating from the macro-generated code (likely requires changes in `former_meta/src/derive_former/former_enum/unit_variant_handler.rs`). (Done)
        2.  Request user run `cargo test --package former --test tests --features enabled -- --test-threads=1 --nocapture former_enum_tests::unit_variant_derive`. Analyze failures using the diagnosis algorithm, comparing generated behavior to the (verified) manual implementation. Fix macro logic if needed. (Done)
    *   **Commit Message:** `fix(former_meta): Correct unit variant enum code generation` (if fixes needed) or `refactor(former_meta): Verify unit variant enum code generation` (if no needed).

*   [✅] **Increment 4: Address TODOs/Issues in Unit Variant Files**
    *   **Goal:** Review and address any outstanding `// xxx :` or `// qqq :` comments specifically within the `unit_variant_derive.rs`, `unit_variant_manual.rs`, or `unit_variant_only_test.rs` files.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Search for `xxx :` and `qqq :` comments in the three `unit_variant_*` files. (Done)
    *   **Detailed Plan Step 2:** Propose solutions or code changes for each identified comment based on its content. (No comments found)
    *   **Crucial Design Rules:** [Comments: Add Tasks and Label Simplifications](#comments-add-tasks-and-label-simplifications), [Comments: Annotate Addressed Tasks](#comments-annotate-addressed-addressed-tasks).
    *   **Verification Strategy:** Request user to apply changes. Run `cargo check --tests --package former --features enabled` and `cargo test --package former --test tests --features enabled -- --test-threads=1 --nocapture former_enum_tests::unit_variant`. Ensure tests still pass and comments are addressed appropriately.
    *   **Commit Message:** `chore(former): Address TODOs in unit variant enum tests`

*   [✅] **Increment 5: Final Focused Verification**
    *   **Goal:** Ensure the activated unit tests pass and the `former` crate is healthy after the focused changes.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Run `cargo check --all-targets --package former --features enabled`. Address any errors or warnings. (Done)
    *   **Detailed Plan Step 2:** Run `cargo test --package former --test tests --features enabled -- --test-threads=1 --nocapture former_enum_tests::unit_variant`. Ensure unit tests pass. (Done)
    *   **Verification Strategy:** Zero errors/warnings from `check`. All tests in `former_enum_tests::unit_variant` pass.
    *   **Commit Message:** `test(former): Verify unit variant enum tests pass`

### Requirements
*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules.
*   **Focus:** Only uncomment and address code related to **unit enum variants**. Leave other enum tests (tuple, struct variants) commented out in `former_enum_tests/mod.rs`.
*   **Preserve Docs:** When adding the Unit Variant Test Matrix to `former_enum_tests/mod.rs`, ensure the existing matrix documentation for Tuple and Named variants is **not removed**.
*   **Incremental Verification:** Verify compilation and test success after each relevant increment.
*   **Failure Analysis:** Follow the "Failure Diagnosis Algorithm" if tests fail.
*   **Approval Gates:** Obtain user approval before starting each increment and after successful verification.

## Notes & Insights
*   This plan significantly narrows the scope to only unit enum variants.
*   It assumes the necessary infrastructure (`former_enum_tests/mod.rs`) exists but focuses activation only on `unit_variant_*`.
*   Verification steps target only the relevant unit tests until the final step.
*   The full "Expected Enum Former Behavior Rules" are kept for context.
*   Test Matrix coverage for unit variants is explicitly noted and will be added to `mod.rs` while preserving existing matrices.