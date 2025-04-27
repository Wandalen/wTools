# Project Plan: Fix Failing Former Enum Tests Iteratively (Revised Consistency v3)

## Increments

*   ✅ Increment 1: Run Tests & Capture Output for `enum_named_fields_derive`
*   ✅ Increment 2: Analyze `enum_named_fields_derive` Failure
*   ✅ Increment 3: Implement Error Handling for Default/`#[subform_scalar]` on Struct-Like Variants (Partial Fix - Needs Revision)
*   ✅ Increment 4: Implement Direct Constructor for `#[scalar]` on Struct-Like Variants
*   ✅ Increment 5: Verify Fixes for `enum_named_fields_derive` (adjusting test expectations)
*   ⏳ **Increment 6: Implement Logic for Unit and Zero-Field Tuple Variants**
    *   Goal: Implement and verify the logic for Unit variants and Tuple variants with zero fields according to the final rules.
    *   Detailed Plan Step 1: Locate the `match &variant.fields` block in `module/core/former_meta/src/derive_former/former_enum.rs`.
    *   Detailed Plan Step 2: **Unit Variant:** Implement logic: Check `wants_subform_scalar` -> Return `syn::Error`. Otherwise (Default/Scalar) -> Generate Direct Constructor (`Enum::variant() -> Enum`). Generate standalone constructor if enabled.
    *   Detailed Plan Step 3: **Tuple Variant (len 0):** Implement logic: Check `wants_subform_scalar` -> Return `syn::Error`. Otherwise (Default/Scalar) -> Generate Direct Constructor (`Enum::variant() -> Enum`). Generate standalone constructor if enabled.
    *   Detailed Plan Step 4: **Enable Tests:** Uncomment tests in `tests/inc/former_enum_tests/unit_variant_*.rs` and relevant zero-field tests in `enum_named_fields_*.rs`.
    *   Crucial Design Rules: [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors).
    *   Verification Strategy: Compile checks (`cargo check --package former_meta`). Run enabled tests (`cargo test --package former --test former_enum_test unit_variant_*, enum_named_fields_only_test::variant_zero_unnamed_*`). **Analyze logs critically.**
*   ⚫ Increment 7: Implement Logic for Single-Field Tuple Variants
    *   Goal: Implement and verify logic for Tuple variants with one field.
    *   Detailed Plan Step 1: Locate `Tuple Variant (len 1)` case in `former_enum.rs`.
    *   Detailed Plan Step 2: Implement logic: Check `wants_scalar` and `wants_subform_scalar` -> Error.
    *   Detailed Plan Step 3: If `wants_scalar` -> Generate Direct Constructor (`Enum::variant(InnerType) -> Enum`). Generate standalone constructor if enabled.
    *   Detailed Plan Step 4: Otherwise (Default/`wants_subform_scalar`) -> Generate Subformer Starter (`Enum::variant() -> InnerFormer<...>`), checking path type requirement. Generate `End` struct and `impl FormingEnd`. Generate standalone constructor if enabled.
    *   Detailed Plan Step 5: **Enable Tests:** Uncomment tests in `tests/inc/former_enum_tests/basic_*.rs`, `generics_in_tuple_variant_*.rs`, `scalar_generic_tuple_*.rs`.
    *   Crucial Design Rules: [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors).
    *   Verification Strategy: Compile checks. Run enabled tests (`cargo test --package former --test former_enum_test basic_*, generics_in_tuple_variant_*, scalar_generic_tuple_*`). **Analyze logs critically.**
*   ⚫ Increment 8: Implement Logic for Multi-Field Tuple Variants
    *   Goal: Implement and verify logic for Tuple variants with more than one field.
    *   Detailed Plan Step 1: Locate `Tuple Variant (len > 1)` case in `former_enum.rs`.
    *   Detailed Plan Step 2: Implement logic: Check `wants_scalar` and `wants_subform_scalar` -> Error.
    *   Detailed Plan Step 3: If `wants_scalar` -> Generate Direct Constructor (`Enum::variant(T1, T2, ...) -> Enum`). Generate standalone constructor if enabled.
    *   Detailed Plan Step 4: Otherwise (Default/`wants_subform_scalar`) -> Return Error (Subformer not supported for multi-field tuple).
    *   Detailed Plan Step 5: **Enable Tests:** Uncomment relevant multi-field tuple tests (if any) in `enum_named_fields_*.rs` or other files.
    *   Crucial Design Rules: [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors).
    *   Verification Strategy: Compile checks. Run enabled tests (expecting errors for default multi-field tuple if any exist). **Analyze logs critically.**
*   ⚫ Increment 9: Implement Logic for Zero-Field Struct Variants
    *   Goal: Implement and verify logic for Struct variants with zero fields.
    *   Detailed Plan Step 1: Locate `Struct Variant (len 0)` case in `former_enum.rs`.
    *   Detailed Plan Step 2: Implement logic: Check `wants_subform_scalar` -> Error.
    *   Detailed Plan Step 3: If `wants_scalar` -> Generate Direct Constructor (`Enum::variant {} -> Enum`). Generate standalone constructor if enabled.
    *   Detailed Plan Step 4: Otherwise (Default) -> Return `syn::Error`.
    *   Detailed Plan Step 5: **Enable Tests:** Uncomment relevant zero-field struct tests in `enum_named_fields_*.rs`.
    *   Crucial Design Rules: [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors).
    *   Verification Strategy: Compile checks. Run enabled tests (`cargo test --package former --test former_enum_test enum_named_fields_only_test::variant_zero_scalar_*`). **Analyze logs critically.**
*   ⚫ Increment 10: Implement Logic for Single-Field Struct Variants
    *   Goal: Implement and verify logic for Struct variants with one field.
    *   Detailed Plan Step 1: Locate `Struct Variant (len 1)` case in `former_enum.rs`.
    *   Detailed Plan Step 2: Implement logic: Check `wants_scalar` and `wants_subform_scalar` -> Error.
    *   Detailed Plan Step 3: If `wants_scalar` -> Generate Direct Constructor (`Enum::variant { field: InnerType } -> Enum`). Generate standalone constructor if enabled.
    *   Detailed Plan Step 4: Otherwise (Default/`wants_subform_scalar`) -> Generate Subformer Starter (`Enum::variant() -> InnerFormer<...>`), checking path type requirement. Generate `End` struct and `impl FormingEnd`. Generate standalone constructor if enabled.
    *   Detailed Plan Step 5: **Enable Tests:** Uncomment relevant single-field struct tests in `enum_named_fields_*.rs`.
    *   Crucial Design Rules: [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors).
    *   Verification Strategy: Compile checks. Run enabled tests (`cargo test --package former --test former_enum_test enum_named_fields_only_test::variant_one_*`). **Analyze logs critically.**
*   ⚫ Increment 11: Implement Logic for Multi-Field Struct Variants - Scalar Case
    *   Goal: Implement and verify the `#[scalar]` case for multi-field struct variants.
    *   Detailed Plan Step 1: Locate `Struct Variant (len > 1)` case in `former_enum.rs`.
    *   Detailed Plan Step 2: Implement logic: Check `wants_scalar` and `wants_subform_scalar` -> Error.
    *   Detailed Plan Step 3: If `wants_scalar` -> Generate Direct Constructor (`Enum::variant { f1: T1, ... } -> Enum`). Generate standalone constructor if enabled.
    *   Detailed Plan Step 4: **Enable Tests:** Uncomment relevant multi-field scalar struct tests in `enum_named_fields_*.rs`.
    *   Crucial Design Rules: [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors).
    *   Verification Strategy: Compile checks. Run enabled tests (`cargo test --package former --test former_enum_test enum_named_fields_only_test::variant_two_scalar_*`). **Analyze logs critically.**
*   ⚫ **Increment 12: Multi-Field Struct Variant - Subformer - Storage**
    *   Goal: Generate the implicit storage struct for the default/subform case.
    *   Hypothesis (H2, H6): The storage struct definition with `Option<FieldType>` for each field and `PhantomData` using `phantom::tuple(&enum_generics_ty)` compiles correctly with the necessary `where` clause (`#merged_where_clause`).
    *   Detailed Plan: Implement Step 8a from previous plan.
    *   Verification Strategy: Compile check (`cargo check --package former_meta`).
*   ⚫ **Increment 13: Multi-Field Struct Variant - Subformer - Storage Impls**
    *   Goal: Generate `impl Storage` and `impl StoragePreform` for the implicit storage.
    *   Hypothesis (H3, H6): The `StoragePreform::preform` logic correctly handles defaults/unwrapping and returns the expected tuple type `( #( #preform_tuple_type ),* )`.
    *   Detailed Plan: Implement Step 8b from previous plan (adjusted for Storage impls).
    *   Verification Strategy: Compile check.
*   ⚫ **Increment 14: Multi-Field Struct Variant - Subformer - DefTypes**
    *   Goal: Generate the implicit DefinitionTypes struct and impl.
    *   Hypothesis (H6): Combining generics using conditional commas works. Associated types reference implicit storage correctly.
    *   Detailed Plan: Implement Step 8b from previous plan.
    *   Verification Strategy: Compile check.
*   ⚫ **Increment 15: Multi-Field Struct Variant - Subformer - Definition**
    *   Goal: Generate the implicit Definition struct and impl.
    *   Hypothesis (H6): Combining generics works. Associated types and `where E: FormingEnd<...>` clause are correct.
    *   Detailed Plan: Implement Step 8c from previous plan.
    *   Verification Strategy: Compile check.
*   ⚫ **Increment 16: Multi-Field Struct Variant - Subformer - Former Struct**
    *   Goal: Generate the implicit Former struct definition.
    *   Hypothesis (H6): Combining generics works. `where Definition: ...` clause is correct.
    *   Detailed Plan: Implement Step 8d from previous plan.
    *   Verification Strategy: Compile check.
*   ⚫ **Increment 17: Multi-Field Struct Variant - Subformer - Former Impl + Setters**
    *   Goal: Generate the `impl` block for the implicit Former, including setters.
    *   Hypothesis (H4, H6): `impl` block generics/where are correct. Standard methods are correct. Setters for *each* variant field are generated correctly.
    *   Detailed Plan: Implement Step 8e from previous plan.
    *   Verification Strategy: Compile check.
*   ⚫ **Increment 18: Multi-Field Struct Variant - Subformer - End Struct**
    *   Goal: Generate the `End` struct definition.
    *   Hypothesis (H6): Generics and `where` clause are correct.
    *   Detailed Plan: Implement Step 8f from previous plan.
    *   Verification Strategy: Compile check.
*   ⚫ **Increment 19: Multi-Field Struct Variant - Subformer - End Impl**
    *   Goal: Generate the `impl FormingEnd` for the `End` struct.
    *   Hypothesis (H3, H5, H6): `impl FormingEnd<...>` generics are correct. `call` signature is correct. Body correctly uses preformed tuple to construct variant.
    *   Detailed Plan: Implement Step 8g from previous plan.
    *   Verification Strategy: Compile check.
*   ⚫ **Increment 20: Multi-Field Struct Variant - Subformer - Static Method**
    *   Goal: Generate the static method on the enum returning the implicit former.
    *   Hypothesis (H1, H7, H6): Return type correctly references implicit former with generics. Body calls `ImplicitFormer::begin` correctly.
    *   Detailed Plan: Implement Step 8h from previous plan.
    *   Verification Strategy: Compile check.
*   ⚫ **Increment 21: Multi-Field Struct Variant - Subformer - Standalone Constructor**
    *   Goal: Generate the standalone constructor (if enabled).
    *   Hypothesis (H6): Signature, return type (implicit former/Self), and body are correct based on `arg_for_constructor`.
    *   Detailed Plan: Implement Step 8i from previous plan.
    *   Verification Strategy: Compile check.
*   ⚫ Increment 22: Update Documentation Comment in `former_enum.rs` (Original Step 9).
*   ⚫ Increment 23: Verify `generics_shared_struct_*` tests pass.
    *   Goal: Ensure the multi-field struct subformer logic works with generics.
    *   Detailed Plan Step 1: **Enable Tests:** Uncomment tests in `tests/inc/former_enum_tests/generics_shared_struct_*.rs`.
    *   Verification Strategy: Run enabled tests (`cargo test --package former --test former_enum_test generics_shared_struct_*`). **Analyze logs critically.**
*   ⚫ Increment 24: Verify `generics_independent_struct_*` tests pass.
    *   Goal: Ensure the multi-field struct subformer logic works with independent generics.
    *   Detailed Plan Step 1: **Enable Tests:** Uncomment tests in `tests/inc/former_enum_tests/generics_independent_struct_*.rs`.
    *   Verification Strategy: Run enabled tests (`cargo test --package former --test former_enum_test generics_independent_struct_*`). **Analyze logs critically.**
*   ⚫ Increment 25: Verify `standalone_constructor_*.rs` tests pass.
    *   Goal: Ensure standalone constructors work correctly for all implemented variant types.
    *   Detailed Plan Step 1: **Enable Tests:** Uncomment tests in `tests/inc/former_enum_tests/standalone_constructor_*.rs` and `standalone_constructor_args_*.rs`.
    *   Verification Strategy: Run enabled tests (`cargo test --package former --test former_enum_test standalone_constructor_*`). **Analyze logs critically.**
*   ⚫ Increment 26: Address remaining test files (e.g., `keyword_variant`, `subform_collection_test`).
    *   Goal: Enable and fix any remaining enum-related tests.
    *   Detailed Plan Step 1: **Enable Tests:** Uncomment remaining test files in `tests/inc/former_enum_tests/`.
    *   Verification Strategy: Run enabled tests (`cargo test --package former --test former_enum_test`). **Analyze logs critically.** Fix any failures.
*   ⚫ Increment 27: Update Documentation (`Readme.md`, `advanced.md`).
*   ⚫ Increment 28: Final Verification (Full Test Suite).

## Notes & Insights

*   [2025-04-24/New Plan] Adopted iterative approach: Fix one failing enum test group at a time. Start with `enum_named_fields_derive`.
*   [2025-04-24/Inc 1] Ran `cargo test --package former --test former_enum_test` with only `basic_*` and `enum_named_fields_*` tests enabled. Captured 3 E0599 errors in `enum_named_fields_only_test.rs` indicating missing static methods (`variant_zero`, `variant_one`, `variant_two`) for struct-like variants. Also observed 5 expected warnings about unused code in `former_meta`.
*   [2025-04-24/Inc 2] Analysis of `enum_named_fields_derive` failure: Confirmed missing implementation for `syn::Fields::Named`. Test expectations need adjustment later. Root cause is missing logic.
*   [2025-04-24/Correction] **Crucial:** Realized previous plan incorrectly made `#[scalar]` generate an implicit former for struct-like variants. **Revised Rule:** `#[scalar]` *always* generates a direct constructor (taking all fields as args) for *any* non-unit variant (single/multi field, tuple/struct). Default behavior for multi-field/struct variants is now an error. Implicit formers are *not* generated for variants. Plan revised accordingly.
*   [2025-04-24/Inc 3] Implemented error handling for struct-like variants without `#[scalar]` or with `#[subform_scalar]`. Removed unused helper functions. Verification confirmed expected compile errors are now generated for `enum_named_fields_derive.rs` as it lacks `#[scalar]`.
*   [2025-04-24/Inc 4] Implemented direct constructor generation logic for struct-like variants with `#[scalar]`.
*   [2025-04-24/Inc 5] Modified `enum_named_fields_derive.rs` to add `#[scalar]` and adjusted `enum_named_fields_only_test.rs` to use direct constructors. Tests for this group now pass.
*   [2025-04-24/Correction 2] **Crucial:** User clarified that `#[subform_scalar]` *should* work on single-field struct variants and multi-field varians as swell, and the default for single-field struct variants and multi-field must be subforming (like single-field tuple). The default for zero-field struct variants should be an error. Plan revised again.
*   **[2024-04-25/Plan Update]** Revised detailed steps for Increment 6 to align with the final rules provided by the user. Added placeholder increments (10-24) to address remaining test files. Renumbered final increments.
*   **[2024-04-25/Inc 6 Correction]** Fixed regressions related to unused variables and scope issues in test files.
*   **[2024-04-25/Inc 6 Correction 2]** Fixed regressions related to syntax errors (extra commas) and logic errors (`FormingEnd::call`) in generated code for implicit formers.
*   **[2024-04-25/Inc 6 Decomposition]** Decomposed Step 8 (Multi-Field Struct Variant - Subformer Case) into smaller sub-steps (8a-8i) to isolate and verify the generation of each implicit former component. Updated plan accordingly. Renumbered subsequent increments.
*   **[2024-04-25/Inc 6 Hypothesis]** Confirmed hypotheses for implicit former generation for multi-field struct variants. Key points: generate dedicated former ecosystem for the variant, storage holds `Option<FieldType>` for all variant fields, `preform` returns tuple, former has setters for all variant fields, `End::call` uses preformed tuple to construct variant. Generics handling (H6) and `End::call` logic (H8) require careful implementation.
*   **[2024-04-25/Inc 6 Plan Revision]** Further decomposed Increment 6. Will now implement logic for each variant type incrementally (Unit, Tuple(0), Tuple(1), Tuple(N), Struct(0), Struct(1), Struct(N)-Scalar). The complex Struct(N)-Subformer case is broken into multiple increments (12-21) based on verified hypotheses.
*   **[2024-04-25/Plan Update 2]** Added explicit test enabling steps to increments 6-11, 23-26. Renumbered final increments.