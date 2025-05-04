# Project Plan: Incrementally Uncomment and Fix Enum Tests in `former` Crate

## Goal

Uncomment the `former_enum_tests` module and then incrementally uncomment **each individual test file** within `module/core/former/tests/inc/former_enum_tests/`. After uncommenting each file, address any `// xxx :` or `// qqq :` tasks found within that specific file and ensure its tests pass before proceeding to the next.

## Requirements

*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules for all modifications. Prioritize these rules over the existing style in the repository if conflicts arise.
*   **Incremental Verification:** After each increment involving uncommenting a test file and making code changes:
    *   Ensure the relevant code compiles (`cargo check --tests --package former`).
    *   Ensure the tests within the *just uncommented file* pass (`cargo test --package former -- --test former_enum_tests::<module_name>`). Critically analyze any failures.
*   **Task Handling:** Address `// xxx :` and `// qqq :` comments found in the currently uncommented test code. If a task is complex, convert it into a standard `// TODO:` comment with a brief explanation or suggest creating a dedicated issue.
*   **Component Model Exclusion:** Do *not* uncomment or attempt to fix tests within `module/core/former/tests/inc/components_tests/`. This module should remain inactive or be deleted as per the component model removal plan (`plan.md`).
*   **Minimal Changes:** Prioritize fixing existing tests with minimal changes. Avoid unnecessary refactoring unless required to make the test pass or adhere to rules.

## Context (Relevant Files & Modules)

The following files and modules are expected to be relevant for analysis and modification during this plan:

*   **Main Test Aggregator:**
    *   `module/core/former/tests/inc/mod.rs` (Needs uncommenting of `mod former_enum_tests;` and then individual `mod <test_file>;` lines within it)
*   **Enum Tests Directory:** `module/core/former/tests/inc/former_enum_tests/`
*   **Individual Enum Test Files (To be uncommented one by one):**
    *   `basic_derive.rs` (Contains `// qqq : xxx :`)
    *   `basic_manual.rs`
    *   `basic_only_test.rs`
    *   `enum_named_fields_derive.rs`
    *   `enum_named_fields_manual.rs`
    *   `enum_named_fields_only_test.rs`
    *   `generics_independent_struct_derive.rs`
    *   `generics_independent_struct_manual.rs`
    *   `generics_independent_struct_only_test.rs`
    *   `generics_independent_tuple_derive.rs` (Contains `// xxx : qqq :`)
    *   `generics_independent_tuple_manual.rs`
    *   `generics_independent_tuple_only_test.rs`
    *   `generics_in_tuple_variant_derive.rs` (Commented out)
    *   `generics_in_tuple_variant_manual.rs`
    *   `generics_in_tuple_variant_only_test.rs`
    *   `generics_shared_struct_derive.rs` (Commented out, contains `// qqq : xxx :`)
    *   `generics_shared_struct_manual.rs` (Commented out, contains `// xxx : qqq :`)
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
    *   `subform_collection_test.rs` (Commented out, contains `// qqq : xxx :`, known compile fail)
    *   `unit_variant_derive.rs`
    *   `unit_variant_manual.rs`
    *   `unit_variant_only_test.rs`
    *   `usecase1.rs` (Commented out, contains `// qqq : xxx :`)
*   **Potential Source Code (If test failures indicate issues):**
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (and its submodules: `unit.rs`, `tuple_zero.rs`, `struct_zero.rs`, `tuple_non_zero.rs`, `struct_non_zero.rs`)
    *   `module/core/former_meta/src/derive_former/field.rs` (and its potential submodules if refactored)
    *   `module/core/former_types/src/**`
    *   `module/core/macro_tools/src/**`

## Increments

*   [⚫] **Increment 1:** Uncomment `former_enum_tests` Module Declaration
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment *only* the line `mod former_enum_tests;`. Leave all `mod <test_file>;` lines within that block commented for now.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests`.
    *   **Verification:** Analyze output. Expect compilation success and likely zero tests run for `former_enum_tests` as all its submodules are still commented out. This confirms the module itself is recognized.

*   [⚫] **Increment 2:** Uncomment and Test `basic_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment the line `mod basic_derive;` within the `mod former_enum_tests { ... }` block.
    *   **Step 2:** Read `module/core/former/tests/inc/former_enum_tests/basic_derive.rs`. Address the `// qqq : xxx : uncomment and make it working` task. Propose necessary code changes (if any) to make the test valid and remove the task comment.
    *   **Step 3:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::basic_derive`.
    *   **Step 4:** Analyze results. If failures occur, investigate (potentially in `former_meta` source) and propose fixes.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::basic_derive`.

*   [⚫] **Increment 3:** Uncomment and Test `basic_manual.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod basic_manual;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::basic_manual`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::basic_manual`.

*   [⚫] **Increment 4:** Uncomment and Test `basic_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod basic_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::basic_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::basic_only_test`.

*   [⚫] **Increment 5:** Uncomment and Test `enum_named_fields_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod enum_named_fields_derive;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::enum_named_fields_derive`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::enum_named_fields_derive`.

*   [⚫] **Increment 6:** Uncomment and Test `enum_named_fields_manual.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod enum_named_fields_manual;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::enum_named_fields_manual`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::enum_named_fields_manual`.

*   [⚫] **Increment 7:** Uncomment and Test `enum_named_fields_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod enum_named_fields_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::enum_named_fields_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::enum_named_fields_only_test`.

*   [⚫] **Increment 8:** Uncomment and Test `generics_independent_struct_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_independent_struct_derive;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_independent_struct_derive`.
    *   **Step 3:** Analyze results. Propose fixes if needed (may involve `former_meta`).
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_independent_struct_derive`.

*   [⚫] **Increment 9:** Uncomment and Test `generics_independent_struct_manual.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_independent_struct_manual;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_independent_struct_manual`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_independent_struct_manual`.

*   [⚫] **Increment 10:** Uncomment and Test `generics_independent_struct_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_independent_struct_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_independent_struct_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_independent_struct_only_test`.

*   [⚫] **Increment 11:** Uncomment and Test `generics_independent_tuple_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_independent_tuple_derive;`.
    *   **Step 2:** Read `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_derive.rs`. Address the `// xxx : qqq : uncomment and make it working` task. Propose necessary code changes and remove the task comment.
    *   **Step 3:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_independent_tuple_derive`.
    *   **Step 4:** Analyze results. Propose fixes if needed (may involve `former_meta`).
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_independent_tuple_derive`.

*   [⚫] **Increment 12:** Uncomment and Test `generics_independent_tuple_manual.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_independent_tuple_manual;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_independent_tuple_manual`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_independent_tuple_manual`.

*   [⚫] **Increment 13:** Uncomment and Test `generics_independent_tuple_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_independent_tuple_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_independent_tuple_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_independent_tuple_only_test`.

*   [⚫] **Increment 14:** Uncomment and Test `generics_in_tuple_variant_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_in_tuple_variant_derive;`.
    *   **Step 2:** Read `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_derive.rs`. Uncomment the file content if necessary.
    *   **Step 3:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_in_tuple_variant_derive`.
    *   **Step 4:** Analyze results. Propose fixes if needed (may involve `former_meta`).
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_in_tuple_variant_derive`.

*   [⚫] **Increment 15:** Uncomment and Test `generics_in_tuple_variant_manual.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_in_tuple_variant_manual;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_in_tuple_variant_manual`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_in_tuple_variant_manual`.

*   [⚫] **Increment 16:** Uncomment and Test `generics_in_tuple_variant_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_in_tuple_variant_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_in_tuple_variant_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_in_tuple_variant_only_test`.

*   [⚫] **Increment 17:** Uncomment and Test `generics_shared_struct_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_shared_struct_derive;`.
    *   **Step 2:** Read `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_derive.rs`. Uncomment file content if necessary. Address the `// qqq : xxx : uncomment and make it working` task. Propose necessary code changes and remove the task comment.
    *   **Step 3:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_shared_struct_derive`.
    *   **Step 4:** Analyze results. Propose fixes if needed (may involve `former_meta`).
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_shared_struct_derive`.

*   [⚫] **Increment 18:** Uncomment and Test `generics_shared_struct_manual.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_shared_struct_manual;`.
    *   **Step 2:** Read `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_manual.rs`. Uncomment file content if necessary. Address the `// xxx : qqq : uncomment and make it working` task. Propose necessary code changes and remove the task comment.
    *   **Step 3:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_shared_struct_manual`.
    *   **Step 4:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_shared_struct_manual`.

*   [⚫] **Increment 19:** Uncomment and Test `generics_shared_struct_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_shared_struct_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_shared_struct_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_shared_struct_only_test`.

*   [⚫] **Increment 20:** Uncomment and Test `generics_shared_tuple_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_shared_tuple_derive;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_shared_tuple_derive`.
    *   **Step 3:** Analyze results. Propose fixes if needed (may involve `former_meta`).
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_shared_tuple_derive`.

*   [⚫] **Increment 21:** Uncomment and Test `generics_shared_tuple_manual.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_shared_tuple_manual;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_shared_tuple_manual`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_shared_tuple_manual`.

*   [⚫] **Increment 22:** Uncomment and Test `generics_shared_tuple_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_shared_tuple_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::generics_shared_tuple_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::generics_shared_tuple_only_test`.

*   [⚫] **Increment 23:** Uncomment and Test `keyword_variant_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod keyword_variant_derive;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::keyword_variant_derive`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::keyword_variant_derive`.

*   [⚫] **Increment 24:** Uncomment and Test `keyword_variant_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod keyword_variant_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::keyword_variant_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::keyword_variant_only_test`.

*   [⚫] **Increment 25:** Uncomment and Test `scalar_generic_tuple_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod scalar_generic_tuple_derive;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::scalar_generic_tuple_derive`.
    *   **Step 3:** Analyze results. Propose fixes if needed (may involve `former_meta`).
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::scalar_generic_tuple_derive`.

*   [⚫] **Increment 26:** Uncomment and Test `scalar_generic_tuple_manual.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod scalar_generic_tuple_manual;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::scalar_generic_tuple_manual`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::scalar_generic_tuple_manual`.

*   [⚫] **Increment 27:** Uncomment and Test `scalar_generic_tuple_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod scalar_generic_tuple_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::scalar_generic_tuple_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::scalar_generic_tuple_only_test`.

*   [⚫] **Increment 28:** Uncomment and Test `standalone_constructor_args_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod standalone_constructor_args_derive;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::standalone_constructor_args_derive`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::standalone_constructor_args_derive`.

*   [⚫] **Increment 29:** Uncomment and Test `standalone_constructor_args_manual.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod standalone_constructor_args_manual;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::standalone_constructor_args_manual`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::standalone_constructor_args_manual`.

*   [⚫] **Increment 30:** Uncomment and Test `standalone_constructor_args_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod standalone_constructor_args_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::standalone_constructor_args_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::standalone_constructor_args_only_test`.

*   [⚫] **Increment 31:** Uncomment and Test `standalone_constructor_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod standalone_constructor_derive;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::standalone_constructor_derive`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::standalone_constructor_derive`.

*   [⚫] **Increment 32:** Uncomment and Test `standalone_constructor_manual.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod standalone_constructor_manual;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::standalone_constructor_manual`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::standalone_constructor_manual`.

*   [⚫] **Increment 33:** Uncomment and Test `standalone_constructor_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod standalone_constructor_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::standalone_constructor_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::standalone_constructor_only_test`.

*   [⚫] **Increment 34:** Uncomment and Test `unit_variant_derive.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod unit_variant_derive;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::unit_variant_derive`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::unit_variant_derive`.

*   [⚫] **Increment 35:** Uncomment and Test `unit_variant_manual.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod unit_variant_manual;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::unit_variant_manual`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::unit_variant_manual`.

*   [⚫] **Increment 36:** Uncomment and Test `unit_variant_only_test.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod unit_variant_only_test;`.
    *   **Step 2:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::unit_variant_only_test`.
    *   **Step 3:** Analyze results. Propose fixes if needed.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::unit_variant_only_test`.

*   [⚫] **Increment 37:** Uncomment and Test `usecase1.rs`
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod usecase1;`.
    *   **Step 2:** Read `module/core/former/tests/inc/former_enum_tests/usecase1.rs`. Uncomment file content if necessary. Address the `// qqq : xxx : uncomment and make it working` task. Propose necessary code changes and remove the task comment.
    *   **Step 3:** Request user to run `cargo check --tests --package former` and `cargo test --package former -- --test former_enum_tests::usecase1`.
    *   **Step 4:** Analyze results. This test uses default subformer generation for enum variants holding structs that also derive `Former`. Investigate `former_meta/src/derive_former/former_enum.rs` (likely `tuple_non_zero.rs` or `struct_non_zero.rs`) if issues arise. Propose fixes.
    *   **Verification:** Successful compilation and passing tests for `former_enum_tests::usecase1`.

*   [⚫] **Increment 38:** Address `subform_collection_test.rs` (Known Compile Fail)
    *   **Step 1:** Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod subform_collection_test;`.
    *   **Step 2:** Read `module/core/former/tests/inc/former_enum_tests/subform_collection_test.rs`. Uncomment file content if necessary. Address the `// qqq : xxx : make it working` task.
    *   **Step 3:** **Confirm with the user if this feature (`#[subform_entry]` for `Vec<Enum>`) should be implemented.** The comments indicate this test is expected to fail compilation because this is not currently supported.
        *   **If YES:** This is a significant feature addition. Propose a sub-plan to implement the necessary logic in `former_meta/src/derive_former/field.rs` (specifically `subform_entry_setter`) to handle enum variants. This involves generating code that can somehow select and start the correct former for a *specific enum variant* within the collection context. This is non-trivial.
        *   **If NO:** Modify the test file. Remove the test code and the file, adding a comment in `mod.rs` explaining the limitation, or comment out the test function with an explanation.
    *   **Step 4:** Apply the chosen solution (implement feature or modify/remove test).
    *   **Step 5:** Request user to run `cargo check --tests --package former`. If the feature was implemented, also run `cargo test --package former -- --test former_enum_tests::subform_collection_test`.
    *   **Verification:** If implemented: Successful compilation and passing test. If removed/commented: Successful compilation and no test failures related to this file.

*   [⚫] **Increment 39:** Final Verification
    *   **Step 1:** Ensure all non-component enum test modules (`mod <test_file>;`) are uncommented in `module/core/former/tests/inc/mod.rs` (except potentially `subform_collection_test` if removed).
    *   **Step 2:** Request user to run `cargo check --tests --package former --all-targets`.
    *   **Step 3:** Request user to run `cargo clippy --package former --all-targets -- -D warnings`.
    *   **Step 4:** Request user to run `cargo test --package former --all-targets`.
    *   **Verification:** Analyze output from user. Expect zero errors and zero warnings from `check` and `clippy`. Expect all tests for the `former` package to pass, paying close attention to the `former_enum_tests` results.

## Notes & Insights

*   *(This section must always be present and preserved)*
*   **[Date/Inc #] Insight:** The `components_tests` module and its contents will be ignored as the component model is being removed per the other plan (`plan.md`).
*   **[Date/Inc #] Insight:** The task for `parametrized_dyn_manual.rs` (struct test) is removed from this plan's scope. It should be handled by `plan_dyn_trait_issue.md`.
*   **[Date/Inc #] Insight:** Several enum tests were initially commented out, suggesting potentially incomplete features or larger refactoring needs, especially around generics and subforms for enums. This plan addresses them incrementally.
*   **[Date/Inc #] Insight:** `subform_collection_test.rs` is known to fail compilation and requires a user decision on whether to implement the underlying feature (`#[subform_entry]` for `Vec<Enum>`).
