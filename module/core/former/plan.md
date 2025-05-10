# Project Plan: Verify and Fix Unit Enum Variant Tests

## Goal
*   Systematically uncomment and verify all tests within the `module/core/former/tests/inc/former_enum_tests/unit_tests/` directory.
*   Ensure the `#[derive(Former)]` macro correctly generates the expected constructors for **unit enum variants** (`enum E { UnitVariant }`) according to the "Expected Enum Former Behavior Rules".
*   Verify the implementation correctly handles `#[scalar]` (which is the default for unit variants) and `#[standalone_constructors]` attributes for unit variants.
*   Fix any bugs found in the macro (`former_meta`) or test logic (`_manual.rs`, `_derive.rs`, `_only_test.rs`) to ensure all unit variant tests pass.
*   Address any `xxx` or `qqq` comments within the activated unit test files.
*   Ensure all code modifications adhere strictly to `code/gen` instructions, Design Rules, and Codestyle Rules.
*   Avoid using `cargo clippy`.

## Relevant Context

**Important:** Before starting implementation, thoroughly review the `Readme.md` and `advanced.md` files for the `former` crate, and the `Readme.md` for `former_meta` to ensure a full understanding of the existing design, features, and intended behaviors.

*   **Primary Test Directory:** `module/core/former/tests/inc/former_enum_tests/unit_tests/`
    *   (Files within this directory, e.g., `unit_variant_derive.rs`, `unit_variant_manual.rs`, `enum_named_fields_unit_derive.rs`, etc., as identified/created in the previous refactoring plan)
    *   `compile_fail/` subdirectory within `unit_tests/` (e.g., `unit_subform_scalar_error.rs`)
*   **Module Files to Update:**
    *   `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs` (Uncomment `mod` declarations for individual test files/groups).
    *   `module/core/former/tests/inc/former_enum_tests/unit_tests/compile_fail/mod.rs` (If it exists and needs updates).
*   **Macro Implementation (Primary Focus):**
    *   `module/core/former_meta/src/derive_former/former_enum/unit_variant_handler.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (Main dispatcher, especially for `#[standalone_constructors]`)
*   **Core Crate Files & Documentation:** (Same as previous plan)

### Expected Enum Former Behavior Rules (Unit Variants Focus)

This section details the expected code generation behavior for `#[derive(Former)]` specifically for **unit enum variants**.

1.  **Default Behavior (No variant-specific attribute) (Rule UV.Def / Rule 3a from full list)**
    *   **Associated Method:** Generates `EnumName::variant_name() -> EnumName`.
    *   **Standalone Constructor (if `#[standalone_constructors]` on enum):** Generates `fn variant_name() -> EnumName`. (Rule 4)

2.  **`#[scalar]` Attribute (on unit variant) (Rule UV.S / Rule 1a from full list)**
    *   **Behavior:** Identical to the default behavior for unit variants.
    *   **Associated Method:** Generates `EnumName::variant_name() -> EnumName`.
    *   **Standalone Constructor (if `#[standalone_constructors]` on enum):** Generates `fn variant_name() -> EnumName`. (Rule 4)

3.  **`#[subform_scalar]` Attribute (on unit variant) (Rule UV.SS / Rule 2a from full list)**
    *   **Behavior:** **Compile-time error.** This attribute is not applicable to unit variants.

4.  **`#[former(default = ...)]` Attribute (Rule UV.FDef)**
    *   **Behavior:** Not applicable to unit variants as they have no fields to default. Attempting to use it should ideally be an error or ignored.

5.  **`#[arg_for_constructor]` Attribute (Rule UV.AFC)**
    *   **Behavior:** Not applicable as unit variants have no fields. If `#[standalone_constructors]` is present, the generated standalone constructor for a unit variant takes no arguments.

**Test Matrix Coverage (Unit Variants):**
(This matrix should already be documented in `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs` from the previous plan, and is included here for planning context.)

*   **Factors:**
    1.  Variant Type: Unit
    2.  Variant-Level Attribute: None (Default), `#[scalar]`
    3.  Enum-Level Attribute: None, `#[standalone_constructors]`

*   **Combinations to Verify:**
    *   U.1: Unit + Default + None (Static method `Enum::variant() -> Enum`)
    *   U.2: Unit + `#[scalar]` + None (Static method `Enum::variant() -> Enum`)
    *   U.3: Unit + Default + `#[standalone_constructors]` (Static method + Standalone `fn variant() -> Enum`)
    *   U.4: Unit + `#[scalar]` + `#[standalone_constructors]` (Static method + Standalone `fn variant() -> Enum`)
    *   U.5: Unit + `#[subform_scalar]` (Any enum attrs) -> **Compile Error**

### Failure Diagnosis Algorithm
*   (Standard algorithm as previously defined, focusing on `unit_variant_handler.rs` if `_derive` fails and `_manual` passes).
*   **Widespread Failure Strategy:** If uncommenting a test group causes numerous failures, propose selectively commenting out (using `//`) only the failing `#[test]` functions or problematic `include!` lines. Avoid commenting out entire files or modules unless absolutely necessary. Re-enable tests incrementally.

## Increments

*   [⚫] **Increment 1: Activate and Verify `unit_variant_*` Test Group**
    *   **Goal:** Uncomment and ensure `unit_variant_derive.rs`, `unit_variant_manual.rs`, and `unit_variant_only_test.rs` pass, covering basic unit variants with and without `#[standalone_constructors]`.
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs`:
        *   Uncomment `pub mod unit_variant_manual;`.
    *   **Detailed Plan Step 2 (Manual Verification):**
        *   **Pre-Analysis:** `unit_variant_manual.rs` should implement `Status::pending()`, `Status::complete()`, and standalone `pending()`, `complete()` all returning `Status`. `unit_variant_only_test.rs` calls these. This covers matrix rows U.1, U.2, U.3, U.4.
        *   Request user to run `cargo test --package former --test tests former_enum_tests::unit_tests::unit_variant_manual`.
        *   Analyze results. Fix any issues in `unit_variant_manual.rs` or `unit_variant_only_test.rs`.
    *   **Detailed Plan Step 3:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs`:
        *   Uncomment `pub mod unit_variant_derive;`.
    *   **Detailed Plan Step 4 (Derive Verification):**
        *   **Pre-Analysis:** `unit_variant_derive.rs` derives `Former` and `#[former(standalone_constructors)]` on `Status` enum. Expect `unit_variant_handler.rs` to generate code matching the manual version.
        *   Request user to run `cargo test --package former --test tests former_enum_tests::unit_tests::unit_variant_derive`.
        *   Analyze results. If failures, use Failure Diagnosis Algorithm. Fixes likely in `former_meta/src/derive_former/former_enum/unit_variant_handler.rs`. *Handle widespread failures selectively.*
    *   **Crucial Design Rules:** Expected Behavior Rules UV.Def, UV.S, E.SC. [Proc Macro: Development Workflow](#proc-macro-development-workflow).
    *   **Verification Strategy:** All tests in `unit_variant_manual` and `unit_variant_derive` pass.
    *   **Commit Message:** `feat(former): Verify basic unit enum variant functionality`

*   [⚫] **Increment 2: Activate and Verify Unit Variants in `enum_named_fields_unit_*`**
    *   **Goal:** Uncomment and ensure unit variant tests within the split `enum_named_fields_unit_*.rs` files pass. These also test `#[standalone_constructors]`.
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs`:
        *   Uncomment `pub mod enum_named_fields_unit_manual;`.
    *   **Detailed Plan Step 2 (Manual Verification):**
        *   **Pre-Analysis:** `enum_named_fields_unit_manual.rs` should implement unit variants `UnitVariantDefault` and `UnitVariantScalar` with static methods and standalone constructors.
        *   Request user to run `cargo test --package former --test tests former_enum_tests::unit_tests::enum_named_fields_unit_manual`. Fix if needed.
    *   **Detailed Plan Step 3:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs`:
        *   Uncomment `pub mod enum_named_fields_unit_derive;`.
    *   **Detailed Plan Step 4 (Derive Verification):**
        *   **Pre-Analysis:** `enum_named_fields_unit_derive.rs` tests `UnitVariantDefault` and `UnitVariantScalar` with `#[standalone_constructors]`.
        *   Request user to run `cargo test --package former --test tests former_enum_tests::unit_tests::enum_named_fields_unit_derive`. Fix macro if needed. *Handle widespread failures selectively.*
    *   **Crucial Design Rules:** Expected Behavior Rules UV.Def, UV.S, E.SC. [Proc Macro: Development Workflow](#proc-macro-development-workflow).
    *   **Verification Strategy:** All tests in `enum_named_fields_unit_manual` and `enum_named_fields_unit_derive` pass.
    *   **Commit Message:** `feat(former): Verify unit variants within mixed enum definitions`

*   [⚫] **Increment 3: Activate and Verify Unit Variants in `generics_in_tuple_variant_unit_*`**
    *   **Goal:** Uncomment and ensure unit variant tests within the split `generics_in_tuple_variant_unit_*.rs` files pass. This tests unit variants in generic enums.
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs`:
        *   Uncomment `pub mod generics_in_tuple_variant_unit_manual;`.
    *   **Detailed Plan Step 2 (Manual Verification):**
        *   **Pre-Analysis:** `generics_in_tuple_variant_unit_manual.rs` should implement `EnumOuter::OtherVariant()` for a generic enum.
        *   Request user to run `cargo test --package former --test tests former_enum_tests::unit_tests::generics_in_tuple_variant_unit_manual`. Fix if needed.
    *   **Detailed Plan Step 3:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs`:
        *   Uncomment `pub mod generics_in_tuple_variant_unit_derive;`.
    *   **Detailed Plan Step 4 (Derive Verification):**
        *   **Pre-Analysis:** `generics_in_tuple_variant_unit_derive.rs` tests `OtherVariant` in `EnumOuter<X>`.
        *   Request user to run `cargo test --package former --test tests former_enum_tests::unit_tests::generics_in_tuple_variant_unit_derive`. Fix macro if needed. *Handle widespread failures selectively.*
    *   **Crucial Design Rules:** Expected Behavior Rules UV.Def, UV.S. [Proc Macro: Development Workflow](#proc-macro-development-workflow).
    *   **Verification Strategy:** All tests pass.
    *   **Commit Message:** `feat(former): Verify unit variants in generic enums`

*   [⚫] **Increment 4: Activate and Verify Unit Variants in `keyword_variant_unit_*`**
    *   **Goal:** Uncomment and ensure unit variant tests (with keyword names) within the split `keyword_variant_unit_*.rs` files pass.
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs`:
        *   Uncomment `pub mod keyword_variant_unit_derive;`. (No manual file for this specific split part).
    *   **Detailed Plan Step 2 (Derive Verification):**
        *   **Pre-Analysis:** `keyword_variant_unit_derive.rs` tests `r#Loop`.
        *   Request user to run `cargo test --package former --test tests former_enum_tests::unit_tests::keyword_variant_unit_derive`. Fix macro if needed. *Handle widespread failures selectively.*
    *   **Crucial Design Rules:** Expected Behavior Rules UV.Def, UV.S. [Proc Macro: Development Workflow](#proc-macro-development-workflow).
    *   **Verification Strategy:** All tests pass.
    *   **Commit Message:** `feat(former): Verify unit variants with keyword identifiers`

*   [⚫] **Increment 5: Activate and Verify Unit Variants in `standalone_constructor_unit_*`**
    *   **Goal:** Uncomment and ensure unit variant tests within the split `standalone_constructor_unit_*.rs` files pass (testing `#[standalone_constructors]` specifically).
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs`:
        *   Uncomment `pub mod standalone_constructor_unit_derive;`. (No manual file for this specific split part, as `standalone_constructor_manual.rs` was for the whole enum).
    *   **Detailed Plan Step 2 (Derive Verification):**
        *   **Pre-Analysis:** `standalone_constructor_unit_derive.rs` tests `UnitVariant` with `#[standalone_constructors]`.
        *   Request user to run `cargo test --package former --test tests former_enum_tests::unit_tests::standalone_constructor_unit_derive`. Fix macro if needed. *Handle widespread failures selectively.*
    *   **Crucial Design Rules:** Expected Behavior Rules UV.Def, UV.S, E.SC. [Proc Macro: Development Workflow](#proc-macro-development-workflow).
    *   **Verification Strategy:** All tests pass.
    *   **Commit Message:** `feat(former): Verify standalone constructors for unit variants`

*   [⚫] **Increment 6: Activate and Verify Unit Variants in `standalone_constructor_args_unit_*`**
    *   **Goal:** Uncomment and ensure unit variant tests within the split `standalone_constructor_args_unit_*.rs` files pass (testing `#[standalone_constructors]` where unit variants naturally have no args).
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs`:
        *   Uncomment `pub mod standalone_constructor_args_unit_manual;`.
    *   **Detailed Plan Step 2 (Manual Verification):**
        *   **Pre-Analysis:** `standalone_constructor_args_unit_manual.rs` tests `UnitVariantArgs`.
        *   Request user to run `cargo test --package former --test tests former_enum_tests::unit_tests::standalone_constructor_args_unit_manual`. Fix if needed.
    *   **Detailed Plan Step 3:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs`:
        *   Uncomment `pub mod standalone_constructor_args_unit_derive;`.
    *   **Detailed Plan Step 4 (Derive Verification):**
        *   **Pre-Analysis:** `standalone_constructor_args_unit_derive.rs` tests `UnitVariantArgs` with `#[standalone_constructors]`.
        *   Request user to run `cargo test --package former --test tests former_enum_tests::unit_tests::standalone_constructor_args_unit_derive`. Fix macro if needed. *Handle widespread failures selectively.*
    *   **Crucial Design Rules:** Expected Behavior Rules UV.Def, UV.S, E.SC. [Proc Macro: Development Workflow](#proc-macro-development-workflow).
    *   **Verification Strategy:** All tests pass.
    *   **Commit Message:** `feat(former): Verify standalone constructors (with args context) for unit variants`

*   [⚫] **Increment 7: Verify Compile-Fail Test for Unit Variants**
    *   **Goal:** Ensure `#[subform_scalar]` on a unit variant correctly causes a compile error.
    *   **Target Crate(s):** `former`, `former_meta`
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/compile_fail/mod.rs`:
        *   Uncomment `mod unit_subform_scalar_error;` (or ensure it's active).
    *   **Detailed Plan Step 2:** Modify `module/core/former/tests/inc/former_enum_tests/unit_tests/mod.rs`:
        *   Uncomment `pub mod compile_fail;`.
    *   **Verification Strategy:** Request user to run `cargo test --package former --test tests former_enum_tests::unit_tests::compile_fail`. The specific test `unit_subform_scalar_error` should fail to compile, and `trybuild` should report this as a pass for the compile-fail test. If the test *compiles*, it's a bug in the macro not erroring.
    *   **Commit Message:** `test(former_meta): Verify compile error for #[subform_scalar] on unit variant`

*   [⚫] **Increment 8: Address TODOs/Issues in Activated Unit Variant Files**
    *   **Goal:** Review and address any outstanding `// xxx :` or `// qqq :` comments specifically within all activated unit variant test files.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Search for `xxx :` and `qqq :` comments in all files within `module/core/former/tests/inc/former_enum_tests/unit_tests/`.
    *   **Detailed Plan Step 2:** Propose solutions or code changes for each identified comment based on its content.
    *   **Crucial Design Rules:** [Comments: Add Tasks and Label Simplifications](#comments-add-tasks-and-label-simplifications), [Comments: Annotate Addressed Tasks](#comments-annotate-addressed-tasks).
    *   **Verification Strategy:** Request user to apply changes. Run `cargo check --package former --tests` and `cargo test --package former --test tests former_enum_tests::unit_tests`. Ensure tests still pass and comments are addressed.
    *   **Commit Message:** `chore(former): Address TODOs in unit variant enum tests`

*   [⚫] **Increment 9: Final Unit Variant Verification**
    *   **Goal:** Ensure all activated unit tests pass and the `former` package is healthy after all unit variant focused changes.
    *   **Target Crate(s):** `former`
    *   **Detailed Plan Step 1:** Run `cargo check --package former --tests`. Address any errors or warnings.
    *   **Detailed Plan Step 2:** Run `cargo test --package former --test tests former_enum_tests::unit_tests`. Ensure all unit tests pass.
    *   **Verification Strategy:** Zero errors/warnings from `check`. All tests in `former_enum_tests::unit_tests` pass.
    *   **Commit Message:** `test(former): All unit variant enum tests pass`

### Requirements
*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules.
*   **Focus:** Only uncomment and address code related to **unit enum variants**.
*   **Incremental Activation:** Uncomment test modules (`mod ...;`) within `unit_tests/mod.rs` one group at a time.
*   **Incremental Verification:** Verify compilation and test success after each relevant increment. Verify `_manual` tests before `_derive` tests. Handle widespread failures by selectively commenting out only failing tests.
*   **Failure Analysis:** Follow the "Failure Diagnosis Algorithm".
*   **Approval Gates:** Obtain user approval before starting each increment and after successful verification.

## Notes & Insights
*   This plan focuses exclusively on unit enum variants.
*   The previous restructuring and audit are assumed complete.
*   The "Expected Enum Former Behavior Rules" section is now more comprehensive.
*   `cargo clippy` is excluded.
*