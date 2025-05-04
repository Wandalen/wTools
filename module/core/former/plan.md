# Project Plan: Incrementally Uncomment and Fix Enum Tests in `former` Crate

## Goal

*   Uncomment the `former_enum_tests` module and then incrementally uncomment **groups of related test files** (typically `_derive`, `_manual`, `_only_test` variants for a feature, following the Proc Macro Development Workflow) within `module/core/former/tests/inc/former_enum_tests/`. After uncommenting each group, perform a pre-analysis against the expected behavior, address any `// xxx :` or `// qqq :` tasks, and ensure all tests pass before proceeding to the next group.

## Context

*   Files to Include in `context.md`:
    *   `module/core/former/tests/inc/mod.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/enum_named_fields_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/enum_named_fields_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/enum_named_fields_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/keyword_variant_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/keyword_variant_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/usecase1.rs`
    *   `module/core/former/tests/inc/former_enum_tests/subform_collection_test.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs`
    *   `module/core/former_meta/src/derive_former/field.rs`
    *   `module/core/former_types/src/lib.rs` # (Example: Include key lib files)
    *   `module/core/macro_tools/src/lib.rs` # (Example: Include key lib files)
*   Crates for Documentation in `context.md`:
    *   `former`
    *   `former_meta`
    *   `former_types`
    *   `macro_tools`

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

## Failure Diagnosis Algorithm

When `cargo test` fails after uncommenting a test group (`_derive`, `_manual`, `_only_test`), follow this algorithm to determine the cause and propose a fix:

1.  **Pre-Analysis Review:** Revisit the "Expected Behavior" stated in the detailed plan for the current increment. Does the *intended* logic in the uncommented `_derive.rs`, `_manual.rs`, and `_only_test.rs` files align with this expectation? If there was a pre-analysis discrepancy noted, start there.
2.  **Analyze Error:** Examine the compiler error or test panic message provided by the user.
    *   **Compile Error in `_derive.rs`:** Likely a macro generation issue (`former_meta`) or a fundamental incompatibility between the enum structure and the "Expected Enum Former Behavior".
    *   **Compile Error in `_manual.rs`:** Likely an error in the manual implementation itself, or a mismatch with the shared `_only_test.rs` logic or the "Expected Enum Former Behavior".
    *   **Compile Error in `_only_test.rs`:** Likely an issue with the test logic itself, inconsistent naming/types between `_derive.rs` and `_manual.rs`, or a mismatch with the "Expected Enum Former Behavior".
    *   **Test Panic/Failure in `_derive.rs`:** The macro generates code that compiles but produces runtime behavior inconsistent with `_only_test.rs` or the "Expected Enum Former Behavior".
    *   **Test Panic/Failure in `_manual.rs`:** The manual implementation has runtime behavior inconsistent with `_only_test.rs` or the "Expected Enum Former Behavior".

3.  **Check `_manual.rs` Test:** Does the `_manual` test pass independently?
    *   **If YES:** The manual implementation aligns with `_only_test.rs`. The issue is likely in the macro (`former_meta`) or the `_derive.rs` setup *not matching the manual implementation or the expected behavior*. Proceed to Step 4.
    *   **If NO:** The issue is likely in the manual implementation (`_manual.rs`) or the shared test logic (`_only_test.rs`).
        *   Review `_manual.rs` against the "Expected Enum Former Behavior" rules and the logic in `_only_test.rs`. Propose fixes to `_manual.rs` or `_only_test.rs` to align them with the expected behavior.

4.  **Check `_derive.rs` Test:** Does the `_derive` test pass independently?
    *   **If YES:** (And `_manual` also passed) The issue might be subtle or related to interactions not covered by individual tests. Re-run all tests for the module. If still failing, re-evaluate the "Expected Enum Former Behavior" and the test logic.
    *   **If NO:** (And `_manual` passed) The issue is almost certainly in the macro implementation (`former_meta`) generating code that is inconsistent with the working `_manual.rs` and the "Expected Enum Former Behavior".
        *   **Compare Generated Code:** Request the user to help capture the macro-generated code. Compare this generated code side-by-side with the *working* `_manual.rs` implementation. Identify discrepancies.
        *   **Propose Macro Fix:** Based on the comparison and the "Expected Enum Former Behavior", propose specific changes to the relevant handler function within `former_meta` to make the generated code match the manual implementation's logic and the expected behavior.

5.  **Verify Behavior Model:** Ensure the final proposed fix results in behavior consistent with the "Expected Enum Former Behavior" rules. If the rules themselves seem incorrect based on the investigation, note this discrepancy and seek clarification.

6.  **Prioritize Recent Changes:** Always consider the code changes made in the current or immediately preceding steps (uncommenting files, applying previous fixes) as the most likely cause of new failures.

## Increments

*   [✅] **Increment 1:** Uncomment `former_enum_tests` Module Declaration
    *   ... (details as before) ...
*   [✅] **Increment 2:** Uncomment and Test Basic Enum (`basic_*`)
    *   ... (details as before, successfully verified) ...
*   [⏳] **Increment 3:** Uncomment and Test Enum Named Fields (`enum_named_fields_*`)
    *   **Goal:** Activate and verify tests for `EnumWithNamedFields`, covering unit, zero-field, single-field, and multi-field variants with named fields, using various attributes (`#[scalar]`, `#[subform_scalar]`) and default behaviors. **Strategy:** Isolate macro generation per variant type before running runtime tests.
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Ensure `mod enum_named_fields_derive;` is uncommented and `mod enum_named_fields_manual;` remains commented. *(Already done)*
    *   **Detailed Plan Step 2:** Modify `module/core/former/tests/inc/former_enum_tests/enum_named_fields_derive.rs`:
        *   Ensure `#[debug]` is present on `EnumWithNamedFields`. *(Already present)*
        *   Comment out the `include!( "enum_named_fields_only_test.rs" );` line.
        *   Comment out *all* variants within the `EnumWithNamedFields` definition.
    *   **Detailed Plan Step 3 (Unit Variants):**
        *   Uncomment `UnitVariantDefault` and `UnitVariantScalar` variants in `enum_named_fields_derive.rs`.
        *   **Pre-Analysis:** Expect direct constructors (Rules 1a, 3a). Handler: `unit.rs`.
        *   **Verification:** Run `cargo check --tests --package former`. Expect success. Analyze `#[debug]` output. Fix macro panics/syntax errors if they occur.
    *   **Detailed Plan Step 4 (Zero-Field Variants):**
        *   Uncomment `VariantZeroScalar {}`, `VariantZeroUnnamedDefault()`, `VariantZeroUnnamedScalar()` variants.
        *   **Pre-Analysis:** Expect direct constructors (Rules 1c, 3b). Handlers: `struct_zero.rs`, `tuple_zero.rs`.
        *   **Verification:** Run `cargo check --tests --package former`. Expect success. Analyze `#[debug]` output. Fix macro panics/syntax errors if they occur.
    *   **Detailed Plan Step 5 (Single-Field Named Variants):**
        *   Uncomment `VariantOneDefault { ... }`, `VariantOneScalar { ... }`, `VariantOneSubform { ... }`.
        *   **Pre-Analysis:** Expect implicit former (Rule 3e), direct constructor (Rule 1e), implicit former (Rule 2e) respectively. Handler: `struct_non_zero.rs`.
        *   **Verification:** Run `cargo check --tests --package former`. Expect success. Analyze `#[debug]` output. Fix macro panics/syntax errors if they occur.
    *   **Detailed Plan Step 6 (Multi-Field Named Variants):**
        *   Uncomment `VariantTwoScalar { ... }`.
        *   **Pre-Analysis:** Expect direct constructor (Rule 1g). Handler: `struct_non_zero.rs`.
        *   **Verification:** Run `cargo check --tests --package former`. Expect success. Analyze `#[debug]` output. Fix macro panics/syntax errors if they occur.
    *   **Detailed Plan Step 7 (Enable Runtime Tests):**
        *   Uncomment the `include!( "enum_named_fields_only_test.rs" );` line in `enum_named_fields_derive.rs`.
        *   **Verification:** Run `cargo test --package former --test tests -- --test-threads=1 --nocapture former_enum_tests::enum_named_fields`. Expect all tests within this module to pass. Fix any E0599 or runtime logic errors by adjusting the macro code generation based on previous analysis.
    *   **Detailed Plan Step 8 (Enable Manual Tests):**
        *   Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod enum_named_fields_manual;`.
        *   **Verification:** Run `cargo test --package former --test tests -- --test-threads=1 --nocapture former_enum_tests::enum_named_fields`. Expect all tests (derive + manual) to pass. Fix any discrepancies, prioritizing fixes in the macro if the manual implementation is correct according to the rules.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), "Expected Enum Former Behavior" rules (all).
    *   **Final Verification Strategy:** Successful execution of `cargo test` for the specific module after Step 8.

*   [⚫] **Increment 4:** Uncomment and Test Generics Independent Struct (`generics_independent_struct_*`)
    *   **Requirement:** Uncomment `generics_independent_struct_derive`, `generics_independent_struct_manual`, and `generics_independent_struct_only_test` modules. Perform pre-analysis against "Expected Enum Former Behavior" (implicit former for struct variant). Verify compilation and test success, diagnosing and fixing failures according to the "Failure Diagnosis Algorithm". Ensure `_derive` and `_manual` align with expected behavior.

*   [⚫] **Increment 5:** Uncomment and Test Generics Independent Tuple (`generics_independent_tuple_*`)
    *   **Requirement:** Uncomment `generics_independent_tuple_derive`, `generics_independent_tuple_manual`, and `generics_independent_tuple_only_test` modules. Perform pre-analysis against "Expected Enum Former Behavior" (scalar constructor for `#[scalar]` single-field tuple). Address any `xxx`/`qqq` tasks in `generics_independent_tuple_derive.rs`. Verify compilation and test success, diagnosing and fixing failures according to the "Failure Diagnosis Algorithm". Ensure `_derive` and `_manual` align with expected behavior.

*   [⚫] **Increment 6:** Uncomment and Test Generics In Tuple Variant (`generics_in_tuple_variant_*`)
    *   **Requirement:** Uncomment `generics_in_tuple_variant_derive`, `generics_in_tuple_variant_manual`, and `generics_in_tuple_variant_only_test` modules. Uncomment code within `generics_in_tuple_variant_derive.rs` if needed. Perform pre-analysis against "Expected Enum Former Behavior" (default subformer for single-field tuple). Verify compilation and test success, diagnosing and fixing failures according to the "Failure Diagnosis Algorithm". Ensure `_derive` and `_manual` align with expected behavior.

*   [⚫] **Increment 7:** Uncomment and Test Generics Shared Struct (`generics_shared_struct_*`)
    *   **Requirement:** Uncomment `generics_shared_struct_derive`, `generics_shared_struct_manual`, and `generics_shared_struct_only_test` modules. Uncomment code within `_derive` and `_manual` files if needed. Address any `xxx`/`qqq` tasks in both files. Perform pre-analysis against "Expected Enum Former Behavior" (implicit former for struct variant). Verify compilation and test success, diagnosing and fixing failures according to the "Failure Diagnosis Algorithm". Ensure `_derive` and `_manual` align with expected behavior.

*   [⚫] **Increment 8:** Uncomment and Test Generics Shared Tuple (`generics_shared_tuple_*`)
    *   **Requirement:** Uncomment `generics_shared_tuple_derive`, `generics_shared_tuple_manual`, and `generics_shared_tuple_only_test` modules. Perform pre-analysis against "Expected Enum Former Behavior" (default subformer for single-field tuple). Verify compilation and test success, diagnosing and fixing failures according to the "Failure Diagnosis Algorithm". Ensure `_derive` and `_manual` align with expected behavior.

*   [⚫] **Increment 9:** Uncomment and Test Keyword Variant (`keyword_variant_*`)
    *   **Requirement:** Uncomment `keyword_variant_derive` and `keyword_variant_only_test` modules. Perform pre-analysis against "Expected Enum Former Behavior" (checking `#[scalar]` vs `#[subform_scalar]` behavior). Verify compilation and test success, diagnosing and fixing failures according to the "Failure Diagnosis Algorithm".

*   [⚫] **Increment 10:** Uncomment and Test Scalar Generic Tuple (`scalar_generic_tuple_*`)
    *   **Requirement:** Uncomment `scalar_generic_tuple_derive`, `scalar_generic_tuple_manual`, and `scalar_generic_tuple_only_test` modules. Perform pre-analysis against "Expected Enum Former Behavior" (scalar constructor for `#[scalar]` variants). Verify compilation and test success, diagnosing and fixing failures according to the "Failure Diagnosis Algorithm". Ensure `_derive` and `_manual` align with expected behavior.

*   [⚫] **Increment 11:** Uncomment and Test Standalone Constructor Args (`standalone_constructor_args_*`)
    *   **Requirement:** Uncomment `standalone_constructor_args_derive`, `standalone_constructor_args_manual`, and `standalone_constructor_args_only_test` modules. Perform pre-analysis against "Expected Enum Former Behavior" (standalone constructors with args). Verify compilation and test success, diagnosing and fixing failures according to the "Failure Diagnosis Algorithm". Ensure `_derive` and `_manual` align with expected behavior.

*   [⚫] **Increment 12:** Uncomment and Test Standalone Constructor (`standalone_constructor_*`)
    *   **Requirement:** Uncomment `standalone_constructor_derive`, `standalone_constructor_manual`, and `standalone_constructor_only_test` modules. Perform pre-analysis against "Expected Enum Former Behavior" (standalone constructors without args). Verify compilation and test success, diagnosing and fixing failures according to the "Failure Diagnosis Algorithm". Ensure `_derive` and `_manual` align with expected behavior.

*   [⚫] **Increment 13:** Uncomment and Test Unit Variant (`unit_variant_*`)
    *   **Requirement:** Uncomment `unit_variant_derive`, `unit_variant_manual`, and `unit_variant_only_test` modules. Perform pre-analysis against "Expected Enum Former Behavior" (scalar constructor for unit variant). Verify compilation and test success, diagnosing and fixing failures according to the "Failure Diagnosis Algorithm". Ensure `_derive` and `_manual` align with expected behavior.

*   [⚫] **Increment 14:** Uncomment and Test `usecase1.rs`
    *   **Requirement:** Uncomment `usecase1` module. Uncomment code within `usecase1.rs` if needed. Address any `xxx`/`qqq` tasks. Perform pre-analysis against "Expected Enum Former Behavior" (default subformer for single-field tuple variants holding Former-derived structs). Verify compilation and test success, diagnosing and fixing failures according to the "Failure Diagnosis Algorithm".

*   [⚫] **Increment 15:** Address `subform_collection_test.rs` (Known Compile Fail)
    *   **Requirement:** Uncomment `subform_collection_test` module. Uncomment code within the file if needed. Address `xxx`/`qqq` task. Confirm with user whether to implement the feature or remove/comment out the test. Apply the chosen solution and verify compilation/test success accordingly, using the "Failure Diagnosis Algorithm" if needed.

*   [⚫] **Increment 16:** Final Verification
    *   **Requirement:** Ensure all relevant enum test modules are uncommented. Run `cargo check`, `cargo clippy`, and `cargo test` for the `former` package with `--all-targets`. Verify zero errors/warnings and all tests passing.

### Requirements

*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules for all modifications. Prioritize these rules over the existing style in the repository if conflicts arise.
*   **Detailed Increment Plan:** Before starting implementation of an increment (Step 4 of the workflow), a detailed plan for *that increment only* must be generated and approved. This plan must include:
    *   Specific file modifications planned (uncommenting modules, addressing tasks).
    *   **Pre-Analysis:** Statement of the "Expected Enum Former Behavior" for the variant/attribute combination being tested, and a brief analysis of the existing code (`_derive`, `_manual`, `_only_test`) against this expectation *before* running tests.
    *   Code snippets to be added or changed (if applicable, e.g., uncommenting, fixing tasks).
    *   Identification of any `xxx`/`qqq` tasks to be addressed.
    *   References to crucial Design Rules or "Expected Enum Former Behavior" rules.
    *   The exact verification commands to be run (`cargo check`, `cargo test`).
    *   The expected outcome of the verification (e.g., "compilation success", "tests X, Y, Z pass and align with expected behavior").
*   **Paired Testing (Proc Macro Rule):** Ensure derived macro output (`_derive` tests) is always tested alongside its intended manual equivalent (`_manual` tests) within the same increment, following the [Proc Macro: Development Workflow](#proc-macro-development-workflow) rule. The `_only_test` files, if present, should also be uncommented in the same increment. **Increments must handle the `_derive`, `_manual`, and `_only_test` files for a feature together.**
*   **Incremental Verification:** After each increment involving uncommenting a group of test files and making code changes:
    *   Ensure the relevant code compiles (`cargo check --tests --package former`).
    *   Run all active tests within the enum test module (`cargo test --package former --test tests -- --test-threads=1 --nocapture former_enum_tests`). **Analyze logs critically**, focusing on the newly added tests (`_derive` and `_manual` variants) while ensuring previously passing tests remain successful.
*   **Failure Analysis:** Before proposing fixes for failing tests, explicitly follow the "Failure Diagnosis Algorithm" defined above, incorporating the pre-analysis step.
*   **Task Handling:** Address `// xxx :` and `// qqq :` comments found in the currently uncommented test code according to the [Comments: Add Tasks and Label Simplifications](#comments-add-tasks-and-label-simplifications) rule. If a task is complex, convert it into a standard `// TODO:` comment with a brief explanation or suggest creating a dedicated issue.
*   **Component Model Exclusion:** Do *not* uncomment or attempt to fix tests within `module/core/former/tests/inc/components_tests/`. This module should remain inactive or be deleted as per the component model removal plan (`plan.md`).
*   **Minimal Changes:** Prioritize fixing existing tests with minimal changes, adhering to the [Minimal Changes](#enhancements-only-implement-whats-requested) rule. Avoid unnecessary refactoring unless required to make the test pass or adhere to rules.
*   **Plan Persistence:** Any modification to this plan (status updates, adding notes, refining steps) **must** be immediately persisted to `module/core/former/plan.md` using the `write_to_file` tool, and user confirmation of the successful write must be received before proceeding.
*   **Approval Gates:** Explicit user approval **must** be obtained before starting implementation of an increment (after detailed planning is finalized and written) and after successful verification of an increment (before moving to the next). User confirmation of successful `write_to_file` operations is also required.
*   **Context Generation:** This plan assumes a `context.md` file has been generated (via `generate_context.sh` planned and executed in prior steps) based on the files and crates listed in the `## Context` section. This `context.md` will be used during implementation.

## Notes & Insights

*   *(This section must always be present and preserved)*
*   **[Date/Inc #] Insight:** The `components_tests` module and its contents will be ignored as the component model is being removed per the other plan (`plan.md`).
*   **[Date/Inc #] Insight:** The task for `parametrized_dyn_manual.rs` (struct test) is removed from this plan's scope. It should be handled by `plan_dyn_trait_issue.md`.
*   **[Date/Inc #] Insight:** Several enum tests were initially commented out, suggesting potentially incomplete features or larger refactoring needs, especially around generics and subforms for enums. This plan addresses them incrementally, grouping related tests.
*   **[Date/Inc #] Insight:** `subform_collection_test.rs` is known to fail compilation and requires a user decision on whether to implement the underlying feature (`#[subform_entry]` for `Vec<Enum>`).