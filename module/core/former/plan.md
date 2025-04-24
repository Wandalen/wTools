# Project Plan: Fix Failing Former Enum Tests Iteratively (Revised Consistency)

## Progress

*   ✅ Increment 1: Run Tests & Capture Output for `enum_named_fields_derive`
*   ✅ Increment 2: Analyze `enum_named_fields_derive` Failure
*   ✅ Increment 3: Implement Error Handling for Default/`#[subform_scalar]` on Struct-Like Variants
*   ✅ **Increment 4: Implement Direct Constructor for `#[scalar]` on Struct-Like Variants**
*   ✅ **Increment 5: Verify Fixes for `enum_named_fields_derive` (adjusting test expectations)**
*   ⏳ **Increment 6: Re-enable `multi_field_*` tests & Capture Output** <-- Current
*   ⚫ Increment 7: Analyze `multi_field_*` Failure & Fix
*   ⚫ Increment 8: Verify Fix for `multi_field_*`
*   ⚫ Increment 9: Re-enable `scalar_generic_tuple_*` tests & Capture Output
*   ⚫ Increment 10: Analyze `scalar_generic_tuple_*` Failure & Fix
*   ⚫ Increment 11: Verify Fix for `scalar_generic_tuple_*`
*   ⚫ ... (Repeat for other commented-out tests: unit, shared_generics, independent_generics, keyword, standalone_constructor, subform_collection) ...
*   ⚫ Increment N: Update Documentation (`Readme.md`, `advanced.md`) with the **final consistent rules**.
*   ⚫ Increment N+1: Final Verification (Full Test Suite)

## Increments

*   ✅ Increment 1: Run Tests & Capture Output for `enum_named_fields_derive`
    *   Goal: Execute the test suite with only the `enum_named_fields_derive` test (and its manual counterpart) enabled within the `former_enum_tests` module to capture the specific compilation errors.
    *   Detailed Plan Step 1: Ensure `module/core/former/tests/inc/mod.rs` reflects the configuration provided by the user (only `basic_*` and `enum_named_fields_*` tests uncommented in `former_enum_tests`).
    *   Detailed Plan Step 2: Run the command `cargo test --package former --test former_enum_test`.
    *   Detailed Plan Step 3: Record the exact compilation errors related to `enum_named_fields_derive.rs` and `enum_named_fields_only_test.rs` in the `## Notes & Insights` section.
    *   Crucial Design Rules: N/A (Observation step).
    *   Verification Strategy: Confirm that the test command was executed and the relevant errors were captured accurately.
*   ✅ Increment 2: Analyze `enum_named_fields_derive` Failure
    *   Goal: Analyze the captured errors and the macro-generated code for `EnumWithNamedFields` to understand why the test fails.
    *   Detailed Plan Step 1: Review the errors recorded in Increment 1 (E0599 missing methods).
    *   Detailed Plan Step 2: Add `#[debug]` to `EnumWithNamedFields` in `enum_named_fields_derive.rs`.
    *   Detailed Plan Step 3: Run `cargo check --package former` to view the generated code snippet. Confirm only `unit_variant` is generated.
    *   Detailed Plan Step 4: Identify that the `syn::Fields::Named` arm in `former_enum.rs` is unimplemented, causing the missing methods. Note that the test expects methods even without `#[scalar]`, which contradicts the revised target rules.
    *   Detailed Plan Step 5: Document findings in `## Notes & Insights`.
    *   Crucial Design Rules: [Comments and Documentation](#comments-and-documentation).
    *   Verification Strategy: Confirm understanding of the root cause (missing implementation) and the discrepancy with test expectations vs. revised rules.
*   ✅ Increment 3: Implement Error Handling for Default/`#[subform_scalar]` on Struct-Like Variants
    *   Goal: Modify `former_enum.rs` to generate compile-time errors for struct-like variants (0, 1, or >1 named fields) when they have no attributes (default) or the `#[subform_scalar]` attribute.
    *   Detailed Plan Step 1: Locate the code block handling `syn::Fields::Named`.
    *   Detailed Plan Step 2: Implement logic for `len == 0`, `len == 1`, and `len > 1`:
        *   If `wants_scalar` is false (Default): Generate a compile-time error stating `#[scalar]` is required for struct-like variants.
        *   If `wants_subform_scalar` is true: Generate a compile-time error stating `#[subform_scalar]` cannot be used on struct-like variants.
    *   Detailed Plan Step 3: Remove the `generate_implicit_former_for_variant` helper function and its callers, as it's no longer needed. Also remove the unused generic helper functions (`generics_of_*_renamed`).
    *   Crucial Design Rules: [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors) (use `syn::Error` for macro errors).
    *   Verification Strategy: Compile checks (`cargo check --package former_meta`). Run `cargo test --package former --test former_enum_test` and verify the *expected* compile errors now appear for `VariantZero`, `VariantOne`, `VariantTwo` in `enum_named_fields_derive.rs` (since they lack `#[scalar]`).
*   ✅ **Increment 4: Implement Direct Constructor for `#[scalar]` on Struct-Like Variants**
    *   Goal: Modify `former_enum.rs` to generate direct static constructor methods for struct-like variants when `#[scalar]` is present.
    *   Detailed Plan Step 1: Locate the code block handling `syn::Fields::Named`.
    *   Detailed Plan Step 2: Implement logic for `len == 0`, `len == 1`, and `len > 1`:
        *   If `wants_scalar` is true: Generate a static method `Enum::variant(...) -> Enum` that takes all fields as arguments (using `impl Into<FieldType>`) and directly constructs the enum variant.
    *   Crucial Design Rules: [Code Style: Do Not Reformat Arbitrarily](#code-style-do-not-reformat-arbitrarily) (apply mandated style).
    *   Verification Strategy: Compile checks (`cargo check --package former_meta`).
*   ✅ **Increment 5: Verify Fixes for `enum_named_fields_derive` (adjusting test expectations)**
    *   Goal: Modify the `enum_named_fields_derive.rs` test case to use `#[scalar]` and confirm it now compiles and passes.
    *   Detailed Plan Step 1: Add `#[scalar]` attribute to `VariantZero`, `VariantOne`, and `VariantTwo` in `module/core/former/tests/inc/former_enum_tests/enum_named_fields_derive.rs`.
    *   Detailed Plan Step 2: Modify the test code in `module/core/former/tests/inc/former_enum_tests/enum_named_fields_only_test.rs` to call the direct constructors generated by `#[scalar]` (e.g., `EnumWithNamedFields::variant_two(42, true)` instead of `EnumWithNamedFields::variant_two().field_b(42)...`).
    *   Detailed Plan Step 3: Run `cargo test --package former --test former_enum_test`.
    *   Detailed Plan Step 4: Verify the test now passes. Address any remaining errors.
    *   Crucial Design Rules: N/A.
    *   Verification Strategy: Successful execution of the target test after modification.
*   ⏳ **Increment 6: Re-enable `multi_field_*` tests & Capture Output**
    *   Goal: Uncomment the `multi_field_manual` and `multi_field_derive` tests in `module/core/former/tests/inc/mod.rs` and capture any new compilation errors.
    *   Detailed Plan Step 1: Edit `module/core/former/tests/inc/mod.rs` and uncomment the lines for `multi_field_manual` and `multi_field_derive`.
    *   Detailed Plan Step 2: Run `cargo test --package former --test former_enum_test`.
    *   Detailed Plan Step 3: Record any new compilation errors in `## Notes & Insights`. If no errors, mark this increment and the next as done.
    *   Crucial Design Rules: N/A (Observation step).
    *   Verification Strategy: Confirm tests were run and errors (or lack thereof) were recorded.
*   ⚫ Increment 7: Analyze `multi_field_*` Failure & Fix
    *   Goal: Analyze and fix errors for `multi_field_derive`. The test likely expects a former builder but should now get a direct constructor due to `#[scalar]`.
    *   Detailed Plan: Analyze errors, modify `multi_field_only_test.rs` to use the direct constructor, ensure `former_enum.rs` generates the correct direct constructor for multi-field tuples with `#[scalar]`.
    *   Crucial Design Rules: TBD.
    *   Verification Strategy: Compile checks, review generated code.
*   ⚫ Increment 8: Verify Fix for `multi_field_*`
    *   Goal: Confirm `multi_field_derive` now passes.
    *   Detailed Plan: Run `cargo test --package former --test former_enum_test`. Verify pass.
    *   Crucial Design Rules: N/A.
    *   Verification Strategy: Successful execution.
*   ⚫ Increment 9: Re-enable `scalar_generic_tuple_*` tests & Capture Output
    *   Goal: Uncomment tests and capture errors.
    *   Detailed Plan Step 1: Edit `mod.rs`, uncomment tests.
    *   Detailed Plan Step 2: Run tests.
    *   Detailed Plan Step 3: Record errors.
    *   Crucial Design Rules: N/A.
    *   Verification Strategy: Confirm tests run, errors recorded.
*   ⚫ Increment 10: Analyze `scalar_generic_tuple_*` Failure & Fix
    *   Goal: Fix errors. Test likely expects direct constructor, ensure `former_enum.rs` generates it correctly for generic tuples with `#[scalar]`. Remove `qqq` comment.
    *   Detailed Plan: Analyze, fix `former_enum.rs`, remove comment from `scalar_generic_tuple_derive.rs`.
    *   Crucial Design Rules: TBD.
    *   Verification Strategy: Compile checks, review generated code.
*   ⚫ Increment 11: Verify Fix for `scalar_generic_tuple_*`
    *   Goal: Confirm tests pass.
    *   Detailed Plan: Run tests.
    *   Crucial Design Rules: N/A.
    *   Verification Strategy: Successful execution.
*   ⚫ ... (Repeat process for other test groups, including explicit uncommenting steps) ...
*   ⚫ Increment N: Update Documentation (`Readme.md`, `advanced.md`) with the **final consistent rules**.
    *   Goal: Update documentation to reflect the final, consistent enum handling logic.
    *   Detailed Plan: Review `Readme.md` and `advanced.md` in `module/core/former/` and update sections related to enum variants, `#[scalar]`, `#[subform_scalar]`, and standalone constructors for enums, ensuring the **consistency** and **direct constructor** behavior of `#[scalar]` is clear.
    *   Crucial Design Rules: [Comments and Documentation](#comments-and-documentation).
    *   Verification Strategy: Manual review of documentation changes.
*   ⚫ Increment N+1: Final Verification (Full Test Suite)
    *   Goal: Run the entire test suite for the `former` crate to ensure all tests pass and there are no regressions.
    *   Detailed Plan: Run `cargo test --package former`.
    *   Crucial Design Rules: N/A.
    *   Verification Strategy: All tests pass.

## Notes & Insights
*   [2025-04-24/New Plan] Adopted iterative approach: Fix one failing enum test group at a time. Start with `enum_named_fields_derive`.
*   [2025-04-24/Inc 1] Ran `cargo test --package former --test former_enum_test` with only `basic_*` and `enum_named_fields_*` tests enabled. Captured 3 E0599 errors in `enum_named_fields_only_test.rs` indicating missing static methods (`variant_zero`, `variant_one`, `variant_two`) for struct-like variants. Also observed 5 expected warnings about unused code in `former_meta`.
*   [2025-04-24/Inc 2] Analysis of `enum_named_fields_derive` failure: Confirmed missing implementation for `syn::Fields::Named`. Test expectations need adjustment later. Root cause is missing logic.
*   [2025-04-24/Correction] **Crucial:** Realized previous plan incorrectly made `#[scalar]` generate an implicit former for struct-like variants. **Revised Rule:** `#[scalar]` *always* generates a direct constructor (taking all fields as args) for *any* non-unit variant (single/multi field, tuple/struct). Default behavior for multi-field/struct variants is now an error. Implicit formers are *not* generated for variants. Plan revised accordingly.
*   [2025-04-24/Inc 3] Implemented error handling for struct-like variants without `#[scalar]` or with `#[subform_scalar]`. Removed unused helper functions. Verification confirmed expected compile errors are now generated for `enum_named_fields_derive.rs` as it lacks `#[scalar]`.
*   [2025-04-24/Inc 4] Implemented direct constructor generation logic for struct-like variants with `#[scalar]`.
*   [2025-04-24/Inc 5] Modified `enum_named_fields_derive.rs` to add `#[scalar]` and adjusted `enum_named_fields_only_test.rs` to use direct constructors. Tests for this group now pass.