# Project Plan: Fix Failing Former Enum Tests (generics_shared_struct_derive Focus)

## Increments

*   ✅ **Increment 1: Implement Multi-Field Struct Variant - Subformer - Storage**
    *   Goal: Generate the implicit storage struct for the default/subform case for multi-field struct variants.
    *   Detailed Plan Step 1: Locate the `Struct Variant (len > 1)` case in `former_enum.rs`.
    *   Detailed Plan Step 2: Remove the `return Err(...)` for the default case.
    *   Detailed Plan Step 3: Implement logic to generate the `VariantNameFormerStorage` struct definition.
        *   Include generics from the enum (`#enum_generics_impl`, `#enum_generics_where`).
        *   Include `Option<FieldType>` for each field in the variant.
        *   Include `_phantom: #phantom_field_type` using `phantom::tuple(&enum_generics_ty)`.
    *   Detailed Plan Step 4: Implement `impl Default` for the storage struct.
    *   Crucial Design Rules: [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   Verification Strategy: Compile check (`cargo check --package former_meta`). **Hypothesis H2, H6 Check:** Verify the generated storage struct compiles with correct fields, generics, and phantom data.
*   ✅ **Increment 2: Implement Multi-Field Struct Variant - Subformer - Storage Impls**
    *   Goal: Generate `impl Storage` and `impl StoragePreform` for the implicit storage struct.
    *   Detailed Plan Step 1: Implement `impl Storage` for `VariantNameFormerStorage`. Define `type Preformed = ( #( #field_types ),* );`.
    *   Detailed Plan Step 2: Implement `impl StoragePreform` for `VariantNameFormerStorage`.
        *   Implement the `preform` method.
        *   Handle unwrapping/defaulting for each field (`self.#field_ident.take().unwrap_or_default()`).
        *   Return the preformed tuple `( #( #preformed_fields ),* )`.
    *   Crucial Design Rules: [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors).
    *   Verification Strategy: Compile check (`cargo check --package former_meta`). **Hypothesis H3, H6 Check:** Verify `preform` returns the correct tuple type and handles defaults.
*   ✅ **Increment 3: Implement Multi-Field Struct Variant - Subformer - DefTypes**
    *   Goal: Generate the implicit DefinitionTypes struct and impl.
    *   Detailed Plan Step 1: Generate the `VariantNameFormerDefinitionTypes` struct definition with appropriate generics (`#enum_generics_impl`, `Context2`, `Formed2`) and phantom data.
    *   Detailed Plan Step 2: Implement `impl Default` for `VariantNameFormerDefinitionTypes`.
    *   Detailed Plan Step 3: Implement `impl FormerDefinitionTypes` for `VariantNameFormerDefinitionTypes`.
        *   Define `Storage = VariantNameFormerStorage< #enum_generics_ty >`.
        *   Define `Context = Context2`.
        *   Define `Formed = Formed2`.
    *   Detailed Plan Step 4: Implement `impl FormerMutator` (empty) for `VariantNameFormerDefinitionTypes`.
    *   Crucial Design Rules: [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   Verification Strategy: Compile check (`cargo check --package former_meta`). **Hypothesis H6 Check:** Verify generics and associated types are correct.
*   ⚫ **Increment 4: Implement Multi-Field Struct Variant - Subformer - Definition**
    *   Goal: Generate the implicit Definition struct and impl.
    *   Detailed Plan Step 1: Generate the `VariantNameFormerDefinition` struct definition with generics (`#enum_generics_impl`, `Context2`, `Formed2`, `End2`) and phantom data. Use `VariantNameEnd< #enum_generics_ty >` as the default for `End2`.
    *   Detailed Plan Step 2: Implement `impl Default` for `VariantNameFormerDefinition`.
    *   Detailed Plan Step 3: Implement `impl FormerDefinition` for `VariantNameFormerDefinition`.
        *   Define `Storage`, `Context`, `Formed`.
        *   Define `Types = VariantNameFormerDefinitionTypes< #enum_generics_ty, Context2, Formed2 >`.
        *   Define `End = End2`.
        *   Include the necessary `where End2: FormingEnd<...>` clause.
    *   Crucial Design Rules: [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   Verification Strategy: Compile check (`cargo check --package former_meta`). **Hypothesis H6 Check:** Verify generics, associated types, and where clause.
*   ⚫ **Increment 5: Implement Multi-Field Struct Variant - Subformer - Former Struct**
    *   Goal: Generate the implicit Former struct definition.
    *   Detailed Plan Step 1: Generate the `VariantNameFormer` struct definition with generics (`#enum_generics_impl`, `Definition`). Use `VariantNameFormerDefinition< #enum_generics_ty >` as the default for `Definition`.
    *   Detailed Plan Step 2: Include `storage`, `context`, `on_end` fields.
    *   Detailed Plan Step 3: Include the necessary `where Definition: ...` clause.
    *   Crucial Design Rules: [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   Verification Strategy: Compile check (`cargo check --package former_meta`). **Hypothesis H6 Check:** Verify generics and where clause.
*   ⚫ **Increment 6: Implement Multi-Field Struct Variant - Subformer - Former Impl + Setters**
    *   Goal: Generate the `impl` block for the implicit Former, including standard methods and setters for variant fields.
    *   Detailed Plan Step 1: Generate `impl< #enum_generics_impl, Definition > VariantNameFormer<...> where ...`.
    *   Detailed Plan Step 2: Implement standard former methods (`new`, `new_coercing`, `begin`, `begin_coercing`, `form`, `end`).
    *   Detailed Plan Step 3: Implement a setter method for *each* field within the struct variant. Use the field identifier as the setter name.
    *   Crucial Design Rules: [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   Verification Strategy: Compile check (`cargo check --package former_meta`). **Hypothesis H4, H6 Check:** Verify impl generics, standard methods, and setters are correct.
*   ⚫ **Increment 7: Implement Multi-Field Struct Variant - Subformer - End Struct**
    *   Goal: Generate the `End` struct definition for the implicit former.
    *   Detailed Plan Step 1: Generate the `VariantNameEnd` struct definition with generics (`#enum_generics_impl`) and phantom data.
    *   Detailed Plan Step 2: Implement `impl Default` for `VariantNameEnd`.
    *   Crucial Design Rules: [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   Verification Strategy: Compile check (`cargo check --package former_meta`). **Hypothesis H6 Check:** Verify generics and where clause.
*   ⚫ **Increment 8: Implement Multi-Field Struct Variant - Subformer - End Impl**
    *   Goal: Generate the `impl FormingEnd` for the `End` struct.
    *   Detailed Plan Step 1: Generate `impl< #enum_generics_impl > FormingEnd< VariantNameFormerDefinitionTypes<...> > for VariantNameEnd<...> where ...`.
    *   Detailed Plan Step 2: Implement the `call` method.
        *   Signature: `fn call(&self, sub_storage: VariantNameFormerStorage<...>, _context: Option<()>) -> EnumName<...>`.
        *   Body: Call `StoragePreform::preform(sub_storage)` to get the tuple. Construct the enum variant using the tuple elements: `EnumName::VariantName { field1: tuple.0, field2: tuple.1, ... }`.
    *   Crucial Design Rules: [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors).
    *   Verification Strategy: Compile check (`cargo check --package former_meta`). **Hypothesis H3, H5, H6 Check:** Verify `impl` generics, `call` signature, and variant construction logic.
*   ⚫ **Increment 9: Implement Multi-Field Struct Variant - Subformer - Static Method**
    *   Goal: Generate the static method on the enum returning the implicit former.
    *   Detailed Plan Step 1: Add the static method `fn variant_name() -> VariantNameFormer<...>` to the `impl EnumName<...>`.
    *   Detailed Plan Step 2: The return type should be the implicit former `VariantNameFormer` specialized with the default definition using `VariantNameEnd`.
    *   Detailed Plan Step 3: The method body should call `VariantNameFormer::begin(None, None, VariantNameEnd::< #enum_generics_ty >::default())`.
    *   Crucial Design Rules: [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   Verification Strategy: Compile check (`cargo check --package former_meta`). **Hypothesis H1, H7, H6 Check:** Verify return type and method body. **Enable Test:** Uncomment `generics_shared_struct_derive.rs` and `generics_shared_struct_only_test.rs`. Run `cargo test --package former --test former_enum_test generics_shared_struct_*`. **Analyze logs critically.** Fix any compilation errors or test failures related to the generated static method and former logic.
*   ⚫ **Increment 10: Implement Multi-Field Struct Variant - Subformer - Standalone Constructor**
    *   Goal: Generate the standalone constructor for the subformer case (if enabled).
    *   Detailed Plan Step 1: Check `struct_attrs.standalone_constructors`.
    *   Detailed Plan Step 2: If enabled, generate the standalone function `fn variant_name(...) -> ...`.
    *   Detailed Plan Step 3: Determine parameters and return type based on `arg_for_constructor` attributes on variant fields (Option 2 logic: return `Self` if all fields are args, otherwise return `VariantNameFormer<...>` initialized with args).
    *   Detailed Plan Step 4: Implement the function body to either construct `Self` directly or call `VariantNameFormer::begin` with initialized storage.
    *   Crucial Design Rules: [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   Verification Strategy: Compile check (`cargo check --package former_meta`). **Hypothesis H6 Check:** Verify signature, return type, and body based on attributes. Run tests again (`cargo test --package former --test former_enum_test generics_shared_struct_*` and potentially standalone constructor tests if enabled). **Analyze logs critically.**
*   ⚫ Increment 11: Update Documentation Comment in `former_enum.rs`.
*   ⚫ Increment 12: Final Verification (Full Enum Test Suite).
    *   Goal: Ensure all enum tests pass after the focused fix.
    *   Detailed Plan Step 1: **Enable Tests:** Uncomment all remaining tests in `tests/inc/former_enum_tests/`.
    *   Verification Strategy: Run `cargo test --package former --test former_enum_test`. **Analyze logs critically.** Address any remaining failures.

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
*   **[2024-04-26/Plan Revision 3]** Focused plan on fixing `generics_shared_struct_derive.rs` failure by implementing the multi-field struct subformer logic (Increments 1-10). Added final verification increment (11). Preserved previous notes.
*   **[2024-04-26/Inc 1]** Completed implementation of storage struct definition and default impl for multi-field struct variant subformer case. Compile check passed.
*   **[2024-04-26/Inc 2]** Completed implementation of `Storage` and `StoragePreform` traits for the implicit storage struct. Compile check passed.
*   **[2024-04-26/Inc 3]** Completed implementation of `DefinitionTypes` struct and its trait impls (`Default`, `FormerDefinitionTypes`, `FormerMutator`) for the implicit former. Compile check passed.