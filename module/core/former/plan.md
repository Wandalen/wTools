# Project Plan: Uncomment and Fix Tests in `former` Crate

## Goal

Uncomment all non-component-related test modules and individual tests within `module/core/former/tests/inc/`. Address any `// xxx :` or `// qqq :` tasks found within these test files and ensure all uncommented tests pass.

## Requirements

*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules for all modifications. Prioritize these rules over the existing style in the repository if conflicts arise.
*   **Verification:** After each increment involving code changes, ensure the relevant code compiles (`cargo check --tests --package former`) and all *uncommented* tests pass (`cargo test --package former`). Critically analyze any failures.
*   **Task Handling:** Address `// xxx :` and `// qqq :` comments found in uncommented test code. If a task is complex, convert it into a standard `// TODO:` comment with a brief explanation or suggest creating a dedicated issue.
*   **Component Model Exclusion:** Do *not* uncomment or attempt to fix tests within `module/core/former/tests/inc/components_tests/`. This module should remain inactive or be deleted as per the component model removal plan (`plan.md`).
*   **Integration:** Increment 5 must coordinate with and potentially update `module/core/former/plan_dyn_trait_issue.md` when addressing `parametrized_dyn_manual.rs`.
*   **Minimal Changes:** Prioritize fixing existing tests with minimal changes. Avoid unnecessary refactoring unless required to make the test pass or adhere to rules.

## Context (Relevant Files & Modules)

The following files and modules are expected to be relevant for analysis and modification during this plan:

*   **Main Test Aggregator:**
    *   `module/core/former/tests/inc/mod.rs` (Needs uncommenting of modules)
*   **Struct Tests (`module/core/former/tests/inc/former_struct_tests/`):**
    *   `collection_former_linked_list.rs` (Contains `// qqq :`)
    *   `collection_former_vec.rs` (Contains `// qqq :`)
    *   `tuple_struct.rs` (Commented out, contains `// xxx : qqq :`)
    *   `keyword_subform_derive.rs` (Contains `// qqq : xxx :`)
    *   `parametrized_dyn_manual.rs` (Commented out, contains `// xxx2 : qqq2 :`, related to `plan_dyn_trait_issue.md`)
    *   Other files in this directory may need review if tests fail after uncommenting `mod former_struct_tests;`.
*   **Enum Tests (`module/core/former/tests/inc/former_enum_tests/`):**
    *   `basic_derive.rs` (Contains `// qqq : xxx :`)
    *   `generics_in_tuple_variant_derive.rs` (Commented out)
    *   `generics_shared_struct_derive.rs` (Commented out, contains `// qqq : xxx :`)
    *   `generics_shared_struct_manual.rs` (Commented out, contains `// xxx : qqq :`)
    *   `generics_independent_tuple_derive.rs` (Contains `// xxx : qqq :`)
    *   `usecase1.rs` (Commented out, contains `// qqq : xxx :`)
    *   `subform_collection_test.rs` (Commented out, contains `// qqq : xxx :`, known compile fail)
    *   Other files in this directory may need review if tests fail after uncommenting `mod former_enum_tests;`.
*   **Potential Source Code (If test failures indicate issues):**
    *   `module/core/former_meta/src/derive_former/former_struct.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (and its submodules)
    *   `module/core/former_types/src/**` (especially collection implementations)
    *   **`module/core/macro_tools/src/` Files:**
        *   `lib.rs`
        *   `attr.rs`
        *   `attr_prop.rs`
        *   `attr_prop/boolean.rs`
        *   `attr_prop/boolean_optional.rs`
        *   `attr_prop/singletone.rs`
        *   `attr_prop/singletone_optional.rs`
        *   `attr_prop/syn.rs`
        *   `attr_prop/syn_optional.rs`
        *   `components.rs`
        *   `container_kind.rs`
        *   `ct.rs`
        *   `ct/str.rs`
        *   `derive.rs`
        *   `derive_former.rs`
        *   `derive_former/field.rs`
        *   `derive_former/field_attrs.rs`
        *   `derive_former/former_enum.rs`
        *   `derive_former/former_enum/struct_non_zero.rs`
        *   `derive_former/former_enum/struct_zero.rs`
        *   `derive_former/former_enum/tuple_non_zero.rs`
        *   `derive_former/former_enum/tuple_zero.rs`
        *   `derive_former/former_enum/unit.rs`
        *   `derive_former/former_struct.rs`
        *   `derive_former/struct_attrs.rs`
        *   `diag.rs`
        *   `equation.rs`
        *   `generic_args.rs`
        *   `generic_params.rs`
        *   `ident.rs`
        *   `item.rs`
        *   `item_struct.rs`
        *   `iter.rs`
        *   `kw.rs`
        *   `name.rs`
        *   `phantom.rs`
        *   `punctuated.rs`
        *   `quantifier.rs`
        *   `struct_like.rs`
        *   `tokens.rs`
        *   `typ.rs`
        *   `typed.rs`
*   **Related Plan:**
    *   `module/core/former/plan_dyn_trait_issue.md`

## Increments

*   [⚫] **Increment 1:** Uncomment Main Test Modules & Get Baseline
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment the line `mod former_struct_tests;`. Keep `mod former_enum_tests;` and `mod components_tests;` commented for now.
    *   **Detailed Plan Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former`.
    *   **Crucial Design Rules:** N/A (Analysis step).
    *   **Verification Strategy:** Analyze the output provided by the user. Identify the list of failing tests within `former_struct_tests`. This establishes the baseline for subsequent increments. Expect compilation errors and test failures.
*   [⚫] **Increment 2:** Address `former_struct_tests` Issues (Collection Formers)
    *   **Detailed Plan Step 1:** Read `module/core/former/tests/inc/former_struct_tests/collection_former_linked_list.rs`. Locate the `// qqq : uncomment and make it working` comment within the `entity_to` test. Uncomment the two `let got = ...` lines related to `EntityToStorage`.
    *   **Detailed Plan Step 2:** Read `module/core/former/tests/inc/former_struct_tests/collection_former_vec.rs`. Locate the `// qqq : uncomment and make it working` comment within the `entity_to` test. Uncomment the two `let got = ...` lines related to `EntityToStorage`.
    *   **Detailed Plan Step 3:** Request user to run `cargo check --tests --package former`. Analyze potential compilation errors. Hypothesize that `EntityToStorage` might not be correctly implemented or derived for these collection types. If errors occur, investigate `former_types/src/collection/linked_list.rs` and `former_types/src/collection/vector.rs` and propose fixes to the `impl crate::EntityToStorage` blocks.
    *   **Detailed Plan Step 4:** Request user to run `cargo test --package former --test collection_former_linked_list` and `cargo test --package former --test collection_former_vec`. Analyze any failures and propose necessary code corrections.
    *   **Crucial Design Rules:** [Prioritize Reuse and Minimal Change](#prioritize-reuse-and-minimal-change).
    *   **Verification Strategy:** Successful compilation (`cargo check`) and passing tests (`cargo test`) for `collection_former_linked_list` and `collection_former_vec`.
*   [⚫] **Increment 3:** Address `former_struct_tests` Issue (`tuple_struct.rs`)
    *   **Detailed Plan Step 1:** Read `module/core/former/tests/inc/former_struct_tests/tuple_struct.rs`.
    *   **Detailed Plan Step 2:** Uncomment the entire file content.
    *   **Detailed Plan Step 3:** Analyze the `// xxx : qqq : make that working` task. The code attempts to use `#[subform_collection]` on a field within a tuple struct `Struct1( #[ subform_collection ] HashMap< Key, Value > )`.
    *   **Detailed Plan Step 4:** Request user to run `cargo check --tests --package former`. Analyze potential compilation errors. Hypothesize that the `Former` derive macro in `former_meta` might not correctly handle attributes on fields within tuple structs.
    *   **Detailed Plan Step 5:** Investigate `former_meta/src/derive_former/former_struct.rs` and potentially `field.rs`/`field_attrs.rs` to understand how tuple struct fields and their attributes are processed. Propose necessary fixes to the macro code to support this pattern.
    *   **Detailed Plan Step 6:** Request user to run `cargo test --package former --test tuple_struct`. Analyze failures and refine fixes.
    *   **Crucial Design Rules:** N/A (Macro implementation focus).
    *   **Verification Strategy:** Successful compilation (`cargo check`) and passing test (`cargo test`) for `tuple_struct`.
*   [⚫] **Increment 4:** Address `former_struct_tests` Issue (`keyword_subform_derive.rs`)
    *   **Detailed Plan Step 1:** Read `module/core/former/tests/inc/former_struct_tests/keyword_subform_derive.rs`.
    *   **Detailed Plan Step 2:** Analyze the `// qqq : xxx : fix it` task. The test uses keywords (`for`, `match`, `impl`) as field names with subform attributes.
    *   **Detailed Plan Step 3:** Request user to run `cargo check --tests --package former`. Analyze potential compilation errors. Hypothesize that the macro might not be generating method names or struct names using raw identifiers (`r#`) correctly when field names are keywords.
    *   **Detailed Plan Step 4:** Investigate `former_meta/src/derive_former/field.rs` (specifically the setter generation functions like `subform_collection_setter`, `subform_entry_setter`, `subform_scalar_setter`) and `macro_tools/src/ident.rs`. Ensure `ident_maybe_raw` is used appropriately when generating identifiers based on field names. Propose fixes.
    *   **Detailed Plan Step 5:** Request user to run `cargo test --package former --test keyword_subform_derive`. Analyze failures and refine fixes.
    *   **Crucial Design Rules:** N/A (Macro implementation focus).
    *   **Verification Strategy:** Successful compilation (`cargo check`) and passing test (`cargo test`) for `keyword_subform_derive`.
*   [⚫] **Increment 5:** Address `former_struct_tests` Issue (`parametrized_dyn_manual.rs` - Integrate with `plan_dyn_trait_issue.md`)
    *   **Detailed Plan Step 1:** Read `module/core/former/tests/inc/former_struct_tests/parametrized_dyn_manual.rs` and `module/core/former/plan_dyn_trait_issue.md`.
    *   **Detailed Plan Step 2:** Systematically execute the detailed plan steps outlined in `plan_dyn_trait_issue.md`. This involves:
        *   Uncommenting the manual implementation in `parametrized_dyn_manual.rs`.
        *   Applying codestyle fixes.
        *   Creating the shared test logic file (`parametrized_dyn_only_test.rs`).
        *   Verifying the manual implementation compiles and passes tests.
        *   Creating the derive macro invocation site (`parametrized_dyn_derive.rs`).
        *   Analyzing the macro failure (likely related to handling `dyn Trait` and lifetimes).
        *   Implementing the fix in `former_meta` (likely `derive_former/field.rs` or `derive_former/former_struct.rs`).
    *   **Detailed Plan Step 3:** Update `plan_dyn_trait_issue.md` to reflect the progress and resolution.
    *   **Crucial Design Rules:** Follow rules specified in `plan_dyn_trait_issue.md`.
    *   **Verification Strategy:** Successful compilation (`cargo check`) and passing tests (`cargo test`) for both `parametrized_dyn_manual` and the new `parametrized_dyn_derive` test.
*   [⚫] **Increment 6:** Uncomment `former_enum_tests` Submodules & Address Basic Issues
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment the line `mod former_enum_tests;`. Within that block, ensure only the following are initially uncommented: `mod basic_derive;`, `mod unit_variant_derive;`, `mod enum_named_fields_derive;`, `mod keyword_variant_derive;`. Keep others commented.
    *   **Detailed Plan Step 2:** Read `module/core/former/tests/inc/former_enum_tests/basic_derive.rs`. Address the `// qqq : xxx : uncomment and make it working` task by ensuring the code is valid and removing the comment.
    *   **Detailed Plan Step 3:** Request user to run `cargo check --tests --package former`. Analyze and fix any compilation errors in the newly uncommented enum tests.
    *   **Detailed Plan Step 4:** Request user to run `cargo test --package former -- --test former_enum_tests::basic_derive --test former_enum_tests::unit_variant_derive --test former_enum_tests::enum_named_fields_derive --test former_enum_tests::keyword_variant_derive`. Analyze failures and propose fixes.
    *   **Crucial Design Rules:** [Prioritize Reuse and Minimal Change](#prioritize-reuse-and-minimal-change).
    *   **Verification Strategy:** Successful compilation (`cargo check`) and passing tests (`cargo test`) for the specified basic enum tests.
*   [⚫] **Increment 7:** Address `former_enum_tests` Generics Issues
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment the generics-related test files within the `former_enum_tests` block: `generics_in_tuple_variant_derive.rs`, `generics_shared_struct_derive.rs`, `generics_shared_struct_manual.rs`, `generics_independent_tuple_derive.rs`, `scalar_generic_tuple_derive.rs`.
    *   **Detailed Plan Step 2:** Address any `// qqq : xxx :` or similar tasks within these files by uncommenting code and removing the task comments.
    *   **Detailed Plan Step 3:** Request user to run `cargo check --tests --package former`. Analyze compilation errors. These are likely complex and may require significant changes to how generics, bounds, and lifetimes are handled in `former_meta/src/derive_former/former_enum.rs` and its submodules.
    *   **Detailed Plan Step 4:** Propose fixes incrementally, focusing on one test file/scenario at a time (e.g., fix `generics_shared_tuple`, then `generics_shared_struct`, etc.). This might involve adjusting how generic parameters are decomposed, merged, or applied in generated code (formers, storage, end handlers).
    *   **Detailed Plan Step 5:** After proposing fixes for a specific test file, request user to run `cargo test --package former --test <test_name>`. Analyze results and refine fixes. Repeat for each generics test file.
    *   **Crucial Design Rules:** N/A (Macro implementation focus).
    *   **Verification Strategy:** Successful compilation (`cargo check`) and passing tests (`cargo test`) for each of the generics-related enum test files individually after fixes are applied.
*   [⚫] **Increment 8:** Address `former_enum_tests` Issue (`usecase1.rs`)
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod usecase1;`.
    *   **Detailed Plan Step 2:** Read `module/core/former/tests/inc/former_enum_tests/usecase1.rs`. Uncomment the file content.
    *   **Detailed Plan Step 3:** Address the `// qqq : xxx : uncomment and make it working` task. Analyze compilation errors or test failures. This test uses default subformer generation for enum variants holding structs that also derive `Former`.
    *   **Detailed Plan Step 4:** Investigate `former_meta/src/derive_former/former_enum.rs` (likely `tuple_non_zero.rs` or `struct_non_zero.rs`) to ensure the default subformer logic (when no `#[scalar]` or `#[subform_scalar]` is present) is correctly generated for single-field variants. Propose fixes.
    *   **Detailed Plan Step 5:** Request user to run `cargo test --package former --test usecase1`. Analyze failures and refine fixes.
    *   **Crucial Design Rules:** N/A (Macro implementation focus).
    *   **Verification Strategy:** Successful compilation (`cargo check`) and passing test (`cargo test`) for `usecase1`.
*   [⚫] **Increment 9:** Address `former_enum_tests` Issue (`subform_collection_test.rs` - Known Compile Fail)
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod subform_collection_test;`.
    *   **Detailed Plan Step 2:** Read `module/core/former/tests/inc/former_enum_tests/subform_collection_test.rs`. Uncomment the file content.
    *   **Detailed Plan Step 3:** Address the `// qqq : xxx : make it working` task. The comments indicate this test is expected to fail compilation because `#[subform_entry]` on `Vec<Enum>` is not currently supported.
    *   **Detailed Plan Step 4:** **Confirm with the user if this feature (`#[subform_entry]` for `Vec<Enum>`) should be implemented.**
        *   **If YES:** This is a significant feature addition. Propose a sub-plan to implement the necessary logic in `former_meta/src/derive_former/field.rs` (specifically `subform_entry_setter`) to handle enum variants. This involves generating code that can somehow select and start the correct former for a *specific enum variant* within the collection context. This is non-trivial.
        *   **If NO:** Modify the test file. Either remove the test code entirely or wrap the failing code block with `#[test] #[should_panic]` (if it panics at runtime) or configure the test suite (e.g., using `compiletest_rs` or similar, though that's outside the scope of direct code generation here) to expect a compilation failure. The simplest approach is likely removing the test code and the file, adding a comment in `mod.rs` explaining the limitation.
    *   **Detailed Plan Step 5:** Apply the chosen solution (implement feature or modify/remove test).
    *   **Crucial Design Rules:** [Enhancements: Only Implement What’s Requested](#enhancements-only-implement-whats-requested) (Requires user confirmation before implementing the feature).
    *   **Verification Strategy:** If implemented: Successful compilation (`cargo check`) and passing test (`cargo test --package former --test subform_collection_test`). If removed/modified: Successful compilation (`cargo check --tests --package former`) and no test failures related to this file.
*   [⚫] **Increment 10:** Final Verification
    *   **Detailed Plan Step 1:** Ensure no non-component test modules or files remain commented out in `module/core/former/tests/inc/mod.rs`.
    *   **Detailed Plan Step 2:** Request user to run `cargo check --tests --package former --all-targets`.
    *   **Detailed Plan Step 3:** Request user to run `cargo clippy --package former --all-targets -- -D warnings`.
    *   **Detailed Plan Step 4:** Request user to run `cargo test --package former --all-targets`.
    *   **Crucial Design Rules:** N/A (Verification step).
    *   **Verification Strategy:** Analyze output from user. Expect zero errors and zero warnings from `check` and `clippy`. Expect all tests for the `former` package to pass.

## Notes & Insights

*   *(This section must always be present and preserved)*
*   **[Date/Inc #] Insight:** The `components_tests` module and its contents will be ignored as the component model is being removed per the other plan (`plan.md`).
*   **[Date/Inc #] Insight:** The task for `parametrized_dyn_manual.rs` seems complex and has its own plan (`plan_dyn_trait_issue.md`). This plan will integrate that work in Increment 5.
*   **[Date/Inc #] Insight:** Several enum tests are fully commented out, suggesting potentially incomplete features or larger refactoring needs, especially around generics and subforms for enums.
*   **[Date/Inc #] Insight:** `subform_collection_test.rs` is known to fail compilation and needs specific attention.