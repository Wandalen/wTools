# Project Plan: Audit and Finalize Single-Aspect Focus for Enum Tests

## Goal
*   Audit all test files within the `module/core/former/tests/inc/` subdirectories (`enum_unit_tests/`, `enum_unnamed_tests/`, `enum_named_tests/`, `enum_complex_tests/`, and their respective `compile_fail/` subdirectories if they exist).
*   Verify that each test file (`_derive.rs`, `_manual.rs`, `_only_test.rs`, or standalone `.rs`) within `enum_unit_tests/`, `enum_unnamed_tests/`, and `enum_named_tests/` strictly focuses on a single enum variant aspect: Unit, Unnamed (tuple), or Named (struct-like) variants, respectively.
*   If any file is found to still cover multiple aspects (an oversight from the previous restructuring), it **must be split** into separate files. Each new file will be dedicated to a single aspect and placed in (or moved to) the correct subdirectory (`enum_unit_tests/`, `enum_unnamed_tests/`, or `enum_named_tests/`).
*   Files within the `enum_complex_tests/` directory will be reviewed. If they can be reasonably refactored to fit into the single-aspect categories, a plan for that will be proposed and executed. Otherwise, they will remain in `enum_complex_tests/`.
*   Update `mod.rs` files within each subdirectory accurately. Module declarations for individual test files will remain **commented out**.
*   Ensure the `former` package compiles without errors or warnings after refactoring (`cargo check --package former --tests`).
*   Ensure `cargo test --package former --test tests` passes (acknowledging that specific enum tests within the refactored area will not run due to commented-out module declarations).
*   Preserve all existing test logic. If a test file, after moving/splitting, causes a persistent compilation error (not related to paths), the specific failing test function or its module declaration will be commented out to allow structural verification to proceed.

## Relevant Context

**Important:** Before starting implementation, thoroughly review the `Readme.md` and `advanced.md` files for the `former` crate, and the `Readme.md` for `former_meta` to ensure a full understanding of the existing design, features, and intended behaviors.

*   **Primary Directories to Audit (Actual Location):**
    *   `module/core/former/tests/inc/enum_unit_tests/`
    *   `module/core/former/tests/inc/enum_unnamed_tests/`
    *   `module/core/former/tests/inc/enum_named_tests/`
    *   `module/core/former/tests/inc/enum_complex_tests/`
    *   Respective `compile_fail/` subdirectories within each of the above (e.g., `enum_unit_tests/compile_fail/`).
*   **Module Files to Update:**
    *   `module/core/former/tests/inc/enum_unit_tests/mod.rs`
    *   `module/core/former/tests/inc/enum_unnamed_tests/mod.rs`
    *   `module/core/former/tests/inc/enum_named_tests/mod.rs`
    *   `module/core/former/tests/inc/enum_complex_tests/mod.rs`
    *   `module/core/former/tests/inc/mod.rs` (for top-level submodule declarations)
    *   (And `mod.rs` files within `compile_fail` subdirectories if applicable)
*   **Core Crate Files (for context on macro behavior):**
    *   `module/core/former/src/lib.rs`
    *   `module/core/former_meta/src/lib.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (and its submodules like `unit_variant_handler.rs`, etc.)
    *   `module/core/former_types/src/lib.rs`
*   **Documentation (for context on features and attributes):**
    *   `module/core/former/Readme.md`
    *   `module/core/former/advanced.md`
    *   `module/core/former_meta/Readme.md`
*   **Assumption:** The previous plan (restructuring `former_enum_tests` into `unit_tests/`, `unnamed_tests/`, `named_tests/`, and `complex_tests/` subdirectories) was intended to create the directories `enum_unit_tests/`, `enum_unnamed_tests/`, `enum_named_tests/`, and `enum_complex_tests/` directly under `tests/inc/`.

## Expected Enum Former Behavior

This plan adheres to the following rules for `#[derive(Former)]` on enums:

1.  **`#[scalar]` Attribute:**
    *   **Unit Variant:** Generates `Enum::variant() -> Enum`. (Handled by: `handle_unit_variant`)
    *   **Zero-Field Variant (Tuple):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_tuple_zero_variant`)
    *   **Zero-Field Variant (Struct):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_struct_zero_variant`)
    *   **Single-Field Variant (Tuple):** Generates `Enum::variant(InnerType) -> Enum`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Single-Field Variant (Struct):** Generates `Enum::variant { field: InnerType } -> Enum`. (Handled by: `handle_struct_non_zero_variant`)
    *   **Multi-Field Variant (Tuple):** Generates `Enum::variant(T1, T2, ...) -> Enum`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Multi-Field Variant (Struct):** Generates `Enum::variant { f1: T1, f2: T2, ... } -> Enum`. (Handled by: `handle_struct_non_zero_variant`)
    *   **Error Cases:** Cannot be combined with `#[subform_scalar]`.

2.  **`#[subform_scalar]` Attribute:**
    *   **Unit Variant:** Error. (Checked in: `handle_unit_variant`)
    *   **Zero-Field Variant (Tuple or Struct):** Error. (Checked in: `handle_tuple_zero_variant`, `handle_struct_zero_variant`)
    *   **Single-Field Variant (Tuple):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Single-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
    *   **Multi-Field Variant (Tuple):** Error. Cannot use `subform_scalar` on multi-field tuple variants. (Checked in: `handle_tuple_non_zero_variant`)
    *   **Multi-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)

3.  **Default Behavior (No Attribute):**
    *   **Unit Variant:** Generates `Enum::variant() -> Enum`. (Handled by: `handle_unit_variant`)
    *   **Zero-Field Variant (Tuple):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_tuple_zero_variant`)
    *   **Zero-Field Variant (Struct):** Error. Requires `#[scalar]`. (Checked in: `handle_struct_zero_variant`)
    *   **Single-Field Variant (Tuple):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Single-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
    *   **Multi-Field Variant (Tuple):** Generates `Enum::variant(Field1Type, Field2Type, ...) -> Enum` (behaves like `#[scalar]`). (Handled by: `handle_tuple_non_zero_variant`)
    *   **Multi-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)

4.  **`#[standalone_constructors]` Attribute (Body Level):**
    *   Generates top-level constructor functions for each variant (e.g., `my_variant()`).
    *   Return type depends on `#[arg_for_constructor]` on fields within the variant (see Option 2 logic in Readme/advanced.md).

## Increments

*   [✅] **Increment 1: Audit Plan for Single-Aspect Focus**
    *   **Goal:** For each test file in its *current* subdirectory (`unit_tests`, `unnamed_tests`, `named_tests`, `complex_tests`, and their `compile_fail` subdirs), verify if it truly adheres to a single aspect. Plan splits for any multi-aspect files.
    *   **Target Crate(s)::** `former` (planning only)
    *   **Detailed Plan Step 1 (List Current Structure):** List all files within each subdirectory of `module/core/former/tests/inc/former_enum_tests/`.
    *   **Detailed Plan Step 2 (Audit and Splitting Strategy):**
        *   **For `unit_tests/`:** Review each file. If it contains non-unit variant tests, plan to move those parts to new files in `../unnamed_tests/` or `../named_tests/`. The file in `unit_tests/` must be reduced to only unit-specific content.
        *   **For `unnamed_tests/`:** Review each file. If it contains unit or named variant tests, plan to move those parts to new files in `../unit_tests/` or `../named_tests/`. The file in `unnamed_tests/` must be reduced to only tuple-specific content.
        *   **For `named_tests/`:** Review each file. If it contains unit or tuple variant tests, plan to move those parts to new files in `../unit_tests/` or `../unnamed_tests/`. The file in `named_tests/` must be reduced to only named-specific content.
        *   **For `complex_tests/`:** Review each file. If a test can be clearly refactored into a single aspect (unit, unnamed, named) without losing its core testing purpose, plan to split/move it. If it genuinely tests complex_tests interactions not fitting a single category, it remains.
        *   **For `compile_fail/` subdirectories:** Ensure tests within (e.g., `unnamed_tests/compile_fail/`) are specific to that aspect. If not, plan to move them.
        *   **Shared `_only_test.rs` files:** If an `_only_test.rs` file serves a `_derive.rs` or `_manual.rs` file that is being split, the `_only_test.rs` file must also be split, or its `include!` directives in the newly split consumer files must be carefully adjusted to only pull relevant test functions.
    *   **Detailed Plan Step 3 (Output):** Present a list of files to be split, detailing how they will be split and where the new resulting files will be located. List files that are confirmed to be single-aspect and correctly located.
    *   **Verification Strategy:** User reviews the audit results and the proposed splitting/relocation plan.
    *   **Commit Message:** `docs(former): Plan for single-aspect audit and refinement of enum tests`
    *   **Notes:** Completed audit of `unit_tests/`, `unnamed_tests/`, `named_tests/`, and `complex_tests/` within the *expected* `former_enum_tests/` subdirectory. Found that all these directories are currently empty. The test files expected to be in these directories are likely located elsewhere. Found actual enum test files in `tests/inc/enum_unit_tests/`, `tests/inc/enum_unnamed_tests/`, `tests/inc/enum_named_tests/`, and `tests/inc/enum_complex_tests/`. The subsequent increments will be revised to operate on these actual directories.

*   [✅] **Increment 2: Audit and Plan Splits/Moves for Enum Test Files in Actual Directories**
    *   **Goal:** For each test file in its *current* subdirectory (`enum_unit_tests`, `enum_unnamed_tests`, `enum_named_tests`, `enum_complex_tests`, and their `compile_fail` subdirs), verify if it truly adheres to a single aspect. Plan splits for any multi-aspect files and plan moves for files in the wrong category directory.
    *   **Target Crate(s)::** `former` (planning only)
    *   **Detailed Plan Step 1 (List Current Structure):** (Already completed in previous steps, found files in `enum_unit_tests/`, `enum_unnamed_tests/`, `enum_named_tests/`, `enum_complex_tests/`).
    *   **Detailed Plan Step 2 (Audit and Splitting/Moving Strategy):**
        *   **For `enum_unit_tests/`:** Review each file. If it contains non-unit variant tests, plan to move those parts to new files in `../enum_unnamed_tests/` or `../enum_named_tests/`. The file in `enum_unit_tests/` must be reduced to only unit-specific content.
        *   **For `enum_unnamed_tests/`:** Review each file. If it contains unit or named variant tests, plan to move those parts to new files in `../unit_tests/` or `../named_tests/`. The file in `enum_unnamed_tests/` must be reduced to only tuple-specific content.
        *   **For `named_tests/`:** Review each file. If it contains unit or tuple variant tests, plan to move those parts to new files in `../unit_tests/` or `../enum_unnamed_tests/`. The file in `named_tests/` must be reduced to only named-specific content.
        *   **For `complex_tests/`:** Review each file. If a test can be clearly refactored into a single aspect (unit, unnamed, named) without losing its core testing purpose, plan to split/move it. If it genuinely tests complex_tests interactions not fitting a single category, it remains.
        *   **For `compile_fail/` subdirectories:** Ensure tests within (e.g., `unnamed_tests/compile_fail/`) are specific to that aspect. If not, plan to move them.
        *   **Shared `_only_test.rs` files:** If an `_only_test.rs` file serves a `_derive.rs` or `_manual.rs` file that is being split, the `_only_test.rs` file must also be split, or its `include!` directives in the newly split consumer files must be carefully adjusted to only pull relevant test functions.
    *   **Detailed Plan Step 3 (Output):** Present a list of files to be split, detailing how they will be split and where the new resulting files will be located. List files that are confirmed to be single-aspect and correctly located.
    *   **Verification Strategy:** User reviews the audit results and the proposed splitting/relocation plan.
    *   **Commit Message:** `docs(former): Audit and plan splits/moves for enum tests based on actual structure`
    *   **Notes:** Completed audit of all enum test directories. Identified files that need moving or splitting/cleanup to ensure single-aspect focus. Noted files needing correction or refinement in later increments.

*   [✅] **Increment 3: Execute Moves for Files in Incorrect Directories**
    *   **Goal:** Move the files identified in Increment 2 that are in the wrong single-aspect directory to their correct location.
    *   **Target Crate(s)::** `former`
    *   **Detailed Plan Step 1:** Move `module/core/former/tests/inc/enum_unit_tests/tuple_zero_fields_derive.rs` to `module/core/former/tests/inc/enum_unnamed_tests/tuple_zero_fields_derive.rs`.
    *   **Detailed Plan Step 2:** Move `module/core/former/tests/inc/enum_unit_tests/tuple_zero_fields_manual.rs` to `module/core/former/tests/inc/enum_unnamed_tests/tuple_zero_fields_manual.rs`.
    *   **Detailed Plan Step 3:** Move `module/core/former/tests/inc/enum_unit_tests/tuple_zero_fields_only_test.rs` to `module/core/former/tests/inc/enum_unnamed_tests/tuple_zero_fields_only_test.rs`.
    *   **Verification Strategy:** User applies changes. Run `cargo check --package former --tests`. Fix path issues.
    *   **Commit Message:** `refactor(former): Move tuple_zero_fields tests to enum_unnamed_tests`
    *   **Notes:** Successfully moved `tuple_zero_fields` files to `enum_unnamed_tests/`. `cargo check --package former --tests` passed with warnings. Increment 3 is complete.

*   [✅] **Increment 4: Execute Splits and Cleanups**
    *   **Goal:** Split the manual test files identified in Increment 2 that cover multiple scenarios and clean up leftover code.
    *   **Target Crate(s)::** `former`
    *   **Detailed Plan Step 1:** Create `module/core/former/tests/inc/enum_unnamed_tests/standalone_constructor_args_tuple_single_manual.rs` with content from `standalone_constructor_args_tuple_manual.rs` relevant to `TupleVariantArgs(i32)`, removing leftover code.
    *   **Detailed Plan Step 2:** Create `module/core/former/tests/inc/enum_unnamed_tests/standalone_constructor_args_tuple_multi_manual.rs` with content from `standalone_constructor_args_tuple_manual.rs` relevant to `MultiTupleArgs(i32, bool)`, removing leftover code.
    *   **Detailed Plan Step 3:** Delete the original `module/core/former/tests/inc/enum_unnamed_tests/standalone_constructor_args_tuple_manual.rs`.
    *   **Detailed Plan Step 4:** Create `module/core/former/tests/inc/enum_named_tests/standalone_constructor_args_named_single_manual.rs` with content from `standalone_constructor_args_named_manual.rs` relevant to `StructVariantArgs { field: String }`, removing leftover code.
    *   **Detailed Plan Step 5:** Create `module/core/former/tests/inc/enum_named_tests/standalone_constructor_args_named_multi_manual.rs` with content from `standalone_constructor_args_named_manual.rs` relevant to `MultiStructArgs { a: i32, b: bool }`, removing leftover code.
    *   **Detailed Plan Step 6:** Delete the original `module/core/former/tests/inc/enum_named_tests/standalone_constructor_args_named_manual.rs`.
    *   **Verification Strategy:** User applies changes. Run `cargo check --package former --tests`. Fix path issues.
    *   **Commit Message:** `refactor(former): Split and cleanup standalone_constructor_args manual tests`
    *   **Notes:** Successfully split and cleaned up `standalone_constructor_args_tuple_manual.rs` and `standalone_constructor_args_named_manual.rs`. Deleted the original files. `cargo check --package former --tests` passed with warnings. Increment 4 is complete. Changes were manually committed due to a git issue.

*   [⚫] **Increment 5: Update `mod.rs` Files**
    *   **Goal:** Update the `mod.rs` files in the enum test directories to reflect the file moves and splits, and remove incorrect tuple variant references in `former_trybuild`.
    *   **Target Crate(s)::** `former`
    *   **Detailed Plan Step 1:** Update `module/core/former/tests/inc/enum_unnamed_tests/mod.rs` to include the moved `tuple_zero_fields` files and the new split `standalone_constructor_args_tuple` manual files (commented out).
    *   **Detailed Plan Step 2:** Update `module/core/former/tests/inc/enum_named_tests/mod.rs` to include the new split `standalone_constructor_args_named` manual files (commented out).
    *   **Detailed Plan Step 3:** Update `module/core/former/tests/inc/enum_unit_tests/compile_fail/mod.rs` to remove tuple variant references in `former_trybuild`.
    *   **Detailed Plan Step 4:** Update `module/core/former/tests/inc/enum_unnamed_tests/compile_fail/mod.rs` to remove tuple variant references in `former_trybuild`.
    *   **Detailed Plan Step 5:** Update `module/core/former/tests/inc/enum_named_tests/compile_fail/mod.rs` to remove tuple variant references in `former_trybuild`.
    *   **Detailed Plan Step 6:** Update `module/core/former/tests/inc/enum_complex_tests/mod.rs` to remove tuple variant references in `former_trybuild`.
    *   **Verification Strategy:** User applies changes. Run `cargo check --package former --tests`. Fix path issues.
    *   **Commit Message:** `refactor(former): Update enum test mod.rs files after restructuring`

*   [⚫] **Increment 6: Address Incorrect Manual Implementation**
    *   **Goal:** Correct or remove the incorrectly implemented manual test file `usecase1_manual.rs`.
    *   **Target Crate(s)::** `former`
    *   **Detailed Plan Step 1:** Review `module/core/former/tests/inc/enum_unnamed_tests/usecase1_manual.rs`. Determine if a manual implementation for this use case is necessary.
    *   **Detailed Plan Step 2:** If necessary, replace the derive macro implementation with a correct manual implementation. If not necessary, delete the file.
    *   **Detailed Plan Step 3:** If the file is deleted, update `module/core/former/tests/inc/enum_unnamed_tests/mod.rs` to remove its module declaration.
    *   **Verification Strategy:** User applies changes. Run `cargo check --package former --tests`. Fix path issues.
    *   **Commit Message:** `refactor(former): Correct or remove usecase1_manual test file`

*   [⚫] **Increment 7: Final Structural Verification and Cleanup**
    *   **Goal:** Ensure all enum test files are correctly categorized with single-aspect focus, splits are complete, module structure is sound, and the `former` package compiles without errors or warnings.
    *   **Target Crate(s)::** `former`
    *   **Detailed Plan Step 1:** Review all subdirectories (`enum_unit_tests/`, `enum_unnamed_tests/`, `enum_named_tests/`, `enum_complex_tests/`) to confirm single-aspect focus per file (except for `enum_complex_tests/` which may retain multi-aspect tests if deemed necessary).
    *   **Detailed Plan Step 2:** Review all `mod.rs` files in the `tests/inc/` hierarchy relevant to enum tests for correctness.
    *   **Detailed Plan Step 3:** Run `cargo check --package former --tests`. Address any compilation errors or warnings.
    *   **Detailed Plan Step 4:** Run `cargo test --package former --test tests`. This should pass as no specific enum tests from the refactored area are actively run (their `mod` declarations in subdirectory `mod.rs` files are still commented).
    *   **Verification Strategy:** `cargo check --package former --tests` passes with no errors/warnings. `cargo test --package former --test tests` passes. Manual review confirms structural integrity, single-aspect focus, and no loss of test logic.
    *   **Commit Message:** `refactor(former): Complete single-aspect audit and restructuring of enum tests (incl. enum_complex_tests)`

### Requirements
*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules.
*   **Single Aspect Focus:** Each test file within `enum_unit_tests/`, `enum_unnamed_tests/`, `enum_named_tests/` must focus on one aspect. Files covering multiple aspects must be split and/or moved. Files in `enum_complex_tests/` should be confirmed as genuinely complex_tests or refactored.
*   **Preserve Logic:** All existing test code (including commented-out tests) must be preserved. If a test causes persistent compilation errors after moving/splitting (not path-related), its specific test function or its `mod` declaration in the subdirectory `mod.rs` should be commented out.
*   **Module Declarations:** All `mod` declarations for individual test files within `enum_unit_tests/mod.rs`, `enum_unnamed_tests/mod.rs`, `enum_named_tests/mod.rs`, and `enum_complex_tests/mod.rs` should remain **commented out**.
*   **Incremental Verification:** `cargo check --package former --tests` should pass after each increment.
*   **Approval Gates:** Obtain user approval for plans and after each increment.

## Notes & Insights
*   This plan is revised based on the actual location of enum test files found in `tests/inc/enum_unit_tests/`, `tests/inc/enum_unnamed_tests/`, `tests/inc/enum_named_tests/`, and `tests/inc/enum_complex_tests/`.
*   The primary focus is ensuring each categorized test file *now* strictly adheres to a single aspect.
*   The `enum_complex_tests/` directory is for tests that genuinely cannot be broken down without losing their intent.
*   This plan sets a clean foundation for subsequent, focused plans to uncomment and verify tests within these well-defined categories.
*   The `compile_fail` tests also need to be audited and reorganized.
*   The strategy for handling problematic tests during this structural phase is to comment them out selectively to ensure `cargo check` can pass for the overall structure.
*   `cargo clippy` and workspace-wide test/check commands are avoided.
*   **Update after Increment 1:** The target directories (`unit_tests/`, `unnamed_tests/`, `named_tests/`, `complex_tests/`) within the *expected* `former_enum_tests/` subdirectory were found to be empty. The test files expected to be in these directories are likely located elsewhere. Found actual enum test files in `tests/inc/enum_unit_tests/`, `tests/inc/enum_unnamed_tests/`, `tests/inc/enum_named_tests/`, and `tests/inc/enum_complex_tests/`. The subsequent increments will be revised to operate on these actual directories.
*   **Update after Increment 2:** Completed audit of all enum test files. Identified files needing moving, splitting/cleanup, correction, or refinement. Proposed a detailed plan for file operations in Increments 3 and 4, and noted necessary updates to `mod.rs` files in Increment 5 and corrections/refinements in Increment 6.
*   **Update after Increment 3:** Successfully moved `tuple_zero_fields` files to `enum_unnamed_tests/`. `cargo check --package former --tests` passed with warnings. Increment 3 is complete.
*   **Update after Increment 4:** Successfully split and cleaned up `standalone_constructor_args_tuple_manual.rs` and `standalone_constructor_args_named_manual.rs`. Deleted the original files. `cargo check --package former --tests` passed with warnings. Increment 4 is complete. Changes were manually committed due to a git issue.