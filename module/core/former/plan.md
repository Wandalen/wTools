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

*   [⚫] **Increment 1: Detailed File Analysis and Relocation/Splitting Plan**
    *   **Goal:** Analyze each file in `former_enum_tests/`, determine its new location(s), and plan any necessary splits if a file covers multiple variant types. Plan `mod.rs` updates.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** List all files and subdirectories currently in `module/core/former/tests/inc/former_enum_tests/`.
    *   **Detailed Plan Step 2 (Categorization & Splitting Strategy):** For each file:
        *   Analyze its content (including any commented-out tests) to identify the primary enum variant type(s) it tests (Unit, Tuple/Unnamed, Named/Struct-like).
        *   **If a file tests ONLY ONE variant type:** Assign it to the corresponding subdirectory (`unit_tests/`, `unnamed_tests/`, `named_tests/`).
        *   **If a file tests MULTIPLE variant types (e.g., `keyword_variant_*.rs` might test unit, tuple, and named variants):**
            *   Propose splitting the file into multiple new files. Each new file should contain only the tests and related helper code (like inner struct definitions if they are specific) for a *single* variant type.
            *   Name the new files descriptively (e.g., `keyword_variant_unit_derive.rs`, `keyword_variant_tuple_derive.rs`, `keyword_variant_named_derive.rs`).
            *   Assign each new split file to its corresponding subdirectory.
            *   If an `_only_test.rs` file is shared by a file that needs splitting, that `_only_test.rs` file might also need to be split, or its `include!` directives adjusted in the newly split `_derive.rs` / `_manual.rs` files to only include relevant test functions. This requires careful analysis.
        *   **`compile_fail/` directory:** Analyze each test within `compile_fail/`.
            *   If a compile-fail test is specific to a variant type, plan to move it into a `compile_fail/` subdirectory within the respective variant type directory (e.g., `unnamed_tests/compile_fail/tuple_zero_error.rs`).
            *   If a compile-fail test is generic or hard to categorize, it can remain in a top-level `former_enum_tests/compile_fail/` directory.
    *   **Detailed Plan Step 3:** Create a clear mapping: `Original File Path -> New File Path(s)`.
    *   **Detailed Plan Step 4:** Plan the `mod.rs` structure:
        *   `former_enum_tests/mod.rs`: Will declare `pub mod unit_tests;`, `pub mod unnamed_tests;`, `pub mod named_tests;` (and potentially `pub mod compile_fail;` if a top-level one is kept).
        *   `former_enum_tests/unit_tests/mod.rs`: Will declare `pub mod ...;` for all files moved/split into `unit_tests/`.
        *   `former_enum_tests/unnamed_tests/mod.rs`: Will declare `pub mod ...;` for all files moved/split into `unnamed_tests/` (and `pub mod compile_fail;` if applicable).
        *   `former_enum_tests/named_tests/mod.rs`: Will declare `pub mod ...;` for all files moved/split into `named_tests/` (and `pub mod compile_fail;` if applicable).
        *   All `mod` declarations for individual test files within these new `mod.rs` files will initially be **commented out**. They will be uncommented by subsequent plans.
    *   **Verification Strategy:** User reviews the proposed file mapping, splitting strategy for mixed-aspect files, and the planned `mod.rs` structures.
    *   **Commit Message:** `docs(former): Plan detailed restructuring of enum tests directory`

*   [⚫] **Increment 2: Create Directory Structure and Top-Level `mod.rs`**
    *   **Goal:** Implement the directory hierarchy and the main `former_enum_tests/mod.rs`.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Create directories:
        *   `module/core/former/tests/inc/former_enum_tests/unit_tests/`
        *   `module/core/former/tests/inc/former_enum_tests/unnamed_tests/`
        *   `module/core/former/tests/inc/former_enum_tests/named_tests/`
        *   If planned: `module/core/former/tests/inc/former_enum_tests/compile_fail/` (top-level)
        *   If planned: `module/core/former/tests/inc/former_enum_tests/unit_tests/compile_fail/`
        *   If planned: `module/core/former/tests/inc/former_enum_tests/unnamed_tests/compile_fail/`
        *   If planned: `module/core/former/tests/inc/former_enum_tests/named_tests/compile_fail/`
    *   **Detailed Plan Step 2:** Create empty `mod.rs` files in each new subdirectory:
        *   `unit_tests/mod.rs`
        *   `unnamed_tests/mod.rs`
        *   `named_tests/mod.rs`
        *   And in their `compile_fail` subdirectories if created.
    *   **Detailed Plan Step 3:** Modify `module/core/former/tests/inc/former_enum_tests/mod.rs`:
        *   Remove all old `mod individual_file;` declarations.
        *   Add `pub mod unit_tests;`, `pub mod unnamed_tests;`, `pub mod named_tests;`.
        *   Add `pub mod compile_fail;` if a top-level one is kept.
        *   Preserve existing module-level documentation (test matrices).
    *   **Verification Strategy:** User applies changes. Run `cargo check --tests --package former`. Expect it to pass (many "file not found" errors for tests are expected from the parent `inc/mod.rs` if it still tries to mod them directly, or just passes if `inc/mod.rs` only mods `former_enum_tests`).
    *   **Commit Message:** `refactor(former): Create directory hierarchy for categorized enum tests`

*   [⚫] **Increment 3: Process and Relocate/Split Unit Variant Test Files**
    *   **Goal:** Move or split-and-move files primarily testing unit variants into `unit_tests/` and update `unit_tests/mod.rs`.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** For each file identified in Increment 1 as belonging (entirely or partially) to unit tests:
        *   If the file *only* tests unit variants (e.g., `unit_variant_derive.rs`): Move it directly to `module/core/former/tests/inc/former_enum_tests/unit_tests/`.
        *   If the file tests *mixed* aspects:
            *   Create a new file in `unit_tests/` (e.g., `keyword_variant_unit_derive.rs`).
            *   Copy only the unit-variant-specific test functions, helper code, and `include!` directives (if the included file can be partially included or is also split) into this new file.
            *   Leave the original file in place for now; its remaining parts will be processed in later increments.
    *   **Detailed Plan Step 2:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs` to add (commented-out) `pub mod ...;` declarations for each new/moved file in this directory.
    *   **Verification Strategy:** User applies changes. Run `cargo check --tests --package former`. Fix any path issues in `use` statements or `include!` macros within the moved/split files.
    *   **Commit Message:** `refactor(former): Relocate and split unit enum test files`

*   [⚫] **Increment 4: Process and Relocate/Split Unnamed (Tuple) Variant Test Files**
    *   **Goal:** Move or split-and-move files primarily testing tuple variants into `unnamed_tests/` and update `unnamed_tests/mod.rs`.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** For each file identified in Increment 1 as belonging (entirely or partially) to tuple tests:
        *   If the file *only* tests tuple variants (e.g., `basic_derive.rs`): Move it directly to `module/core/former/tests/inc/former_enum_tests/unnamed_tests/`.
        *   If the file tests *mixed* aspects (and wasn't fully processed in Increment 3):
            *   Create a new file in `unnamed_tests/` (e.g., `keyword_variant_tuple_derive.rs`).
            *   Copy only the tuple-variant-specific test functions, etc., into this new file.
            *   If this step empties the original mixed-aspect file of its tuple content, the original might be deleted if its other aspects were also processed.
        *   Move relevant `compile_fail/tuple_*.rs` files into `unnamed_tests/compile_fail/` and update `unnamed_tests/compile_fail/mod.rs` (commented out).
    *   **Detailed Plan Step 2:** Modify `module/core/former/tests/inc/former_enum_tests/unnamed_tests/mod.rs` to add (commented-out) `pub mod ...;` declarations for new/moved files and `pub mod compile_fail;` if applicable.
    *   **Verification Strategy:** User applies changes. Run `cargo check --tests --package former`. Fix paths.
    *   **Commit Message:** `refactor(former): Relocate and split unnamed (tuple) enum test files`

*   [⚫] **Increment 5: Process and Relocate/Split Named (Struct-like) Variant Test Files**
    *   **Goal:** Move or split-and-move files primarily testing named variants into `named_tests/` and update `named_tests/mod.rs`.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** For each file identified in Increment 1 as belonging (entirely or partially) to named tests:
        *   If the file *only* tests named variants (e.g., `enum_named_fields_derive.rs` after tuple parts were potentially moved out): Move it to `module/core/former/tests/inc/former_enum_tests/named_tests/`.
        *   If the file tests *mixed* aspects (and wasn't fully processed):
            *   Create a new file in `named_tests/` (e.g., `keyword_variant_named_derive.rs`).
            *   Copy only the named-variant-specific content.
            *   If this step processes the last remaining part of an original mixed-aspect file, that original file can now be deleted.
        *   Move relevant `compile_fail/*struct*.rs` files into `named_tests/compile_fail/` and update `named_tests/compile_fail/mod.rs` (commented out).
    *   **Detailed Plan Step 2:** Modify `module/core/former/tests/inc/former_enum_tests/named_tests/mod.rs` to add (commented-out) `pub mod ...;` declarations and `pub mod compile_fail;` if applicable.
    *   **Verification Strategy:** User applies changes. Run `cargo check --tests --package former`. Fix paths.
    *   **Commit Message:** `refactor(former): Relocate and split named (struct) enum test files`

*   [⚫] **Increment 6: Final Cleanup and Verification of Structure**
    *   **Goal:** Ensure the main `former_enum_tests/mod.rs` is clean, all original files from `former_enum_tests/` have been either moved, split and moved, or intentionally deleted (if their content was fully redistributed). Verify the overall project still compiles.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Review `module/core/former/tests/inc/former_enum_tests/`. Ensure no old test files remain directly in this directory (unless it's a top-level `compile_fail/mod.rs` or other non-variant-specific files).
    *   **Detailed Plan Step 2:** Review `module/core/former/tests/inc/former_enum_tests/mod.rs` to ensure it only contains `pub mod unit_tests; pub mod unnamed_tests; pub mod named_tests;` (and potentially `pub mod compile_fail;`) and necessary `use` statements/documentation.
    *   **Detailed Plan Step 3:** Run `cargo check --tests --package former`. Address any remaining path or module system errors. The goal here is successful compilation of the new structure, not necessarily passing all tests (as most test `mod` declarations inside subdirectories are still commented out).
    *   **Verification Strategy:** `cargo check` passes. Manual review confirms no test files/logic were lost and categorization is correct.
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
