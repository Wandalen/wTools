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

*   [❌] **Increment 3:** Test Unit Variants with Keyword Identifiers
    *   Commit Message: `test(former): Add manual tests for keyword variants; identify derive bug`

*   [❌] **Increment 4:** Test Unit Variants within Generic Enums
    *   Commit Message: `test(former): Add manual tests for generic enum unit variants; identify derive issue`

*   [❌] **Increment 5:** Test Unit Variants within Enums using Named Field Syntax (for other variants)
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Goal:** Ensure a unit variant in an enum that also contains variants with named fields is correctly handled by `Former` derive.
    *   **Status:** Manual tests for `MixedEnum { SimpleUnit, Complex { data: String } }` pass. The derive macro generates the static constructor for `SimpleUnit` correctly if the `Complex` variant is removed and `#[former(standalone_constructors)]` is removed. However, if `#[former(standalone_constructors)]` is present, the standalone constructor for `SimpleUnit` is not generated, even if `Complex` is removed or simplified to `data: i32`. This indicates an issue with `#[former(standalone_constructors)]` for this specific enum structure (`MixedEnum`) that was not present for the simpler `Status` enum in Increment 1.
    *   **Conclusion for Increment 5:** The derive macro functionality for standalone constructors for unit variants is problematic when the enum is named `MixedEnum` or has other structural differences from the basic `Status` enum, even if other variants are simplified or removed. The static method generation for the unit variant is fine. This increment is blocked for the standalone constructor derive part.
    *   **Detailed Plan Steps (Recap & Block):**
        1.  Create Test Files: (Completed)
        2.  Implement Manual Version: (Completed)
        3.  Implement Shared Test Logic: (Completed)
        4.  Verify Manual Implementation: (Completed, tests pass)
        5.  Implement Derive Version: (Completed, with simplifications during debugging)
        6.  Verify Derive Implementation: (Failed for standalone constructors, passed for static method after simplifications.)
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow], [Testing: Plan with a Test Matrix When Writing Tests]
    *   **Relevant Behavioral Rules:** Rule 1a, 3a, 4a.
    *   **Test Matrix (Mixed Enum - Unit Variant Focus):** (Valid for expected behavior, but derive fails for standalone)
        | ID   | Variant Name | Other Variants Present | Enum Attribute              | Expected Static Method Signature | Expected Standalone Constructor | Rule(s) | Handler (Meta)        |
        |------|--------------|------------------------|-----------------------------|----------------------------------|---------------------------------|---------|-----------------------|
        | T5.1 | `SimpleUnit` | Named fields variant   | None                        | `Enum::simple_unit() -> Enum`    | N/A                             | 1a/3a   | unit\_variant\_handler.rs |
        | T5.2 | `SimpleUnit` | Named fields variant   | `#[standalone_constructors]` | `Enum::simple_unit() -> Enum`    | `fn simple_unit() -> Enum`      | 1a/3a,4 | unit\_variant\_handler.rs |
    *   **Verification Strategy:** Manual tests pass. Derive test for static method passes (when isolated). Derive test for standalone constructor fails.
    *   Commit Message: `test(former): Add manual tests for mixed enums; identify standalone ctor issue`

*   [⚫] **Increment 6:** Test Compile-Fail: Unit Variant with `#[subform_scalar]`
    *   Target Crate(s): `former`, `former_meta` (if macro fixes are needed)
    *   Commit Message: [To be proposed upon successful completion of this increment]
*   [⚫] **Increment 7:** Final Verification of All Unit Variant Tests
    *   Target Crate(s): `former`
    *   Commit Message: [To be proposed upon successful completion of this increment]

### Requirements
*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules (especially "Proc Macro: Development Workflow"), and Codestyle Rules for all modifications.
*   **Incremental Verification:** After each increment involving code changes:
    *   Ensure the relevant code compiles (`cargo check --package former --tests`).
    *   Run all active tests within the `enum_unit_tests` module (`cargo test --package former --test tests -- --test-threads=1 --nocapture enum_unit_tests`). Analyze logs critically.
*   **Failure Analysis:** If tests fail, explicitly consider if the failure is due to an **incorrect test expectation** or a **bug in the macro implementation**. Utilize the `#[debug]` attribute on the enum in the `_derive.rs` file to output the generated code. Analyze this output and compare it with the `_manual.rs` implementation to pinpoint the source of the error before proposing fixes.
*   **Proc Macro Workflow:** Each test-focused increment (1-5) will meticulously follow the Proc Macro Development Workflow:
    1.  Plan and ensure `_manual.rs` implementation exists and is correct.
    2.  Plan and ensure `_only_test.rs` shared test logic exists and is correct.
    3.  Verify manual implementation (`_manual.rs` + `_only_test.rs`) passes.
    4.  Plan and ensure `_derive.rs` macro invocation site exists.
    5.  If `_derive.rs` tests fail while manual passes, analyze (using `#[debug]` output if helpful) and propose fixes to `former_meta` or the test itself if the expectation is wrong.
    6.  Verify `_derive.rs` implementation passes.
*   **No Plan Commits:** This plan file (`-plan.md`) will not be committed to version control.
*   **Scoped Testing:** Test execution will be limited to the `former` package and specifically the relevant test modules (e.g., `enum_unit_tests`), avoiding full workspace tests.
*   **No Clippy:** Clippy checks will not be part of the verification steps.

## Notes & Insights
*   This plan focuses exclusively on the unit variant aspect of enum formers.
*   The "Expected Enum Former Behavior" rules (1a, 2a, 3a, 4a) are central to this plan.
*   If `_manual.rs` files are missing for existing `_derive.rs`/`_only_test.rs` pairs, their creation will be part of the increment.
*   **Identified Bug (Increment 3):** `former::Former` derive macro fails to compile when applied to enums with raw keyword identifiers (e.g., `r#fn`) as variants.
*   **Identified Issue (Increment 4):** `former::Former` derive macro fails to compile for generic enums due to complex trait bound requirements for generic parameters.
*   **Identified Issue (Increment 5):** `former::Former` derive macro fails to generate standalone constructors for `MixedEnum` when `#[former(standalone_constructors)]` is used, unlike simpler enums.