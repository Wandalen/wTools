# Project Plan: Refactor Enum Variant Handling in Former Derive

## Increments

*   ⏳ Increment 1: Set up module structure for variant former_enum
    *   Detailed Plan Step 1: Create directory `module/core/former_meta/src/derive_former/former_enum/`.
    *   Detailed Plan Step 2: Create module file `module/core/former_meta/src/derive_former/former_enum/mod.rs`.
    *   Detailed Plan Step 3: Add `mod former_enum;` to `module/core/former_meta/src/derive_former.rs`.
    *   Crucial Design Rules: [Structuring: Organize by Feature or Layer](code/rules/design.md#structuring-organize-by-feature-or-layer), [Structuring: Add Module Declaration Before Content](code/rules/design.md#structuring-add-module-declaration-before-content)
    *   Verification Strategy: Ensure `cargo test` completes successfully within the `former` crate. <!-- Updated -->
*   ⚫ Increment 2: Extract handler for Unit variants
*   ⚫ Increment 3: Extract handler for Tuple variants with zero fields
*   ⚫ Increment 4: Extract handler for Struct variants with zero fields
*   ⚫ Increment 5: Extract handler for Tuple variants with non-zero fields
*   ⚫ Increment 6: Extract handler for Struct variants with non-zero fields
*   ⚫ Increment 7: Update main match statement to use new former_enum
*   ⚫ Increment 8: Verify refactoring with tests

## Notes & Insights

*   *(No notes yet)*
