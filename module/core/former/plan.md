# Project Plan: Make Former Enum Implementation Consistent (Iterative Testing)

## Increments

*   ⚫ **Increment 1: Analyze `former_enum.rs` & Setup**
    *   Goal: Understand existing code, compare against new rules, identify changes, prepare file.
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