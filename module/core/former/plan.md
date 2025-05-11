# Project Plan: Test `former` Crate - Enum Unit Variant Aspect

## Goal
*   Systematically ensure comprehensive test coverage for the **unit variant aspect** of the `#[derive(Former)]` macro in the `former` crate.
*   For each identified feature or rule related to unit variants:
    1.  Ensure a manual implementation test (`_manual.rs`) exists and passes, using shared test logic from `_only_test.rs`.
    2.  Ensure a derive macro invocation test (`_derive.rs`) exists and passes, using the same shared test logic.
    3.  If discrepancies arise where the manual test passes but the derive test fails, investigate and propose fixes to the `former_meta` crate. This investigation should consider if the test's expectation is incorrect or if there's a bug in the macro implementation. Utilize the `#[debug]` attribute on the enum in the `_derive.rs` file to output the generated code for analysis and comparison against the manual implementation.
*   All modifications will strictly adhere to `code/gen` instructions, Design Rules (especially "Proc Macro: Development Workflow"), and Codestyle Rules.
*   Verification will be done via `cargo test --package former --test <specific_test_module_or_file>` after each increment. Workspace-level tests and clippy checks will be avoided.

## Relevant Context
*   **Primary Test Directory:** `module/core/former/tests/inc/enum_unit_tests/`
*   **Supporting Files (potential review/modification):**
    *   `module/core/former/tests/inc/mod.rs` (to ensure test modules are active)
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (macro implementation)
    *   `module/core/former_meta/src/derive_former/former_enum/unit_variant_handler.rs` (specific handler for unit variants)
    *   `module/core/former_meta/src/derive_former/former_enum/tuple_single_field_scalar.rs` (handler for scalar tuple variants)
    *   `module/core/former_meta/src/derive_former/former_enum/struct_single_field_subform.rs` (handler for struct variants with subform behavior)
    *   `module/core/former_meta/src/derive_former/struct_attrs.rs` (attribute parsing)
*   **Key Documentation for Reference:**
    *   `module/core/former/Readme.md`
    *   `module/core/former/advanced.md`
    *   This plan's "Expected Enum Former Behavior" section.
*   **Workspace:** Yes, this is part of a Cargo workspace.
*   **Target File Structure:** Primarily working within existing test files or creating new ones following the `_manual.rs`, `_derive.rs`, `_only_test.rs` pattern within `enum_unit_tests`.

### Expected Enum Former Behavior

This plan adheres to the following rules for `#[derive(Former)]` on enums:

1.  **`#[scalar]` Attribute:**
    *   **Unit Variant (Rule 1a):** Generates `Enum::variant() -> Enum`. (Handled by: `unit_variant_handler.rs`)
    *   **Zero-Field Variant (Tuple) (Rule 1b):** Generates `Enum::variant() -> Enum`. (Handled by: `tuple_zero_fields_handler.rs`)
    *   **Zero-Field Variant (Struct) (Rule 1c):** Generates `Enum::variant() -> Enum`. (Handled by: `struct_zero_fields_handler.rs`)
    *   **Single-Field Variant (Tuple) (Rule 1d):** Generates `Enum::variant(InnerType) -> Enum`. (Handled by: `tuple_single_field_scalar.rs`)
    *   **Single-Field Variant (Struct) (Rule 1e):** Generates `Enum::variant { field: InnerType } -> Enum`. (Handled by: `struct_single_field_scalar.rs`)
    *   **Multi-Field Variant (Tuple) (Rule 1f):** Generates `Enum::variant(T1, T2, ...) -> Enum`. (Handled by: `tuple_multi_fields_scalar.rs`)
    *   **Multi-Field Variant (Struct) (Rule 1g):** Generates `Enum::variant { f1: T1, f2: T2, ... } -> Enum`. (Handled by: `struct_multi_fields_scalar.rs`)
    *   **Error Cases:** Cannot be combined with `#[subform_scalar]`.

2.  **`#[subform_scalar]` Attribute:**
    *   **Unit Variant (Rule 2a):** Error. (Checked in: `unit_variant_handler.rs`)
    *   **Zero-Field Variant (Tuple or Struct) (Rule 2b, 2c):** Error. (Checked in: `tuple_zero_fields_handler.rs`, `struct_zero_fields_handler.rs`)
    *   **Single-Field Variant (Tuple) (Rule 2d):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `tuple_single_field_subform.rs`)
    *   **Single-Field Variant (Struct) (Rule 2e):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `struct_single_field_subform.rs`)
    *   **Multi-Field Variant (Tuple) (Rule 2f):** Error. Cannot use `subform_scalar` on multi-field tuple variants. (Checked in dispatch logic / `tuple_multi_fields_scalar.rs`)
    *   **Multi-Field Variant (Struct) (Rule 2g):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `struct_multi_fields_subform.rs`)

3.  **Default Behavior (No Attribute):**
    *   **Unit Variant (Rule 3a):** Generates `Enum::variant() -> Enum`. (Handled by: `unit_variant_handler.rs`)
    *   **Zero-Field Variant (Tuple) (Rule 3b):** Generates `Enum::variant() -> Enum`. (Handled by: `tuple_zero_fields_handler.rs`)
    *   **Zero-Field Variant (Struct) (Rule 3c):** Error. Requires `#[scalar]`. (Checked in: `struct_zero_fields_handler.rs`)
    *   **Single-Field Variant (Tuple) (Rule 3d):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `tuple_single_field_subform.rs`)
    *   **Single-Field Variant (Struct) (Rule 3e):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `struct_single_field_subform.rs`)
    *   **Multi-Field Variant (Tuple) (Rule 3f):** Generates `Enum::variant(Field1Type, Field2Type, ...) -> Enum` (behaves like `#[scalar]`). (Handled by: `tuple_multi_fields_scalar.rs`)
    *   **Multi-Field Variant (Struct) (Rule 3g):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `struct_multi_fields_subform.rs`)

4.  **`#[standalone_constructors]` Attribute (Body Level) (Rule 4):**
    *   **Rule 4a:** Generates top-level constructor functions for each variant (e.g., `fn my_variant()`).
    *   **Rule 4b (Option 2 Logic):** Return type depends on `#[arg_for_constructor]` on fields within the variant.

## Increments

*   [✅] **Increment 1:** Test Basic Unit Variants (Default and `#[scalar]`)
    *   Commit Message: `feat(former): Verify basic unit variant constructors (default, scalar, standalone)`

*   [✅] **Increment 2:** Test Unit Variants with `#[standalone_constructors]`
    *   Commit Message: `chore(former): Confirm standalone constructors for unit variants covered by previous tests`

*   [✅] **Increment 3:** Test Unit Variants with Keyword Identifiers
    *   Commit Message: `fix(former_meta): Handle raw identifiers and attribute parsing for enum formers`

*   [✅] **Increment 4:** Test Unit Variants within Generic Enums
    *   Commit Message: `fix(former_meta): Correctly handle generics in enum variant constructor generation`

*   [✅] **Increment 5:** Test Unit Variants within Enums using Named Field Syntax (for other variants)
    *   **Pre-Analysis:** `former::Former` derive was failing for mixed enums due to issues with implicit former generation for struct-like variants.
    *   **Detailed Plan Steps:**
        1.  **Verify/Create Manual Implementation:** `mixed_enum_unit_manual.rs` and `_only_test.rs` confirmed aligned. Manual tests passed.
        2.  **Verify/Create Derive Implementation (Initial Failure):** `mixed_enum_unit_derive.rs` updated. Initial tests failed (E0412/E0433 - type `MixedEnumComplexFormer` not found).
        3.  **Analyze Failure & Diagnose:** Identified that `struct_single_field_subform.rs` was not generating the definition for the implicit `VariantFormer` (e.g., `MixedEnumComplexFormer`). Also, the emission of `end_impls` (containing these definitions) was commented out in `former_enum.rs`.
        4.  **Propose Fix:** Proposed to uncomment `end_impls` emission in `former_enum.rs` and to add minimal `VariantFormer` struct definition (including generics and `Default` derive) in `struct_single_field_subform.rs`.
        5.  **Implement Fix:** Applied fixes to `former_enum.rs` and `struct_single_field_subform.rs`.
        6.  **Verify Fix:** Tests for `mixed_enum_unit_derive.rs` now pass.
    *   **Crucial Design Rules:** "Proc Macro: Development Workflow"
    *   **Relevant Behavior Rules:** Rule 3a, 3e/3g (default behavior for unit and struct-like variants), Rule 4a.
    *   **Verification Strategy:** Manual tests passed. Derive tests (including standalone for unit variant) passed after fixes.
    *   **Test Matrix:**
        *   ID: T5.1
        *   Factor: Mixed Variant Types (Unit + Struct-like with named fields)
        *   Level: Enum has `UnitVariant` and `StructVariant { field: String }`. `#[former(standalone_constructors)]` is applied.
        *   Expected Outcome (Manual): Standalone constructor for `UnitVariant` exists and works. (Achieved)
        *   Expected Outcome (Derive - Before Fix): Standalone constructor for `UnitVariant` is missing or incorrect. (Observed: E0412/E0433 due to struct variant issues)
        *   Expected Outcome (Derive - After Fix): Standalone constructor for `UnitVariant` is correctly generated and test passes. (Achieved)
        *   Handler (Meta): `former_enum.rs`, `unit_variant_handler.rs`, `struct_single_field_subform.rs`.
    *   Commit Message: `fix(former_meta): Ensure implicit variant formers are defined and emitted for mixed enums`

*   [✅] **Increment 6:** Test Compile-Fail: Unit Variant with `#[subform_scalar]`
    *   Commit Message: `test(former): Add compile-fail test for subform_scalar on unit variant`

*   [✅] **Increment 7:** Final Verification of All Unit Variant Tests
    *   Commit Message: `test(former): Verify all working unit variant tests in enum_unit_tests module`

### Requirements
*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules (especially "Proc Macro: Development Workflow"), and Codestyle Rules for all modifications.
*   **Incremental Verification:** After each increment involving code changes:
    *   Ensure the relevant code compiles (`cargo check --package former --tests`).
    *   Run all active tests within the `enum_unit_tests` module (`cargo test --package former --test tests -- --test-threads=1 --nocapture enum_unit_tests`). Analyze logs critically.
*   **Failure Analysis:** If tests fail, explicitly consider if the failure is due to an **incorrect test expectation** or a **bug in the macro implementation**. Utilize the `#[debug]` attribute on the enum in the `_derive.rs` file to output the generated code. Analyze this output and compare it with the `_manual.rs` implementation to pinpoint the source of the error before proposing fixes.
*   **Proc Macro Workflow:** Each test-focused increment (1-5) will meticulously follow the Proc Macro Development Workflow.
*   **No Plan Commits:** This plan file (`-plan.md`) will not be committed to version control.
*   **Scoped Testing:** Test execution will be limited to the `former` package and specifically the relevant test modules.
*   **No Clippy:** Clippy checks will not be part of the verification steps.

## Notes & Insights
*   This plan focuses exclusively on the unit variant aspect of enum formers.
*   The "Expected Enum Former Behavior" rules (1a, 2a, 3a, 4a) are central to this plan.
*   If `_manual.rs` files are missing for existing `_derive.rs`/`_only_test.rs` pairs, their creation will be part of the increment.
*   **Identified Bug (Increment 3):** `former::Former` derive macro fails to compile when applied to enums with raw keyword identifiers (e.g., `r#fn`) as variants. (NOW FIXED)
*   **Identified Issue (Increment 4):** `former::Former` derive macro fails to compile for generic enums due to complex trait bound requirements for generic parameters. (NOW FIXED)
*   **Identified Issue (Increment 5):** `former::Former` derive macro fails to generate standalone constructors for unit variants in an enum that also contains variants with named fields (struct-like variants), when `#[former(standalone_constructors)]` is used on the enum. (NOW FIXED - by ensuring implicit formers for other variants are correctly defined and emitted).