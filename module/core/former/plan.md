# Project Plan: Comprehensive Testing of `former` Crate for Enum Unnamed (Tuple) Variants

## Goal
*   Systematically test the `#[derive(Former)]` macro for Rust enum **unnamed (tuple) variants**.
*   Cover combinations of relevant `former` attributes (`#[scalar]`, `#[subform_scalar]`, default behavior, `#[standalone_constructors]`, `#[arg_for_constructor]`) for tuple variants with 0, 1, and multiple fields.
*   Address scenarios where the field type within a single-field tuple variant does or does not derive `Former`.
*   Incrementally uncomment, pre-analyze, fix, and verify existing test files related to tuple variants within `module/core/former/tests/inc/former_enum_tests/`.
*   **Embed the "Test Matrix for Unnamed (Tuple) Variants" (or a clear reference to it) as documentation within `module/core/former/tests/inc/mod.rs`.**
*   Ensure all code modifications adhere strictly to `code/gen` instructions, Design Rules, and Codestyle Rules.

## Relevant Context

*   **Primary Test Directory:** `module/core/former/tests/inc/former_enum_tests/`
    *   Files like `enum_named_fields_*.rs` (for zero-field tuple variants: `VariantZeroUnnamedDefault()`, `VariantZeroUnnamedScalar()`).
    *   Files like `basic_*.rs` (for single-field tuple with Former-derived inner type: `Break(Break)`).
    *   Files like `generics_independent_tuple_*.rs` (for single-field tuple with `#[scalar]` and generic inner type).
    *   Files like `generics_in_tuple_variant_*.rs` (for single-field tuple with generic inner type, default subformer).
    *   Files like `generics_shared_tuple_*.rs` (for single-field tuple with shared generic inner type, default subformer).
    *   Files like `scalar_generic_tuple_*.rs` (for single and multi-field tuple variants with `#[scalar]` and generic inner types).
    *   Files like `standalone_constructor_*.rs` and `standalone_constructor_args_*.rs` (for tuple variants with these enum-level attributes).
    *   `usecase1.rs` (multiple single-field tuple variants with Former-derived inner types).
*   **Main Test Module File:** `module/core/former/tests/inc/mod.rs`.
*   **Macro Implementation:** `module/core/former_meta/src/derive_former/former_enum/`
    *   `tuple_zero_fields_handler.rs`
    *   `tuple_single_field_scalar.rs`
    *   `tuple_single_field_subform.rs`
    *   `tuple_multi_fields_scalar.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (main dispatch)
*   **Core Types & Traits:** `module/core/former_types/src/lib.rs`
*   **Documentation:**
    *   `module/core/former/advanced.md`
    *   `module/core/former/Readme.md`

### Test Matrix for Unnamed (Tuple) Variants

**(This section will be embedded as documentation in `module/core/former/tests/inc/mod.rs` as per Increment 1. For brevity in this plan file, it's referenced here. The full matrix is defined in the previous interaction and will be used for the actual documentation.)**

*   **Factors:**
    1.  Number of Fields: Zero, One, Multiple.
    2.  Field Type `T1` (Single-Field): Derives `Former`, Does NOT derive `Former`.
    3.  Variant-Level Attribute: None, `#[scalar]`, `#[subform_scalar]`.
    4.  Enum-Level Attribute: None, `#[standalone_constructors]`.
    5.  Field-Level Attribute `#[arg_for_constructor]`.
*   **Combinations Tables:** (As detailed previously for Zero-Field, Single-Field, Multi-Field)

### Target File Structure for Unnamed (Tuple) Variant Tests

Within `module/core/former/tests/inc/former_enum_tests/`:
New files might be needed if existing ones don't cover specific matrix combinations cleanly.
Documentation for this matrix will go into `module/core/former/tests/inc/mod.rs` within the `former_enum_tests` module scope.

```
module/core/former/tests/inc/
├── mod.rs                      // Declares `former_enum_tests` and its test files.
│                               // Will contain the Test Matrix documentation for tuple variants.
└── former_enum_tests/
    ├── basic_derive.rs           // Covers T1.1
    ├── basic_manual.rs
    └── basic_only_test.rs
    // Potentially new files for tuple variants if existing ones are not suitable:
    // ├── tuple_zero_derive.rs
    // ├── tuple_zero_manual.rs
    // └── tuple_zero_only_test.rs
    // ... (other files as needed based on matrix coverage during detailed planning) ...
    └── compile_fail/
        ├── tuple_single_subform_non_former_error.rs // For T1.5
        └── tuple_multi_subform_error.rs             // For TN.3
```

### Expected Enum Former Behavior Rules (Unnamed/Tuple Variants Only)

1.  **`#[scalar]` Attribute (on variant):**
    *   Zero-Field Tuple Variant (`V()`): `Enum::variant() -> Enum`. (Rule 1b)
    *   Single-Field Tuple Variant (`V(T1)`): `Enum::variant(T1) -> Enum`. (Rule 1d)
    *   Multi-Field Tuple Variant (`V(T1, T2, ...)`): `Enum::variant(T1, T2, ...) -> Enum`. (Rule 1f)
2.  **`#[subform_scalar]` Attribute (on variant):**
    *   Zero-Field Tuple Variant: Error. (Rule 2b)
    *   Single-Field Tuple Variant (`V(T1)` where `T1` derives `Former`): `Enum::variant() -> T1Former<...>`. (Rule 2d)
    *   Single-Field Tuple Variant (`V(T1)` where `T1` does NOT derive `Former`): Error. (Rule 2d)
    *   Multi-Field Tuple Variant: Error. (Rule 2f)
3.  **Default Behavior (No `#[scalar]` or `#[subform_scalar]` on variant):**
    *   Zero-Field Tuple Variant (`V()`): `Enum::variant() -> Enum`. (Rule 3b)
    *   Single-Field Tuple Variant (`V(T1)` where `T1` derives `Former`): `Enum::variant() -> T1Former<...>`. (Rule 3d.i)
    *   Single-Field Tuple Variant (`V(T1)` where `T1` does NOT derive `Former`): `Enum::variant(T1) -> Enum`. (Rule 3d.ii)
    *   Multi-Field Tuple Variant (`V(T1, T2, ...)`): `Enum::variant(T1, T2, ...) -> Enum`. (Rule 3f)
4.  **`#[standalone_constructors]` Attribute (on enum):**
    *   (As per general Rule 4, applied to the outcomes of Rules 1-3 above for tuple variants).

### Failure Diagnosis Algorithm
*   (Standard algorithm as previously defined: Pre-Analysis -> Analyze Error -> Isolate Manual -> Isolate Derive -> Verify Model -> Prioritize Recent Changes)

## Increments

*   [⏳] **Increment 1: Document Test Matrix for Tuple Variants**
    *   **Goal:** Embed the "Test Matrix for Unnamed (Tuple) Variants" into the documentation within `module/core/former/tests/inc/mod.rs`.
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/mod.rs`.
        *   Locate the `mod former_enum_tests { ... }` block (or the simple `mod former_enum_tests;` line if it's not a block yet).
        *   If it's not a block, convert it to `mod former_enum_tests { /* existing submodule declarations */ }`.
        *   Add a module-level documentation comment (`//!`) *inside* the `former_enum_tests` module block. This comment will contain:
            *   A clear title, e.g., "## Test Matrix for Enum Unnamed (Tuple) Variants".
            *   The full "Test Matrix for Unnamed (Tuple) Variants" tables (Zero-Field, Single-Field, Multi-Field).
            *   A brief explanation stating that this matrix guides the testing of tuple variants, linking attributes and variant structures to expected behaviors and relevant internal rule numbers (e.g., "Rule 3b").
            *   A note that this documentation will be expanded as testing for other variant types (struct, unit) is planned.
    *   **Pre-Analysis:** This is a documentation-only change. No functional code is altered.
    *   **Crucial Design Rules:** [Comments and Documentation](#comments-and-documentation).
    *   **Verification Strategy:**
        1.  Request user to apply the changes (full file content for `module/core/former/tests/inc/mod.rs` will be provided).
        2.  Request user to run `cargo check --tests --package former`. Expect compilation success.
        3.  Request user to run `cargo doc --package former --no-deps --open` (or similar command to build and view docs) and manually verify that the "Test Matrix for Unnamed (Tuple) Variants" is correctly rendered in the documentation for the `former_enum_tests` module.

*   [⚫] **Increment 2: Zero-Field Tuple Variants (Combinations T0.1 - T0.4)**
    *   **Goal:** Test `V()` variants.
    *   **Files:** Adapt `enum_named_fields_*` (which contains `VariantZeroUnnamedDefault`, `VariantZeroUnnamedScalar`) or create dedicated `tuple_zero_*` files if cleaner.
    *   **Matrix Coverage:** T0.1, T0.2, T0.3, T0.4.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rules 1b, 3b, 4.
    *   **Verification Strategy:** Staged testing (manual first, then derive) for each combination.

*   [⚫] **Increment 3: Single-Field Tuple Variants - `T1` derives `Former` (Default & Subform Scalar)**
    *   **Goal:** Test `V(T1)` where `T1` derives `Former`, covering default subformer behavior and explicit `#[subform_scalar]`.
    *   **Files:** `basic_*`, `generics_in_tuple_variant_*`, `generics_shared_tuple_*`, `usecase1.rs`. May need to adapt or create `tuple_single_former_*` files.
    *   **Matrix Coverage:** T1.1 (Default), T1.4 (`#[subform_scalar]`).
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rules 3d.i, 2d.
    *   **Verification Strategy:** Staged testing.

*   [⚫] **Increment 4: Single-Field Tuple Variants - `T1` does NOT derive `Former` (Default Scalar-like)**
    *   **Goal:** Test `V(T1)` where `T1` does NOT derive `Former`, covering default scalar-like behavior.
    *   **Files:** Adapt `scalar_generic_tuple_*` or create `tuple_single_non_former_*` files.
    *   **Matrix Coverage:** T1.2 (Default).
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rule 3d.ii.
    *   **Verification Strategy:** Staged testing.

*   [⚫] **Increment 5: Single-Field Tuple Variants - `#[scalar]`**
    *   **Goal:** Test `V(T1)` with `#[scalar]`, for both `T1` deriving Former and not.
    *   **Files:** Adapt `generics_independent_tuple_*`, `scalar_generic_tuple_*`.
    *   **Matrix Coverage:** T1.3.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rule 1d.
    *   **Verification Strategy:** Staged testing.

*   [⚫] **Increment 6: Single-Field Tuple Variants - `#[standalone_constructors]`**
    *   **Goal:** Test `#[standalone_constructors]` with single-field tuple variants.
    *   **Files:** Adapt existing or create new tests focusing on `standalone_constructor_*` patterns for single-field tuples.
    *   **Matrix Coverage:** T1.6, T1.7, T1.8, T1.9, T1.10.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rule 4 in conjunction with 1d, 2d, 3d.
    *   **Verification Strategy:** Staged testing.

*   [⚫] **Increment 7: Multi-Field Tuple Variants (Default & `#[scalar]`)**
    *   **Goal:** Test `V(T1, T2, ...)` variants with default and `#[scalar]` attributes.
    *   **Files:** Adapt `scalar_generic_tuple_*` or create `tuple_multi_*` files.
    *   **Matrix Coverage:** TN.1 (Default), TN.2 (`#[scalar]`).
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rules 1f, 3f.
    *   **Verification Strategy:** Staged testing.

*   [⚫] **Increment 8: Multi-Field Tuple Variants - `#[standalone_constructors]`**
    *   **Goal:** Test `#[standalone_constructors]` with multi-field tuple variants.
    *   **Files:** Adapt existing or create new tests focusing on `standalone_constructor_*` patterns for multi-field tuples.
    *   **Matrix Coverage:** TN.4, TN.5, TN.6.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rule 4 in conjunction with 1f, 3f.
    *   **Verification Strategy:** Staged testing.

*   [⚫] **Increment 9: Error Cases for Tuple Variants (T0.5, T1.5, TN.3)**
    *   **Goal:** Verify compile errors for invalid attribute usage on tuple variants.
    *   **Files:** Create new `trybuild` tests in `module/core/former/tests/inc/former_enum_tests/compile_fail/`:
        *   `tuple_zero_subform_scalar_error.rs` (for T0.5)
        *   `tuple_single_subform_non_former_error.rs` (for T1.5)
        *   `tuple_multi_subform_scalar_error.rs` (for TN.3)
    *   **Crucial Design Rules:** Expected Behavior Rules 2b, 2d (error case), 2f.
    *   **Verification Strategy:** Add `trybuild` test cases asserting specific compilation failures.

*   [⚫] **Increment 10: Final Review and Full Test Suite for Tuple Variants**
    *   **Goal:** Ensure all tuple variant tests are active and passing.
    *   **Verification Strategy:** `cargo check --all-targets --package former`, `cargo clippy ...`, `cargo test ... former_enum_tests`.

### Requirements
*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules for all modifications.
*   **Detailed Increment Plan:** Before starting implementation of an increment, a detailed plan for *that increment only* must be generated and approved.
*   **Paired Testing:** Follow the [Proc Macro: Development Workflow](#proc-macro-development-workflow) rule.
*   **Incremental Verification:** Verify after each increment.
*   **Failure Analysis:** Follow the "Failure Diagnosis Algorithm".
*   **Minimal Changes:** Prioritize minimal changes.

## Notes & Insights
*   This plan focuses specifically on unnamed (tuple) variants.
*   The "Test Matrix for Unnamed (Tuple) Variants" will be embedded in `module/core/former/tests/inc/mod.rs`.
*   The "Expected Enum Former Behavior Rules" are focused on tuple variants for this plan.
*   Existing test files will be leveraged. New files (`tuple_zero_*`, `tuple_single_former_*`, etc.) might be created if existing files are not granular enough for clear matrix coverage. This will be decided during detailed planning for each increment.
