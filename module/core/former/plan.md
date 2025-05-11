# Project Plan: Test `former` Crate - Enum Unit Variant Aspect

## Goal
*   Systematically ensure comprehensive test coverage for the **unit variant aspect** of the `#[derive(Former)]` macro in the `former` crate.
*   For each identified feature or rule related to unit variants:
    1.  Ensure a manual implementation test (`_manual.rs`) exists and passes, using shared test logic from `_only_test.rs`.
    2.  Ensure a derive macro invocation test (`_derive.rs`) exists and passes, using the same shared test logic.
    3.  If discrepancies arise where the manual test passes but the derive test fails, investigate and propose fixes to the `former_meta` crate. This investigation should consider if the test's expectation is incorrect or if there's a bug in the macro implementation. Utilize the `#[debug]` attribute on the enum in the `_derive.rs` file to output the generated code for analysis and comparison against the manual implementation.
*   All modifications will strictly adhere to `code/gen` instructions, Design Rules (especially "Proc Macro: Development Workflow"), and Codestyle Rules.
*   Verification will be done via `cargo test --package former --test <specific_test_module_or_file>` after each increment. Workspace-level tests and clippy checks will be avoided.
*   **New Goal (from feedback):** Analyze all remaining commented-out tests in `module/core/former/tests/inc/enum_unit_tests/mod.rs`. For each:
    *   If relevant to unit variants and not redundant: uncomment, ensure test files are aligned, test, and fix if necessary.
    *   If redundant: remove the module declaration and associated files.
    *   If not relevant to unit variants: move to an appropriate test directory or a new `enum_other_tests` directory.
    *   Ensure overall `enum_unit_tests` provides complete coverage for unit variants.
*   **New Goal (from user feedback after initial completion):**
    1.  Ensure no garbage files are left in `module/core/former/tests/inc/enum_unit_tests`.
    2.  Ensure `module/core/former/tests/inc/enum_unit_tests/mod.rs` has comments explaining which factors each group of tests covers.
*   **New Goal (from further user feedback):**
    1.  Re-evaluate if `tuple_zero_fields` tests truly belong in `enum_unit_tests` or if they should be moved due to the distinction between unit variants and zero-field tuple variants.
    2.  Ensure all test groups in `enum_unit_tests` strictly pertain to unit variants.


## Relevant Context
*   **Primary Test Directory:** `module/core/former/tests/inc/enum_unit_tests/`
*   **Supporting Files (potential review/modification):**
    *   `module/core/former/tests/inc/mod.rs` (to ensure test modules are active)
    *   `module/core/former_meta/src/derive_former.rs` (main derive entry)
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (macro implementation for enums)
    *   `module/core/former_meta/src/derive_former/former_struct.rs` (macro implementation for structs)
    *   `module/core/former_meta/src/derive_former/former_enum/*_handler.rs` (variant handlers)
    *   `module/core/former_meta/src/derive_former/struct_attrs.rs` (attribute parsing)
*   **Key Documentation for Reference:**
    *   `module/core/former/Readme.md`
    *   `module/core/former/advanced.md`
    *   This plan's "Expected Enum Former Behavior" section.
*   **Workspace:** Yes, this is part of a Cargo workspace.
*   **Target File Structure:** Primarily working within existing test files or creating new ones following the `_manual.rs`, `_derive.rs`, `_only_test.rs` pattern within `enum_unit_tests` or other relevant test directories.

### Expected Enum Former Behavior
(Content remains the same as before)

1.  **`#[scalar]` Attribute:**
    *   **Unit Variant (Rule 1a):** Generates `Enum::variant() -> Enum`.
    *   **Zero-Field Variant (Tuple) (Rule 1b):** Generates `Enum::variant() -> Enum`.
    *   **Zero-Field Variant (Struct) (Rule 1c):** Generates `Enum::variant() -> Enum`.
    *   ... (rest of rules)

2.  **`#[subform_scalar]` Attribute:**
    *   ... (rest of rules)

3.  **Default Behavior (No Attribute):**
    *   ... (rest of rules)

4.  **`#[standalone_constructors]` Attribute (Body Level) (Rule 4):**
    *   ... (rest of rules)

## Increments

*   [✅] **Increment 1 - 18:** (All previous increments completed)

*   [✅] **Increment 19: Relocate `tuple_zero_fields` Tests**
    *   **Pre-Analysis:** User feedback questioned if `tuple_zero_fields` tests (testing `Variant()`) belong in `enum_unit_tests`. Syntactically, `Variant()` is a tuple variant, not a unit variant (`Variant`). The "Expected Enum Former Behavior" rules also distinguish them (1a vs 1b, 3a vs 3b). For strictness and clarity, these tests should be moved.
    *   **Detailed Plan Steps:**
        1.  Create new directory: `module/core/former/tests/inc/enum_tuple_zero_field_tests/`. (Done by writing mod.rs)
        2.  Move `module/core/former/tests/inc/enum_unit_tests/tuple_zero_fields_derive.rs` to `module/core/former/tests/inc/enum_tuple_zero_field_tests/tuple_zero_fields_derive.rs`. (Done)
        3.  Move `module/core/former/tests/inc/enum_unit_tests/tuple_zero_fields_manual.rs` to `module/core/former/tests/inc/enum_tuple_zero_field_tests/tuple_zero_fields_manual.rs`. (Done)
        4.  Move `module/core/former/tests/inc/enum_unit_tests/tuple_zero_fields_only_test.rs` to `module/core/former/tests/inc/enum_tuple_zero_field_tests/tuple_zero_fields_only_test.rs`. (Done)
        5.  Create `module/core/former/tests/inc/enum_tuple_zero_field_tests/mod.rs`. (Done)
        6.  Modify `module/core/former/tests/inc/enum_unit_tests/mod.rs` to remove `tuple_zero_fields` modules and comments. (Done)
        7.  Modify `module/core/former/tests/inc/mod.rs` to add `pub mod enum_tuple_zero_field_tests;`. (Done)
    *   **Crucial Design Rules:** Structuring: Organize by Feature or Layer.
    *   **Verification Strategy:**
        *   `cargo test --package former --test tests -- inc::enum_tuple_zero_field_tests` passed.
        *   `cargo test --package former --test tests -- inc::enum_unit_tests` passed.
    *   Commit Message: `refactor(former): Move tuple_zero_fields tests to dedicated enum_tuple_zero_field_tests module`

*   [⏳] **Increment 20: Review Other Test Groups in `enum_unit_tests` for Strict Relevance**
    *   **Pre-Analysis:** Ensure all *other* remaining test groups in `enum_unit_tests` strictly pertain to *unit variants* or unit variants in a specific context (like generics or mixed enums).
    *   **Detailed Plan Steps:**
        1.  For each remaining active test group in `enum_unit_tests/mod.rs` (e.g., `unit_variant_*`, `keyword_variant_*`, `generic_unit_variant_*`, `mixed_enum_unit_*`, `enum_named_fields_unit_*`, `generic_enum_simple_unit_*`):
            *   Briefly re-verify that the core items being tested are indeed *unit variants* (e.g., `Variant`, not `Variant()` or `Variant{}`).
            *   If a group is found to primarily test non-unit variants (even if zero-field), plan its relocation similar to Increment 19.
        2.  Update `enum_unit_tests/mod.rs` comments if any further reclassification occurs or if comments for moved modules need to be fully removed.
    *   **Crucial Design Rules:** Structuring: Organize by Feature or Layer.
    *   **Verification Strategy:** `cargo test --package former --test tests -- inc::enum_unit_tests` must pass.
    *   Commit Message: `chore(former): Confirm relevance of remaining tests in enum_unit_tests module`

### Requirements
(Content remains the same as before)

## Notes & Insights
(Content remains the same as before, new issues identified in increments will be added here)
*   **Core Fix (Increment 8):** The `has_debug` flag (and `ItemAttributes` generally) was not being correctly determined and propagated from the main derive macro entry point (`derive_former.rs`) to `former_for_enum` and `former_for_struct`. This was fixed by parsing `ItemAttributes` once in `derive_former.rs` and passing the attributes and the derived `has_debug` boolean down.
*   **Standalone Constructor Naming (Increment 8):** Handlers like `tuple_zero_fields_handler.rs` were generating standalone constructors with names that could clash if multiple enums were in the same file. Fixed by prefixing with enum name (e.g., `zero_tuple_variant`).
*   **PhantomData Issue (Increment 10.A):** `former::Former` derive attempts to create formers for `PhantomData` variants/fields, causing compilation errors. Fixed by modifying `tuple_single_field_subform.rs` to generate a direct/scalar-like constructor for variants whose single field is `PhantomData`.
