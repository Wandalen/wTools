# Project Plan: Restructure `former_enum_tests` Directory by Variant Type

## Goal
*   Reorganize the `module/core/former/tests/inc/former_enum_tests/` directory by creating three subdirectories:
    *   `unit_tests/`
    *   `unnamed_tests/` (for tuple variants)
    *   `named_tests/` (for struct-like variants with named fields)
*   Move existing enum test files into the appropriate new subdirectory.
*   **If an existing test file covers multiple variant types (unit, tuple, named), it must be split into separate files, each focusing on a single variant type, and then moved to the correct subdirectory.**
*   Update `module/core/former/tests/inc/former_enum_tests/mod.rs` and newly created subdirectory `mod.rs` files to reflect the new structure, ensuring all `mod` declarations point to the correct new paths.
*   Ensure all tests (even if internally commented out in their final locations) compile after the refactoring. The primary goal is structural integrity; test logic verification is for subsequent plans.
*   Preserve all existing test logic, including any currently commented-out tests (they will remain commented out in their new locations).

## Relevant Context

*   **Primary Directory to Refactor:** `module/core/former/tests/inc/former_enum_tests/`
*   **Module File to Update:** `module/core/former/tests/inc/former_enum_tests/mod.rs`
*   **Parent Module File:** `module/core/former/tests/inc/mod.rs`
*   **Existing Test Files within `former_enum_tests/`:** (Full list will be generated in Increment 1)
    *   Includes `_derive.rs`, `_manual.rs`, `_only_test.rs` patterns, and single files like `usecase1.rs`.
    *   Includes the `compile_fail/` directory.
*   **Documentation:**
    *   `module/core/former/advanced.md`
    *   `module/core/former/Readme.md`

### Expected Enum Former Behavior Rules (Full Set for Context)

(This section will be preserved as it's crucial for understanding the purpose of the tests being moved, even if not directly acted upon in *this* refactoring plan.)
1.  **`#[scalar]` Attribute (on variant):** ...
2.  **`#[subform_scalar]` Attribute (on variant):** ...
3.  **Default Behavior (No `#[scalar]` or `#[subform_scalar]` on variant):** ...
4.  **`#[standalone_constructors]` Attribute (on enum):** ...

## Increments

*   [✅] **Increment 1: Detailed File Analysis and Relocation/Splitting Plan**
    *   **Goal:** Analyze each file in `former_enum_tests/`, determine its new location(s), and plan any necessary splits if a file covers multiple variant types. Plan `mod.rs` updates.
    *   **Target Crate(s):** `former`
    *   **Pre-Analysis:** The `former_enum_tests` directory contains various test files, some of which may test multiple enum variant types (unit, tuple, named). A detailed listing and analysis of each file's content is needed to determine the correct categorization and identify files requiring splitting. Compile-fail tests also need categorization.
    *   **Crucial Design Rules:** Structuring: Organize by Feature or Layer, Structuring: Add Module Declaration Before Content, Structuring: Split Large Files Methodically (If Requested), Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks.
    *   **Relevant Behavior Rules:** Expected Enum Former Behavior Rules (referenced for context on test purpose).
    *   **Detailed Plan Step 1:** Listed all files and subdirectories currently in `module/core/former/tests/inc/former_enum_tests/`. (Completed)
    *   **Detailed Plan Step 2 (Categorization & Splitting Strategy):** Analyzed each file to identify variant types and plan splitting/relocation. (Completed)
        *   **Files to Move (No Splitting Needed):**
            - `module/core/former/tests/inc/former_enum_tests/basic_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/basic_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/basic_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/basic_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/basic_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/basic_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/generics_in_tuple_variant_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/named_tests/generics_independent_struct_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/named_tests/generics_independent_struct_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/named_tests/generics_independent_struct_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/generics_independent_tuple_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/generics_independent_tuple_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/generics_independent_tuple_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/named_tests/generics_shared_struct_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/named_tests/generics_shared_struct_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/named_tests/generics_shared_struct_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/generics_shared_tuple_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/generics_shared_tuple_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/generics_shared_tuple_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/scalar_generic_tuple_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/scalar_generic_tuple_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/scalar_generic_tuple_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/subform_collection_test.rs` -> `module/core/former/tests/inc/former_enum_tests/compile_fail/subform_collection_test.rs` (remains top-level compile_fail)
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_default_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_default_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_default_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_default_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_default_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_default_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_scalar_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_scalar_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_scalar_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_scalar_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_scalar_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_scalar_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_args_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_standalone_args_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_args_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_standalone_args_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_args_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_standalone_args_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_standalone_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_standalone_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/tuple_multi_standalone_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_zero_fields_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/unit_tests/tuple_zero_fields_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_zero_fields_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/unit_tests/tuple_zero_fields_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/tuple_zero_fields_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unit_tests/tuple_zero_fields_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/unit_variant_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/unit_tests/unit_variant_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/unit_variant_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/unit_tests/unit_variant_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/unit_variant_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unit_tests/unit_variant_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/usecase1_derive.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/usecase1_derive.rs`
            - `module/core/former/tests/inc/former_enum_tests/usecase1_manual.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/usecase1_manual.rs`
            - `module/core/former/tests/inc/former_enum_tests/usecase1_only_test.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/usecase1_only_test.rs`
            - `module/core/former/tests/inc/former_enum_tests/usecase1.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/usecase1.rs`
        *   **Files to Split and Move:**
            - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_derive.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/enum_named_fields_unit_derive.rs` (Unit variants)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/enum_named_fields_unnamed_derive.rs` (Zero-field Unnamed variants)
                - `module/core/former/tests/inc/former_enum_tests/named_tests/enum_named_fields_named_derive.rs` (Zero, One, Two Fields Named variants, InnerForSubform struct)
            - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_manual.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/enum_named_fields_unit_manual.rs` (Manual impls for Unit variants)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/enum_named_fields_unnamed_manual.rs` (Manual impls for Zero-field Unnamed variants)
                - `module/core/former/tests/inc/former_enum_tests/named_tests/enum_named_fields_named_manual.rs` (Manual impls for Zero, One, Two Fields Named variants, InnerForSubform struct, InnerForSubformFormer, FormingEnd for named variants)
            - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_only_test.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/enum_named_fields_unit_only_test.rs` (Unit Variant tests)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/enum_named_fields_unnamed_only_test.rs` (Zero Fields Unnamed tests)
                - `module/core/former/tests/inc/former_enum_tests/named_tests/enum_named_fields_named_only_test.rs` (Zero, One, Two Fields Named tests)
            - `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_derive.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/generics_in_tuple_variant_unit_derive.rs` (Unit variant)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/generics_in_tuple_variant_tuple_derive.rs` (Tuple variant with generics, InnerGeneric struct)
            - `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_manual.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/generics_in_tuple_variant_unit_manual.rs` (Manual impls for Unit variant)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/generics_in_tuple_variant_tuple_manual.rs` (Manual impls for Tuple variant with generics, InnerGeneric struct, related former infrastructure)
            - `module/core/former/tests/inc/former_enum_tests/keyword_variant_derive.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/keyword_variant_unit_derive.rs` (Unit variant r#Loop)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/keyword_variant_tuple_derive.rs` (Tuple variants, StringFormerStub, InnerData)
            - `module/core/former/tests/inc/former_enum_tests/keyword_variant_only_test.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/keyword_variant_unit_only_test.rs` (Test for r#Loop)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/keyword_variant_tuple_only_test.rs` (Tests for tuple variants)
            - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_derive.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_unit_derive.rs` (UnitVariant)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/standalone_constructor_tuple_derive.rs` (TupleVariant)
                - `module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_named_derive.rs` (StructVariant)
            - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_only_test.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_unit_only_test.rs` (unit_variant_test)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/standalone_constructor_tuple_only_test.rs` (tuple_variant_test)
                - `module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_named_only_test.rs` (struct_variant_test)
            - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_derive.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_args_unit_derive.rs` (UnitVariantArgs)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/standalone_constructor_args_tuple_derive.rs` (TupleVariantArgs, MultiTupleArgs)
                - `module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_args_named_derive.rs` (StructVariantArgs, MultiStructArgs)
            - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_manual.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_args_unit_manual.rs` (Manual impls for UnitVariantArgs)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/standalone_constructor_args_tuple_manual.rs` (Manual impls for TupleVariantArgs, MultiTupleArgs)
                - `module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_args_named_manual.rs` (Manual impls for StructVariantArgs, MultiStructArgs)
            - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs` ->
                - `module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_args_unit_only_test.rs` (unit_variant_args_test)
                - `module/core/former/tests/inc/former_enum_tests/unnamed_tests/standalone_constructor_args_tuple_only_test.rs` (tuple_variant_args_test, multi_tuple_variant_args_test)
                - `module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_args_named_only_test.rs` (struct_variant_args_test, multi_struct_variant_args_test)

        *   **Compile-Fail Files to Move:**
            - `module/core/former/tests/inc/former_enum_tests/compile_fail/struct_zero_default_error.rs` -> `module/core/former/tests/inc/former_enum_tests/named_tests/compile_fail/struct_zero_default_error.rs`
            - `module/core/former/tests/inc/former_enum_tests/compile_fail/struct_zero_subform_scalar_error.rs` -> `module/core/former/tests/inc/former_enum_tests/named_tests/compile_fail/struct_zero_subform_scalar_error.rs`
            - `module/core/former/tests/inc/former_enum_tests/compile_fail/tuple_multi_subform_scalar_error.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/compile_fail/tuple_multi_subform_scalar_error.rs`
            - `module/core/former/tests/inc/former_enum_tests/compile_fail/tuple_single_subform_non_former_error.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/compile_fail/tuple_single_subform_non_former_error.rs`
            - `module/core/former/tests/inc/former_enum_tests/compile_fail/tuple_zero_subform_scalar_error.rs` -> `module/core/former/tests/inc/former_enum_tests/unnamed_tests/compile_fail/tuple_zero_subform_scalar_error.rs`
            - `module/core/former/tests/inc/former_enum_tests/compile_fail/unit_subform_scalar_error.rs` -> `module/core/former/tests/inc/former_enum_tests/unit_tests/compile_fail/unit_subform_scalar_error.rs`
            - `module/core/former/tests/inc/former_enum_tests/compile_fail/subform_collection_test.rs` -> `module/core/former/tests/inc/former_enum_tests/compile_fail/subform_collection_test.rs` (remains top-level compile_fail)

    *   **Detailed Plan Step 3:** Create a clear mapping: `Original File Path -> New File Path(s)`. (Completed in analysis above)
    *   **Detailed Plan Step 4:** Plan the `mod.rs` structure: (Completed in analysis above)
        *   `former_enum_tests/mod.rs`: Will declare `pub mod unit_tests;`, `pub mod unnamed_tests;`, `pub mod named_tests;`. Add `pub mod compile_fail;`. Preserve existing module-level documentation (test matrices).
        *   `former_enum_tests/unit_tests/mod.rs`: Will declare commented-out `pub mod ...;` for all files moved/split into `unit_tests/`. Add `pub mod compile_fail;` (commented out).
        *   `former_enum_tests/unnamed_tests/mod.rs`: Will declare commented-out `pub mod ...;` for all files moved/split into `unnamed_tests/`. Add `pub mod compile_fail;` (commented out).
        *   `former_enum_tests/named_tests/mod.rs`: Will declare commented-out `pub mod ...;` for all files moved/split into `named_tests/`. Add `pub mod compile_fail;` (commented out).
        *   `module/core/former/tests/inc/former_enum_tests/unit_tests/compile_fail/mod.rs`: Create this file, initially empty.
        *   `module/core/former/tests/inc/former_enum_tests/unnamed_tests/compile_fail/mod.rs`: Create this file, initially empty.
        *   `module/core/former/tests/inc/former_enum_tests/named_tests/compile_fail/mod.rs`: Create this file, initially empty.
    *   **Verification Strategy:** User reviews the proposed file mapping, splitting strategy for mixed-aspect files, and the planned `mod.rs` structures.
    *   **Test Matrix:** N/A for this planning increment.
    *   **Commit Message:** `docs(former): Plan detailed restructuring of enum tests directory`

*   [✅] **Increment 2: Create Directory Structure and Top-Level `mod.rs`**
    *   **Goal:** Implement the directory hierarchy and the main `former_enum_tests/mod.rs`.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Create directories:
        *   `module/core/former/tests/inc/former_enum_tests/unit_tests/`
        *   `module/core/former/tests/inc/former_enum_tests/unnamed_tests/`
        *   `module/core/former/tests/inc/former_enum_tests/named_tests/`
        *   `module/core/former/tests/inc/former_enum_tests/unit_tests/compile_fail/`
        *   `module/core/former/tests/inc/former_enum_tests/unnamed_tests/compile_fail/`
        *   `module/core/former/tests/inc/former_enum_tests/named_tests/compile_fail/`
    *   **Detailed Plan Step 2:** Create empty `mod.rs` files in each new subdirectory:
        *   `unit_tests/mod.rs`
        *   `unnamed_tests/mod.rs`
        *   `named_tests/mod.rs`
        *   `unit_tests/compile_fail/mod.rs`
        *   `unnamed_tests/compile_fail/mod.rs`
        *   `named_tests/compile_fail/mod.rs`
    *   **Detailed Plan Step 3:** Modify `module/core/former/tests/inc/former_enum_tests/mod.rs`:
        *   Remove all old `mod individual_file;` declarations.
        *   Add `pub mod unit_tests;`, `pub mod unnamed_tests;`, `pub mod named_tests;`.
        *   Add `pub mod compile_fail;`.
        *   Preserve existing module-level documentation (test matrices).
    *   **Verification Strategy:** User applies changes. Run `cargo check --tests --package former`. Expect it to pass (many "file not found" errors for tests are expected from the parent `inc/mod.rs` if it still tries to mod them directly, or just passes if `inc/mod.rs` only mods `former_enum_tests`).
    *   **Commit Message:** `refactor(former): Create directory hierarchy for categorized enum tests`

*   [✅] **Increment 3: Process and Relocate/Split Unit Variant Test Files**
    *   **Goal:** Move or split-and-move files primarily testing unit variants into `unit_tests/` and update `unit_tests/mod.rs`.
    *   **Target Crate(s):** `former`
    *   **Pre-Analysis:** Based on the analysis in Increment 1, the following files are categorized as Unit variant tests and need to be moved or split and moved to `unit_tests/`:
        - `module/core/former/tests/inc/former_enum_tests/tuple_zero_fields_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_zero_fields_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_zero_fields_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/unit_variant_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/unit_variant_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/unit_variant_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_manual.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_only_test.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_manual.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/keyword_variant_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/keyword_variant_only_test.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_only_test.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_manual.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/compile_fail/unit_subform_scalar_error.rs` (Move to compile_fail subdirectory)
    *   **Crucial Design Rules:** Structuring: Organize by Feature or Layer, Structuring: Add Module Declaration Before Content, Structuring: Split Large Files Methodically (If Requested), Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks.
    *   **Relevant Behavior Rules:** Expected Enum Former Behavior Rules (referenced for context on test purpose).
    *   **Detailed Plan Step 1:** Move files that test *only* unit variants to `module/core/former/tests/inc/former_enum_tests/unit_tests/`. (Completed)
    *   **Detailed Plan Step 2:** For files that test *mixed* aspects and include unit variants, create new files in `unit_tests/` containing only the unit-variant-specific code and update `include!` directives. (Completed)
    *   **Detailed Plan Step 3:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs` to add (commented-out) `pub mod ...;` declarations for each new/moved file in this directory. (Completed)
    *   **Detailed Plan Step 4:** Move relevant compile-fail files identified for `unit_tests/compile_fail/` into that directory and update `unit_tests/compile_fail/mod.rs` (commented out). (Completed)
    *   **Verification Strategy:** User applies changes. Run `cargo check --tests --package former`. Fix any path issues in `use` statements or `include!` macros within the moved/split files.
    *   **Test Matrix:** N/A for this implementation increment. The Test Matrix is in the `mod.rs` file.
    *   **Commit Message:** `refactor(former): Relocate and split unit enum test files`

*   [✅] **Increment 4: Process and Relocate/Split Unnamed (Tuple) Variant Test Files**
    *   **Goal:** Move or split-and-move files primarily testing tuple variants into `unnamed_tests/` and update `unnamed_tests/mod.rs`.
    *   **Target Crate(s):** `former`
    *   **Pre-Analysis:** Based on the analysis in Increment 1, the following files are categorized as Tuple variant tests and need to be moved or split and moved to `unnamed_tests/`:
        - `module/core/former/tests/inc/former_enum_tests/basic_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/basic_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/basic_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_default_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_default_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_default_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_scalar_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_scalar_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_scalar_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_args_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_args_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_args_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/tuple_multi_standalone_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/usecase1_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/usecase1_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/usecase1_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/usecase1.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_manual.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_only_test.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_manual.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/keyword_variant_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/keyword_variant_only_test.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_only_test.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_manual.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/compile_fail/tuple_multi_subform_scalar_error.rs` (Move to compile_fail subdirectory)
        - `module/core/former/tests/inc/former_enum_tests/compile_fail/tuple_single_subform_non_former_error.rs` (Move to compile_fail subdirectory)
        - `module/core/former/tests/inc/former_enum_tests/compile_fail/tuple_zero_subform_scalar_error.rs` (Move to compile_fail subdirectory)
    *   **Crucial Design Rules:** Structuring: Organize by Feature or Layer, Structuring: Add Module Declaration Before Content, Structuring: Split Large Files Methodically (If Requested), Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks.
    *   **Relevant Behavior Rules:** Expected Enum Former Behavior Rules (referenced for context on test purpose).
    *   **Detailed Plan Step 1:** Move files that test *only* tuple variants to `module/core/former/tests/inc/former_enum_tests/unnamed_tests/`. (Completed)
    *   **Detailed Plan Step 2:** For files that test *mixed* aspects and include tuple variants, create new files in `unnamed_tests/` containing only the tuple-variant-specific code and update `include!` directives. (Completed)
    *   **Detailed Plan Step 3:** Modify `module/core/former/tests/inc/former_enum_tests/unnamed_tests/mod.rs` to add (commented-out) `pub mod ...;` declarations for each new/moved file in this directory. (Completed)
    *   **Detailed Plan Step 4:** Move relevant compile-fail files identified for `unnamed_tests/compile_fail/` into that directory and update `unnamed_tests/compile_fail/mod.rs` (commented out). (Completed)
    *   **Verification Strategy:** User applies changes. Run `cargo check --tests --package former`. Fix any path issues in `use` statements or `include!` macros within the moved/split files.
    *   **Test Matrix:** N/A for this implementation increment. The Test Matrix is in the `mod.rs` file.
    *   **Commit Message:** `refactor(former): Relocate and split unnamed (tuple) enum test files`

*   [✅] **Increment 5: Process and Relocate/Split Named (Struct-like) Variant Test Files**
    *   **Goal:** Move or split-and-move files primarily testing named variants into `named_tests/` and update `named_tests/mod.rs`.
    *   **Target Crate(s):** `former`
    *   **Pre-Analysis:** Based on the analysis in Increment 1, the following files are categorized as Named variant tests and need to be moved or split and moved to `named_tests/`:
        - `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_derive.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_manual.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_only_test.rs` (Move)
        - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_manual.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/enum_named_fields_only_test.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_only_test.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_derive.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_manual.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs` (Split)
        - `module/core/former/tests/inc/former_enum_tests/compile_fail/struct_zero_default_error.rs` (Move to compile_fail subdirectory)
        - `module/core/former/tests/inc/former_enum_tests/compile_fail/struct_zero_subform_scalar_error.rs` (Move to compile_fail subdirectory)
    *   **Crucial Design Rules:** Structuring: Organize by Feature or Layer, Structuring: Add Module Declaration Before Content, Structuring: Split Large Files Methodically (If Requested), Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks.
    *   **Relevant Behavior Rules:** Expected Enum Former Behavior Rules (referenced for context on test purpose).
    *   **Detailed Plan Step 1:** Move files that test *only* named variants to `module/core/former/tests/inc/former_enum_tests/named_tests/`. (Completed)
    *   **Detailed Plan Step 2:** For files that test *mixed* aspects and include named variants, create new files in `named_tests/` containing only the named-variant-specific code and update `include!` directives. (Completed)
    *   **Detailed Plan Step 3:** Modify `module/core/former/tests/inc/former_enum_tests/named_tests/mod.rs` to add (commented-out) `pub mod ...;` declarations for each new/moved file in this directory. (Completed)
    *   **Detailed Plan Step 4:** Move relevant compile-fail files identified for `named_tests/compile_fail/` into that directory and update `named_tests/compile_fail/mod.rs` (commented out). (Completed)
    *   **Verification Strategy:** User applies changes. Run `cargo check --tests --package former`. Fix any path issues in `use` statements or `include!` macros within the moved/split files.
    *   **Test Matrix:** N/A for this implementation increment. The Test Matrix is in the `mod.rs` file.
    *   **Commit Message:** `refactor(former): Relocate and split named (struct) enum test files`

*   [✅] **Increment 6: Final Cleanup and Verification of Structure**
    *   **Goal:** Ensure the main `former_enum_tests/mod.rs` is clean, all original files from `former_enum_tests/` have been either moved, split and moved, or intentionally deleted (if their content was fully redistributed). Verify the overall project still compiles.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Review `module/core/former/tests/inc/former_enum_tests/`. Ensure no old test files remain directly in this directory (unless it's a top-level `compile_fail/mod.rs` or other non-variant-specific files). (Completed)
    *   **Detailed Plan Step 2:** Review `module/core/former/tests/inc/former_enum_tests/mod.rs` to ensure it only contains `pub mod unit_tests; pub mod unnamed_tests; pub mod named_tests;` and `pub mod compile_fail;` and necessary `use` statements/documentation. (Completed)
    *   **Detailed Plan Step 3:** Run `cargo check --tests --package former`. Address any remaining path or module system errors. The goal here is successful compilation of the new structure, not necessarily passing all tests (as most test `mod` declarations inside subdirectories are still commented out). (Completed)
    *   **Verification Strategy:** `cargo check --tests --package former` passes. Manual review confirms no test files/logic were lost and categorization is correct.
    *   **Commit Message:** `refactor(former): Finalize restructuring of enum tests directory`

### Requirements
*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules.
*   **Focus:** This plan is *only* for restructuring files. No test logic changes or uncommenting of actual tests within the files being moved, unless necessary to fix paths after moving.
*   **Preserve Docs & Comments:** Existing documentation in `former_enum_tests/mod.rs` (like test matrices) should be preserved. All test code, including currently commented-out tests, must be preserved in its new location(s).
*   **File Splitting:** Files testing multiple variant types *must* be split.
*   **Incremental Verification:** Verify `cargo check` after each major step.
*   **Approval Gates:** Obtain user approval before starting each increment and after successful verification.

## Notes & Insights
*   This plan focuses on a structural refactoring to improve organization before tackling test logic.
*   The splitting of mixed-aspect files is a key part of this refactoring.
*   The actual uncommenting and fixing of tests within these new subdirectories will be handled by subsequent, focused plans.
*   The `mod.rs` files within the new subdirectories (`unit_tests/mod.rs`, etc.) will initially have their `mod` declarations for individual test files commented out.
