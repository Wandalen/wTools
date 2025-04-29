# Project Plan: Make Former Enum Implementation Consistent (Iterative Testing)

## Increments

*   ⏳ **Increment 1: Analyze `former_enum.rs` & Setup**
    *   Goal: Understand existing code, compare against new rules, identify changes, prepare file.
    *   Detailed Plan:
        *   Step 1.1: Read `module/core/former_meta/src/derive_former/former_enum.rs`.
        *   Step 1.2: Identify the main code generation logic within the file, likely focusing on the iteration over enum variants and the conditional logic based on `variant.fields` and attributes (`#[scalar]`, `#[subform_scalar]`).
        *   Step 1.3: Systematically compare the current code generation for each case (Unit, Zero-Field Tuple/Struct, Single-Field Tuple/Struct, Multi-Field Tuple/Struct) against the *new rules* provided in the prompt.
        *   Step 1.4: Document discrepancies and required changes (e.g., in internal notes or comments within the plan). Key areas to check:
            *   Correct constructor generation (direct `-> Self` vs. subformer `-> InnerFormer`) for each case and attribute combination.
            *   Correct error handling (e.g., `#[subform_scalar]` on unit/zero-field, default on multi-field tuple, `#[scalar]` + `#[subform_scalar]`).
            *   Correct handling of path type requirements for subform cases.
            *   Presence and correctness of logic for generating the implicit former ecosystem (Storage, DefTypes, Def, Former, End) for multi-field subform cases (default and `#[subform_scalar]`).
            *   Integration points for standalone constructor logic (to be implemented in Increment 5).
        *   Step 1.5: Add `// RULE:` comments within `former_enum.rs` referencing specific rules from the prompt where changes will be needed (will be done in the implementation step, Output 4).
        *   Step 1.6: Verify that `FieldAttributes` and `ItemAttributes` correctly parse variant-level attributes (`#[scalar]`, `#[subform_scalar]`, `#[arg_for_constructor]`) and struct-level attributes (`#[standalone_constructors]`).
    *   Crucial Design Rules: [Implementation: Complete One Sub-Task Before Starting Another](code/rules/design.md#implementation-complete-one-sub-task-before-starting-another) (Focus on analysis first).
    *   Verification Strategy: Manual review of the analysis notes against the provided rules and the `former_enum.rs` code. Confirm that all discrepancies and required changes based on the new rules have been identified. Compile check (`cargo check --package former_meta`) to ensure the file is still syntactically valid after any potential comment additions.
*   ⚫ **Increment 2: Implement & Verify Unit/Zero-Field Variants**
    *   Goal: Implement logic for Unit and Zero-Field variants (scalar default) in `former_meta` and ensure `unit_variant_*` tests pass.
*   ⚫ **Increment 3: Implement & Verify Single-Field `#[scalar]` Variants**
    *   Goal: Implement direct constructor logic for `#[scalar]` on single-field variants in `former_meta` and ensure relevant tests pass (e.g., `enum_named_fields_derive::VariantOneScalar`, `scalar_generic_tuple::Variant1`).
*   ⚫ **Increment 4: Implement & Verify Single-Field Default/`#[subform_scalar]` Variants**
    *   Goal: Implement subformer logic for default/`#[subform_scalar]` on single-field variants in `former_meta` and ensure relevant tests pass (e.g., `basic_*`, `enum_named_fields_derive::VariantOneDefault/Subform`, `generics_*`).
*   ⚫ **Increment 5: Implement & Verify Multi-Field `#[scalar]` Variants**
    *   Goal: Implement direct constructor logic for `#[scalar]` on multi-field variants in `former_meta` and ensure relevant tests pass (e.g., `enum_named_fields_derive::VariantTwoScalar`, `scalar_generic_tuple::Variant2`).
*   ⚫ **Increment 6: Implement & Verify Multi-Field Default/`#[subform_scalar]` Variants (Implicit Former)**
    *   Goal: Implement implicit former generation logic for default/`#[subform_scalar]` on multi-field variants in `former_meta` and ensure relevant tests pass (e.g., `generics_shared_struct_*`, potentially parts of `enum_named_fields` if adjusted).
*   ⚫ **Increment 7: Implement & Verify Standalone Constructors**
    *   Goal: Implement standalone constructor logic (Option 2) in `former_meta` and ensure `standalone_constructor_*` tests pass.
*   ⚫ **Increment 8: Implement & Verify Keyword Variants**
    *   Goal: Ensure keyword variants work correctly with the implemented logic and verify `keyword_variant_*` tests.
*   ⚫ **Increment 9: Address Remaining Tests & Edge Cases**
    *   Goal: Address `usecase1.rs`, `subform_collection_test.rs`, and any other remaining inconsistencies or failures.
*   ⚫ **Increment 10: Final Verification & Cleanup**
    *   Goal: Run the full enum test suite (`cargo test --package former --test former_enum_test`), fix any remaining issues, and clean up code.

## Notes & Insights

*   [2024-05-01/Plan] This plan focuses on enum consistency and testing first, modifying the existing `former_enum.rs`. Refactoring of `former_enum.rs` (from `former_meta/plan.md`) is deferred.
*   [2024-05-01/Plan] Confirmed understanding: Default and `#[subform_scalar]` on multi-field variants should generate an implicit former for the variant itself.