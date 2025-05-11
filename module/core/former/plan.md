# Project Plan: Review and Document Enum Tests in `former` Crate

## Goal
*   Systematically review all **active** (i.e., compiled as part of `cargo check --tests`) enum-related test files within the `former` crate (`module/core/former/tests/inc/enum_*_tests/`).
*   For each targeted test file:
    1.  Add a `//! Purpose: ...` comment block.
    2.  Add a `//! Coverage: ...` comment block.
    3.  Add a `//! Test Relevance/Acceptance Criteria: ...` comment block.
*   Ensure all added documentation comments are clear, accurate, and adhere to specified content criteria and Rust documentation best practices.
*   Ensure all modifications strictly adhere to `code/gen` instructions, Design Rules, and Codestyle Rules.
*   Structure the work into logical increments, processing one test file or a closely related group of test files (i.e., `_derive.rs`, `_manual.rs`, and their shared `_only_test.rs`) per increment, with each increment having a narrow focus on a specific enum aspect (Unit, Unnamed/Tuple, Named/Struct, or Complex/Mixed).
*   **Crucially, this plan focuses *only* on adding documentation. Pre-existing test failures or logic errors are out of scope. Changes will only be committed if `cargo check --package former --tests` passes after adding comments.**

## Relevant Context
*   **Primary Test Directories:**
    *   `module/core/former/tests/inc/enum_unit_tests/`
    *   `module/core/former/tests/inc/enum_unnamed_tests/` (Tuple-like variants)
    *   `module/core/former/tests/inc/enum_named_tests/` (Struct-like variants with named fields)
    *   `module/core/former/tests/inc/enum_complex_tests/`
*   **Module Files to Update (Potentially for review):**
    *   `module/core/former/tests/inc/enum_unit_tests/mod.rs`
    *   `module/core/former/tests/inc/enum_unnamed_tests/mod.rs`
    *   `module/core/former/tests/inc/enum_named_tests/mod.rs`
    *   `module/core/former/tests/inc/enum_complex_tests/mod.rs`
*   **Key Documentation for Reference:**
    *   `module/core/former/Readme.md`
    *   `module/core/former/advanced.md`
    *   This `plan.md` for the "Expected Enum Former Behavior Rules".
*   **Workspace:** Yes, this is part of a Cargo workspace.
*   **Target File Structure:** No major structural changes, primarily adding comments to existing files.

### Expected Enum Former Behavior

This plan adheres to the following rules for `#[derive(Former)]` on enums:

1.  **`#[scalar]` Attribute:**
    *   **Unit Variant (Rule 1a):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_unit_variant`)
    *   **Zero-Field Variant (Tuple) (Rule 1b):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_tuple_zero_variant`)
    *   **Zero-Field Variant (Struct) (Rule 1c):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_struct_zero_variant`)
    *   **Single-Field Variant (Tuple) (Rule 1d):** Generates `Enum::variant(InnerType) -> Enum`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Single-Field Variant (Struct) (Rule 1e):** Generates `Enum::variant { field: InnerType } -> Enum`. (Handled by: `handle_struct_non_zero_variant`)
    *   **Multi-Field Variant (Tuple) (Rule 1f):** Generates `Enum::variant(T1, T2, ...) -> Enum`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Multi-Field Variant (Struct) (Rule 1g):** Generates `Enum::variant { f1: T1, f2: T2, ... } -> Enum`. (Handled by: `handle_struct_non_zero_variant`)
    *   **Error Cases:** Cannot be combined with `#[subform_scalar]`.

2.  **`#[subform_scalar]` Attribute:**
    *   **Unit Variant (Rule 2a):** Error. (Checked in: `handle_unit_variant`)
    *   **Zero-Field Variant (Tuple or Struct) (Rule 2b, 2c):** Error. (Checked in: `handle_tuple_zero_variant`, `handle_struct_zero_variant`)
    *   **Single-Field Variant (Tuple) (Rule 2d):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Single-Field Variant (Struct) (Rule 2e):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
    *   **Multi-Field Variant (Tuple) (Rule 2f):** Error. Cannot use `subform_scalar` on multi-field tuple variants. (Checked in: `handle_tuple_non_zero_variant`)
    *   **Multi-Field Variant (Struct) (Rule 2g):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)

3.  **Default Behavior (No Attribute):**
    *   **Unit Variant (Rule 3a):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_unit_variant`)
    *   **Zero-Field Variant (Tuple) (Rule 3b):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_tuple_zero_variant`)
    *   **Zero-Field Variant (Struct) (Rule 3c):** Error. Requires `#[scalar]`. (Checked in: `handle_struct_zero_variant`)
    *   **Single-Field Variant (Tuple) (Rule 3d):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `handle_tuple_non_zero_variant`)
    *   **Single-Field Variant (Struct) (Rule 3e):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
    *   **Multi-Field Variant (Tuple) (Rule 3f):** Generates `Enum::variant(Field1Type, Field2Type, ...) -> Enum` (behaves like `#[scalar]`). (Handled by: `handle_tuple_non_zero_variant`)
    *   **Multi-Field Variant (Struct) (Rule 3g):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)

4.  **`#[standalone_constructors]` Attribute (Body Level) (Rule 4):**
    *   **Rule 4a:** Generates top-level constructor functions for each variant (e.g., `my_variant()`).
    *   **Rule 4b (Option 2 Logic):** Return type depends on `#[arg_for_constructor]` on fields within the variant.

### Example of Expected Documentation Comments

This section shows an example of the documentation comments that will be added to a test file. The content should adhere to the criteria outlined in the `### Requirements` section under "Comment Content".

**For a file like `module/core/former/tests/inc/enum_unit_tests/generics_in_tuple_variant_unit_derive.rs`:**
```rust
//! Purpose: Tests the `#[derive(Former)]` macro's generation of constructors for unit variants
//! within an enum that has generic parameters and bounds. This file focuses on verifying
//! the derive-based implementation.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Verifies `Enum::variant() -> Enum` for a generic enum.
//! - Rule 1a (Unit + `#[scalar]`): Verifies `Enum::variant() -> Enum` (as default for unit is scalar) for a generic enum.
//! - (Implicitly) Rule 4a: If `#[standalone_constructors]` were active on `EnumOuter`, this test would also cover
//!   the generation of `fn other_variant() -> EnumOuter<X>`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a generic enum `EnumOuter<X: Copy>` with a unit variant `OtherVariant`.
//! - Instantiates `EnumOuter` with a concrete type `MyType` that fulfills the `Copy` bound.
//! - Invokes the derived static method `EnumOuter::<MyType>::other_variant()`.
//! - Asserts that the `got` instance is equal to an `expected` instance, which is manually
//!   constructed as `EnumOuter::<MyType>::OtherVariant`. This confirms the constructor produces the correct variant instance.
```

## Increments

**Increment Template: Document Test File/Group**
*   **Target Crate(s):** `former`
*   **Enum Aspect Focus:** [Unit | Unnamed/Tuple | Named/Struct | Complex/Mixed]
*   **Target File(s):** [List of specific `.rs` files for this increment]
*   **Pre-Analysis (AI to output this in Detailed Planning - Output 4):**
    *   Identified enum variant structures in target file(s): [e.g., "Unit variants", "Single-field tuple variant with `#[scalar]`"]
    *   Key attributes present: [e.e., `#[scalar]`, `#[standalone_constructors]` on enum]
    *   Relevant "Expected Enum Former Behavior Rule IDs": [e.g., "1a, 4a"]
    *   Brief summary of how test functions appear to exercise these rules: [e.g., "Test `basic_construction` calls `Enum::variant()` and compares with manual construction. Test `standalone_construction` calls top-level `variant()`."]
*   **Proposed Comments:**
    *   AI will propose the three `//!` comment blocks (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file, adhering to the "Comment Content" requirements.
*   **Verification Strategy:** After comments are added by the user, the AI will request the user to run `cargo check --package former --tests`. The code must compile without errors.
*   **Commit Message:** `docs(former): Add purpose and coverage to [enum_aspect_focus] [specific_test_file_or_group_name]`

---
**Phase 1: Unit Variant Tests (`enum_unit_tests`)**

*   [✅] **Increment 1:** Document `unit_variant_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Unit variants.
        *   Key attributes present: `#[derive(Former)]`, `#[former( standalone_constructors )]` on the enum in `_derive.rs`. Manual implementations in `_manual.rs`.
        *   Relevant "Expected Enum Former Behavior Rule IDs": 3a, 1a, 4a.
        *   Brief summary of how test functions appear to exercise these rules: `unit_variant_constructors` tests static methods (`Status::pending()`, `Status::complete()`). `unit_variant_standalone_constructors` tests standalone functions (`pending()`, `complete()`). Both compare results with direct enum variants.
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks.
    *   Relevant Behavior Rules: Rule 3a (Unit + Default), Rule 1a (Unit + `#[scalar]`), Rule 4a (#[standalone_constructors]).
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unit
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unit_tests/unit_variant_derive.rs`
        *   `module/core/former/tests/inc/enum_unit_tests/unit_variant_manual.rs`
        *   `module/core/former/tests/inc/enum_unit_tests/unit_variant_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to unit_variant enum tests`

*   [✅] **Increment 2:** Document `enum_named_fields_unit_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Unit variants.
        *   Key attributes present: `#[derive(Former)]`, `#[debug]`, `#[standalone_constructors]` on the enum in `_derive.rs`. Manual implementations in `_manual.rs`.
        *   Relevant "Expected Enum Former Behavior Rule IDs": 3a, 1a, 4a.
        *   Brief summary of how test functions appear to exercise these rules: `unit_variant_scalar_test` and `unit_variant_default_construction` test static methods (`EnumWithNamedFields::unit_variant_scalar()`, `EnumWithNamedFields::unit_variant_default()`) and compare results with direct enum variants. Standalone constructors are present due to `#[standalone_constructors]` but not explicitly tested in `_only_test.rs`.
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks.
    *   Relevant Behavior Rules: Rule 3a (Unit + Default), Rule 1a (Unit + `#[scalar]`), Rule 4a (#[standalone_constructors]).
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unit (within a named-fields style enum definition)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unit_tests/enum_named_fields_unit_derive.rs`
        *   `module/core/former/tests/inc/enum_unit_tests/enum_named_fields_unit_manual.rs`
        *   `module/core/former/tests/inc/enum_unit_tests/enum_named_fields_unit_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to enum_named_fields_unit tests`

*   [✅] **Increment 3:** Document `generics_in_tuple_variant_unit_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Unit variants within a generic enum with bounds.
        *   Key attributes present: `#[derive(Former)]`, `#[debug]` on the enum in `_derive.rs`. Manual implementation in `_manual.rs`.
        *   Relevant "Expected Enum Former Behavior Rule IDs": 3a, 1a. (Rule 4a is implicitly covered by the enum having `#[derive(Former)]` but not explicitly tested in these files).
        *   Brief summary of how test functions appear to exercise these rules: No test functions are present in these specific files. The comment in both files indicates that the original `_only_test.rs` file for tuple variants did not test the unit variant. This means these files likely rely on broader tests or were intended for future test logic.
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks.
    *   Relevant Behavior Rules: Rule 3a (Unit + Default), Rule 1a (Unit + `#[scalar]`).
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unit (within generic enums)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unit_tests/generics_in_tuple_variant_unit_derive.rs`
        *   `module/core/former/tests/inc/enum_unit_tests/generics_in_tuple_variant_unit_manual.rs`
        *   `module/core/former/tests/inc/enum_unit_tests/generics_in_tuple_variant_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to generics_in_tuple_variant_unit tests`

*   [✅] **Increment 4:** Document `keyword_variant_unit_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Unit variant with a keyword identifier (`r#Loop`).
        *   Key attributes present: `#[derive(Former)]` on the enum in `_derive.rs`.
        *   Relevant "Expected Enum Former Behavior Rule IDs": 3a, 1a. (Rule 4a is implicitly covered by the enum having `#[derive(Former)]` but not explicitly tested in these files).
        *   Brief summary of how test functions appear to exercise these rules: `keyword_variant_constructors` tests the static method (`KeywordVariantEnum::r#loop()`) and compares the result with the direct enum variant (`KeywordVariantEnum::r#Loop`).
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks.
    *   Relevant Behavior Rules: Rule 3a (Unit + Default), Rule 1a (Unit + `#[scalar]`).
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unit (with keyword identifiers)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unit_tests/keyword_variant_unit_derive.rs`
        *   `module/core/former/tests/inc/enum_unit_tests/keyword_variant_unit_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to keyword_variant_unit tests`

*   [✅] **Increment 5:** Document `standalone_constructor_unit_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Unit variants.
        *   Key attributes present: `#[derive(Former)]`, `#[standalone_constructors]` on the enum in `_derive.rs`.
        *   Relevant "Expected Enum Former Behavior Rule IDs": 3a, 1a, 4a.
        *   Brief summary of how test functions appear to exercise these rules: `unit_variant_test` tests the standalone constructor function (`unit_variant()`) and compares the result with the direct enum variant (`TestEnum::UnitVariant`).
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks.
    *   Relevant Behavior Rules: Rule 3a (Unit + Default), Rule 1a (Unit + `#[scalar]`), Rule 4a (#[standalone_constructors]).
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unit (with `#[standalone_constructors]`)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unit_tests/standalone_constructor_unit_derive.rs`
        *   `module/core/former/tests/inc/enum_unit_tests/standalone_constructor_unit_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to standalone_constructor_unit tests`

*   [✅] **Increment 6:** Document `standalone_constructor_args_unit_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Unit variants.
        *   Key attributes present: `#[derive(Former)]`, `#[standalone_constructors]`, `#[debug]` on the enum in `_derive.rs`. Manual implementation in `_manual.rs`.
        *   Relevant "Expected Enum Former Behavior Rule IDs": 3a, 1a, 4a. (Rule 4b is mentioned in the plan but not applicable to unit variants).
        *   Brief summary of how test functions appear to exercise these rules: `unit_variant_args_test` tests the standalone constructor function (`unit_variant_args()`) and compares the result with the direct enum variant (`TestEnumArgs::UnitVariantArgs`).
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks.
    *   Relevant Behavior Rules: Rule 3a (Unit + Default), Rule 1a (Unit + `#[scalar]`), Rule 4a (#[standalone_constructors]).
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unit (with `#[standalone_constructors]` and `#[arg_for_constructor]` context - though unit variants have no args)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unit_tests/standalone_constructor_args_unit_derive.rs`
        *   `module/core/former/tests/inc/enum_unit_tests/standalone_constructor_args_unit_manual.rs`
        *   `module/core/former/tests/inc/enum_unit_tests/standalone_constructor_args_unit_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to standalone_constructor_args_unit tests`

*   [✅] **Increment 7:** Document `compile_fail/unit_subform_scalar_error.rs`
    *   Detailed Plan Step 1: Read the content of the target file to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for the target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target file using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command (`cargo check --package former --tests`).
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Unit variant with `#[subform_scalar]`.
        *   Key attributes present: `#[derive(Former)]`, `#[standalone_constructors]` on the enum, `#[subform_scalar]` on the variant.
        *   Relevant "Expected Enum Former Behavior Rule IDs": Rule 2a (Unit + `#[subform_scalar]` -> Error).
        *   Brief summary of how test functions appear to exercise these rules: This is a compile-fail test file intended for use with `trybuild`. It defines the invalid structure that should cause a compilation error.
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks.
    *   Relevant Behavior Rules: Rule 2a (Unit + `#[subform_scalar]` -> Error).
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors. The compile-fail test itself is verified by `trybuild` which is part of the broader test suite, but this increment only verifies that adding comments doesn't break compilation.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unit (compile-fail scenario)
    *   Target File(s): `module/core/former/tests/inc/enum_unit_tests/compile_fail/unit_subform_scalar_error.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to unit_subform_scalar_error compile_fail test`

*   [✅] **Increment 8:** Document `basic_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Single-field tuple variants (`Break(Break)`, `Run(Run)`).
        *   Key attributes present: `#[derive(Former)]`, `#[standalone_constructors]` on the enum; `#[subform_scalar]` on the `Break` variant.
        *   Relevant "Expected Enum Former Behavior Rule IDs": 3d, 2d, 4a, 4b. Rule 1d is not applicable to this test case.
        *   Brief summary of how test functions appear to exercise these rules: `basic_only_test.rs` contains tests that call the static methods (`FunctionStep::r#break()`, `FunctionStep::run()`) and the standalone constructor (`FunctionStep::break_variant()`). These tests then use the returned subformers to set fields and call `.form()`, asserting the final enum instance matches the expected value.
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks, Structuring: Proc Macro Development Workflow.
    *   Relevant Behavior Rules: Rule 3d (Tuple + Default -> Subform), Rule 2d (Tuple + `#[subform_scalar]` -> InnerFormer), Rule 4a (#[standalone_constructors]), Rule 4b (Option 2 Logic).
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unnamed/Tuple (basic single-field subform)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/basic_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/basic_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/basic_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to basic unnamed enum tests`

*   [✅] **Increment 9:** Document `enum_named_fields_unnamed_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Zero-field unnamed (tuple) variants (`VariantZeroUnnamedDefault()`, `VariantZeroUnnamedScalar()`).
        *   Key attributes present: `#[derive(Former)]`, `#[debug]`, `#[standalone_constructors]` on the enum; `#[scalar]` on the `VariantZeroUnnamedScalar` variant.
        *   Relevant "Expected Enum Former Behavior Rule IDs": 3b, 1b, 4a. Rule 4a is applicable due to the enum attribute but not explicitly tested in the provided test file.
        *   Brief summary of how test functions appear to exercise these rules: `enum_named_fields_unnamed_only_test.rs` contains tests that call the static methods (`EnumWithNamedFields::variant_zero_unnamed_scalar()`, `EnumWithNamedFields::variant_zero_unnamed_default()`) and assert that the returned value is the direct enum variant.
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks, Structuring: Proc Macro Development Workflow.
    *   Relevant Behavior Rules: Rule 3b (Tuple + Zero-Field + Default), Rule 1b (Tuple + Zero-Field + `#[scalar]`), Rule 4a (#[standalone_constructors]).
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unnamed/Tuple (zero-field tuple variants)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/enum_named_fields_unnamed_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/enum_named_fields_unnamed_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/enum_named_fields_unnamed_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to enum_named_fields_unnamed tests`

*   [✅] **Increment 10:** Document `generics_independent_tuple_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Single-field tuple variant (`V1`) within a generic enum (`EnumG5`). The variant's field is a generic struct (`InnerG5`) instantiated with a concrete type (`TypeForU`), and the variant also contains `PhantomData<T>` to use the enum's generic `T`.
        *   Key attributes present: `#[derive(Former)]` on both the enum and inner struct. `#[scalar]` on the `V1` variant. `#[standalone_constructors]` is on the enum but not explicitly tested in these files.
        *   Relevant "Expected Enum Former Behavior Rule IDs": Rule 1d (Tuple + Single-Field + `#[scalar]` -> Scalar), Rule 4b (Option 2 Logic - related to the subformer mechanism used).
        *   Brief summary of how test functions appear to exercise these rules: The tests in `_only_test.rs` call the static method `v_1()` (provided by the derive/manual file), which returns a former for the inner type (`InnerG5<TypeForU>`). They use the setter `._0()` on this former to set the inner field and then call `.form()`. They assert this instance is equal to a manually constructed `EnumG5::V1` variant. This verifies that the `#[scalar]` attribute on the tuple variant correctly results in a constructor that takes the inner type's value (via the subformer) and produces the enum variant, handling the independent generics correctly.
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks, Structuring: Proc Macro Development Workflow.
    *   Relevant Behavior Rules: Rule 1d, Rule 4b.
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unnamed/Tuple (single-field tuple with independent generics, `#[scalar]`)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/generics_independent_tuple_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/generics_independent_tuple_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/generics_independent_tuple_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to generics_independent_tuple tests`

*   [✅] **Increment 11:** Document `generics_in_tuple_variant_tuple_*` and shared `_only_test`
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Single-field tuple variant (`Variant`) within a generic enum (`EnumOuter`), and unit variant (`OtherVariant`) within the same generic enum. The tuple variant's field is a generic struct (`InnerGeneric`) instantiated with the enum's generic `X`.
        *   Key attributes present: `#[derive(Former)]` on both the enum and inner struct. `#[debug]` on the enum. No specific variant attributes (`#[scalar]`, `#[subform_scalar]`) are used on the tested variants in this increment, relying on default behavior. `#[standalone_constructors]` is on the enum but not explicitly tested in these files.
        *   Relevant "Expected Enum Former Behavior Rule IDs": Rule 3d (Tuple + Single-Field + Default -> Subform), Rule 4b (Option 2 Logic - related to the subformer mechanism used), Rule 3a (Unit + Default).
        *   Brief summary of how test functions appear to exercise these rules: The tests in `_only_test.rs` call the static methods `variant()` (for the tuple variant) and `other_variant()` (for the unit variant) provided by the including file (derive/manual). For the tuple variant, they use the returned subformer's setter (`.inner_field()`) and `.form()`. For the unit variant, they directly assert the returned enum instance. Both test the handling of shared generics and bounds.
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks, Structuring: Proc Macro Development Workflow.
    *   Relevant Behavior Rules: Rule 3d, Rule 4b, Rule 3a.
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unnamed/Tuple (single-field tuple with shared generics, default subform) and Unit (with shared generics, default scalar)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/generics_in_tuple_variant_tuple_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/generics_in_tuple_variant_tuple_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/generics_in_tuple_variant_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to generics_in_tuple_variant_tuple tests`

*   [✅] **Increment 12:** Document `generics_shared_tuple_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Single-field tuple variant (`V1`) within a generic enum (`EnumG3`). The variant's field is a generic struct (`InnerG3`) instantiated with the enum's generic `T`.
        *   Key attributes present: `#[derive(Former)]` on both the enum and inner struct. No specific variant attributes (`#[scalar]`, `#[subform_scalar]`) are used, relying on default behavior.
        *   Relevant "Expected Enum Former Behavior Rule IDs": Rule 3d (Tuple + Single-Field + Default -> Subform), Rule 4b (Option 2 Logic - related to the subformer mechanism used).
        *   Brief summary of how test functions appear to exercise these rules: The tests in `_only_test.rs` call the static method `v_1()` (provided by the derive/manual file), which returns a former for the inner type (`InnerG3<T>`). They use the setter `.inner_field()` on this former to set the inner field and then call `.form()`. They assert this instance is equal to a manually constructed `EnumG3::V1` variant. This verifies that the default behavior for a single-field tuple variant is to generate a subformer, handling the shared generics correctly.
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks, Structuring: Proc Macro Development Workflow.
    *   Relevant Behavior Rules: Rule 3d, Rule 4b.
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unnamed/Tuple (single-field tuple with shared generics, default subform)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/generics_shared_tuple_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/generics_shared_tuple_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/generics_shared_tuple_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to generics_shared_tuple tests`

*   [✅] **Increment 13:** Document `keyword_variant_tuple_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Single-field tuple variants with keyword identifiers (`r#use(u32)`, `r#break(Break)`).
    *   Enum Aspect Focus: Unnamed/Tuple (variants with keyword identifiers, mixed scalar/subform)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/keyword_variant_tuple_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/keyword_variant_tuple_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to keyword_variant_tuple tests`
    *   **[2025-05-11/Inc 13] Note:** Pre-analysis for Increment 13 complete based on file contents. Relevant Behavior Rules identified as 1d, 3d, and 4b.
    *   **[2025-05-11/Inc 13] Note:** Reviewed target files for Increment 13. Existing documentation comments already meet the requirements. No file modifications were necessary. Verification (`cargo check --package former --tests`) passed. Increment 13 complete.

*   [✅] **Increment 14:** Document `scalar_generic_tuple_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis. (✅)
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules. (✅)
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements. (✅)
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`. (✅)
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Single-field tuple variant (`Variant1(InnerScalar<T>)`) and multi-field tuple variant (`Variant2(InnerScalar<T>, bool)`) within a generic enum (`EnumScalarGeneric<T>`). The enum and inner struct have bounds (`T: Bound`).
        *   Key attributes present: `#[derive(Former)]`, `#[debug]`, `#[PartialEq]`, `#[Clone]` on the enum in `_derive.rs`. `#[derive(Debug, Clone, PartialEq, Default)]` on the inner struct `InnerScalar`. The `#[scalar]` attribute is commented out on both `Variant1` and `Variant2` in `_derive.rs`. The `#[standalone_constructors]` attribute is not present on the enum. The manual implementation in `_manual.rs` correctly implements the scalar constructor for `Variant1` and a former builder for `Variant2`.
        *   Relevant "Expected Enum Former Behavior Rule IDs": 3d, 3f, 1d, 1f, 4b.
        *   Brief summary of how test functions appear to exercise these rules: The test `scalar_on_single_generic_tuple_variant` in `_only_test.rs` calls `EnumScalarGeneric::<MyType>::variant_1()` and expects a direct scalar value, then uses `.into()`. The test `scalar_on_multi_generic_tuple_variant` in `_only_test.rs` calls `EnumScalarGeneric::<MyType>::variant_2()`, uses setters `._0()` and `._1()`, and calls `.form()`. These tests seem to expect scalar behavior for single-field tuple variants and subformer behavior for multi-field tuple variants, which aligns with Rule 3d but contradicts Rule 3f (multi-field default should be scalar). The `#[scalar]` attributes are commented out in the derive file, which should result in default behavior. The manual file implements scalar for Variant1 and subformer for Variant2, matching the test logic but not fully matching the expected derive behavior based on the plan rules.
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks, Structuring: Proc Macro Development Workflow.
    *   Relevant Behavior Rules: Rule 3d, 3f, 1d, 1f, 4b.
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unnamed/Tuple (generic tuple variants with `#[scalar]`)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/scalar_generic_tuple_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/scalar_generic_tuple_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/scalar_generic_tuple_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to scalar_generic_tuple tests`
    *   **[2025-05-11/Inc 14] Note:** Pre-analysis for Increment 14 complete based on file contents. Relevant Behavior Rules identified as 3d, 3f, 1d, 1f, and 4b.
    *   **[2025-05-11/Inc 14] Note:** Found a discrepancy between the documented "Expected Enum Former Behavior Rules" (Rule 3f: Multi-field tuple default is scalar) and the test logic/manual implementation for `Variant2` in `scalar_generic_tuple_*` files (which tests/implements subformer behavior). Also, the `#[scalar]` attributes are commented out in the `_derive.rs` file, which should result in default behavior according to the rules, but the tests seem to expect scalar behavior for `Variant1` and subformer for `Variant2`. The documentation added will reflect the current state and behavior of the tests/manual implementation, and this discrepancy is noted here. Addressing this functional/test logic inconsistency is out of scope for this documentation task.
    *   **[2025-05-11/Inc 14] Note:** Detailed planning for Increment 14 complete. Drafted comments for target files. Noted discrepancy between Rule 3f and test/manual implementation behavior. Applied comments to target files using `write_to_file`.

*   [⏳] **Increment 15:** Document `standalone_constructor_args_tuple_*` files
    *   Detailed Plan Step 1: Read the content of the target files to perform pre-analysis.
    *   Detailed Plan Step 2: Perform pre-analysis based on file content and plan rules.
    *   Detailed Plan Step 3: Draft the `//!` comments (Purpose, Coverage, Test Relevance/Acceptance Criteria) for each target file based on pre-analysis and plan requirements.
    *   Detailed Plan Step 4: Apply the drafted comments to the target files using `write_to_file`.
    *   Detailed Plan Step 5: Request user to run verification command.
    *   Pre-Analysis:
        *   Identified enum variant structures in target file(s): Single-field tuple variant (`Variant1(u32)`), multi-field tuple variant (`Variant2(u32, String)`).
        *   Key attributes present: `#[derive(Former)]`, `#[standalone_constructors]` on the enum; `#[arg_for_constructor]` on fields within variants.
        *   Relevant "Expected Enum Former Behavior Rule IDs": 4a, 4b, 3d, 3f.
        *   Brief summary of how test functions appear to exercise these rules: The `_only_test.rs` file contains tests that call the standalone constructor functions (`variant1`, `variant2`). The `variant1` test calls `variant1(value)`, expecting a scalar return. The `variant2` test calls `variant2(value1, value2)`, expecting a scalar return. This tests Rule 4a (standalone constructors) and Rule 4b (Option 2 Logic - specifically the case where `#[arg_for_constructor]` on all fields results in a scalar standalone constructor).
    *   Crucial Design Rules: Comments and Documentation, Comments: Spaces, Comments: Focus on Rationale, Preserve Existing Tasks, Comments: Add Tasks and Label Simplifications, Comments: Annotate Addressed Tasks, Structuring: Proc Macro Development Workflow.
    *   Relevant Behavior Rules: Rule 4a, 4b, 3d, 3f.
    *   Verification Strategy: After comments are added, request user to run `cargo check --package former --tests`. The code must compile without errors.
    *   Test Matrix: N/A
    *   Enum Aspect Focus: Unnamed/Tuple (with `#[standalone_constructors]` and `#[arg_for_constructor]`)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/standalone_constructor_args_tuple_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_standalone_args_tuple_multi_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_standalone_args_tuple_single_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/standalone_constructor_args_tuple_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to standalone_constructor_args_tuple tests`
    *   Proposed Comments for `module/core/former/tests/inc/enum_unnamed_tests/standalone_constructor_args_tuple_derive.rs`:
        //! Purpose: Tests the `#[derive(Former)]` macro's generation of standalone constructor functions for tuple variants when the enum has the `#[standalone_constructors]` attribute and fields within the variants have the `#[arg_for_constructor]` attribute. This file focuses on verifying the derive-based implementation.
        //!
        //! Coverage:
        //! - Rule 4a (#[standalone_constructors]): Verifies the generation of top-level constructor functions (`variant1`, `variant2`).
        //! - Rule 4b (Option 2 Logic): Verifies that when all fields in a tuple variant have `#[arg_for_constructor]`, the standalone constructor takes arguments for those fields and returns the final enum instance (scalar style).
        //! - Rule 3d (Tuple + Single-Field + Default): Implicitly relevant as `Variant1` is a single-field tuple variant.
        //! - Rule 3f (Tuple + Multi-Field + Default): Implicitly relevant as `Variant2` is a multi-field tuple variant.
        //!
        //! Test Relevance/Acceptance Criteria:
        //! - Defines an enum `TestEnum` with single-field (`Variant1(u32)`) and multi-field (`Variant2(u32, String)`) tuple variants.
        //! - Applies `#[derive(Former)]` and `#[standalone_constructors]` to the enum.
        //! - Applies `#[arg_for_constructor]` to the fields within the variants.
        //! - Includes shared test logic from `standalone_constructor_args_tuple_only_test.rs`.
        //! - The included tests call the standalone constructor functions (`variant1(value)`, `variant2(value1, value2)`) and assert that the returned enum instances match manually constructed expected values. This verifies that the standalone constructors are generated correctly and handle arguments as specified by `#[arg_for_constructor]`.
    *   Proposed Comments for `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_standalone_args_tuple_multi_manual.rs`:
        //! Purpose: Provides a hand-written implementation of the `Former` pattern's standalone constructor function for a multi-field tuple variant (`Variant2(u32, String)`) within an enum that has the `#[standalone_constructors]` attribute and fields with `#[arg_for_constructor]`. This file focuses on demonstrating the manual implementation of the scalar standalone constructor for a multi-field tuple variant with field arguments.
        //!
        //! Coverage:
        //! - Rule 4a (#[standalone_constructors]): Manually implements the top-level constructor function (`variant2`).
        //! - Rule 4b (Option 2 Logic): Manually implements the logic for a scalar standalone constructor that takes arguments for all fields in a multi-field tuple variant.
        //! - Rule 3f (Tuple + Multi-Field + Default): Implicitly relevant as `Variant2` is a multi-field tuple variant.
        //!
        //! Test Relevance/Acceptance Criteria:
        //! - Defines the `TestEnum` enum with the `Variant2(u32, String)` variant.
        //! - Provides a hand-written `variant2` function that takes `u32` and `String` as arguments and returns `TestEnum::Variant2(u32, String)`.
        //! - This file is intended to be included by a test file that calls this function and asserts its output. It demonstrates the manual implementation corresponding to the derived behavior tested in `standalone_constructor_args_tuple_only_test.rs`.
    *   Proposed Comments for `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_standalone_args_tuple_single_manual.rs`:
        //! Purpose: Provides a hand-written implementation of the `Former` pattern's standalone constructor function for a single-field tuple variant (`Variant1(u32)`) within an enum that has the `#[standalone_constructors]` attribute and a field with `#[arg_for_constructor]`. This file focuses on demonstrating the manual implementation of the scalar standalone constructor for a single-field tuple variant with a field argument.
        //!
        //! Coverage:
        //! - Rule 4a (#[standalone_constructors]): Manually implements the top-level constructor function (`variant1`).
        //! - Rule 4b (Option 2 Logic): Manually implements the logic for a scalar standalone constructor that takes an argument for the single field in a tuple variant.
        //! - Rule 3d (Tuple + Single-Field + Default): Implicitly relevant as `Variant1` is a single-field tuple variant.
        //!
        //! Test Relevance/Acceptance Criteria:
        //! - Defines the `TestEnum` enum with the `Variant1(u32)` variant.
        //! - Provides a hand-written `variant1` function that takes `u32` as an argument and returns `TestEnum::Variant1(u32)`.
        //! - This file is intended to be included by a test file that calls this function and asserts its output. It demonstrates the manual implementation corresponding to the derived behavior tested in `standalone_constructor_args_tuple_only_test.rs`.
    *   Proposed Comments for `module/core/former/tests/inc/enum_unnamed_tests/standalone_constructor_args_tuple_only_test.rs`:
        //! Purpose: Provides shared test assertions and logic for both the derived and manual implementations of standalone constructor functions for tuple variants with `#[arg_for_constructor]` fields. It tests that standalone constructors generated/implemented when the enum has `#[standalone_constructors]` and variant fields have `#[arg_for_constructor]` behave as expected (scalar style).
        //!
        //! Coverage:
        //! - Rule 4a (#[standalone_constructors]): Tests the existence and functionality of top-level constructor functions (`variant1`, `variant2`).
        //! - Rule 4b (Option 2 Logic): Tests that these standalone constructors take arguments corresponding to the `#[arg_for_constructor]` fields and return the final enum instance.
        //! - Rule 3d (Tuple + Single-Field + Default): Implicitly tested via `Variant1`.
        //! - Rule 3f (Tuple + Multi-Field + Default): Implicitly tested via `Variant2`.
        //!
        //! Test Relevance/Acceptance Criteria:
        //! - Defines the `TestEnum` enum structure with `Variant1(u32)` and `Variant2(u32, String)`.
        //! - Contains test functions (`variant1_test`, `variant2_test`) that are included by the derive and manual test files.
        //! - Calls the standalone constructor functions (`variant1(value)`, `variant2(value1, value2)`) provided by the including file.
        //! - Asserts that the returned enum instances match manually constructed expected values (`TestEnum::Variant1(value)`, `TestEnum::Variant2(value1, value2)`). This verifies that both derived and manual standalone constructors correctly handle field arguments and produce the final enum variant.
    *   **[2025-05-11/Inc 15] Note:** Detailed planning for Increment 15 complete. Drafted comments for target files.

*   [⚫] **Increment 16:** Document `standalone_constructor_tuple_*` files
    *   Enum Aspect Focus: Unnamed/Tuple (with `#[standalone_constructors]`, no field args)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/standalone_constructor_tuple_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/standalone_constructor_tuple_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to standalone_constructor_tuple tests`

*   [⚫] **Increment 17:** Document `tuple_multi_default_*` files
    *   Enum Aspect Focus: Unnamed/Tuple (multi-field, default scalar behavior)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_default_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_default_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_default_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to tuple_multi_default tests`
*   [⚫] **Increment 18:** Document `tuple_multi_scalar_*` files
    *   Enum Aspect Focus: Unnamed/Tuple (multi-field with `#[scalar]`)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_scalar_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_scalar_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_scalar_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to tuple_multi_scalar tests`

*   [⚫] **Increment 19:** Document `tuple_multi_standalone_args_*` files
    *   Enum Aspect Focus: Unnamed/Tuple (multi-field with `#[standalone_constructors]` and `#[arg_for_constructor]`)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_standalone_args_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_standalone_args_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_standalone_args_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to tuple_multi_standalone_args tests`

*   [⚫] **Increment 20:** Document `tuple_multi_standalone_*` files
    *   Enum Aspect Focus: Unnamed/Tuple (multi-field with `#[standalone_constructors]`, no field args)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_standalone_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_standalone_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_multi_standalone_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to tuple_multi_standalone tests`

*   [⚫] **Increment 21:** Document `tuple_zero_fields_*` files
    *   Enum Aspect Focus: Unnamed/Tuple (zero-field tuple variants)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_zero_fields_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_zero_fields_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_zero_fields_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to tuple_zero_fields tests`

*   [⚫] **Increment 22:** Document `usecase1*` files
    *   Enum Aspect Focus: Unnamed/Tuple (single-field tuple, default subform, multiple variants)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/usecase1.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/usecase1_derive.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/usecase1_manual.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/usecase1_only_test.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to usecase1 unnamed enum tests`

*   [⚫] **Increment 23:** Document `compile_fail/*` files for unnamed variants
    *   Enum Aspect Focus: Unnamed/Tuple (compile-fail scenarios)
    *   Target File(s):
        *   `module/core/former/tests/inc/enum_unnamed_tests/compile_fail/tuple_multi_subform_scalar_error.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/compile_fail/tuple_single_subform_non_former_error.rs`
        *   `module/core/former/tests/inc/enum_unnamed_tests/compile_fail/tuple_zero_subform_scalar_error.rs`
    *   Commit Message: `docs(former): Add purpose and coverage to unnamed enum compile_fail tests`

### Requirements
*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules for all modifications.
*   **Comment Content:** Each targeted test file **must** have the following three `//!` (file-level doc comments) added at the very beginning, before any `use` statements or code, in the specified order:
    1.  **`//! Purpose: ...`**:
        *   Start with "Purpose:".
        *   Clearly and concisely describe the main goal of the test file. What specific aspect of the `Former` derive macro's behavior for enums is this file intended to verify?
        *   Mention the specific enum variant structure(s) (e.g., "unit variants", "single-field tuple variants with generics", "multi-field named struct variants") and any key attributes (e.g., `#[scalar]`, `#[subform_scalar]`, `#[standalone_constructors]`) being tested in this file.
        *   State whether the file is for `derive` macro testing, `manual` implementation testing, or `shared test logic` (`_only_test.rs`).
        *   For `compile_fail` tests, clearly state the specific incorrect usage or error condition it's designed to trigger and verify, referencing the relevant behavior rule that is being intentionally violated.
        *   **For `_only_test.rs` files:** The purpose should state that it provides shared test assertions/logic for both derived and manual implementations of [specific feature/variant type].
    2.  **`//! Coverage: ...`**:
        *   Start with "Coverage:".
        *   List the specific Rule IDs (e.e., "Rule 1a", "Rule 3d.i") from the "Expected Enum Former Behavior Rules" section that the tests in this file primarily demonstrate or validate.
        *   Briefly explain *what aspect* of the rule is being tested if the rule is broad and the test is specific (e.g., "Rule 4b - specifically the 'returns Former' case for standalone constructors with partial args").
        *   If a test covers interactions between multiple rules (e.g., a variant attribute combined with an enum-level attribute), list all relevant rules and briefly note the interaction.
        *   **For `_only_test.rs` files:** This comment should summarize all rules covered by the test functions within it, which are then applied to both `_derive.rs` and `_manual.rs` files that include it.
    3.  **`//! Test Relevance/Acceptance Criteria: ...`**:
        *   Start with "Test Relevance/Acceptance Criteria:".
        *   Describe the key actions performed by the test code and the primary assertions made that validate its stated purpose and coverage. This should explain *how* the test verifies the intended behavior.
        *   Be specific about the test's mechanics:
            *   What specific enum structures or attributes are defined/used in this test?
            *   What specific generated/manual methods are invoked (e.g., `MyEnum::variant_x()`, `former.field_y()`, standalone `variant_z()`)?
            *   What are the key inputs provided to these methods?
            *   What is the nature of the primary assertion (e.g., "Asserts the `got` instance (produced by the former) is equal to an `expected` instance (manually constructed to represent the correct state).", "Asserts that a subformer is returned and can be used to set inner fields.", "Asserts that a compile-time error occurs for an invalid attribute combination using `trybuild`.").
        *   **For `_derive.rs` files:** Mention that it relies on `#[derive(Former)]` for code generation and typically includes shared test logic via `include!("...")`.
        *   **For `_manual.rs` files:** Mention that it contains a hand-written former implementation and includes shared test logic via `include!("...")`.
        *   **For `compile_fail/*.rs` files:** The file contains code that intentionally uses an attribute or enum structure in a way that violates a documented behavior rule (i.e., `#[subform_scalar]` on a unit variant). The test is accepted if `trybuild` confirms this results in a compilation error, thereby validating the macro's error reporting for this specific invalid scenario."
*   **Comment Style:** All added `//!` comments should be clear, concise, grammatically correct, and follow Rust documentation comment conventions. Use Markdown for lists or emphasis if it enhances readability. Aim for reasonable line lengths.
*   **Pre-Analysis Output:** Before proposing comments for an increment, the AI must provide its pre-analysis findings for the targeted file(s) as specified in the "Increment Template".
*   **Incremental Processing:** Modify files one increment at a time, following the "Increment Template."
*   **Verification:** After each increment, request user to apply changes and run `cargo check --package former --tests`. **The code must compile successfully after adding comments. If adding comments introduces a compilation error (e.e., a syntax error in the comment itself), that specific error must be fixed. Pre-existing test failures or logic errors are out of scope.**
*   **No Functional Changes:** This task is purely for documentation and review. No functional code changes should be made to the tests or macro logic unless a comment itself causes a trivial syntax issue that prevents compilation.
*   **Handling `xxx`/`qqq` Comments:** During the review of each test file, if any existing `// xxx :` or `// qqq :` comments are encountered, their presence and a brief summary of their content should be noted in the "Notes & Insights" section of the `plan.md` for that increment. Addressing or resolving these comments is out of scope for this plan.
*   **`mod.rs` Files Review:** If, during the review of test files, it's discovered that an enum test file exists in the directories but is not declared in its respective `mod.rs` file, this should be noted in the "Notes & Insights" for that increment. Activating it is out of scope.

## Notes & Insights
*   This plan focuses exclusively on documenting existing enum tests by adding comments. It does not involve fixing failing tests or implementing new features.
*   The "Expected Enum Former Behavior Rules" section is critical for determining coverage.
*   The "Increment Template" will be used for detailed planning of each increment.
*   The `_only_test.rs` files, when shared, will have their documentation reflect their broader applicability.
*   **[Date/Inc #] Note:** Increment 3 and 11 both reference `generics_in_tuple_variant_only_test.rs`. The documentation for this shared file should be comprehensive enough to cover its usage in both unit and tuple variant contexts, likely handled in Increment 11.
*   **[Date/Inc #] Note:** The commit messages in the Increment Template now include `[enum_aspect_focus]` for better categorization.
*   **[2025-05-10/Inc 1] Note:** Started detailed planning for Increment 1: Document `unit_variant_*` files. Pre-analysis complete. Proceeding to draft and apply comments.
*   **[2025-05-10/Inc 1] Note:** Encountered repeated failures using `apply_diff` to add comments to `unit_variant_only_test.rs`. Changing strategy for Detailed Plan Step 4 to use `write_to_file` as a fallback to replace the entire file content with the desired version containing the corrected comments.
*   **[2025-05-10/Inc 1] Note:** Successfully applied comments and verified compilation with `cargo check --package former --tests`. Increment 1 complete.
*   **[2025-05-10/Inc 2] Note:** Started detailed planning for Increment 2: Document `enum_named_fields_unit_*` files. Pre-analysis complete. Proceeding to draft and apply comments. Successfully applied comments and verified compilation with `cargo check --package former --tests`. Increment 2 complete.
*   **[2025-05-10/Inc 3] Note:** Started detailed planning for Increment 3: Document `generics_in_tuple_variant_unit_*` files. Pre-analysis complete. Proceeding to draft and apply comments. Successfully applied comments and verified compilation with `cargo check --package former --tests`. Increment 3 complete.
*   **[2025-05-10/Inc 4] Note:** Started detailed planning for Increment 4: Document `keyword_variant_unit_*` files. Pre-analysis complete. Proceeding to draft and apply comments. Successfully applied comments and verified compilation with `cargo check --package former --tests`. Increment 4 complete.
*   **[2025-05-10/Inc 5] Note:** Started detailed planning for Increment 5: Document `standalone_constructor_unit_*` files. Pre-analysis complete. Proceeding to draft and apply comments. Successfully applied comments and verified compilation with `cargo check --package former --tests`. Increment 5 complete.
*   **[2025-05-10/Inc 6] Note:** Started detailed planning for Increment 6: Document `standalone_constructor_args_unit_*` files. Pre-analysis complete. Proceeding to draft and apply comments. Successfully applied comments and verified compilation with `cargo check --package former --tests`. Increment 6 complete.
