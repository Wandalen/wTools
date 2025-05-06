# Project Plan: Incrementally Uncomment and Fix Enum Tests in `former` Crate

## Goal

*   Uncomment the `former_enum_tests` module and then incrementally uncomment **groups of related test files** (typically `_derive`, `_manual`, `_only_test` variants for a feature, following the Proc Macro Development Workflow) within `module/core/former/tests/inc/former_enum_tests/`. After uncommenting each group, perform a pre-analysis against the expected behavior, address any `// xxx :` or `// qqq :` tasks, and ensure all tests pass before proceeding to the next group.

## Context

*   Files to Include in `context.md`:
    *   `module/core/former/tests/inc/mod.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/enum_named_fields_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/enum_named_fields_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/enum_named_fields_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/keyword_variant_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/keyword_variant_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_derive.rs`
    *   `module/core/former/tests/inc/scalar_generic_tuple_manual.rs`
    *   `module/core/former/tests/inc/scalar_generic_tuple_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_derive.rs`
    *   `module/core/former/tests/inc/standalone_constructor_manual.rs`
    *   `module/core/former/tests/inc/standalone_constructor_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/usecase1.rs`
    *   `module/core/former/tests/inc/former_enum_tests/subform_collection_test.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs`
    *   `module/core/former_meta/src/derive_former/field.rs`
    *   `module/core/former_types/src/lib.rs` # (Example: Include key lib files)
    *   `module/core/macro_tools/src/lib.rs` # (Example: Include key lib files)
*   Crates for Documentation in `context.md`:
    *   `former`
    *   `former_meta`
    *   `former_types`
    *   `macro_tools`

## Expected Enum Former Behavior

This plan adheres to the following rules for `#[derive(Former)]` on enums:

1.  **`#[scalar]` Attribute:**
    *   **Unit Variant:** Generates `Enum::variant() -> Enum`. (Handled by: `handle_unit_variant`)
    *   **Zero-Field Variant (Tuple):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_tuple_zero_variant`)
    *   **Zero-Field Variant (Struct):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_struct_zero_variant`)
    *   **Single-Field Variant (Tuple):** Generates `Enum::variant(InnerType) -> Enum`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Single-Field Variant (Struct):** Generates `Enum::variant { field: InnerType } -> Enum`. (Handled by: `handle_struct_non_zero_variant`)
    *   **Multi-Field Variant (Tuple):** Generates `Enum::variant(T1, T2, ...) -> Enum`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Multi-Field Variant (Struct):** Generates `Enum::variant { f1: T1, f2: T2, ... } -> Enum`. (Handled by: `handle_struct_non_zero_variant`)
    *   **Error Cases:** Cannot be combined with `#[subform_scalar]`.

2.  **`#[subform_scalar]` Attribute:**
    *   **Unit Variant:** Error. (Checked in: `handle_unit_variant`)
    *   **Zero-Field Variant (Tuple or Struct):** Error. (Checked in: `handle_tuple_zero_variant`, `handle_struct_zero_variant`)
    *   **Single-Field Variant (Tuple):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Single-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
    *   **Multi-Field Variant (Tuple):** Error. Cannot use `subform_scalar` on multi-field tuple variants. (Checked in: `handle_tuple_non_zero_variant`)
    *   **Multi-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)

3.  **Default Behavior (No Attribute):**
    *   **Unit Variant:** Generates `Enum::variant() -> Enum`. (Handled by: `handle_unit_variant`)
    *   **Zero-Field Variant (Tuple):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_tuple_zero_variant`)
    *   **Zero-Field Variant (Struct):** Error. Requires `#[scalar]`. (Checked in: `handle_struct_zero_variant`)
    *   **Single-Field Variant (Tuple):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Single-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
    *   **Multi-Field Variant (Tuple):** Generates `Enum::variant(Field1Type, Field2Type, ...) -> Enum` (behaves like `#[scalar]`). (Handled by: `handle_tuple_non_zero_variant`)
    *   **Multi-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)

4.  **`#[standalone_constructors]` Attribute (Body Level):**
    *   Generates top-level constructor functions for each variant (e.g., `my_variant()`).
    *   Return type depends on `#[arg_for_constructor]` on fields within the variant (see Option 2 logic in Readme/advanced.md).

## Failure Diagnosis Algorithm

When `cargo test` fails after uncommenting a test group (`_derive`, `_manual`, `_only_test`), follow this algorithm to determine the cause and propose a fix:

1.  **Pre-Analysis Review:** Revisit the "Expected Behavior" stated in the detailed plan for the current increment. Does the *intended* logic in the uncommented `_derive.rs`, `_manual.rs`, and `_only_test.rs` files align with this expectation? If there was a pre-analysis discrepancy noted, start there.
2.  **Analyze Error:** Examine the compiler error or test panic message provided by the user.
    *   **Compile Error in `_derive.rs`:** Likely a macro generation issue (`former_meta`) or a fundamental incompatibility between the enum structure and the "Expected Enum Former Behavior".
    *   **Compile Error in `_manual.rs`:** Likely an error in the manual implementation itself, or a mismatch with the shared `_only_test.rs` logic or the "Expected Enum Former Behavior".
    *   **Compile Error in `_only_test.rs`:** Likely an issue with the test logic itself, inconsistent naming/types between `_derive.rs` and `_manual.rs`, or a mismatch with the "Expected Enum Former Behavior".
    *   **Test Panic/Failure in `_derive.rs`:** The macro generates code that compiles but produces runtime behavior inconsistent with `_only_test.rs` or the "Expected Enum Former Behavior".
    *   **Test Panic/Failure in `_manual.rs`:** The manual implementation has runtime behavior inconsistent with `_only_test.rs` or the "Expected Enum Former Behavior".

3.  **Check `_manual.rs` Test:** Does the `_manual` test pass independently?
    *   **If YES:** The manual implementation aligns with `_only_test.rs`. The issue is likely in the macro (`former_meta`) or the `_derive.rs` setup *not matching the manual implementation or the expected behavior*. Proceed to Step 4.
    *   **If NO:** The issue is likely in the manual implementation (`_manual.rs`) or the shared test logic (`_only_test.rs`).
        *   Review `_manual.rs` against the "Expected Enum Former Behavior" rules and the logic in `_only_test.rs`. Propose fixes to `_manual.rs` or `_only_test.rs` to align them with the expected behavior.

4.  **Check `_derive.rs` Test:** Does the `_derive` test pass independently?
    *   **If YES:** (And `_manual` also passed) The issue might be subtle or related to interactions not covered by individual tests. Re-run all tests for the module. If still failing, re-evaluate the "Expected Enum Former Behavior" and the test logic.
    *   **If NO:** (And `_manual` passed) The issue is almost certainly in the macro implementation (`former_meta`) generating code that is inconsistent with the working `_manual.rs` and the "Expected Enum Former Behavior".
        *   **Compare Generated Code:** Request the user to help capture the macro-generated code. Compare this generated code side-by-side with the *working* `_manual.rs` implementation. Identify discrepancies.
        *   **Propose Macro Fix:** Based on the comparison and the "Expected Enum Former Behavior", propose specific changes to the relevant handler function within `former_meta` to make the generated code match the manual implementation's logic and the expected behavior.

5.  **Verify Behavior Model:** Ensure the final proposed fix results in behavior consistent with the "Expected Enum Former Behavior" rules. If the rules themselves seem incorrect based on the investigation, note this discrepancy and seek clarification.

6.  **Prioritize Recent Changes:** Always consider the code changes made in the current or immediately preceding steps (uncommenting files, applying previous fixes) as the most likely cause of new failures.

## Increments

## Notes & Insights
