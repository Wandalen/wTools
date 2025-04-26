# Project Plan: Fix Failing Former Enum Tests Iteratively (Revised Consistency v3)

## Progress

*   ✅ Increment 1: Run Tests & Capture Output for `enum_named_fields_derive`
*   ✅ Increment 2: Analyze `enum_named_fields_derive` Failure
*   ✅ Increment 3: Implement Error Handling for Default/`#[subform_scalar]` on Struct-Like Variants (Partial Fix - Needs Revision)
*   ✅ Increment 4: Implement Direct Constructor for `#[scalar]` on Struct-Like Variants
*   ✅ Increment 5: Verify Fixes for `enum_named_fields_derive` (adjusting test expectations)
*   ⏳ **Increment 6: Refactor `former_enum.rs` for Correct Default/Subform Behavior & Error Handling (FINAL)** <-- Current
*   ⚫ Increment 7: Re-enable `generics_shared_struct_*` tests & Capture Output
*   ⚫ Increment 8: Analyze `generics_shared_struct_*` Failure & Fix
*   ⚫ Increment 9: Verify Fix for `generics_shared_struct_*`
*   ⚫ ... (Renumber subsequent increments) ...
*   ⚫ Increment N: Update Documentation (`Readme.md`, `advanced.md`) with the **final consistent rules**.
*   ⚫ Increment N+1: Final Verification (Full Test Suite)

## Increments

*   ... (Increments 1-5 details omitted) ...
*   ⏳ **Increment 6: Refactor `former_enum.rs` for Correct Default/Subform Behavior & Error Handling (FINAL)**
    *   Goal: Correct the logic in `former_enum.rs` for all variant types according to the FINAL rules, ensuring `#[subform_scalar]` errors correctly on multi-field/zero-field/unit variants, and default errors correctly on multi-field/zero-field-struct variants.
    *   Detailed Plan Step 1: Locate the `match &variant.fields` block.
    *   Detailed Plan Step 2: **Unit Variant:** Add check: if `wants_subform_scalar`, return error. Keep direct constructor generation for `wants_scalar` or default.
    *   Detailed Plan Step 3: **Tuple Variant (len 0):** Add check: if `wants_subform_scalar`, return error. Keep direct constructor generation for `wants_scalar` or default.
    *   Detailed Plan Step 4: **Tuple Variant (len 1):** Add check: if `wants_scalar` and `wants_subform_scalar`, return error. Keep logic: if `wants_scalar`, generate direct constructor; otherwise (default or `wants_subform_scalar`), generate subformer starter.
    *   Detailed Plan Step 5: **Tuple Variant (len > 1):** Add check: if `wants_subform_scalar`, return error. Keep logic: if `wants_scalar`, generate direct constructor; otherwise (default), return error.
    *   Detailed Plan Step 6: **Struct Variant (len 0):** Add check: if `wants_subform_scalar`, return error. Keep logic: if `wants_scalar`, generate direct constructor; otherwise (default), return error.
    *   Detailed Plan Step 7: **Struct Variant (len 1):** Add check: if `wants_scalar` and `wants_subform_scalar`, return error. Keep logic: if `wants_scalar`, generate direct constructor; otherwise (default or `wants_subform_scalar`), generate subformer starter.
    *   Detailed Plan Step 8: **Struct Variant (len > 1):** Add check: if `wants_subform_scalar`, return error. Keep logic: if `wants_scalar`, generate direct constructor; otherwise (default), return error.
    *   Detailed Plan Step 9: Update the documentation comment block at the top of the file with the FINAL rules.
    *   Crucial Design Rules: [Code Style: Do Not Reformat Arbitrarily](#code-style-do-not-reformat-arbitrarily), [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors).
    *   Verification Strategy: Compile checks (`cargo check --package former_meta`). Run `cargo test --package former --test former_enum_test` (with only `basic_*` and `enum_named_fields_*` enabled) and verify it still passes.
*   ⚫ Increment 7: Re-enable `generics_shared_struct_*` tests & Capture Output
    *   Goal: Uncomment tests and capture errors.
    *   Detailed Plan Step 1: Edit `mod.rs`, uncomment tests.
    *   Detailed Plan Step 2: Run tests.
    *   Detailed Plan Step 3: Record errors.
    *   Crucial Design Rules: N/A.
    *   Verification Strategy: Confirm tests run, errors recorded.
*   ⚫ Increment 8: Analyze `generics_shared_struct_*` Failure & Fix
    *   Goal: Fix errors. The test uses a default multi-field struct variant, which should now correctly generate an implicit former starter. Adjust test code if needed.
    *   Detailed Plan: Analyze, fix `former_enum.rs` if implicit former logic has issues, potentially adjust test.
    *   Crucial Design Rules: TBD.
    *   Verification Strategy: Compile checks, review generated code.
*   ⚫ Increment 9: Verify Fix for `generics_shared_struct_*`
    *   Goal: Confirm tests pass.
    *   Detailed Plan: Run tests.
    *   Crucial Design Rules: N/A.
    *   Verification Strategy: Successful execution.
*   ... (Subsequent increments renumbered) ...

## Notes & Insights

-*   [2025-04-24/New Plan] Adopted iterative approach: Fix one failing enum test group at a time. Start with `enum_named_fields_derive`.
-*   [2025-04-24/Inc 1] Ran `cargo test --package former --test former_enum_test` with only `basic_*` and `enum_named_fields_*` tests enabled. Captured 3 E0599 errors in `enum_named_fields_only_test.rs` indicating missing static methods (`variant_zero`, `variant_one`, `variant_two`) for struct-like variants. Also observed 5 expected warnings about unused code in `former_meta`.
-*   [2025-04-24/Inc 2] Analysis of `enum_named_fields_derive` failure: Confirmed missing implementation for `syn::Fields::Named`. Test expectations need adjustment later. Root cause is missing logic.
-*   [2025-04-24/Correction] **Crucial:** Realized previous plan incorrectly made `#[scalar]` generate an implicit former for struct-like variants. **Revised Rule:** `#[scalar]` *always* generates a direct constructor (taking all fields as args) for *any* non-unit variant (single/multi field, tuple/struct). Default behavior for multi-field/struct variants is now an error. Implicit formers are *not* generated for variants. Plan revised accordingly.
-*   [2025-04-24/Inc 3] Implemented error handling for struct-like variants without `#[scalar]` or with `#[subform_scalar]`. Removed unused helper functions. Verification confirmed expected compile errors are now generated for `enum_named_fields_derive.rs` as it lacks `#[scalar]`.
-*   [2025-04-24/Inc 4] Implemented direct constructor generation logic for struct-like variants with `#[scalar]`.
-*   [2025-04-24/Inc 5] Modified `enum_named_fields_derive.rs` to add `#[scalar]` and adjusted `enum_named_fields_only_test.rs` to use direct constructors. Tests for this group now pass.
-*   [2025-04-24/Correction 2] **Crucial:** User clarified that `#[subform_scalar]` *should* work on single-field struct variants and multi-field varians as swell, and the default for single-field struct variants and multi-field must be subforming (like single-field tuple). The default for zero-field struct variants should be an error. Plan revised again.
