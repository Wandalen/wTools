# Project Plan: Consistent Enum Variant Handling in Former Derive

## Progress

*   ⚫ Increment 1: Analyze current enum macro logic (`former_enum.rs`)
*   ⚫ Increment 2: Refactor `former_enum.rs` for consistent unit/single-field scalar/subform behavior
*   ⚫ Increment 3: Refactor `former_enum.rs` for consistent multi-field/struct variant behavior
*   ⚫ Increment 4: Update tests for unit/single-field variants
*   ⚫ Increment 5: Update tests for multi-field/struct variants & remove qqq comment
*   ⚫ Increment 6: Update documentation (`Readme.md`, `advanced.md`)
*   ⚫ Increment 7: Final verification

## Increments

*   ⚫ Increment 1: Analyze current enum macro logic (`former_enum.rs`)
    *   Goal: Understand the existing implementation for handling different variant kinds and attributes (`#[scalar]`, `#[subform_scalar]`). Identify discrepancies with the target consistent behavior rules.
*   ⚫ Increment 2: Refactor `former_enum.rs` for consistent unit/single-field scalar/subform behavior
    *   Goal: Modify the macro code to correctly generate direct constructors or subformer starters for unit and single-field variants according to the defined rules (considering `#[scalar]`, `#[subform_scalar]`, and whether the inner type has `Former`).
*   ⚫ Increment 3: Refactor `former_enum.rs` for consistent multi-field/struct variant behavior
    *   Goal: Modify the macro code to generate implicit former builders for multi-field/struct variants *only* when `#[scalar]` is present. Generate compile-time errors for multi-field/struct variants without `#[scalar]`. Ensure struct(0) variants behave like multi-field.
*   ⚫ Increment 4: Update tests for unit/single-field variants
    *   Goal: Review and update tests in `former/tests/inc/former_enum_tests/` related to unit and single-field variants (e.g., `unit_*`, `basic_*`, `scalar_generic_tuple_*`, `generics_shared_tuple_*`, `generics_independent_tuple_*`) to ensure they align with the refactored, consistent logic. Update corresponding `*_manual.rs` files.
*   ⚫ Increment 5: Update tests for multi-field/struct variants & remove qqq comment
    *   Goal: Review and update tests related to multi-field/struct variants (e.g., `multi_field_*`, `enum_named_fields_*`, `generics_shared_struct_*`, `generics_independent_struct_*`). Ensure they test the implicit former builder generation with `#[scalar]` and potentially add tests for the error case without `#[scalar]`. Remove the misleading `qqq` comment from `scalar_generic_tuple_derive.rs`. Update corresponding `*_manual.rs` files.
*   ⚫ Increment 6: Update documentation (`Readme.md`, `advanced.md`)
    *   Goal: Clearly document the consistent rules for how `#[derive(Former)]` handles different enum variants and the effects of `#[scalar]` and `#[subform_scalar]` attributes in the main `former` crate documentation.
*   ⚫ Increment 7: Final verification
    *   Goal: Run the entire test suite for the `former` crate (`cargo test`) to ensure all tests pass and there are no regressions.

## Notes & Insights

*   *(No notes yet)*