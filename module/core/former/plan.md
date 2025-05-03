# Project Plan: Uncomment and Fix Enum Tests in `former` Crate

## Goal

Uncomment the `former_enum_tests` module and all its submodules/files within `module/core/former/tests/inc/`. Address any `// xxx :` or `// qqq :` tasks found within these test files and ensure all uncommented enum tests pass.

## Requirements

*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules for all modifications. Prioritize these rules over the existing style in the repository if conflicts arise.
*   **Verification:** After each increment involving code changes, ensure the relevant code compiles (`cargo check --tests --package former`) and all *uncommented* tests pass (`cargo test --package former -- --test former_enum_tests`). Critically analyze any failures.
*   **Task Handling:** Address `// xxx :` and `// qqq :` comments found in uncommented test code. If a task is complex, convert it into a standard `// TODO:` comment with a brief explanation or suggest creating a dedicated issue.
*   **Component Model Exclusion:** Do *not* uncomment or attempt to fix tests within `module/core/former/tests/inc/components_tests/`. This module should remain inactive or be deleted as per the component model removal plan (`plan.md`).
*   **Minimal Changes:** Prioritize fixing existing tests with minimal changes. Avoid unnecessary refactoring unless required to make the test pass or adhere to rules.

## Context (Relevant Files & Modules)

The following files and modules are expected to be relevant for analysis and modification during this plan:

*   **Main Test Aggregator:**
    *   `module/core/former/tests/inc/mod.rs` (Needs uncommenting of `mod former_enum_tests;` and its contents)
*   **Enum Tests (`module/core/former/tests/inc/former_enum_tests/`):**
    *   `basic_derive.rs` (Contains `// qqq : xxx :`)
    *   `generics_in_tuple_variant_derive.rs` (Commented out)
    *   `generics_shared_struct_derive.rs` (Commented out, contains `// qqq : xxx :`)
    *   `generics_shared_struct_manual.rs` (Commented out, contains `// xxx : qqq :`)
    *   `generics_independent_tuple_derive.rs` (Contains `// xxx : qqq :`)
    *   `usecase1.rs` (Commented out, contains `// qqq : xxx :`)
    *   `subform_collection_test.rs` (Commented out, contains `// qqq : xxx :`, known compile fail)
    *   `basic_manual.rs`
    *   `basic_only_test.rs`
    *   `enum_named_fields_derive.rs`
    *   `enum_named_fields_manual.rs`
    *   `enum_named_fields_only_test.rs`
    *   `generics_independent_struct_derive.rs`
    *   `generics_independent_struct_manual.rs`
    *   `generics_independent_struct_only_test.rs`
    *   `generics_independent_tuple_manual.rs`
    *   `generics_independent_tuple_only_test.rs`
    *   `generics_in_tuple_variant_manual.rs`
    *   `generics_in_tuple_variant_only_test.rs`
    *   `generics_shared_struct_only_test.rs`
    *   `generics_shared_tuple_derive.rs`
    *   `generics_shared_tuple_manual.rs`
    *   `generics_shared_tuple_only_test.rs`
    *   `keyword_variant_derive.rs`
    *   `keyword_variant_only_test.rs`
    *   `scalar_generic_tuple_derive.rs`
    *   `scalar_generic_tuple_manual.rs`
    *   `scalar_generic_tuple_only_test.rs`
    *   `standalone_constructor_args_derive.rs`
    *   `standalone_constructor_args_manual.rs`
    *   `standalone_constructor_args_only_test.rs`
    *   `standalone_constructor_derive.rs`
    *   `standalone_constructor_manual.rs`
    *   `standalone_constructor_only_test.rs`
    *   `unit_variant_derive.rs`
    *   `unit_variant_manual.rs`
    *   `unit_variant_only_test.rs`
*   **Potential Source Code (If test failures indicate issues):**
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (and its submodules: `unit.rs`, `tuple_zero.rs`, `struct_zero.rs`, `tuple_non_zero.rs`, `struct_non_zero.rs`)
    *   `module/core/former_meta/src/derive_former/field.rs` (and its potential submodules if refactored)
    *   `module/core/former_types/src/**`
    *   **`module/core/macro_tools/src/` Files:** (Full list omitted for brevity, see previous message if needed)

## Increments

*   [⚫] **Increment 1:** Uncomment `former_enum_tests` Module & Get Baseline
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment the line `mod former_enum_tests;`. Within that block, uncomment *all* submodule declarations (`mod basic_manual;`, `mod basic_derive;`, etc.).
    *   **Detailed Plan Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests`.
    *   **Crucial Design Rules:** N/A (Analysis step).
    *   **Verification Strategy:** Analyze the output provided by the user. Identify the list of failing tests and compilation errors specifically within `former_enum_tests`. This establishes the baseline for subsequent increments. Expect compilation errors and test failures.
*   [⚫] **Increment 2:** Address Basic Enum Test Issues (`basic_derive.rs`, etc.)
    *   **Detailed Plan Step 1:** Read `module/core/former/tests/inc/former_enum_tests/basic_derive.rs`. Address the `// qqq : xxx : uncomment and make it working` task by ensuring the code is valid and removing the comment.
    *   **Detailed Plan Step 2:** Based on the baseline from Increment 1, identify any other simple compilation errors or test failures in non-generic, non-subform enum tests (e.g., `unit_variant_derive`, `enum_named_fields_derive`, `keyword_variant_derive`).
    *   **Detailed Plan Step 3:** Propose fixes for these basic issues. This might involve correcting syntax in the tests or simple adjustments in the `former_meta/src/derive_former/former_enum.rs` submodules (`unit.rs`, `tuple_zero.rs`, `struct_zero.rs`).
    *   **Detailed Plan Step 4:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::basic_derive --test former_enum_tests::unit_variant_derive --test former_enum_tests::enum_named_fields_derive --test former_enum_tests::keyword_variant_derive`. Analyze results and refine fixes.
    *   **Crucial Design Rules:** [Prioritize Reuse and Minimal Change](#prioritize-reuse-and-minimal-change).
    *   **Verification Strategy:** Successful compilation (`cargo check`) and passing tests (`cargo test`) for the targeted basic enum tests.
*   [⚫] **Increment 3:** Address Enum Generics Issues
    *   **Detailed Plan Step 1:** Systematically address the commented-out code and `// qqq : xxx :` / `// xxx : qqq :` tasks in the generics-related test files:
        *   `generics_in_tuple_variant_derive.rs` (Uncomment file)
        *   `generics_shared_struct_derive.rs` (Uncomment file, address task)
        *   `generics_shared_struct_manual.rs` (Uncomment file, address task)
        *   `generics_independent_tuple_derive.rs` (Address task)
        *   `scalar_generic_tuple_derive.rs` (Address task)
    *   **Detailed Plan Step 2:** Request user to run `cargo check --tests --package former`. Analyze compilation errors related to these files. These are likely complex and may require significant changes to how generics, bounds, and lifetimes are handled in `former_meta/src/derive_former/former_enum.rs` and its submodules.
    *   **Detailed Plan Step 3:** Propose fixes incrementally, focusing on one test file/scenario at a time (e.g., fix `generics_shared_tuple`, then `generics_shared_struct`, etc.). This might involve adjusting how generic parameters are decomposed, merged, or applied in generated code (formers, storage, end handlers). Pay close attention to bound propagation and lifetime handling.
    *   **Detailed Plan Step 4:** After proposing fixes for a specific test file, request user to run `cargo test --package former --test <test_name>`. Analyze results and refine fixes. Repeat for each generics test file.
    *   **Crucial Design Rules:** N/A (Macro implementation focus).
    *   **Verification Strategy:** Successful compilation (`cargo check`) and passing tests (`cargo test`) for each of the generics-related enum test files individually after fixes are applied.
*   [⚫] **Increment 4:** Address `former_enum_tests` Issue (`usecase1.rs`)
    *   **Detailed Plan Step 1:** Read `module/core/former/tests/inc/former_enum_tests/usecase1.rs`. Uncomment the file content.
    *   **Detailed Plan Step 2:** Address the `// qqq : xxx : uncomment and make it working` task. Analyze compilation errors or test failures. This test uses default subformer generation for enum variants holding structs that also derive `Former`.
    *   **Detailed Plan Step 3:** Investigate `former_meta/src/derive_former/former_enum.rs` (likely `tuple_non_zero.rs` or `struct_non_zero.rs`) to ensure the default subformer logic (when no `#[scalar]` or `#[subform_scalar]` is present) is correctly generated for single-field variants holding `Former`-derived types. Propose fixes.
    *   **Detailed Plan Step 4:** Request user to run `cargo test --package former --test usecase1`. Analyze failures and refine fixes.
    *   **Crucial Design Rules:** N/A (Macro implementation focus).
    *   **Verification Strategy:** Successful compilation (`cargo check`) and passing test (`cargo test`) for `usecase1`.
*   [⚫] **Increment 5:** Address `former_enum_tests` Issue (`subform_collection_test.rs` - Known Compile Fail)
    *   **Detailed Plan Step 1:** Read `module/core/former/tests/inc/former_enum_tests/subform_collection_test.rs`. Uncomment the file content.
    *   **Detailed Plan Step 2:** Address the `// qqq : xxx : make it working` task. The comments indicate this test is expected to fail compilation because `#[subform_entry]` on `Vec<Enum>` is not currently supported.
    *   **Detailed Plan Step 3:** **Confirm with the user if this feature (`#[subform_entry]` for `Vec<Enum>`) should be implemented.**
        *   **If YES:** This is a significant feature addition. Propose a sub-plan to implement the necessary logic in `former_meta/src/derive_former/field.rs` (specifically `subform_entry_setter`) to handle enum variants. This involves generating code that can somehow select and start the correct former for a *specific enum variant* within the collection context. This is non-trivial.
        *   **If NO:** Modify the test file. Remove the test code and the file, adding a comment in `mod.rs` explaining the limitation.
    *   **Detailed Plan Step 4:** Apply the chosen solution (implement feature or remove test).
    *   **Crucial Design Rules:** [Enhancements: Only Implement What’s Requested](#enhancements-only-implement-whats-requested) (Requires user confirmation before implementing the feature).
    *   **Verification Strategy:** If implemented: Successful compilation (`cargo check`) and passing test (`cargo test --package former --test subform_collection_test`). If removed: Successful compilation (`cargo check --tests --package former`) and no test failures related to this file.
*   [⚫] **Increment 6:** Final Verification
    *   **Detailed Plan Step 1:** Ensure no non-component enum test modules or files remain commented out in `module/core/former/tests/inc/mod.rs`.
    *   **Detailed Plan Step 2:** Request user to run `cargo check --tests --package former --all-targets`.
    *   **Detailed Plan Step 3:** Request user to run `cargo clippy --package former --all-targets -- -D warnings`.
    *   **Detailed Plan Step 4:** Request user to run `cargo test --package former --all-targets`.
    *   **Crucial Design Rules:** N/A (Verification step).
    *   **Verification Strategy:** Analyze output from user. Expect zero errors and zero warnings from `check` and `clippy`. Expect all tests for the `former` package to pass, focusing on the `former_enum_tests` results.

## Notes & Insights

*   *(This section must always be present and preserved)*
*   **[Date/Inc #] Insight:** The `components_tests` module and its contents will be ignored as the component model is being removed per the other plan (`plan.md`).
*   **[Date/Inc #] Insight:** The task for `parametrized_dyn_manual.rs` (struct test) is removed from this plan's scope. It should be handled by `plan_dyn_trait_issue.md`.
*   **[Date/Inc #] Insight:** Several enum tests are fully commented out, suggesting potentially incomplete features or larger refactoring needs, especially around generics and subforms for enums.
*   **[Date/Inc #] Insight:** `subform_collection_test.rs` is known to fail compilation and needs specific attention, likely requiring a feature decision.