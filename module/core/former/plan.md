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
*   ⏳ Increment 6: Extract handler for Struct variants with non-zero fields
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/struct_non_zero.rs`.
    *   Detailed Plan Step 2: Add `mod struct_non_zero;` to `module/core/former_meta/src/derive_former/former_enum.rs`.
    *   Detailed Plan Step 3: Define function `handle_struct_non_zero_variant` in `struct_non_zero.rs` and move non-zero-field Struct variant handling logic into it.
    *   Detailed Plan Step 4: Update `former_for_enum` to call `handle_struct_non_zero_variant`.
    *   Detailed Plan Step 5: Add necessary `use` statements.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](code/rules/design.md#structuring-organize-by-feature-or-layer), [Structuring: Add Module Declaration Before Content](code/rules/design.md#structuring-add-module-declaration-before-content)
    *   Verification Strategy: Ensure `cargo check` completes successfully within the `former_meta` crate and manually review the moved code.
*   ⚫ Increment 7: Update main match statement to use new former_enum
*   ⚫ Increment 8: Verify refactoring with tests

## Notes & Insights

*   *(No notes yet)*
*   **[2025-04-29] Skipped Increment:** Increment 5 (Extract handler for Tuple variants with non-zero fields) was skipped due to persistent issues with applying automated changes to `module/core/former_meta/src/derive_former/former_enum.rs`. Manual intervention is required to complete this increment.
