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

*   [✅] **Increment 1:** Test Basic Unit Variants (Default and `#[scalar]`)
    *   Commit Message: `feat(former): Verify basic unit variant constructors (default, scalar, standalone)`

*   [✅] **Increment 2:** Test Unit Variants with `#[standalone_constructors]`
    *   Commit Message: `chore(former): Confirm standalone constructors for unit variants covered by previous tests`

*   [✅] **Increment 3:** Test Unit Variants with Keyword Identifiers
    *   Commit Message: `fix(former_meta): Handle raw identifiers and attribute parsing for enum formers`

*   [✅] **Increment 4:** Test Unit Variants within Generic Enums
    *   Commit Message: `fix(former_meta): Correctly handle generics in enum variant constructor generation`

*   [✅] **Increment 5:** Test Unit Variants within Enums using Named Field Syntax (for other variants)
    *   Commit Message: `fix(former_meta): Ensure implicit variant formers are defined and emitted for mixed enums`

*   [✅] **Increment 6:** Test Compile-Fail: Unit Variant with `#[subform_scalar]`
    *   Commit Message: `test(former): Add compile-fail test for subform_scalar on unit variant`

*   [✅] **Increment 7:** Final Verification of All Unit Variant Tests (Initial Pass)
    *   Commit Message: `test(former): Verify all working unit variant tests in enum_unit_tests module`

*   [✅] **Increment 8:** Create and Test `tuple_zero_fields_*` tests
    *   **Pre-Analysis:** Files for `tuple_zero_fields_derive` and `tuple_zero_fields_manual` did not exist. Created them.
    *   **Detailed Plan Steps:**
        1.  **Create `tuple_zero_fields_only_test.rs`:** Done.
        2.  **Create `tuple_zero_fields_manual.rs`:** Done. Aligned standalone constructor names.
        3.  **Activate and Test Manual Implementation:** Done. Manual tests passed.
        4.  **Create `tuple_zero_fields_derive.rs`:** Done.
        5.  **Activate and Test Derive Implementation:** Done. Initial failures due to `#[former(scalar)]` misuse and standalone constructor name clashes. Fixed `ItemAttributes` parsing propagation in `derive_former.rs`, `former_enum.rs`, `former_struct.rs`. Fixed standalone constructor naming in `tuple_zero_fields_handler.rs`. Derive tests now pass.
    *   **Crucial Design Rules:** "Proc Macro: Development Workflow"
    *   **Relevant Behavior Rules:** Rules 1b (scalar), 3b (default), 4a (standalone).
    *   **Verification Strategy:** Manual and derive tests passed.
    *   **Test Matrix:**
        *   ID: T8.1, Factor: Variant Type, Level: Zero-Field Tuple (e.g. `V()`), Attribute: Default, Expected: `Enum::v() -> Enum`, Standalone: `enum_name_v() -> Enum`
        *   ID: T8.2, Factor: Variant Type, Level: Zero-Field Tuple (e.g. `V()`), Attribute: `#[scalar]` (on variant, though default is same), Expected: `Enum::v() -> Enum`, Standalone: `enum_name_v() -> Enum`
    *   Commit Message: `fix(former_meta): Correct ItemAttributes parsing and standalone ctor names for enums` (This commit will cover the core fix. A subsequent commit will add the new tests.)

*   [✅] **Increment 9:** Analyze and Address `enum_named_fields_unit_*` tests
    *   **Pre-Analysis:** Comment suggests "Not part of this plan's scope for unit variants". However, these (`enum_named_fields_unit_derive`, `enum_named_fields_unit_manual`) might test a unit variant within an enum that *also* has variants with named struct fields. This is similar to Increment 5 (`mixed_enum_unit_*`) and is relevant.
    *   **Detailed Plan Steps:**
        1.  Inspect file contents of `enum_named_fields_unit_manual.rs` and `enum_named_fields_unit_derive.rs` (and any `_only_test.rs`). (Done)
        2.  If they test a unit variant's constructor behavior: Align, uncomment, test manual, test derive, fix `former_meta` if needed. (Done: Uncommented, manual and derive tests passed.)
        3.  If truly not about unit variant constructors, decide if they are redundant or should be moved. (Not applicable)
    *   **Crucial Design Rules:** "Proc Macro: Development Workflow"
    *   **Relevant Behavior Rules:** Rules 1a, 3a, 4a.
    *   **Verification Strategy:** Relevant tests must pass or files moved/removed. (Passed)
    *   Commit Message: `test(former): Analyze and integrate/refactor enum_named_fields_unit tests`

*   [✅] **Increment 10.A: Fix `former_meta` to Ignore `PhantomData` in Enums**
    *   **Pre-Analysis:** The `former::Former` derive macro incorrectly attempts to generate "former" constructors for `core::marker::PhantomData` variants/fields in enums, leading to compilation errors (E0223).
    *   **Detailed Plan Steps:**
        1.  Read `module/core/former_meta/src/derive_former/former_enum.rs`. (Done)
        2.  Identified `tuple_single_field_subform.rs` as the key handler for `Variant(PhantomData<T>)`. (Done)
        3.  Modified `tuple_single_field_subform.rs` to check if `field_info.ty` is `PhantomData`. (Done)
        4.  If it is `PhantomData`, the handler now generates a scalar-like direct constructor instead of attempting to create a `::Former` for `PhantomData`. (Done)
        5.  Checked other handlers; `struct_single_field_subform.rs` and `*_multi_fields_subform.rs` seem okay as they would embed `PhantomData` directly into their `VariantFormer` structs, which derive `Default` correctly. (Done)
    *   **Crucial Design Rules:** Minimal Change.
    *   **Relevant Behavior Rules:** N/A (fixing macro internals).
    *   **Verification Strategy:** `generic_enum_simple_unit_derive.rs` (with `_Phantom` variant) compiled and its test passed. (Done)
    *   Commit Message: `fix(former_meta): Prevent derive macro from generating formers for PhantomData variants`

*   [⏳] **Increment 10.B:** Refactor and Test `generics_in_tuple_variant_unit_*` (as `generic_enum_simple_unit_*`)
    *   **Pre-Analysis:** (As before, but assuming 10.A is done)
    *   **Detailed Plan Steps:**
        *   (Steps 5.1-5.3, 8, 9 for file renaming, creation, and mod.rs update are already done or in progress from previous attempt at Increment 10)
        *   Ensure `generic_enum_simple_unit_manual.rs` has `_Phantom(core::marker::PhantomData::<X>)` and includes `_only_test.rs`. (Done)
        *   Ensure `generic_enum_simple_unit_derive.rs` has `_Phantom(core::marker::PhantomData::<X>)` and includes `_only_test.rs`. (Done)
        *   Test Manual Implementation: `cargo test --package former --test tests -- inc::enum_unit_tests::generic_enum_simple_unit_manual`.
        *   Test Derive Implementation: If manual passes, `cargo test --package former --test tests -- inc::enum_unit_tests::generic_enum_simple_unit_derive`.
        *   Fix `former_meta` if needed (hopefully not, after 10.A).
    *   **Crucial Design Rules:** "Proc Macro: Development Workflow"
    *   **Relevant Behavior Rules:** Rules 1a, 3a.
    *   **Verification Strategy:** Manual and derive tests for `generic_enum_simple_unit_*` must pass.
    *   Commit Message: `test(former): Refactor and test unit variants in simple generic enum`

*   [⚫] **Increment 11:** Analyze and Address `keyword_variant_unit_derive`
    *   Commit Message: `test(former): Analyze and cleanup/integrate keyword_variant_unit_derive test`

*   [⚫] **Increment 12:** Analyze and Address `standalone_constructor_unit_derive`
    *   Commit Message: `test(former): Analyze and cleanup/integrate standalone_constructor_unit_derive test`

*   [⚫] **Increment 13:** Analyze and Address `standalone_constructor_args_*` tests
    *   Commit Message: `test(former): Analyze and refactor/move standalone_constructor_args_unit tests`

*   [⚫] **Increment 14:** Analyze and Address `compile_fail` module
    *   Commit Message: `test(former): Consolidate and verify compile-fail tests for enum unit variants`

*   [⚫] **Increment 15:** Final Cleanup and Verification of `enum_unit_tests`
    *   Commit Message: `test(former): Finalize and verify all enum unit tests`

### Requirements
(Content remains the same as before)

## Notes & Insights
(Content remains the same as before, new issues identified in increments will be added here)
*   **Core Fix (Increment 8):** The `has_debug` flag (and `ItemAttributes` generally) was not being correctly determined and propagated from the main derive macro entry point (`derive_former.rs`) to `former_for_enum` and `former_for_struct`. This was fixed by parsing `ItemAttributes` once in `derive_former.rs` and passing the attributes and the derived `has_debug` boolean down.
*   **Standalone Constructor Naming (Increment 8):** Handlers like `tuple_zero_fields_handler.rs` were generating standalone constructors with names that could clash if multiple enums were in the same file. Fixed by prefixing with enum name (e.g., `zero_tuple_variant`).
*   **PhantomData Issue (Increment 10.A):** `former::Former` derive attempts to create formers for `PhantomData` variants/fields, causing compilation errors. Fixed by modifying `tuple_single_field_subform.rs` to generate a direct/scalar-like constructor for variants whose single field is `PhantomData`.