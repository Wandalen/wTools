
# Project Plan: Refactor Enum Variant Handling in Former Derive

## Initial Task

 move out each branch of `match &variant.fields` into a separate function in a separate file and there should be files for each of these cases:

- Unit
- Zero-Field Variant Tuple
- Zero-Field Variant Struct
- non Zero-Field Variant Tuple
- non Zero-Field Variant Struct

## Increments

*   ✅ Increment 1: Set up module structure for variant former_enum
    *   Detailed Plan Step 1: Create directory `module/core/former_meta/src/derive_former/former_enum/`.
    *   Detailed Plan Step 2: Create module file `module/core/former_meta/src/derive_former/former_enum/mod.rs`.
    *   Detailed Plan Step 3: Add `mod former_enum;` to `module/core/former_meta/src/derive_former.rs`.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](code/rules/design.md#structuring-organize-by-feature-or-layer), [Structuring: Add Module Declaration Before Content](code/rules/design.md#structuring-add-module-declaration-before-content)
    *   Verification Strategy: Ensure `cargo test` completes successfully within the `former` crate. <!-- Updated -->
*   ✅ Increment 2: Extract handler for Unit variants
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/unit.rs`.
    *   Detailed Plan Step 2: Add `mod unit;` to `module/core/former_meta/src/derive_former/former_enum.rs`.
    *   Detailed Plan Step 3: Define function `handle_unit_variant` in `unit.rs` and move Unit variant handling logic into it.
    *   Detailed Plan Step 4: Update `former_for_enum` to call `handle_unit_variant`.
    *   Detailed Plan Step 5: Add necessary `use` statements.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](code/rules/design.md#structuring-organize-by-feature-or-layer), [Structuring: Add Module Declaration Before Content](code/rules/design.md#structuring-add-module-declaration-before-content)
    *   Verification Strategy: Ensure `cargo check` completes successfully within the `former_meta` crate and manually review the moved code.
*   ✅ Increment 3: Extract handler for Tuple variants with zero fields
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/tuple_zero.rs`.
    *   Detailed Plan Step 2: Add `mod tuple_zero;` to `module/core/former_meta/src/derive_former/former_enum.rs`.
    *   Detailed Plan Step 3: Define function `handle_tuple_zero_variant` in `tuple_zero.rs` and move zero-field Tuple variant handling logic into it.
    *   Detailed Plan Step 4: Update `former_for_enum` to call `handle_tuple_zero_variant`.
    *   Detailed Plan Step 5: Add necessary `use` statements.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](code/rules/design.md#structuring-organize-by-feature-or-layer), [Structuring: Add Module Declaration Before Content](code/rules/design.md#structuring-add-module-declaration-before-content)
    *   Verification Strategy: Ensure `cargo check` completes successfully within the `former_meta` crate and manually review the moved code.
*   ✅ Increment 4: Extract handler for Struct variants with zero fields
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/struct_zero.rs`.
    *   Detailed Plan Step 2: Add `mod struct_zero;` to `module/core/former_meta/src/derive_former/former_enum.rs`.
    *   Detailed Plan Step 3: Define function `handle_struct_zero_variant` in `struct_zero.rs` and move zero-field Struct variant handling logic into it.
    *   Detailed Plan Step 4: Update `former_for_enum` to call `handle_struct_zero_variant`.
    *   Detailed Plan Step 5: Add necessary `use` statements.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](code/rules/design.md#structuring-organize-by-feature-or-layer), [Structuring: Add Module Declaration Before Content](code/rules/design.md#structuring-add-module-declaration-before-content)
    *   Verification Strategy: Ensure `cargo check` completes successfully within the `former_meta` crate and manually review the moved code.
*   ❌ Increment 5: Extract handler for Tuple variants with non-zero fields
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/tuple_non_zero.rs`.
    *   Detailed Plan Step 2: Add `mod tuple_non_zero;` to `module/core/former_meta/src/derive_former/former_enum.rs`.
    *   Detailed Plan Step 3: Define function `handle_tuple_non_zero_variant` in `tuple_non_zero.rs` and move non-zero-field Tuple variant handling logic into it.
    *   Detailed Plan Step 4: Update `former_for_enum` to call `handle_tuple_non_zero_variant`.
    *   Detailed Plan Step 5: Add necessary `use` statements.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](code/rules/design.md#structuring-organize-by-feature-or-layer), [Structuring: Add Module Declaration Before Content](code/rules/design.md#structuring-add-module-declaration-before-content)
    *   Verification Strategy: Ensure `cargo check` completes successfully within the `former_meta` crate and manually review the moved code.
*   ✅ Increment 6: Extract handler for Struct variants with non-zero fields
    *   Detailed Plan Step 1: **Fix E0599 Errors:** Modify the calls to `handle_unit_variant`, `handle_tuple_zero_variant`, `handle_struct_zero_variant`, and the *future* call to `handle_struct_non_zero_variant` within `former_for_enum` in `former_enum.rs`. Change the last argument from `merged_where_clause.map(|wc| &wc.predicates)` back to `merged_where_clause`.
    *   Detailed Plan Step 2: **Update Handler Signatures:** Modify the function signatures in `unit.rs`, `tuple_zero.rs`, and `struct_zero.rs` to accept `merged_where_clause : Option< &'a syn::WhereClause >` instead of `Option< &'a syn::punctuated::Punctuated<...> >`. Adjust the internal logic of these handlers if they were using the predicates directly (they likely weren't, as the error occurred during the call).
    *   Detailed Plan Step 3: **Create File:** Create `module/core/former_meta/src/derive_former/former_enum/struct_non_zero.rs`.
    *   Detailed Plan Step 4: **Add Module Declaration:** Add `mod struct_non_zero;` and `use struct_non_zero::handle_struct_non_zero_variant;` to `module/core/former_meta/src/derive_former/former_enum/mod.rs`.
    *   Detailed Plan Step 5: **Define Function:** Define the function signature for `handle_struct_non_zero_variant` in `struct_non_zero.rs`, ensuring it accepts all necessary parameters (including `merged_where_clause : Option< &'a syn::WhereClause >`) and has the correct visibility (`pub(super)` likely).
    *   Detailed Plan Step 6: **Move Logic:** Cut the code block corresponding to the `Fields::Named( fields )` match arm (where `fields.named.len() > 0`) from `former_for_enum` in `former_enum.rs` and paste it into the body of `handle_struct_non_zero_variant` in `struct_non_zero.rs`.
    *   Detailed Plan Step 7: **Adjust Paths/Visibility:** Correct any import paths or visibility issues within the moved code block. Ensure it uses the passed parameters correctly.
    *   Detailed Plan Step 8: **Update Caller:** In `former_for_enum` (in `former_enum.rs`), replace the moved code block with a call to `handle_struct_non_zero_variant`, passing the necessary arguments.
    *   Detailed Plan Step 9: **Apply Codestyle:** Strictly apply codestyle rules (spacing, newlines, indentation) to `struct_non_zero.rs` and the modified `former_enum.rs`.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](code/rules/design.md#structuring-organize-by-feature-or-layer), [Structuring: Add Module Declaration Before Content](code/rules/design.md#structuring-add-module-declaration-before-content), [Visibility: Keep Implementation Details Private](code/rules/design.md#visibility-keep-implementation-details-private).
    *   Verification Strategy: Run `cargo check --package former_meta` after fixing E0599 errors (Step 1 & 2). Analyze output. Run `cargo check --package former_meta` after moving the code (Step 3-8). Analyze output. Run `cargo test --package former` to ensure semantic equivalence after the refactoring is complete. **Analyze logs critically.**
*   ⚫ Increment 7: Update main match statement to use new former_enum
*   ⚫ Increment 8: Verify refactoring with tests

## Notes & Insights

*   *(No notes yet)*
*   **[2025-04-29] Skipped Increment:** Increment 5 (Extract handler for Tuple variants with non-zero fields) was skipped due to persistent issues with applying automated changes to `module/core/former_meta/src/derive_former/former_enum.rs`. Manual intervention is required to complete this increment.
*   **[2025-04-29] Stuck in Increment 6:** Encountered persistent compilation errors after moving code into `handle_struct_non_zero_variant`. Initiating Stuck Resolution Process.
*   **[2025-04-29] Hypotheses for Increment 6:**
    *   Hypothesis 1: The generated `Storage` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 2: The generated `DefinitionTypes` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 3: The generated `Definition` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 4: The generated `Former` struct contains a brace mismatch or syntax error.
    *   Hypothesis 5: The issue arises from the combination or interaction of the individually generated components, not the components themselves.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 1: The generated `Storage` struct or its implementations contain a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` blocks generating the `Storage` struct and its `impl` blocks (`storage_def`, `storage_default_impl`, `storage_trait_impl`, `storage_preform_impl`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 2: The generated `DefinitionTypes` struct or its implementations contain a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` blocks generating the `DefinitionTypes` struct and its `impl` blocks (`def_types_struct`, `def_types_default_impl`, `def_types_former_impl`, `def_types_mutator_impl`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 3: The generated `Definition` struct or its implementations contain a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` blocks generating the `Definition` struct and its `impl` blocks (`def_struct`, `def_default_impl`, `def_former_impl`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 4: The generated `Former` struct contains a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` block generating the `Former` struct definition (`former_struct_def`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2024-04-30/Increment 6] Fix:** Resolved E0599 compilation errors by changing how `merged_where_clause` is passed to handler functions (passing `Option<&WhereClause>` instead of `Option<&Punctuated<...>>`).
